# limnus-wgpu-math 🧱

[![Crates.io](https://img.shields.io/crates/v/limnus-gpu-math)](https://crates.io/crates/limnus-gpu-math)
[![Documentation](https://docs.rs/limnus-gpu-math/badge.svg)](https://docs.rs/limnus-gpu-math)

limnus-wgpu-math is a Rust library that provides easy-to-use types and operations for matrix and vector math,
specifically designed for compatibility with wgpu shaders. This library allows you to work seamlessly with
transformations, projections, and other essential mathematical operations without the need for conversions, making it
ideal for game development and graphical applications.

## ✨ Features

- Matrix and Vector Types: Simple, Rust-native types like Matrix4 and Vec4 for common operations in 3D graphics.
- Orthographic Projection: Easily create orthographic projection matrices for 2D and 3D rendering.
- Basic Transformations: Includes functions for scaling, translation, and identity matrices.
- Intuitive Operations: Basic math operations (Add, Mul, Index) implemented for matrices and vectors, making them
  straightforward to use with wgpu shaders.

## 📦 Installation

Add limnus-wgpu-math to your project:

```toml
[dependencies]
limnus-wgpu-math = "0.0.16"
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
