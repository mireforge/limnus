# limnus-wgpu-window 🧱

[![Crates.io](https://img.shields.io/crates/v/limnus-wgpu-window)](https://crates.io/crates/limnus-wgpu-window)
[![Documentation](https://docs.rs/limnus-wgpu-window/badge.svg)](https://docs.rs/limnus-wgpu-window)

Limnus WGPU Window is a Rust crate that provides a simple and efficient abstraction for creating 
and managing a window with rendering surfaces using winit and wgpu. It streamlines the setup
of the wgpu device, queue, and surface, handling window resizing and rendering, making it easier 
to build high-performance graphics applications.

## ✨ Features

- Easy Window and Surface Management: Simplifies the creation and configuration of windows and wgpu surfaces using winit.
- Device and Queue Setup: Automatically initializes the wgpu device and queue with sensible defaults.
- Responsive Resizing: Handles window resizing events and reconfigures the rendering surface accordingly.
- Flexible Rendering Pipeline: Provides a render function callback to integrate custom rendering logic seamlessly.
- Cross-Platform Support: Compatible with major operating systems supported by winit and wgpu.

## 📦 Installation

Add limnus-wgpu-window to your project’s Cargo.toml:

```toml
[dependencies]
limnus-wgpu-window = "0.0.16"
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
