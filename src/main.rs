fn main() {
    greet("world");
    count();
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn count(){
    let mut numbers = vec![10, 20, 30, 40];

    /*
    for number in numbers {
        println!("{number}");
    }
     */
    
    numbers.push(4); 
    numbers.push(5);

    for number in numbers {
        println!("{number}");
    }
}
