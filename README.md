# Limnus

Limnus is a foundational game engine framework designed to serve as a robust platform for building custom game engines. Rather than being a standalone engine, it provides the essential building blocks and architecture that game engine developers need.

## Overview

**Limnus** offers a modular, extensible architecture that handles core game engine concerns such as:
- Resource management
- Platform abstraction
- Message systems
- Asset handling
- Audio systems
- Input processing
- Rendering abstractions

## Related Projects

If you're looking for a complete game engine solution, consider these projects built on Limnus:

### [Swamp Engine](https://github.com/swamp/swamp)
A full-featured 2D game engine that leverages Limnus's architecture to provide an optimized development experience for 2D games.

### [Mangrove Engine](https://github.com/swamp/mangrove)
A 2D game engine with integrated scripting capabilities, built on the Limnus Engine (and in turn Limnus) framework. Ideal for developers who want the power of a custom engine with the flexibility of scripted gameplay logic.

## Architecture

**Limnus** is built with modularity in mind, allowing engine developers to:
- Pick and choose components they need
- Extend existing functionality
- Replace modules with custom implementations

## Target Audience

**Limnus** is designed for:
- Game engine developers
- Teams building custom engine solutions
- Developers who need fine-grained control over their engine architecture

## Installation

Add the following to your `Cargo.toml`:

```toml
limnus = "0.0.10"
```

## Detailed information

### Crates

#### Core Framework

| Crate | Description |
|-------|-------------|
| `app` | Application management. Entry point for registering plugins and systems |
| `boot` | Standard application bootstrapping and initialization |
| `log` | Platform-specific logging initialization for tracing |

#### Task System

| Crate | Description |
|-------|-------------|
| `system` | Task registration with automatic dependency injection |
| `system-runner` | Task execution engine with dependency ordering |
| `system-state` | Shared state and resource management for tasks |
| `message` | Inter-task communication |

#### Window Management

| Crate | Description |
|-------|-------------|
| `window` | Platform-independent window abstraction |
| `window-runner` | Window event loop and runtime |
| `screen` | Screen configuration and management |
| `wgpu-window` | WGPU-based rendering integration |

#### Resource Management

| Crate | Description |
|-------|-------------|
| `resource` | Core resource management |
| `local-resource` | Local resource handling |
| `loader` | Generic resource loading framework |
| `loader-plugin` | Extensible loader plugin system |

#### Asset System

| Crate | Description |
|-------|-------------|
| `assets` | Asset management and loading |
| `assets-loader` | Asset loading implementations |
| `asset-registry` | Asset registration and tracking |
| `asset-id` | Asset identification and referencing |

#### Input Handling

| Crate | Description |
|-------|-------------|
| `basic-input` | Keyboard and mouse input handling |
| `gamepad` | Gamepad abstraction layer |
| `gamepad-gilrs` | Gamepad implementation using gilrs |


### Audio System
| Crate | Description |
|-------|-------------|
| `audio` | Audio system management and playback |
| `audio-sample` | Audio sample loading and processing |

#### Other

| Crate | Description |
|-------|-------------|
| `macros` | macros for Resource, LocalResource, Message and Asset |

## About Contributions

This is an open source project with a single copyright holder. 
While the code is publicly available under [LICENSE](LICENSE), I am not accepting external contributions at this time. 

You are welcome to:
- Use the code according to the license terms
- Fork the project for your own use
- Report issues
- Provide feedback
- Share the project

If you have suggestions or find bugs, please feel free to open an issue for discussion. While I cannot accept pull requests, I value your feedback and engagement with the project.

Thank you for your understanding and interest in the project! 🙏

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

_Copyright (c) 2024 Peter Bjorklund. All rights reserved._
