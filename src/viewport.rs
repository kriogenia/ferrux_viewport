//! Package containing the viewport logic

mod factory;
pub use factory::ViewportFactory;

use crate::error::ViewportError;

use crate::pixel::Pixel;
use crate::render::{Render, Resize};
use crate::util::{as_signed, buffer_index, calculate_intersection, sort_vectors, to_pixel};
use crate::{PixelSize, Position, Voxel};
use bresenham_zip::build_zip;
use line_drawing::Bresenham3d;
use log::info;

/// Entity in charge of offering the functions to draw on the screen and handle to logic of the operation.
/// It works using three-dimensional normalized vectors of type (x: f32, y: f32, z: f32).
/// The point to draw in the screen will be the one relative to given position in the `x` and `y` axes.
/// So, any point outside the (-1.0, 1.0) range will not be drawn.
/// The `z` value works as a layer function, it will draw only the point with the highest `z` on the same translated pixel.
///
/// **The viewport doesn't perform projection**, that should be handled by the user before calling the functions.
/// Viewport just draws the pixels of the highest depth relative to the given coordinates.
///
/// The axes directions are:
/// * `x`: west -> east
/// * `y`: north -> south
/// * `z`: far -> near
pub struct Viewport<'a, S, R> {
    width: S,
    height: S,
    depth: S,
    buffer: Vec<Pixel<'a>>,
    renderer: R,
}

impl<'a, S: PixelSize, R> Viewport<'a, S, R> {
    /// Builds a new Viewport to use.
    ///
    /// # Arguments
    /// * `width`. Width in pixels of the screen, must be an unsigned value.
    /// * `height`. Height in pixels of the screen, must be an unsigned value.
    /// * `depth`. Depth to assume in the `z` axis calculations, must be an unsigned value.
    /// * `renderer`: Renderer to draw on
    ///
    pub(crate) fn new(width: S, height: S, depth: S, renderer: R) -> Self {
        assert!(width > S::zero());
        assert!(height > S::zero());
        assert!(depth > S::zero());

        let buffer_size = usize::cast(width * height);
        info!("Buffer size = {buffer_size:?}");
        Viewport {
            width,
            height,
            depth,
            buffer: vec![Pixel::default(); buffer_size],
            renderer,
        }
    }

    /// Returns the width of the current window
    pub fn width(&self) -> S {
        self.width
    }

    /// Returns the height of the current window
    pub fn height(&self) -> S {
        self.height
    }

    /// Returns the depth of the current window
    pub fn depth(&self) -> S {
        self.depth
    }

    /// Returns the sizes of the viewport in usize to use in the pixels calculation
    fn sizes(&self) -> (usize, usize, usize) {
        (
            usize::cast(self.width),
            usize::cast(self.height),
            usize::cast(self.depth),
        )
    }

    /// Adds a pixel to the buffer. It also verifies the color array and throws a panic if it's not correct.
    fn push_pixel(&mut self, (x, y, z): Voxel<usize>, color: &'a [u8]) {
        assert_eq!(4, color.len());
        let i = buffer_index(x, y, usize::cast(self.width));
        if i < self.buffer.len() && z >= self.buffer[i].depth {
            self.buffer[i] = Pixel { color, depth: z };
        }
    }

    /// Adds the pixels between two points to the buffer using the `push_pixel` function.
    fn push_line(&mut self, start: Voxel<isize>, end: Voxel<isize>, color: &'a [u8]) {
        for (x, y, z) in Bresenham3d::new(start, end) {
            self.push_pixel((x as usize, y as usize, z as usize), color);
        }
    }

    /// Commands the drawing of a point in the window. It will be rendered in the next call to [`Viewport::render`].
    /// If two drawn points fall on the same pixel, the point with the lowest `z` will be ignored.
    ///
    /// # Arguments
    /// * `position`, coordinates of the point in `(f32, f32, f32)`.
    /// * `color`, color of the point to draw. It should be provided as raw RGB values, alpha is included,
    /// so the expectation is a &[u8; 4] color like `&[255, 0, 0, 255]` for red with 100% opacity.
    ///
    /// # Example
    /// ```no_run
	/// # use std::error::Error;
	/// # fn main() -> Result<(), Box<dyn Error>> {
    /// # let event_loop = winit::event_loop::EventLoop::new();
    /// # let window = winit::window::Window::new(&event_loop).unwrap();
    /// # let mut viewport = ferrux_viewport::viewport::ViewportFactory::winit(&window, 100).unwrap();
    /// viewport.draw_point((0.0, 0.0, 0.0), &[255, 255, 255, 255]); // white point in the center of the screen
    /// viewport.render()?; // renders the point in the window
	/// # Ok (())
	/// # }
    /// ```
    ///
    /// # Panic
    /// Passing a color with the wrong number of members will throw a panic. It's required to have length four (R, G, B, A);
    ///
    pub fn draw_point(&mut self, position: Position, color: &'a [u8]) {
        let voxel = to_pixel(position, self.sizes());
        self.push_pixel(voxel, color);
    }

    /// Commands the drawing of a line in the window. It will be rendered in the next call to [`Viewport::render`].
    ///
    /// # Arguments
    /// * `start`, coordinates of the starting point of the line.
    /// * `end`, coordinates of the ending point of the line.
    /// * `color`, color of the line to draw. It should be provided as raw RGB values, alpha is included,
    /// so the expectation is a &[u8; 4] color like `&[255, 0, 0, 255]` for red with 100% opacity.
    ///
    /// # Example
    /// ```no_run
	/// # use std::error::Error;
	/// # fn main() -> Result<(), Box<dyn Error>> {
    /// # let event_loop = winit::event_loop::EventLoop::new();
    /// # let window = winit::window::Window::new(&event_loop).unwrap();
    /// # let mut viewport = ferrux_viewport::viewport::ViewportFactory::winit(&window, 100).unwrap();
    /// viewport.draw_line((-0.5, -0.5, -0.5), (0.25, 0.5, 0.0), &[255, 255, 255, 255]);
    /// viewport.render()?; // renders the line in the window
	/// # Ok (())
	/// # }
    /// ```
    ///
    /// # Panic
    /// Passing a color with the wrong number of members will throw a panic. It's required to have length four (R, G, B, A);
    ///
    pub fn draw_line(&mut self, start: Position, end: Position, color: &'a [u8]) {
        let start = to_pixel(start, self.sizes());
        let end = to_pixel(end, self.sizes());
        self.push_line(as_signed(start), as_signed(end), color);
    }

    /// Commands the drawing of a triangle in the window. It will be rendered in the next call to [`Viewport::render`].
    ///
    /// # Arguments
    /// * `point_a`, `point_b`, `point_c`. Coordinates of the points of the triangle.
    /// * `color`, color of the line to draw. It should be provided as raw RGB values, alpha is included,
    /// so the expectation is a &[u8; 4] color like `&[255, 0, 0, 255]` for red with 100% opacity.
    ///
    /// # Example
    /// ```no_run
	/// # use std::error::Error;
	/// # fn main() -> Result<(), Box<dyn Error>> {
    /// # let event_loop = winit::event_loop::EventLoop::new();
    /// # let window = winit::window::Window::new(&event_loop).unwrap();
    /// # let mut viewport = ferrux_viewport::viewport::ViewportFactory::winit(&window, 100).unwrap();
    /// viewport.draw_triangle((0.0, 0.0, -0.5), (-0.5, 0.5, 0.0), (0.5, 0.5, 0.0), &[255, 255, 255, 255]);
    /// viewport.render()?; // renders the triangle in the window
	/// # Ok (())
	/// # }
    /// ```
    ///
    /// # Panic
    /// Passing a color with the wrong number of members will throw a panic. It's required to have length four (R, G, B, A);
    ///
    pub fn draw_triangle(
        &mut self,
        point_a: Position,
        point_b: Position,
        point_c: Position,
        color: &'a [u8],
    ) {
        self.draw_line(point_a, point_b, color);
        self.draw_line(point_b, point_c, color);
        self.draw_line(point_c, point_a, color);
    }

    /// Commands the drawing and filling of a triangle in the window. It will be rendered in the next call to [`Viewport::render`].
    ///
    /// # Arguments
    /// * `point_a`, `point_b`, `point_c`. Coordinates of the points of the triangle.
    /// * `color`, color of the line to draw. It should be provided as raw RGB values, alpha is included,
    /// so the expectation is a &[u8; 4] color like `&[255, 0, 0, 255]` for red with 100% opacity.
    ///
    /// # Example
    /// ```no_run
	/// # use std::error::Error;
	/// # fn main() -> Result<(), Box<dyn Error>> {
    /// # let event_loop = winit::event_loop::EventLoop::new();
    /// # let window = winit::window::Window::new(&event_loop).unwrap();
    /// # let mut viewport = ferrux_viewport::viewport::ViewportFactory::winit(&window, 100).unwrap();
    /// viewport.fill_triangle((0.0, 0.0, -0.5), (-0.5, 0.5, 0.0), (0.5, 0.5, 0.0), &[255, 255, 255, 255]);
    /// viewport.render()?; // renders the triangle in the window
	/// # Ok (())
	/// # }
    /// ```
    ///
    /// # Panic
    /// Passing a color with the wrong number of members will throw a panic. It's required to have length four (R, G, B, A);
    ///
    pub fn fill_triangle(
        &mut self,
        point_a: Position,
        point_b: Position,
        point_c: Position,
        color: &'a [u8],
    ) {
        let point_a = as_signed(to_pixel(point_a, self.sizes()));
        let point_b = as_signed(to_pixel(point_b, self.sizes()));
        let point_c = as_signed(to_pixel(point_c, self.sizes()));

        let (point_a, point_b, point_c) = sort_vectors(point_a, point_b, point_c);
        match point_b {
            (_, y, _) if y == point_c.1 => {
                self.fill_flat_triangle(point_a, point_b, point_c, color)
            }
            (_, y, _) if y == point_a.1 => {
                self.fill_flat_triangle(point_c, point_a, point_b, color)
            }
            _ => {
                let intersection = calculate_intersection(point_c, point_b, point_a);
                self.fill_flat_triangle(point_a, point_b, intersection, color);
                self.fill_flat_triangle(point_c, point_b, intersection, color);
            }
        }
    }

    /// Uses BresenhamZip to push the pixels to draw and fill a flat Y triangle (top or bot)
    fn fill_flat_triangle(
        &mut self,
        peak: Voxel<isize>,
        side_a: Voxel<isize>,
        side_b: Voxel<isize>,
        color: &'a [u8],
    ) {
        let bresenham = build_zip!(3D:Y - peak -> side_a, side_b)
            .expect("Side points of a flat triangle should share the same Y value");
        for (left, right) in bresenham {
            self.push_line(left, right, color);
        }
    }

    /// Resets the buffer clearing all its current content
    pub fn reset_buffer(&mut self) {
        self.buffer = vec![Pixel::default(); usize::cast(self.width) * usize::cast(self.height)];
    }
}

impl<'a, S: PixelSize, R: Resize<S>> Viewport<'a, S, R> {
    /// Changes the size of the rendered window. Doing it will **reset the buffer**, clearing the current content.
    ///
    /// # Arguments
    /// * `width`. New width of the window.
    /// * `height`. New height of the window.
    ///
    pub fn resize(&mut self, width: S, height: S) {
        self.width = width;
        self.height = height;
		self.reset_buffer();
        self.renderer.resize(width, height);
    }
}

impl<'a, S: PixelSize, R: Render> Viewport<'a, S, R> {
    /// Renders the content of the buffer in the Window. 
	/// It doesn't clear the buffer afterwards, to do that call [Viewport::reset_buffer].
	/// 
	/// # Example
    /// ```no_run
	/// # use std::error::Error;
	/// # fn main() -> Result<(), Box<dyn Error>> {
    /// # let event_loop = winit::event_loop::EventLoop::new();
    /// # let window = winit::window::Window::new(&event_loop).unwrap();
    /// # let mut viewport = ferrux_viewport::viewport::ViewportFactory::winit(&window, 100).unwrap();
    /// viewport.fill_triangle((0.0, 0.0, -0.5), (-0.5, 0.5, 0.0), (0.5, 0.5, 0.0), &[255, 255, 255, 255]);
    /// viewport.render()?; // renders the triangle in the window
	/// viewport.reset_buffer(); // clears the buffer to prepare the drawing of a new frame
	/// # Ok (())
	/// # }
    /// ```
    pub fn render(&mut self) -> Result<(), ViewportError> {
        self.renderer.render(&self.buffer)
    }

    /// Draws an empty frame without the needing of resetting the buffer.
	/// This is an optimal way of drawing an empty frame keeping the current drawing buffer without the need
	/// to save it, resetting buffer, rendering and redrawing it.
	/// 
	/// # Example
    /// ```no_run
	/// # use std::error::Error;
	/// # fn main() -> Result<(), Box<dyn Error>> {
    /// # let event_loop = winit::event_loop::EventLoop::new();
    /// # let window = winit::window::Window::new(&event_loop).unwrap();
    /// # let mut viewport = ferrux_viewport::viewport::ViewportFactory::winit(&window, 100).unwrap();
    /// viewport.fill_triangle((0.0, 0.0, -0.5), (-0.5, 0.5, 0.0), (0.5, 0.5, 0.0), &[255, 255, 255, 255]);
    /// viewport.render()?;         // renders the triangle in the window
	/// viewport.clear_frame()?;    // renders an empty frame
	/// viewport.fill_triangle((0.25, 0.75, -0.5), (-0.25, 0.25, 0.0), (0.25, 0.25, 0.0), &[255, 0, 0, 255]);
	/// viewport.render()?;         // renders both triangles as the previous one was not deleted from the buffer
	/// # Ok (())
	/// # }
    /// ```
    pub fn clear_frame(&mut self) -> Result<(), ViewportError> {
        self.renderer.clear()
    }
}

#[cfg(test)]
mod test {
    use crate::{pixel::Pixel, viewport::ViewportFactory};

    #[test]
    fn draw_point() {
        let mut viewport = ViewportFactory::test(640, 480, 1000);
        let color = &[255, 255, 255, 255];

        viewport.draw_point((-1.0, -1.0, -1.0), color);
        viewport.draw_point((1.0, 1.0, 1.0), color); // will be ignored
        viewport.draw_point((0.0, 0.0, 0.0), color);
        viewport.draw_point((0.0, 0.0, 0.5), color); // will override the previous one
        viewport.draw_point((-0.25, 0.25, 0.25), color);
        viewport.draw_point((-0.25, 0.25, -0.25), color); // will not override the previous

        assert_eq!(viewport.buffer[0], Pixel { color, depth: 0 });
        assert_eq!(viewport.buffer[153920], Pixel { color, depth: 750 });
        assert_eq!(viewport.buffer[192240], Pixel { color, depth: 625 });
    }

    #[test]
    fn draw_line() {
        let mut viewport = ViewportFactory::test(24, 24, 10);
        let color = &[255, 255, 255, 255];

        viewport.draw_line((-0.25, -0.25, 0.0), (0.25, 0.25, 0.0), color);

        for i in 0..7 {
            assert_eq!(viewport.buffer[225 + i * 25], Pixel { color, depth: 5 });
        }
    }

    #[test]
    fn draw_triangle() {
        let mut viewport = ViewportFactory::test(16, 16, 10);
        let color = &[255, 255, 255, 255];

        viewport.draw_triangle(
            (0.0, -0.25, 0.0),
            (-0.25, 0.0, 0.0),
            (0.25, 0.0, 0.0),
            color,
        );

        // Check points in each of the lines
        assert_eq!(viewport.buffer[119], Pixel { color, depth: 5 });
        assert_eq!(viewport.buffer[135], Pixel { color, depth: 5 });
        assert_eq!(viewport.buffer[121], Pixel { color, depth: 5 });
    }

    #[test]
    fn fill_triangle() {
        let mut viewport = ViewportFactory::test(16, 16, 10);
        let color = &[255, 255, 255, 255];

        viewport.fill_triangle(
            (0.0, -0.25, 0.0),
            (-0.25, 0.0, 0.0),
            (0.25, 0.0, 0.0),
            color,
        );

        // Check points in each of the lines
        assert_eq!(viewport.buffer[119], Pixel { color, depth: 5 });
        assert_eq!(viewport.buffer[135], Pixel { color, depth: 5 });
        assert_eq!(viewport.buffer[121], Pixel { color, depth: 5 });

        // Check point inside
        assert_eq!(viewport.buffer[120], Pixel { color, depth: 5 });
    }

    #[test]
    fn reset_buffer() {
        let mut viewport = ViewportFactory::test(16, 16, 10);
        let color = &[255, 255, 255, 255];
        viewport.draw_point((-1.0, -1.0, -1.0), &[255, 255, 255, 255]);
        assert_eq!(viewport.buffer[0], Pixel { color, depth: 0 });

        viewport.reset_buffer();
        assert_eq!(viewport.buffer[0], Pixel::default());
    }

	#[test]
	fn render() {
		let mut viewport = ViewportFactory::test(16, 16, 10);
        assert_eq!(viewport.renderer.render_calls, 0);
		viewport.render().unwrap();
        assert_eq!(viewport.renderer.render_calls, 1);
	}

	#[test]
	fn clear() {
		let mut viewport = ViewportFactory::test(16, 16, 10);
        assert_eq!(viewport.renderer.clear_calls, 0);
		viewport.clear_frame().unwrap();
        assert_eq!(viewport.renderer.clear_calls, 1);
	}

	#[test]
	fn resize_buffer() {
		let mut viewport = ViewportFactory::test(16, 16, 10);
		assert_eq!(viewport.buffer.len(), 16 * 16);
		viewport.resize(32, 32);
		assert_eq!(viewport.buffer.len(), 32 * 32);
		assert_eq!(viewport.renderer.size, (32, 32));
	}

    #[test]
    #[should_panic]
    fn wrong_color() {
        ViewportFactory::test(640, 480, 10).draw_point((0.0, 0.0, 0.0), &[0, 0, 0]);
    }
}
