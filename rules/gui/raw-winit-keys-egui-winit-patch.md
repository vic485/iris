# iris-gui keys come from a raw winit tap in a patched egui-winit

Status: fixed (2026-10). iris-gui used to take keyboard input from egui's
`Event::Key` and map `egui::Key` back to a winit `KeyCode` (`map_key`). That
lost every key `egui::Key` can't name:

- JIS ろ (`IntlRo`), ¥ (`IntlYen`), 変換/無変換/かな. Shift+ろ (`_`) typed nothing.
- The numpad, which egui folds into the main row (`Num0-9`, `Plus`, ...).
- NumLock, ScrollLock, CapsLock, PrintScreen, ContextMenu.

It also needed workarounds: egui-winit eats Ctrl+C/X/V into `Event::Copy/Cut/
Paste` (re-synthesised in `pump`), and the logical-key fallback leaked the host
layout. eframe 0.36 has no raw `WindowEvent` hook for the app.

**Fix:** `third_party/egui-winit-0.36.2` (applied by `[patch.crates-io]`) adds
`src/raw_keys.rs`. At the top of `State::on_window_event`'s `KeyboardInput` arm,
before the IME filter and key translation, it pushes `{ physical KeyCode,
pressed, repeat }` to a global queue (capped at 512, oldest dropped). Synthetic
presses (focus-in) are skipped, as egui-winit does; synthetic releases pass.
`input::pump` drains it every frame:

- Not captured: discard. `engage_capture` also drains, so keys typed into the
  side panel never reach the guest.
- Captured: forward each `KeyCode` to `push_kb` in order, so Shift+ろ arrives as
  Shift down, ろ down, not reordered against modifiers. F11 is still skipped
  unless Ctrl+Alt (GUI fullscreen toggle). The release chord and
  Ctrl+Alt+Esc are unchanged.

This makes iris-gui send exactly what the standalone window (`src/ui.rs`,
`KeyEvent::physical_key`) sends. Causes 2 and 3 and "What is NOT fixable" in
[keyboard-capture-egui-steals-tab-arrows-esc-and-ctrl-cxv.md](keyboard-capture-egui-steals-tab-arrows-esc-and-ctrl-cxv.md)
are superseded. The focus-lock filter (cause 1) is still needed: it keeps egui
from moving focus off the framebuffer when Tab or the arrows are pressed.

**Bumping egui:** the vendored egui-winit version must equal what eframe
requires, or the patch silently doesn't apply. Re-copy the new release from the
cargo registry and re-apply the patch (grep "IRIS patch"). Check: the
`egui-winit` block in `Cargo.lock` has **no `source` line**.
