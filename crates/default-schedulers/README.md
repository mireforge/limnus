# Limnus Default Schedulers

Schedulers are essential components in any system that require tasks to be executed
either periodically or based on specific triggers. Limnus offers a set of default 
schedulers that cater to a variety of use cases, ensuring that your tasks run smoothly and efficiently.

Schedulers runs a **Stage** feeding them the current SystemState.

* **MainScheduler**. Runs as often as possible. Only constrained by the hardware.
* **FixedScheduler**. Runs at a fixed tick rate. Default is 16 ms tick time.
* **RenderScheduler**. Currently runs as often as possible. Will be able to set certain FPS limits in the future.

## Schedulers

### MainScheduler

**Description:**  
The `MainScheduler` is designed to run tasks as often as possible, limited only by the underlying hardware 
capabilities. It is ideal for tasks that need to be executed continuously without any fixed timing constraints.
In reality, it is often capped by the rendering such as vsync, depending on the system.

### FixedScheduler

**Description:**  

The `FixedScheduler` operates at a fixed tick rate, providing consistent and predictable task execution 
intervals. By default, it runs with a 16 ms tick time, but this can be customized to suit your 
application's requirements.


### RenderScheduler

**Description:**

Currently, the RenderScheduler runs as often as possible, similar to the MainScheduler. However, future 
updates will allow you to set specific FPS (Frames Per Second) limits, providing greater control over 
rendering performance and resource usage.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
