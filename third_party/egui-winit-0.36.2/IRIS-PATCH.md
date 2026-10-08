# egui-winit 0.36.2, patched for IRIS

Unmodified crates.io `egui-winit` 0.36.2 (MIT OR Apache-2.0, by the egui
authors) plus one change: `src/raw_keys.rs` and a few lines in
`State::on_window_event` that record every real `KeyboardInput` (physical
`KeyCode`, pressed, repeat) before egui translates it. iris-gui drains that queue
to feed the guest's keyboard. Applied by `[patch.crates-io]` in the workspace
`Cargo.toml`. See `rules/gui/raw-winit-keys-egui-winit-patch.md`.

The version must match the `egui-winit` that eframe asks for. When bumping egui,
re-copy the new release from `~/.cargo/registry/src/*/egui-winit-<ver>/` and
re-apply the patch (grep for "IRIS patch").
