use num_complex::Complex;
use std::sync::Arc;
use std::thread;
use image::{ImageBuffer, Rgb};

fn main(){
    // Création d'un nombre complexe par un complexe z initial 
    // et un nombre complexe c représentant un pixel
    // test_iter_mandlebrot_monothread(Complex::new(-0.5,0.6009), 1000);
    let width = 640;
    let height = 480;
    let nmax = 100;

    let min_real = -2.5;
    let max_real = 2.5;

    let min_imaginary = -2.5;
    let max_imaginary = 2.5;

    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            // Transformation pixel -> plan complexe
            let real = min_real + (x as f64 / width as f64) * (max_real - min_real);

            let imaginary = max_imaginary
                - (y as f64 / height as f64)
                    * (max_imaginary - min_imaginary);

            let c = Complex::new(real, imaginary);

            let inside = is_in_mandlebrot_monot(c, nmax);

            let pixel = if inside {
                Rgb([0, 0, 0])
            } else {
                Rgb([255, 255, 255])
            };

            image.put_pixel(x, y, pixel);
        }
    }

    image.save("mandelbrot.png").unwrap();
}



// utilise simplement le main thread pour l'équation de mandlebrot
fn test_iter_mandlebrot_monothread(c: Complex<f64>, nmax:i32){

    let mut z = Complex::new(0.0, 0.0);
    // l'équation à tester est, pour chaque itération de n,
    // si le module de z_n est plus grand que 2.
    //Création d'un monothread de calcul :

    println!("z initial : {z}");

    for n in 0..nmax {

        z = z * z + c;    

        //vérifie si |z| est plus grande que 2 :
        if z.norm() > 2.0 {
            return println!("Ce point diverge après {n} itérations ! module : {:?}", z.norm());

        }

    }
    return println!("ce point ne diverge pas après {nmax} itérations. Dernier module de z : {:?}", z.norm());
        

}

//monothread
fn is_in_mandlebrot_monot(c: Complex<f64>, nmax:i32) -> bool {
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

fn draw_mandlebrot(){

}