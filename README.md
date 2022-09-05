# Graphics Lib

This is a simple wrapper around [Pixels](https://github.com/parasyte/pixels), designed to be used with [Buffer Graphlics Lib](https://github.com/raybritton/buffer-graphics-lib)

## Usage

### Cargo

In your `Cargo.toml` file add
```toml
pixels-graphics-lib = "0.5.2"
winit = "0.27.2"
winit_input_helper = "0.13.0"
```

### Code

This bit of boilerplate/framework must be used inside your code to use this library:
```rust
let event_loop = EventLoop::new();
let mut input = WinitInputHelper::new();
let (mut window, mut pixels) = setup((240, 160), WindowScaling::Auto, "Example", &event_loop)?;

event_loop.run(move |event, _, control_flow| {
    if let Event::RedrawRequested(_) = event {
        //put your rendering code here
        //e.g. 
        //  let graphics = Graphics::new(pixels.get_frame(),240,160).unwrap();
        //  graphics.clear(BLACK);
        if pixels
        .render()
        .map_err( | e | eprintln ! ("pixels.render() failed: {:?}", e))
        .is_err()
        {
            *control_flow = ControlFlow::Exit;
            return;
        }
    }

    //put your update code here
    
    if input.update( & event) {
        if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
            *control_flow = ControlFlow::Exit;
            return;
        }
        
        if let Some(size) = input.window_resized() {
            pixels.resize_surface(size.width, size.height);
        }

        //put your input handling code here

        window.request_redraw();
    }
});
```

## Features

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
