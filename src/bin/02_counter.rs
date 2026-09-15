fn main() {
    count();
}

fn count(){
    // This is a vector, which creates a dynamic array that can change size.
    // mut (for mutable) allows us to change the contents of the vector, and vec![] is a macro that creates a new vector.
    let mut numbers = vec![10, 20, 30, 40];

    // we're using the & reference operator to borrow the vector, 
    // which allows us to read its contents without taking ownership of it.
    for number in &numbers {
        println!("{number}");
    }
    
    
    // adding numbers to the vector 
    numbers.push(4); 
    numbers.push(5);

    // And here since we didn't use the & reference operator, we are taking ownership of the vector, 
    //which means that nothing else can use it after this point. If we do for example numbers.push(6); after this point, 
    // it will throw an error because we have already taken ownership of the vector.
    for number in numbers {
        println!("{number}");
    }

    // this will not work.
    // numbers.push(6)?;    

}
