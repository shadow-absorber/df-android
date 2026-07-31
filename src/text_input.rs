#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeTextControl {
    MoveLeft,
    MoveRight,
    SelectLeft,
    SelectRight,
    Backspace,
    Delete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextInputAction {
    Preedit(String, Option<(usize, usize)>),
    Commit(String),
    Control {
        code: NativeTextControl,
        repeat: usize,
    },
}

#[derive(Debug, Clone)]
pub struct TextInputSnapshot {
    text: String,
    selection: TextRange,
    composition: Option<TextRange>,
}

impl TextInputSnapshot {
    pub fn from_utf16(
        text: String,
        selection: (usize, usize),
        composition: Option<(usize, usize)>,
    ) -> Self {
        let selection = TextRange::new(
            utf16_to_byte_index(&text, selection.0),
            utf16_to_byte_index(&text, selection.1),
        );
        let composition = composition
            .map(|range| {
                TextRange::new(
                    utf16_to_byte_index(&text, range.0),
                    utf16_to_byte_index(&text, range.1),
                )
                .ordered()
            })
            .filter(|range| !range.is_caret());

        Self {
            text,
            selection,
            composition,
        }
    }
}

#[derive(Debug, Default)]
pub struct TextInputStateMachine {
    committed: CommittedState,
    composition_start: Option<usize>,
}

impl TextInputStateMachine {
    pub fn reset(&mut self, snapshot: TextInputSnapshot) {
        let parsed = ParsedInput::from(snapshot);
        self.committed = parsed.committed;
        self.composition_start = parsed.composition.map(|composition| composition.insertion);
    }

    pub fn handle(&mut self, snapshot: TextInputSnapshot) -> Vec<TextInputAction> {
        let parsed = ParsedInput::from(snapshot);
        let edit = find_edit(&self.committed, &parsed.committed);
        let current_composition_start = parsed
            .composition
            .as_ref()
            .map(|composition| composition.insertion);
        let clear_composition = self.composition_start.is_some()
            && (edit.is_some() || self.composition_start != current_composition_start);
        let mut actions = Vec::new();
        let mut resulting_selection = self.committed.selection;

        if clear_composition {
            actions.push(TextInputAction::Preedit(String::new(), None));
            resulting_selection = TextRange::caret(self.composition_start.unwrap_or_default());
        }

        if let Some(edit) = edit {
            resulting_selection = apply_edit(
                &mut actions,
                &self.committed.text,
                resulting_selection,
                edit,
            );
        }

        if let Some(composition) = &parsed.composition {
            if clear_composition || self.composition_start.is_none() {
                append_navigation(
                    &mut actions,
                    &parsed.committed.text,
                    resulting_selection,
                    TextRange::caret(composition.insertion),
                );
            }
            actions.push(TextInputAction::Preedit(
                composition.text.clone(),
                Some((composition.cursor.anchor, composition.cursor.cursor)),
            ));
        } else {
            append_navigation(
                &mut actions,
                &parsed.committed.text,
                resulting_selection,
                parsed.committed.selection,
            );
        }

        self.committed = parsed.committed;
        self.composition_start = current_composition_start;
        actions
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct TextRange {
    anchor: usize,
    cursor: usize,
}

impl TextRange {
    fn new(anchor: usize, cursor: usize) -> Self {
        Self { anchor, cursor }
    }

    fn caret(position: usize) -> Self {
        Self::new(position, position)
    }

    fn ordered(self) -> Self {
        Self::new(self.start(), self.end())
    }

    fn start(self) -> usize {
        self.anchor.min(self.cursor)
    }

    fn end(self) -> usize {
        self.anchor.max(self.cursor)
    }

    fn is_caret(self) -> bool {
        self.anchor == self.cursor
    }
}

#[derive(Debug, Default)]
struct CommittedState {
    text: String,
    selection: TextRange,
}

struct ParsedInput {
    committed: CommittedState,
    composition: Option<Composition>,
}

struct Composition {
    text: String,
    cursor: TextRange,
    insertion: usize,
}

impl From<TextInputSnapshot> for ParsedInput {
    fn from(snapshot: TextInputSnapshot) -> Self {
        let Some(composition_range) = snapshot.composition else {
            return Self {
                committed: CommittedState {
                    text: snapshot.text,
                    selection: snapshot.selection,
                },
                composition: None,
            };
        };

        let composition_text =
            snapshot.text[composition_range.start()..composition_range.end()].to_string();
        let cursor = TextRange::new(
            snapshot
                .selection
                .anchor
                .clamp(composition_range.start(), composition_range.end())
                - composition_range.start(),
            snapshot
                .selection
                .cursor
                .clamp(composition_range.start(), composition_range.end())
                - composition_range.start(),
        );
        let mut committed_text = String::with_capacity(
            snapshot.text.len() - (composition_range.end() - composition_range.start()),
        );
        committed_text.push_str(&snapshot.text[..composition_range.start()]);
        committed_text.push_str(&snapshot.text[composition_range.end()..]);
        let committed_selection = TextRange::new(
            without_composition(snapshot.selection.anchor, composition_range),
            without_composition(snapshot.selection.cursor, composition_range),
        );

        Self {
            committed: CommittedState {
                text: committed_text,
                selection: committed_selection,
            },
            composition: Some(Composition {
                text: composition_text,
                cursor,
                insertion: composition_range.start(),
            }),
        }
    }
}

struct TextEdit {
    removed: TextRange,
    inserted: String,
}

fn find_edit(previous: &CommittedState, current: &CommittedState) -> Option<TextEdit> {
    if previous.text == current.text {
        return None;
    }

    let selection = previous.selection.ordered();
    if !selection.is_caret()
        && current.text.len() >= previous.text.len() - (selection.end() - selection.start())
        && current
            .text
            .starts_with(&previous.text[..selection.start()])
        && current.text.ends_with(&previous.text[selection.end()..])
    {
        let inserted_end = current.text.len() - (previous.text.len() - selection.end());
        return Some(TextEdit {
            removed: selection,
            inserted: current.text[selection.start()..inserted_end].to_string(),
        });
    }

    let prefix = common_prefix_len(&previous.text, &current.text);
    let suffix = common_suffix_len(&previous.text[prefix..], &current.text[prefix..]);
    Some(TextEdit {
        removed: TextRange::new(prefix, previous.text.len() - suffix),
        inserted: current.text[prefix..current.text.len() - suffix].to_string(),
    })
}

fn apply_edit(
    actions: &mut Vec<TextInputAction>,
    previous_text: &str,
    mut selection: TextRange,
    edit: TextEdit,
) -> TextRange {
    if !edit.removed.is_caret() {
        let removed_characters = previous_text[edit.removed.start()..edit.removed.end()]
            .chars()
            .count();
        if selection.ordered() == edit.removed && !selection.is_caret() {
            push_control(actions, NativeTextControl::Backspace, 1);
        } else if selection.is_caret() && selection.cursor == edit.removed.end() {
            push_control(actions, NativeTextControl::Backspace, removed_characters);
        } else if selection.is_caret() && selection.cursor == edit.removed.start() {
            push_control(actions, NativeTextControl::Delete, removed_characters);
        } else {
            append_navigation(actions, previous_text, selection, edit.removed);
            push_control(actions, NativeTextControl::Backspace, 1);
        }
        selection = TextRange::caret(edit.removed.start());
    } else if selection != TextRange::caret(edit.removed.start()) {
        append_navigation(
            actions,
            previous_text,
            selection,
            TextRange::caret(edit.removed.start()),
        );
        selection = TextRange::caret(edit.removed.start());
    }

    if !edit.inserted.is_empty() {
        let insertion_end = edit.removed.start() + edit.inserted.len();
        actions.push(TextInputAction::Commit(edit.inserted));
        selection = TextRange::caret(insertion_end);
    }

    selection
}

fn append_navigation(
    actions: &mut Vec<TextInputAction>,
    text: &str,
    from: TextRange,
    to: TextRange,
) {
    if from == to {
        return;
    }

    let mut position = from.cursor;
    if !from.is_caret() {
        if to.is_caret() && to.cursor == from.start() {
            push_control(actions, NativeTextControl::MoveLeft, 1);
            return;
        }
        if to.is_caret() && to.cursor == from.end() {
            push_control(actions, NativeTextControl::MoveRight, 1);
            return;
        }
        push_control(actions, NativeTextControl::MoveLeft, 1);
        position = from.start();
    }

    append_directional_navigation(actions, text, position, to.anchor, false);
    append_directional_navigation(actions, text, to.anchor, to.cursor, true);
}

fn append_directional_navigation(
    actions: &mut Vec<TextInputAction>,
    text: &str,
    from: usize,
    to: usize,
    select: bool,
) {
    if from == to {
        return;
    }

    let (code, range) = if from > to {
        (
            if select {
                NativeTextControl::SelectLeft
            } else {
                NativeTextControl::MoveLeft
            },
            to..from,
        )
    } else {
        (
            if select {
                NativeTextControl::SelectRight
            } else {
                NativeTextControl::MoveRight
            },
            from..to,
        )
    };
    push_control(actions, code, text[range].chars().count());
}

fn push_control(actions: &mut Vec<TextInputAction>, code: NativeTextControl, repeat: usize) {
    if repeat == 0 {
        return;
    }

    if let Some(TextInputAction::Control {
        code: previous_code,
        repeat: previous_repeat,
    }) = actions.last_mut()
        && *previous_code == code
    {
        *previous_repeat += repeat;
    } else {
        actions.push(TextInputAction::Control { code, repeat });
    }
}

fn without_composition(position: usize, composition: TextRange) -> usize {
    if position <= composition.start() {
        position
    } else if position >= composition.end() {
        position - (composition.end() - composition.start())
    } else {
        composition.start()
    }
}

fn common_prefix_len(left: &str, right: &str) -> usize {
    left.chars()
        .zip(right.chars())
        .take_while(|(left, right)| left == right)
        .map(|(character, _)| character.len_utf8())
        .sum()
}

fn common_suffix_len(left: &str, right: &str) -> usize {
    left.chars()
        .rev()
        .zip(right.chars().rev())
        .take_while(|(left, right)| left == right)
        .map(|(character, _)| character.len_utf8())
        .sum()
}

fn utf16_to_byte_index(text: &str, utf16_index: usize) -> usize {
    let mut utf16_position = 0;
    for (byte_index, character) in text.char_indices() {
        if utf16_index <= utf16_position {
            return byte_index;
        }
        let next_position = utf16_position + character.len_utf16();
        if utf16_index < next_position {
            return byte_index;
        }
        utf16_position = next_position;
    }
    text.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(
        text: &str,
        selection: (usize, usize),
        composition: Option<(usize, usize)>,
    ) -> TextInputSnapshot {
        TextInputSnapshot::from_utf16(text.to_string(), selection, composition)
    }

    fn control(code: NativeTextControl, repeat: usize) -> TextInputAction {
        TextInputAction::Control { code, repeat }
    }

    #[test]
    fn commits_only_the_added_text_from_full_buffer_snapshots() {
        let mut state = TextInputStateMachine::default();

        assert_eq!(
            state.handle(snapshot("h", (1, 1), None)),
            vec![TextInputAction::Commit("h".to_string())]
        );
        assert_eq!(
            state.handle(snapshot("hello", (5, 5), None)),
            vec![TextInputAction::Commit("ello".to_string())]
        );
    }

    #[test]
    fn updates_and_commits_composition() {
        let mut state = TextInputStateMachine::default();

        assert_eq!(
            state.handle(snapshot("n", (1, 1), Some((0, 1)))),
            vec![TextInputAction::Preedit("n".to_string(), Some((1, 1)))]
        );
        assert_eq!(
            state.handle(snapshot("に", (1, 1), Some((0, 1)))),
            vec![TextInputAction::Preedit("に".to_string(), Some((3, 3)))]
        );
        assert_eq!(
            state.handle(snapshot("に", (1, 1), None)),
            vec![
                TextInputAction::Preedit(String::new(), None),
                TextInputAction::Commit("に".to_string()),
            ]
        );
    }

    #[test]
    fn clears_cancelled_composition_without_committing() {
        let mut state = TextInputStateMachine::default();
        state.handle(snapshot("draft", (5, 5), Some((0, 5))));

        assert_eq!(
            state.handle(snapshot("", (0, 0), None)),
            vec![TextInputAction::Preedit(String::new(), None)]
        );
    }

    #[test]
    fn commits_completed_prefix_before_starting_next_composition() {
        let mut state = TextInputStateMachine::default();
        state.handle(snapshot("nihon", (5, 5), Some((0, 5))));

        assert_eq!(
            state.handle(snapshot("日本g", (3, 3), Some((2, 3)))),
            vec![
                TextInputAction::Preedit(String::new(), None),
                TextInputAction::Commit("日本".to_string()),
                TextInputAction::Preedit("g".to_string(), Some((1, 1))),
            ]
        );
    }

    #[test]
    fn emits_backspace_for_text_removed_before_the_caret() {
        let mut state = TextInputStateMachine::default();
        state.handle(snapshot("hello", (5, 5), None));

        assert_eq!(
            state.handle(snapshot("hel", (3, 3), None)),
            vec![control(NativeTextControl::Backspace, 2)]
        );
    }

    #[test]
    fn emits_delete_for_text_removed_after_the_caret() {
        let mut state = TextInputStateMachine::default();
        state.handle(snapshot("hello", (2, 2), None));

        assert_eq!(
            state.handle(snapshot("heo", (2, 2), None)),
            vec![control(NativeTextControl::Delete, 2)]
        );
    }

    #[test]
    fn replaces_the_previous_selection() {
        let mut state = TextInputStateMachine::default();
        state.handle(snapshot("hello", (1, 4), None));

        assert_eq!(
            state.handle(snapshot("hio", (2, 2), None)),
            vec![
                control(NativeTextControl::Backspace, 1),
                TextInputAction::Commit("i".to_string()),
            ]
        );
    }

    #[test]
    fn commits_native_paste_as_one_chunk() {
        let mut state = TextInputStateMachine::default();
        state.handle(snapshot("go", (2, 2), None));

        assert_eq!(
            state.handle(snapshot("go🦀 now", (8, 8), None)),
            vec![TextInputAction::Commit("🦀 now".to_string())]
        );
    }

    #[test]
    fn converts_android_utf16_spans_to_utf8_preedit_offsets() {
        let mut state = TextInputStateMachine::default();
        state.handle(snapshot("ab", (1, 1), None));

        assert_eq!(
            state.handle(snapshot("a🦀b", (3, 3), Some((1, 3)))),
            vec![TextInputAction::Preedit("🦀".to_string(), Some((4, 4)))]
        );
    }

    #[test]
    fn moves_and_extends_the_selection() {
        let mut state = TextInputStateMachine::default();
        state.handle(snapshot("hello", (5, 5), None));

        assert_eq!(
            state.handle(snapshot("hello", (3, 3), None)),
            vec![control(NativeTextControl::MoveLeft, 2)]
        );
        assert_eq!(
            state.handle(snapshot("hello", (3, 5), None)),
            vec![control(NativeTextControl::SelectRight, 2)]
        );
    }

    #[test]
    fn resetting_a_guard_buffer_allows_repeated_backspace() {
        let mut state = TextInputStateMachine::default();
        let guard = "\u{200b}\u{2060}";

        for _ in 0..2 {
            state.reset(snapshot(guard, (1, 1), None));
            assert_eq!(
                state.handle(snapshot("\u{2060}", (0, 0), None)),
                vec![control(NativeTextControl::Backspace, 1)]
            );
        }
    }

    #[test]
    fn guard_buffer_supports_forward_delete() {
        let mut state = TextInputStateMachine::default();
        state.reset(snapshot("\u{200b}\u{2060}", (1, 1), None));

        assert_eq!(
            state.handle(snapshot("\u{200b}", (1, 1), None)),
            vec![control(NativeTextControl::Delete, 1)]
        );
    }

    #[test]
    fn guard_characters_are_not_committed_with_direct_input() {
        let mut state = TextInputStateMachine::default();
        state.reset(snapshot("\u{200b}\u{2060}", (1, 1), None));

        assert_eq!(
            state.handle(snapshot("\u{200b}paste\u{2060}", (6, 6), None)),
            vec![TextInputAction::Commit("paste".to_string())]
        );
    }

    #[test]
    fn guard_characters_are_not_included_in_composition() {
        let mut state = TextInputStateMachine::default();
        state.reset(snapshot("\u{200b}\u{2060}", (1, 1), None));

        assert_eq!(
            state.handle(snapshot("\u{200b}に\u{2060}", (2, 2), Some((1, 2)))),
            vec![TextInputAction::Preedit("に".to_string(), Some((3, 3)))]
        );
    }

    #[test]
    fn safely_clamps_reversed_and_out_of_bounds_composition_spans() {
        let mut state = TextInputStateMachine::default();

        assert_eq!(
            state.handle(snapshot("abc", (99, 99), Some((99, 1)))),
            vec![
                TextInputAction::Commit("a".to_string()),
                TextInputAction::Preedit("bc".to_string(), Some((2, 2))),
            ]
        );
    }
}
