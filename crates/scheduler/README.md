# Limnus Scheduler

**Limnus Scheduler** is a component of the [Limnus](https://github.com/swamp/limnus) ecosystem, designed 
to manage and execute stages composed of systems in an efficient manner.


```rust
pub trait Scheduler: Debug + 'static {
    fn schedule(&self, stages: &Stages, state: &mut State);
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
