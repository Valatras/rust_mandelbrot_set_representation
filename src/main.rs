use num_complex::Complex;
use rust_mandelbrot_set_representation::*;
use std::sync::Arc;

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
    // here I create a window. The Arc option allows multiple accessing for ... ? 
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
}

// The constructor of our App structure. 
impl App {
    fn new() -> Self {
        Self {
            window: None,
            pixels: None,
        }
    }
}


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

        //what's a surface texture ? Is it to put interactivity on the window we created ? Is it a second layer above window ?
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, window.clone());

        // drawing with pixels
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

        // we fill our fields with what we just created.
        self.window = Some(window);
        self.pixels = Some(pixels);
        

        // are we are accessing the content of our window option (which is a window object)
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
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            // A pixel has four values (RGBA)
            WindowEvent::RedrawRequested => {
                let pixels = self.pixels.as_mut().unwrap();

                let frame = pixels.frame_mut();

                for pixel in frame.chunks_exact_mut(4) {
                    pixel[0] = 255; // Rouge
                    pixel[1] = 0;   // Vert
                    pixel[2] = 0;   // Bleu
                    pixel[3] = 255; // Alpha
                }

                // now we draw the pixel buffers on the window
                pixels.render().unwrap();
            }
            
            // every other kind of events gets no triggers.
            _ => {}
        }
    }
}

fn main() {
    // main loop that detects events for our window
    let event_loop = EventLoop::new().unwrap();

    //our app function that we just created above.
    let mut app = App::new();

    // we run the app here I guess ?
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
