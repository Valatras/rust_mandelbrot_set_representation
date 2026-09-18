use num_complex::Complex;
use rust_mandelbrot_set_representation::*;
use std::sync::Arc;
use pixels::{Pixels, SurfaceTexture};
use winit::{
application::ApplicationHandler, event::{ElementState::Pressed, KeyEvent, MouseScrollDelta, WindowEvent}, event_loop::{ActiveEventLoop, EventLoop}, keyboard::{KeyCode::{ArrowDown, ArrowUp}, PhysicalKey}, window::Window,
};
  
//initial size of my window
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
  
struct App {
// here I create a window. The Arc option allows multiple accessing for ... ?
window: Option<Arc<Window>>,
pixels: Option<Pixels<'static>>,
min_real: f64,
max_real: f64,
min_imaginary: f64,
max_imaginary: f64,
nmax: i32,
}
 
// The constructor of our App structure.
impl App {fn new() -> Self {
Self {
window: None,
pixels: None,
min_real: -3.0,
max_real: 3.0,
min_imaginary: -2.0,
max_imaginary: 2.0,
nmax: 30,
}
}}
  
// A method (implementation in rust) that we create for our App
impl ApplicationHandler for App {
// resumed is an ApplicationHandler's method. Used to render
// when a graphic app is created (desktop), or resumed back
// on it (going back to the app on android).
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let window = Arc::new(
			event_loop
			.create_window(
				Window::default_attributes()
				.with_title("Rust Mandelbrot")
				.with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT)),
			)
			.unwrap(),
		);  
		
		//Linking Pixel's rendering surface to our winit window. 
		let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, window.clone());
			  
		// creating a new Pixels buffer
		let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();
		
		// we fill our fields with what we just created.
		self.window = Some(window);
		self.pixels = Some(pixels);
		  
		// we are accessing the content of our window option (can be None or <Arc<Window>>)
		//to use a redraw method on it.
		self.window.as_ref().unwrap().request_redraw();
	}
	  
	// just copy this. Its to look for window (and potentially system) events
	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		_window_id: winit::window::WindowId,
		event: WindowEvent,
		) {
	// when we receive a matching event from winit's WindowEvent events.
	match event {
		// close button = quit the app.
		WindowEvent::CloseRequested => {
		event_loop.exit();
		}
		
		WindowEvent::RedrawRequested => {
		let pixels = self.pixels.as_mut().unwrap();
		let frame = pixels.frame_mut();
		draw_mandelbrot(frame,
		self.min_real,
        self.max_real,
        self.min_imaginary,
        self.max_imaginary,
		self.nmax
		);
		// now we draw the pixel buffers on the window
		pixels.render().unwrap();
		}

		WindowEvent::MouseWheel { delta, .. } => {
			let zoom_factor = match delta {
				MouseScrollDelta::LineDelta(_, y) => 1.0 + 0.1 * y as f64,
				MouseScrollDelta::PixelDelta(d) => 1.0 + 0.001 * d.y as f64,
				
			};

			// Zoom around the center of the current viewport
			let center_real = (self.min_real + self.max_real) / 2.0;
			let center_imag = (self.min_imaginary + self.max_imaginary) / 2.0;
			let half_w = (self.max_real - self.min_real) / 2.0 * zoom_factor;
			let half_h = (self.max_imaginary - self.min_imaginary) / 2.0 * zoom_factor;

			self.min_real = center_real - half_w;
			self.max_real = center_real + half_w;
			self.min_imaginary = center_imag - half_h;
			self.max_imaginary = center_imag + half_h;

			self.window.as_ref().unwrap().request_redraw();
		}
		
		WindowEvent::KeyboardInput { device_id, event: KeyEvent { physical_key:PhysicalKey::Code(ArrowUp), text, state:Pressed, repeat:false, .. }, is_synthetic:false } => {
			// In your update (e.g. about_to_wait or redraw):
			
			self.nmax+= 10;
			println!("{:?}",self.nmax);
			
			self.window.as_ref().unwrap().request_redraw();
		}

		WindowEvent::KeyboardInput { device_id, event: KeyEvent { physical_key:PhysicalKey::Code(ArrowDown), text, state:Pressed, repeat:false, .. }, is_synthetic:false } => {
			// In your update (e.g. about_to_wait or redraw):
			
			self.nmax-= 10;
			println!("{:?}",self.nmax);
			
			self.window.as_ref().unwrap().request_redraw();
		}

		// every other kind of events gets no triggers.
		_ => {}
		}
	}
}


fn draw_mandelbrot(frame: &mut [u8],min_real: f64,
    max_real: f64,
    min_imaginary: f64,
    max_imaginary: f64,
	nmax:i32){
    
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {

            let x_pos = (i % WIDTH as usize) as f32;
            // we'll use / operator as the index grows in x. So Each row contains WIDTH pixels, and there are HEIGHT rows. 
            // We still want an f32 result as we want a ratio to multiply with 255 the color intensity.
            let y_pos = (i / WIDTH as usize) as f32;
            //pixel is a &mut [u8] variable. 
            // Transformation pixel -> plan complexe : position finale=début+pourcentage de progression×taille de l’intervalle
            let real = min_real + (x_pos as f64 / WIDTH as f64) * (max_real - min_real);
            let imaginary = max_imaginary - (y_pos as f64 / HEIGHT as f64) * (max_imaginary - min_imaginary);
            let c = Complex::new(real, imaginary);
            // On vérifie si le pixel appartient à mandelbrot
            let inside = is_in_mandlebrot_monot(c, nmax);
            
            if inside == true{
                pixel[0] = 0;
                pixel[1] = 0; // Vert
                pixel[2] = 0; // Bleu
            }else {    
                pixel[0] = 255;
                pixel[1] = 255; // Vert
                pixel[2] = 255; // Bleu   
            }
            pixel[3] = 255; // Alpha
		}
}
  

fn main() {
// main loop that detects events for our window
let event_loop = EventLoop::new().unwrap();
//our app function that we just created above.
let mut app = App::new();
// we run the app here. (unwrap to get () on success cuz it's a Result<(), EventLoopError>). 
// On a production code we won't do that. We'll propagate the error instead.
event_loop.run_app(&mut app).unwrap();
}

// fn main(){
//     let width = 1920;
//     let height = 1080;
//     let nmax = 100;


//     // zone du domaine complexe à visualiser. 
//     // Plus la zone est petite plus on regarde la fractale en profondeur
//     // valeurs à comparer : 1.0, puis 2.0
//     let min_real = -1.0;
//     let max_real = 1.0;
//     let min_imaginary = -1.0;
//     let max_imaginary = 1.0;

//     let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

//     for x in 0..width {
//         for y in 0..height {

//             // Transformation pixel -> plan complexe : position finale=début+pourcentage de progression×taille de l’intervalle
//             let real = min_real + (x as f64 / width as f64) * (max_real - min_real);
//             let imaginary = max_imaginary - (y as f64 / height as f64) * (max_imaginary - min_imaginary);
            
//             // création du nombre complexe c (représente un pixel dans le plan complexe à chaque fois)
//             let c = Complex::new(real, imaginary);
//             // On vérifie si le pixel appartient à mandelbrot
//             let inside = is_in_mandlebrot_monot(c, nmax);
            
//             // on colorie le pixel si il est dedans.
//             let pixel = if inside {
//                 Rgb([0, 0, 0])
//             } else {
//                 Rgb([255, 255, 255])
//             };

//             image.put_pixel(x, y, pixel);
//         }
//     }
//     image.save("mandelbrot4.png").unwrap();
// }
