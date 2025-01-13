# Limnus Loader 🧱

Welcome to **Limnus Loader**! This tiny but mighty library lets you load files as `Blob`s and send them wherever they're
needed, all in async style! Just think of it as a special delivery service for your asset files, 
from storage to wherever you need them in your app.

## ✨ Features

- Reads files asynchronously with any `AssetReader` you provide.
- Wraps the file path and content in a handy `Blob`.
- Sends the `Blob` through a channel so other parts of your app can use it.

## 📦 Installation

To use `limnus-loader`, add it to your `Cargo.toml`:

```toml
[dependencies]
limnus-loader = "0.0.15"
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contents credits

- [Kenny.nl](https://kenney.nl/assets/platformer-art-deluxe) "platform art deluxe"
