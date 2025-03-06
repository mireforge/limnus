# Limnus Default Stages Plugin

## ✨ Features

The DefaultStagePlugin adds the following stages to your Limnus application:

### Main

- **First**: Initial setup tasks.
- **PreUpdate**: Tasks to run before the main update loop.
- **Update**: Update loop tasks.
- **PostUpdate**: Tasks to run after the main update loop.

### Fixed Rate

- **FixedFirst**: Fixed-rate initial setup tasks.
- **FixedPreUpdate**: Fixed-rate tasks before update.
- **FixedUpdate**: Fixed-rate update tasks.
- **FixedPostUpdate**: Fixed-rate tasks after update.

### Render

- **RenderFirst**: Rendering setup tasks.
- **RenderPreUpdate**: Rendering tasks before update.
- **RenderUpdate**: Rendering tasks.
- **RenderPostUpdate**: Rendering tasks after update.

## 📦 Installation

Add limnus-default-stages-plugin to your project’s Cargo.toml:

```toml
[dependencies]
limnus-default-stages-plugin = "0.0.17"
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
