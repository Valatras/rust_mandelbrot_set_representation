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

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            pixels: None,
        }
    }
}

impl ApplicationHandler for App {
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

        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, window.clone());

        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);

        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                let pixels = self.pixels.as_mut().unwrap();

                let frame = pixels.frame_mut();

                for pixel in frame.chunks_exact_mut(4) {
                    pixel[0] = 255; // Rouge
                    pixel[1] = 0;   // Vert
                    pixel[2] = 0;   // Bleu
                    pixel[3] = 255; // Alpha
                }

                pixels.render().unwrap();
            }

            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut app = App::new();

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
