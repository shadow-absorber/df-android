use crate::{custom_event::RuffleEvent, EventSender};
use ruffle_core::backend::ui::{
    DialogResultFuture, FileFilter, FontDefinition, FullscreenError, LanguageIdentifier,
    MouseCursor, MultiDialogResultFuture, NullUiBackend, UiBackend,
};
use ruffle_core::font::FontQuery;
use url::Url;

/// The [`UiBackend`] for Android.
///
/// Ruffle calls `open_virtual_keyboard` / `close_virtual_keyboard` whenever an editable text
/// field gains or loses focus. Those requests are queued for the Android event loop, while every
/// other backend operation is delegated to [`NullUiBackend`] until a need arises.
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
        self.inner.clipboard_content()
    }

    fn set_clipboard_content(&mut self, content: String) {
        self.inner.set_clipboard_content(content)
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
