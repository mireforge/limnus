 ## Build for Steam Deck

 target: `x86_64-unknown-linux-gnu`

### Cross compile

#### Setup

```shell
cargo install cross --git https://github.com/cross-rs/cross
```

```shell
cross build --target x86_64-unknown-linux-gnu --release
```
