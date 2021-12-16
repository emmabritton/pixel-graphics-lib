# Graphics Lib

This is a simple wrapper around [Pixels](https://github.com/parasyte/pixels), providing basic shape drawing, bitmap text and image rendering.

## Usage

### Cargo

In your `Cargo.toml` file add
```toml
rust_graphics_lib = { git = "https://github.com/raybritton/rust-graphics-lib"}
winit = "0.25"
winit_input_helper = "0.10"
```

### Code

This bit of boilerplate/framework must be used inside your code to use this library:
```rust
let event_loop = EventLoop::new();
let mut input = WinitInputHelper::new();
let (mut window, pixels) = setup(240, 160, "Example", true, &event_loop)?;

let mut graphics = PixelWrapper::new(pixels, 240);
graphics.init();

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
    }
    
    //put your rendering code here 
    
    if input.update( & event) {
        if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
            *control_flow = ControlFlow::Exit;
            return;
        }
        
        if let Some(size) = input.window_resized() {
            graphics.pixels.resize_surface(size.width, size.height);
        }

        //put your update/input handling code here

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
