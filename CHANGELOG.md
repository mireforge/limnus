# Changelog

## 🔖 [v0.0.14](https://github.com/swamp/limnus/releases/tag/v0.0.14) (2025-01-13)

More work on wasm

### [limnus-window](https://crates.io/crates/limnus-window)

* 🛠️ Upgrade to winit `0.30.8`

### [limnus-window-runner](https://crates.io/crates/limnus-window-runner)

* 🛠️ Upgrade to winit `0.30.8`

### [limnus-default-keys](https://crates.io/crates/limnus-default-keys)

* ✨ `alt-enter`: Fullscreen toggle. `alt-shift-enter`: Fullscreen toggle with always on top.

### [limnus-clock](https://crates.io/crates/limnus-clock)

* 🛠️ Upgrade to `monotonic-time-rs 0.0.9` to support wasm

### [limnus-system-params](https://crates.io/crates/limnus-system-params)

* 🛠️ Uses `get()` instead of `fetch()`. Params return an `Option<Item>` instead of `Item`.

### [limnus-system](https://crates.io/crates/limnus-system)

* 🛠️ Use new system-params `get()` instead of `fetch()`. A system function will only be called if all system parameters are available.

## 🔖 [v0.0.12](https://github.com/swamp/limnus/releases/tag/v0.0.12) (2024-12-03)

Improved audio config filter support. Added `audio_tester` example. Missing preludes.

## 🔖 [v0.0.11](https://github.com/swamp/limnus/releases/tag/v0.0.11) (2024-11-30)

Gamepad support

### [limnus-gamepad](https://crates.io/crates/limnus-gamepad)

* ✨ Gamepad types and message queue.

### [limnus-gamepad-gilrs](https://crates.io/crates/limnus-gamepad-gilrs)

* ✨ Gamepad support using gilrs.

### [limnus-system-params](https://crates.io/crates/limnus-system-params)

* ✨ Local Resource system parameters (`LoRe` and `LoReM`).

## 🔖 [v0.0.10](https://github.com/swamp/limnus/releases/tag/v0.0.10) (2024-11-24)

Moved from Swamp.
