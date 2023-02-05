# Graphics Lib

This is a simple wrapper around [Pixels](https://github.com/parasyte/pixels), designed to be used with [Buffer Graphlics Lib](https://github.com/raybritton/buffer-graphics-lib)

## Usage

### Cargo

In your `Cargo.toml` file add
```toml
pixels-graphics-lib = "0.8.3"
winit_input_helper = "0.13.0" #only needed if you're not using `run()`
```

### Code

This bit of boilerplate/framework must be used inside your code to use this library:
```rust
use pixels_graphics_lib::prelude::*;

struct Example {}

fn main() -> Result<()> {
    let system = Box::new(Example {});
    run(240, 160, WindowScaling::Auto, "Example", system, ExecutionSpeed::standard())?;
    Ok(())
}

impl System for Example {
    fn update(&mut self, timing: &Timing) {}
    fn render(&self, graphics: &mut Graphics) {}
}
```

## Features

### `window_prefs`

Save and restore window position and size

To use this the `impl System` must override `System::window_prefs()`

## Projects

### Retro Games

A few retro games

[Repo](https://github.com/emmabritton/retro-games)

### ICI Image editor

Editor for `IndexedImage`

[Repo](https://github.com/emmabritton/ici-image-editor)

### USFX Tester

Test GUI for [USFX](https://github.com/tversteeg/usfx)

[Repo](https://github.com/emmabritton/uxfs-test)