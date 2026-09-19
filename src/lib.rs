use num_complex::Complex;


//monothread
pub fn is_in_mandelbrot(i: usize,
    width: u32,
    height: u32,
    min_real: f64,
    max_real: f64,
    min_imaginary: f64,
    max_imaginary: f64,
    nmax:u32) -> bool {
    let x_pos = (i % width as usize) as f64;
    // we'll use / operator as the index grows in x. So Each row contains WIDTH pixels, and there are HEIGHT rows. 
    // We still want an f32 result as we want a ratio to multiply with 255 the color intensity.
    let y_pos = (i / width as usize) as f64;
    //pixel is a &mut [u8] variable. 
    // Transformation pixel -> plan complexe : position finale=début+pourcentage de progression×taille de l’intervalle
    let real = min_real + (x_pos / width as f64) * (max_real - min_real);
    let imaginary = max_imaginary - (y_pos / height as f64) * (max_imaginary - min_imaginary);
    let c = Complex::new(real, imaginary);
    // On vérifie si le pixel appartient à mandelbrot
    
    
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

