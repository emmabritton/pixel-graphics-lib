[![Crates.io](https://img.shields.io/crates/v/pixels-graphics-lib)](https://crates.io/crates/pixels-graphics-lib "Crates.io version")
[![Documentation](https://img.shields.io/docsrs/pixels-graphics-lib)](https://docs.rs/pixels-graphics-lib "Documentation")

# Graphics Lib

This is a simple wrapper around [Pixels](https://github.com/parasyte/pixels), designed to be used
with [Buffer Graphics Lib](https://github.com/emmabritton/buffer-graphics-lib)

## Usage

### Cargo

In your `Cargo.toml` file add

```toml
pixels-graphics-lib = "0.17.0"
winit_input_helper = "0.16.0" #only needed if you're not using `run()`
```

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
    fn update(&mut self, timing: &Timing) {}
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

Enables `graphic-shapes/mint`

## Projects

### [Retro Games](https://github.com/emmabritton/retro-games)

A few retro games

### [ICI Image editor](https://github.com/emmabritton/ici-image-editor)

Editor for `IndexedImage`

### [USFX Tester](https://github.com/emmabritton/uxfs-test)

Test GUI for [USFX](https://github.com/tversteeg/usfx)
