use num_complex::Complex;
use std::sync::Arc;
use std::thread;

fn main(){
    // Création d'un nombre complexe par un complexe z initial 
    // et un nombre complexe c représentant un pixel
    iter_mandlebrot_monothread(Complex::new(-0.5,0.6009), 1000);
}

// utilise simplement le main thread pour l'équation de mandlebrot
fn iter_mandlebrot_monothread(c: Complex<f64>, nmax:i32){

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