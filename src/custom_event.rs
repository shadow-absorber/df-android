//! Custom event type for Ruffle on Android

use crate::PlayerRunnable;

/// User-defined events.
pub enum RuffleEvent {
    /// Indicates that a task is ready to be polled.
    TaskPoll(PlayerRunnable),
    SetVirtualKeyboardVisible(bool),
    RunContextMenuCallback(usize),
    ClearContextMenu,
    RequestContextMenu,
}
