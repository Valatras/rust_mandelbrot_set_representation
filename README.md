# Rust Mandelbrot Set

This is a Rust project for exploring the Mandelbrot set and learning Rust along the way. The main application renders the fractal in an interactive desktop window; the repository also contains small, runnable exercises covering Rust fundamentals and concurrency.

## The Mandelbrot application

The renderer maps each screen pixel to a point \(c\) in the complex plane and iterates \(z_{n+1} = z_n^2 + c\), starting with \(z_0 = 0\). It uses the escape-time method to decide whether a point is in the set and to color points that escape.

The current renderer uses eight worker threads to calculate separate portions of each frame. In the window:

- Scroll the mouse wheel to zoom in or out around the center of the view.
- Hold the left mouse button and drag to pan.
- Use the up and down arrow keys to increase or decrease the iteration limit.
- Close the window to exit.

Run the application from the project directory:

```sh
cargo run
```

## Rust exercises

The standalone exercises are in `src/bin/`. Run one by its filename (without `.rs`) with Cargo:

```sh
cargo run --bin 01_hello_world
```

| Binary | Topic |
| --- | --- |
| `01_hello_world` | Printing and the `println!` macro |
| `02_counter` | Vectors, mutability, and borrowing |
| `03_structs` | Structs and methods |
| `04_threads` | Spawning and joining threads |
| `05_arc` | Shared ownership with `Rc` and `Arc` |
| `06_mutex` | Synchronization with `Mutex` and `RwLock` |
| `07_gui_gradient` | Drawing a gradient with a window and pixel buffer |
| `F01_hello_to_mutexes` | A full threads-and-mutexes exercise covering topics 01 to 06 |

## Rust crates

The application is built with these crates, declared in `Cargo.toml`:

| Crate | Role |
| --- | --- |
| [`num-complex`](https://crates.io/crates/num-complex) | Complex numbers used in the Mandelbrot iteration |
| [`winit`](https://crates.io/crates/winit) | Desktop window creation and input events |
| [`pixels`](https://crates.io/crates/pixels) | Pixel framebuffer used to display the rendered image |
| [`image`](https://crates.io/crates/image) | Declared as a project dependency; not currently used by the renderer |

## Requirements

- Rust and Cargo, with support for the Rust 2024 edition.
- A desktop environment with graphics support to run the windowed applications.

Build the project without launching it:

```sh
cargo build
```

This is an early release and a learning project; features and the renderer may change as the project develops.
