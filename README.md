[![Crates.io](https://img.shields.io/crates/v/pixels-graphics-lib)](https://crates.io/crates/pixels-graphics-lib "Crates.io version")
[![Documentation](https://img.shields.io/docsrs/pixels-graphics-lib)](https://docs.rs/pixels-graphics-lib "Documentation")

# Pixels Graphics Lib

Pixel buffer graphics and GUI library. It helps simplify window setup and creation, and event looping.
It uses [buffer graphics lib](https://github.com/emmabritton/buffer-graphics-lib) for drawing to the buffer.

## Usage

### Cargo

In your `Cargo.toml` file add

```toml
pixels-graphics-lib = { version = "0.20.2", features = [] }
```

Inside `features` you **MUST** put one of these:

| Feature      | Renderer                                                   | Window creation                                        |
|--------------|------------------------------------------------------------|--------------------------------------------------------|
| `pixels`     | [Pixels](https://github.com/parasyte/pixels)               | [Winit](https://github.com/rust-windowing/winit) v0.29 |
| `softbuffer` | [Softbuffer](https://github.com/rust-windowing/softbuffer) | [Winit](https://github.com/rust-windowing/winit) v0.30 |

Both of these use `rwh06`

This will control how the window is created and managed and how the buffer is rendered to the screen. The main
differences are when the window is scaled to a non integer value (1.2 opposed than 2.0) then `pixels` will draw your
content in the middle of the window, whereas `softbuffer` will draw in the top left. Additionally, `pixels` uses
hardware
scaling and `softbuffer` uses software scaling.

### Code

You can use scenes using `run_scenes` (requires default feature `scenes`):

```rust
fn main() -> Result<()> {
    // Window prefs allow the size and position of the window to be saved and restored
    let window_prefs = WindowPreferences::new("com", "example", "app", 1)?;
    // Options contains scaling, UPS, etc
    let options = Options::default();
    // The switcher is how new scenes are created
    let switcher: SceneSwitcher<SceneResult, SceneName> =
        |style, scene_stack, new_scene| match new_scene {
            SceneName::Example => scene_stack.push(ExampleScene::new()),
        };
    let first_scene = ExampleScene::new();
    run_scenes(
        300,
        300,
        "Scenes Example",
        Some(window_prefs),
        switcher,
        first_scene,
        options,
        empty_pre_post()
    )?;
    Ok(())
}

// The scene name is the id used so the switcher knows which one to create
#[derive(Clone, Debug, PartialEq)]
enum SceneName {
    Example,
}

// After a scene is finished it can return values to it's parent using scene result
#[derive(Clone, Debug, PartialEq)]
enum SceneResult {}

struct ExampleScene {}

impl ExampleScene {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Scene<SceneResult, SceneName> for ExampleScene {
    fn render(
        &mut self,
        graphics: &mut Graphics,
        mouse_xy: Coord,
        held_keys: &[KeyCode]) {
        todo!()
    }

    fn update(
        &mut self,
        timing: &Timing,
        mouse_xy: Coord,
        held_keys: &[KeyCode],
        window: &Window
    ) -> SceneUpdateResult<SceneResult, SceneName> {
        todo!()
    }
}
```

or a more low level with `run`

```rust
struct Example {}

fn main() -> Result<()> {
    let system = Box::new(Example {});
    run(240, 160, "Example", Box::new(system), Options::default())?;
    Ok(())
}

//Check `src/scenes.rs` for examples of implementing held keys, etc
impl System for Example {
    fn update(&mut self, timing: &Timing, window: &Window) {}
    fn render(&mut self, graphics: &mut Graphics) {}
}
```

## Features

> Default features: `window_prefs`, `sound`, `serde`, `scenes`

### `window_prefs`

Save and restore window position and size

To use this the `impl System` must override `System::window_prefs()`

### `scenes`

Enables `Scene` and `run_scenes`

Includes `window_prefs`

### `controller`

* Adds gamepad support
* Adds gamepad state to `Scene::update`, `Scene::render`

### `controller_xinput`

As above but using xinput, windows only

### `sound`

Play music or sound effects

### `serde`

Adds `Serialize` and `Deserialize` to most structs and enums

### `images`

Loading and displaying of PNGs, JPEGs, BMPs

### `file_dialogs`

Built in file selection dialogs, not recommended, use `rfd`

### `mint`

Enables `buffer-graphics-lib/mint`,
see [Buffer graphics readme](https://github.com/emmabritton/buffer-graphics-lib?tab=readme-ov-file#features)

### `notosan`

Enables `buffer-graphics-lib/notosans`,
see [Buffer graphics readme](https://github.com/emmabritton/buffer-graphics-lib?tab=readme-ov-file#features)

### `embedded`

Enables `buffer-graphics-lib/embedded`,
see [Buffer graphics readme](https://github.com/emmabritton/buffer-graphics-lib?tab=readme-ov-file#features)

### `pixels_serde` and `softbuffer_serde`

Enables `serde` for the `winit` crate being used by `pixels` or `softbuffer`

## Examples

Each example must be run with a renderer (pixels or softbuffer), like this:

`cargo run --example basic --features "pixels"`

or

`cargo run --example relative_test --features "softbuffer"`

## Projects

### [Retro Games](https://github.com/emmabritton/retro-games)

A few retro games

### [Wordle](https://github.com/emmabritton/wordle)

A wordle clone

### [ICI Image editor](https://github.com/emmabritton/ici-image-editor)

Editor for `IndexedImage`, ICI files

### [USFX Tester](https://github.com/emmabritton/uxfs-test)

GUI for [USFX](https://github.com/tversteeg/usfx)

### [Fontpad](https://github.com/emmabritton/fontpad)

Program used to create fonts for [Buffer graphics](https://github.com/emmabritton/buffer-graphics-lib)