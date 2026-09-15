fn main(){
    // A struct is a custom data type we can add ourselves.
    // Here I created a struct called Pixel, which has 3 fields: x, y and color.
    struct Pixel {
    x: usize,
    y: usize,
    color: u8,
}

    // Here we create a new instance of the Pixel struct, and assign values to its fields.
    let pixel = Pixel {
        x: 10,
        y: 20,
        color: 255,
    };

    // using impl, we can implement methods for our struct, which are functions that belong to the struct and can be called on instances of the struct.
    impl Pixel {
        fn show_pos (&self) {
            println!("Pixel position: ({}, {})", self.x, self.y);
        }
    }

    pixel.show_pos();
    println!("{}", pixel.color);
}