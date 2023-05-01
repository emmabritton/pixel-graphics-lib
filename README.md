[![Crates.io](https://img.shields.io/crates/v/pixels-graphics-lib)](https://crates.io/crates/pixels-graphics-lib "Crates.io version")
[![Documentation](https://img.shields.io/docsrs/pixels-graphics-lib)](https://docs.rs/pixels-graphics-lib "Documentation")

# Graphics Lib

This is a simple wrapper around [Pixels](https://github.com/parasyte/pixels), designed to be used with [Buffer Graphics Lib](https://github.com/emmabritton/buffer-graphics-lib)

## Usage

### Cargo

In your `Cargo.toml` file add
```toml
pixels-graphics-lib = "0.10.7"
winit_input_helper = "0.14.1" #only needed if you're not using `run()`
```

### Code

This bit of boilerplate/framework must be used inside your code to use this library:
```rust
use pixels_graphics_lib::prelude::*;

struct Example {}

fn main() -> Result<()> {
    let system = Box::new(Example {});
    run(240, 160, "Example", Box::new(system), Options::default())?;
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

### [Retro Games](https://github.com/emmabritton/retro-games)

A few retro games

### [ICI Image editor](https://github.com/emmabritton/ici-image-editor)

Editor for `IndexedImage`

### [USFX Tester](https://github.com/emmabritton/uxfs-test)

Test GUI for [USFX](https://github.com/tversteeg/usfx)
