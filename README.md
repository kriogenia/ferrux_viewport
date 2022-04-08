# FerruX Viewport

Ferrux Viewport is an abstraction layer over the [Pixels](https://crates.io/crates/pixels) crate to manage the drawing of figures and entities on a 3D space. It manages the pixel buffer exposing simple operations to draw pixels, lines and figures in the screen. In its current state it only works with [Winit](https://crates.io/crates/winit).

_FerruX Viewport doesn't perform the pespective projection, it just manages the drawing of points already calculated into the normalized 3D space_

## Usage

### Building a viewport

Right now, the only Viewport provided is WinitCanvas (and one mock for testing), which requires a winit Window, which will need itself an EventLoop reference.

```rust
let event_loop = winit::event_loop::EventLoop::new();
let window = winit::window::Window::new(&event_loop)?;
let viewport = ferrux_viewport::viewport::ViewportFactory::winit(&window, 100)?;
```

### Running the viewport

The main and recommended flow to use the viewport is:

 * Use the drawing functions like `draw_line` and `draw_triangle`.
 * Call the `render` method to print it on screen.
 * Use `reset_frame` to clear the current buffer and draw a new frame.

The following example takes the `Viewport` we built and draws a morphing line into triangles.

```rust
let mut i = 0.0;
let step = 0.05;
let mut incrementing = true;

event_loop.run(move |event, _, control_flow| {
  match event {
    Event::MainEventsCleared => {
      i += if incrementing { step } else { -step };
      if i >= 1.0 { incrementing = false } else if i <= 0.0 { incrementing = true }
      window.request_redraw();
    }
    Event::RedrawRequested(_) => {
      viewport.fill_triangle((0.0, 0.0, -0.1), (0.0 + i, -0.5 * i/2.0, -0.2), (0.0 + i, -0.3 * i/2.0, -0.2), 
        &[255, 0, 0, 255]);
      viewport.render().expect("render failed");
      viewport.reset_buffer();
    }
    _ => (),
}
```

## Examples

You can run the current example with

```sh
cargo run --package basic_example
```

## About

The FerruX Viewport is a tool developed while creating the FerruXengine, an attempt of 3D graphics engine I was trying to make. I made this viewport as an improvement from my previous tool to manage the use of the Pixels buffer used by the engine (being that the [FerruX Canvas](https://crates.io/crates/ferrux_canvas)).

## License

Licensed, at your option, under either of:

* Apache License, Version 2.0 (LICENSE-APACHE)
* MIT license (LICENSE-MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.