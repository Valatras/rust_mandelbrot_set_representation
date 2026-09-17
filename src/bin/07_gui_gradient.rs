// this is an exercice where I'll draw a gradient using winit and pixels.
// use num_complex::Complex;
// use rust_mandelbrot_set_representation::*;
use std::{sync::Arc};
use pixels::{Pixels, SurfaceTexture};
use winit::{
application::ApplicationHandler,
event::WindowEvent,
event_loop::{ActiveEventLoop, EventLoop},
window::Window,
};
  
//initial size of my window
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
  
struct App {
// here I create a window. The Arc option allows multiple accessing.
// Graphics libraries like pixels or wgpu require creating a surface or context that often involves asynchronous operations
window: Option<Arc<Window>>,
pixels: Option<Pixels<'static>>,
}
 
// The constructor of our App structure.
impl App {fn new() -> Self {
Self {
window: None,
pixels: None,
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
		
		// we fill App's fields with what we just created.
		self.window = Some(window);
		self.pixels = Some(pixels);
		  
		// we are accessing the content of our window option (can be None or <Arc<Window>>)
		// to use a redraw method on it.
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
        
        //types of gradients
        //red_repeated_gradient(frame);
        // horizontal_continuous_red_gradient(frame);
        hc_red_vc_blue_gradient(frame);

        
		// now we draw the pixel buffers on the window
		pixels.render().unwrap();
        
		}
		// every other kind of events gets no triggers.
		_ => {}
		}
	}
}  

// fn red_repeated_gradient(pixel_buffer: &mut [u8]){
//     let mut red_gradient  = 0;
// 		for pixel in pixel_buffer.chunks_exact_mut(4) {
		
//             // here a pixel has four values [R,G,B,A]
//             if red_gradient == 255{
//                 red_gradient = 0;
//             }else {
//                 red_gradient += 1;
//             }
//             pixel[0] = red_gradient;
//             pixel[1] = 0; // Vert
//             pixel[2] = 0; // Bleu
//             pixel[3] = 255; // Alpha
// 		}
// }


// fn horizontal_continuous_red_gradient(pixel_buffer: &mut [u8]){
    
//     // we'll use enumerate to get the pixel's index too this time.
//     for (i, pixel) in pixel_buffer.chunks_exact_mut(4).enumerate() {
            
//             //We'll use float here as we'll work with a ratio between the width and the color intensity(0 to 255).
//             let x_pos = (i % WIDTH as usize) as f32;
            
//             let hc_red_gradient = (x_pos / WIDTH as f32 * 255.0 )as u8;
//             print!("index : {i} x_pos : {x_pos} color : {hc_red_gradient}");
//             pixel[0] = hc_red_gradient;
    
//             pixel[1] = 0; // Vert
//             pixel[2] = 0; // Bleu
//             pixel[3] = 255; // Alpha
// 		}
// }

fn hc_red_vc_blue_gradient(pixel_buffer: &mut [u8]){
    
    // we'll use enumerate to get the pixel's index too this time.
    for (i, pixel) in pixel_buffer.chunks_exact_mut(4).enumerate() {
            
            let x_pos = (i % WIDTH as usize) as f32;
            // we'll use / operator as the index grows in x. So Each row contains WIDTH pixels, and there are HEIGHT rows. 
            // We still want an f32 result as we want a ratio to multiply with 255 the color intensity.
            let y_pos = (i / WIDTH as usize) as f32;
            //pixel is a &mut [u8] variable. 
            let hc_red_gradient = (x_pos / WIDTH as f32 * 255.0 )as u8;
            let vc_blue_gradient = (y_pos / HEIGHT as f32 * 255.0)as u8;

            pixel[0] = hc_red_gradient;
            pixel[1] = 0; // Vert
            pixel[2] = vc_blue_gradient; // Bleu
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

