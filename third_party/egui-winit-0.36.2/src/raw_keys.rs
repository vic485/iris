//! IRIS patch: a tap on the raw winit key events, before egui translates them.
//!
//! `egui::Key` can't name every key (no JIS Ro/Yen/Henkan/Muhenkan/Kana, no
//! separate numpad, no lock keys), and egui-winit turns Ctrl+C/X/V into
//! clipboard commands and drops keys the IME processed. An emulator wants the
//! physical key for all of them, in order. `State::on_window_event` records each
//! real `KeyboardInput` here; the application drains the queue once per frame.

use std::collections::VecDeque;
use std::sync::Mutex;

use winit::keyboard::KeyCode;

/// One key transition. `pressed` with `repeat` is an auto-repeat.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawKey {
    pub code: KeyCode,
    pub pressed: bool,
    pub repeat: bool,
}

/// Bounds the queue if nothing drains it (oldest dropped first).
const CAP: usize = 512;

static QUEUE: Mutex<VecDeque<RawKey>> = Mutex::new(VecDeque::new());

pub(crate) fn push(key: RawKey) {
    let mut q = QUEUE.lock().unwrap_or_else(|e| e.into_inner());
    if q.len() == CAP {
        q.pop_front();
    }
    q.push_back(key);
}

/// Take every key event recorded since the last call, oldest first. Events from
/// all viewports share one queue.
pub fn drain() -> Vec<RawKey> {
    QUEUE.lock().unwrap_or_else(|e| e.into_inner()).drain(..).collect()
}
