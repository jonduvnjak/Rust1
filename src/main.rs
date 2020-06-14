use std::io;                 // (a)

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let mut numbers = String::new();
    

    io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");
        
    print_type_of(&numbers);

    let numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    for num in numbers {
        println!("{}", num);
        print_type_of(&num);
    }
}