use rust_mandelbrot_set_representation::is_in_mandelbrot;
use std::sync::Arc;
use std::thread;
use pixels::{Pixels, SurfaceTexture};
use winit::{
application::ApplicationHandler, event::{ElementState, KeyEvent, MouseScrollDelta, WindowEvent}, event_loop::{ActiveEventLoop, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::Window,
};
  
//initial size of my window
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
// Nombre de threads que nous voulons utiliser.
const THREAD_COUNT: usize = 8;
  
struct App {
// here I create a window. The Arc option allows multiple accessing for ... ?
window: Option<Arc<Window>>,
pixels: Option<Pixels<'static>>,
min_real: f64,
max_real: f64,
min_imaginary: f64,
max_imaginary: f64,
nmax: u32,
}
 
// The constructor of our App structure.
impl App {fn new() -> Self {
Self {
window: None,
pixels: None,
min_real: -2.0,
max_real: 2.8,
min_imaginary: -2.4,
max_imaginary: 2.0,
nmax: 30,
}
}}
  
// A method (implementation in rust) that we create for our App [A]
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

                /*
                 * IMPORTANT :
                 *
                 * On récupère les paramètres avant de créer les threads.
                 *
                 * Nous ne voulons surtout pas donner `self` aux threads.
                 * Ces valeurs sont des types simples (f64/i32) et sont donc
                 * copiées dans chaque thread grâce à `move`.
                 */
                let min_real = self.min_real;
                let max_real = self.max_real;
                let min_imaginary = self.min_imaginary;
                let max_imaginary = self.max_imaginary;
                let nmax = self.nmax;

                /*
                 * Le framebuffer contient des OCTETS.
                 *
                 * 800 × 600 pixels
                 * × 4 octets par pixel
                 * = 1 920 000 octets
                 *
                 * Avec 4 threads :
                 * 1 920 000 / 4 = 480 000 octets par thread.
                 */
                let chunk_size = frame.len() / THREAD_COUNT;

                let mut handles = Vec::new();

                /*
                 * Nous créons quatre threads.
                 *
                 * Chaque thread reçoit une plage de pixels et construit
                 * son propre Vec<u8>.
                 *
                 * Cela évite de faire modifier `frame` directement par
                 * plusieurs threads.
                 */
                for thread_id in 0..THREAD_COUNT {
                    let start_byte = thread_id * chunk_size;

                    let end_byte = if thread_id == THREAD_COUNT - 1 {
                        frame.len()
                    } else {
                        start_byte + chunk_size
                    };

                    // Nombre de bytes dans cette partition.
                    let current_chunk_size = end_byte - start_byte;

                    /*
                     * Le thread ne retourne pas simplement "rien".
                     *
                     * Il va retourner son buffer :
                     *
                     *     Vec<u8>
                     *
                     * Le JoinHandle aura donc le type :
                     *
                     *     JoinHandle<Vec<u8>>
                     */
                    let handle = thread::spawn(move || {
                        // Buffer appartenant exclusivement à ce thread.
                        let mut result = vec![0u8; current_chunk_size];

                        /*
                         * On parcourt les pixels de NOTRE partition.
                         *
                         * Chaque pixel = 4 bytes RGBA.
                         */
                        for (local_pixel_index, pixel) in
                            result.chunks_exact_mut(4).enumerate()
                        {
                            /*
                             * local_pixel_index commence à 0 pour chaque thread.
                             *
                             * Il faut donc retrouver l'index GLOBAL du pixel
                             * dans l'image.
                             */
                            let global_pixel_index =
                                (start_byte / 4) + local_pixel_index;

                            let inside = is_in_mandelbrot(
                                global_pixel_index,
                                WIDTH,
                                HEIGHT,
                                min_real,
                                max_real,
                                min_imaginary,
                                max_imaginary,
                                nmax,
                            );

                            if inside.0 {
                                pixel[0] = 0;
                                pixel[1] = 0;
                                pixel[2] = 0;
                            } else {
                                // Juste pour changer un peu la couleur selon le nombre d'itération. 
                                pixel[0] = (255.0 * inside.1) as u8;
                                pixel[1] = 255;
                                pixel[2] = 255;
                            }

                            pixel[3] = 255;
                        }

                        /*
                         * Cette valeur sera récupérée par `join()`.
                         */
                        result
                    });

                    /*
                     * On conserve le handle.
                     *
                     * Si on faisait immédiatement `handle.join()`,
                     * on attendrait le thread avant de créer le suivant,
                     * ce qui supprimerait pratiquement tout le parallélisme.
                     */
                    handles.push(handle);
                }

                /*
                 * Maintenant que les 4 threads ont été lancés,
                 * on attend leurs résultats.
                 */
                for (thread_id, handle) in handles.into_iter().enumerate() {
                    let result = handle
                        .join()
                        .expect("A worker thread panicked");

                    /*
                     * Le thread principal récupère le Vec<u8>
                     * et le copie dans la bonne partie du framebuffer.
                     */
                    let start_byte = thread_id * chunk_size;

                    let end_byte = start_byte + result.len();

                    frame[start_byte..end_byte].copy_from_slice(&result);
                }

                // Une fois tous les threads terminés, l'image est complète.
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
		
		//for inputs comming from the keyboard.
		WindowEvent::KeyboardInput {
			event,
			is_synthetic: false,
			..
		} => {
			if event.state == ElementState::Pressed && !event.repeat {
				self.handle_keyboard_input(event);
			}
		}

		// every other kind of events gets no triggers.
		_ => {}
		}
	}
	
}



// Creating the input handler. [A]
impl App {
	fn handle_keyboard_input(&mut self, event: KeyEvent) {
		match event.physical_key {
			PhysicalKey::Code(KeyCode::ArrowUp) => {
				if self.nmax < 30 {
					self.nmax += 1;
				} else {
					self.nmax += 10;
				}
			}

			PhysicalKey::Code(KeyCode::ArrowDown) => {
				if self.nmax <= 30 {
					if self.nmax > 0 {
						self.nmax -= 1;
					}
				} else {
					self.nmax -= 10;
				}
			}

			_ => return,
    }

    self.window.as_ref().unwrap().request_redraw();
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
