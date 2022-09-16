# Graphics Lib

This is a simple wrapper around [Pixels](https://github.com/parasyte/pixels), designed to be used with [Buffer Graphlics Lib](https://github.com/raybritton/buffer-graphics-lib)

## Usage

### Cargo

In your `Cargo.toml` file add
```toml
graphics-shapes = "0.1.3"
buffer-graphics-lib = "0.7.0"
pixels-graphics-lib = "0.6.3"
winit = "0.27.2"
winit_input_helper = "0.13.0" #only needed if you're not using `run()`
```

### Code

This bit of boilerplate/framework must be used inside your code to use this library:
```rust
struct Example {}

fn main() -> Result<()> {
    let system = Box::new(Example::new());
    run(240, 160, WindowScaling::Auto, "Example", system)?;
    Ok(())
}

impl System for Example {
    fn render(&self, graphics: &mut Graphics) {}
    fn update(&mut self, delta: f32) {}
}
```

## Features

### `window_prefs`

Save and restore window position and size
