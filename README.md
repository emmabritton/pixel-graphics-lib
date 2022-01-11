# Graphics Lib

This is a simple wrapper around [Pixels](https://github.com/parasyte/pixels), providing basic shape drawing, bitmap text and image rendering.

## Usage

### Cargo

In your `Cargo.toml` file add
```toml
pixels-graphics-lib = "0.2.0"
winit = "0.25"
winit_input_helper = "0.10"
```

### Code

This bit of boilerplate/framework must be used inside your code to use this library:
```rust
let event_loop = EventLoop::new();
let mut input = WinitInputHelper::new();
let (mut window, mut graphics) = setup(240, 160, "Example", true, &event_loop)?;

event_loop.run(move |event, _, control_flow| {
    if let Event::RedrawRequested(_) = event {
        if graphics.pixels
        .render()
        .map_err( | e | eprintln ! ("pixels.render() failed: {:?}", e))
        .is_err()
        {
            *control_flow = ControlFlow::Exit;
            return;
        }

        //put your rendering code here
    }

    //put your update code here
    
    if input.update( & event) {
        if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
            *control_flow = ControlFlow::Exit;
            return;
        }
        
        if let Some(size) = input.window_resized() {
            graphics.pixels.resize_surface(size.width, size.height);
        }

        //put your input handling code here

        window.request_redraw();
    }
});
```

Drawing is then quite simple:
```rust
graphics.draw_text("Some text", 4, 1, 1, TextSize::Normal, BLACK);
graphics.draw_image(20, 20, &image);
graphics.draw_rect(1, 1, 100, 100, GREY);
```

## Features

Both features are enabled by default

### `image_loading`

Load files as `Image`s

#### Code
```rust
let image = load_image("resources/example.png")?;
graphics.draw_image(40,20,&image);
```

### `window_prefs`

Save and restore window position and size

#### Code
First an instance of `WindowPreferences` has be created:
```rust
let mut prefs = WindowPreferences::new(
    "<qualifier>",
    "<org name>",
    "<program name>",
)?;
``` 

Then the file has to be created/loaded with 
`prefs.load()?;`

To set the window size and position call
`prefs.restore(&mut window);`

To store the window size and position call
`prefs.store(&window);`

This only saves the data to memory, to save to disk call
`prefs.save()?;` (after `store()`)
