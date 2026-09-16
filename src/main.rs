use num_complex::Complex;
// use std::sync::Arc;
// use std::thread;
use image::{ImageBuffer, Rgb};

fn main(){
    let width = 1920;
    let height = 1080;
    let nmax = 100;


    // zone du domaine complexe à visualiser. 
    // Plus la zone est petite plus on regarde la fractale en profondeur
    // valeurs à comparer : 1.0, puis 2.0
    let min_real = -1.0;
    let max_real = 1.0;
    let min_imaginary = -1.0;
    let max_imaginary = 1.0;

    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {

            // Transformation pixel -> plan complexe : position finale=début+pourcentage de progression×taille de l’intervalle
            let real = min_real + (x as f64 / width as f64) * (max_real - min_real);
            let imaginary = max_imaginary - (y as f64 / height as f64) * (max_imaginary - min_imaginary);
            
            // création du nombre complexe c (représente un pixel dans le plan complexe à chaque fois)
            let c = Complex::new(real, imaginary);
            // On vérifie si le pixel appartient à mandelbrot
            let inside = is_in_mandlebrot_monot(c, nmax);
            
            // on colorie le pixel si il est dedans.
            let pixel = if inside {
                Rgb([0, 0, 0])
            } else {
                Rgb([255, 255, 255])
            };

            image.put_pixel(x, y, pixel);
        }
    }

    image.save("mandelbrot5.png").unwrap();
}

//monothread
fn is_in_mandlebrot_monot(c: Complex<f64>, nmax:i32) -> bool {

    // Création d'un nombre complexe par un complexe z initial 
    // et un nombre complexe c représentant un pixel
    let mut z = Complex::new(0.0,0.0);
    for _n in 1..=nmax{
        //équation de mandelbrot
        z = z * z + c;

        //condition pour dire que le point diverge
        if z.norm() > 2.0 {
            return false;
        }
    }

    return true;
}
