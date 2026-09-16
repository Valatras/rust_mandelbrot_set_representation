use num_complex::Complex;


//monothread
pub fn is_in_mandlebrot_monot(c: Complex<f64>, nmax:i32) -> bool {

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