use crate::{EventSender, custom_event::RuffleEvent, get_jvm, java::JavaInterface};
use jni::{Env, objects::JObject};
use ruffle_core::backend::ui::{
    DialogResultFuture, FileFilter, FontDefinition, FullscreenError, LanguageIdentifier,
    MouseCursor, MultiDialogResultFuture, NullUiBackend, UiBackend,
};
use ruffle_core::font::FontQuery;
use url::Url;

/// The [`UiBackend`] for Android.
///
/// Ruffle calls `open_virtual_keyboard` / `close_virtual_keyboard` whenever an editable text
/// field gains or loses focus. Those requests are queued for the Android event loop. Clipboard
/// access uses Android's system clipboard, while unsupported operations use [`NullUiBackend`].
pub struct AndroidUiBackend {
    event_loop: EventSender,
    inner: NullUiBackend,
}

impl AndroidUiBackend {
    pub fn new(event_loop: EventSender) -> Self {
        Self {
            event_loop,
            inner: NullUiBackend::new(),
        }
    }
}

fn with_player_activity<T>(callback: impl FnOnce(&mut Env, &JObject) -> T) -> Option<T> {
    let (jvm, activity_ptr) = get_jvm();
    jvm.attach_current_thread(|env| -> jni::errors::Result<T> {
        let activity = unsafe { JObject::from_raw(env, activity_ptr) };
        Ok(callback(env, &activity))
    })
    .map_err(|error| log::error!("Unable to attach clipboard operation to the JVM: {error}"))
    .ok()
}

impl UiBackend for AndroidUiBackend {
    fn open_virtual_keyboard(&self) {
        self.event_loop
            .send(RuffleEvent::SetVirtualKeyboardVisible(true));
    }

    fn close_virtual_keyboard(&self) {
        self.event_loop
            .send(RuffleEvent::SetVirtualKeyboardVisible(false));
    }

    fn mouse_visible(&self) -> bool {
        self.inner.mouse_visible()
    }

    fn set_mouse_visible(&mut self, visible: bool) {
        self.inner.set_mouse_visible(visible)
    }

    fn set_mouse_cursor(&mut self, cursor: MouseCursor) {
        self.inner.set_mouse_cursor(cursor)
    }

    fn clipboard_content(&mut self) -> String {
        with_player_activity(JavaInterface::get_clipboard_content).unwrap_or_default()
    }

    fn set_clipboard_content(&mut self, content: String) {
        with_player_activity(|env, activity| {
            JavaInterface::set_clipboard_content(env, activity, &content)
        });
    }

    fn set_fullscreen(&mut self, is_full: bool) -> Result<(), FullscreenError> {
        self.inner.set_fullscreen(is_full)
    }

    fn display_root_movie_download_failed_message(&self, invalid_swf: bool, fetched_error: String) {
        self.inner
            .display_root_movie_download_failed_message(invalid_swf, fetched_error)
    }

    fn message(&self, message: &str) {
        self.inner.message(message)
    }

    fn language(&self) -> LanguageIdentifier {
        self.inner.language()
    }

    fn display_unsupported_video(&self, url: Url) {
        self.inner.display_unsupported_video(url)
    }

    fn load_device_font(&self, query: &FontQuery, register: &mut dyn FnMut(FontDefinition)) {
        self.inner.load_device_font(query, register)
    }

    fn sort_device_fonts(
        &self,
        query: &FontQuery,
        register: &mut dyn FnMut(FontDefinition),
    ) -> Vec<FontQuery> {
        self.inner.sort_device_fonts(query, register)
    }

    fn display_file_open_dialog(&mut self, filters: Vec<FileFilter>) -> Option<DialogResultFuture> {
        self.inner.display_file_open_dialog(filters)
    }

    fn display_file_open_dialog_multiple(
        &mut self,
        filters: Vec<FileFilter>,
    ) -> Option<MultiDialogResultFuture> {
        self.inner.display_file_open_dialog_multiple(filters)
    }

    fn display_file_save_dialog(
        &mut self,
        file_name: String,
        title: String,
    ) -> Option<DialogResultFuture> {
        self.inner.display_file_save_dialog(file_name, title)
    }

    fn close_file_dialog(&mut self) {
        self.inner.close_file_dialog()
    }
}
