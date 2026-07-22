use std::io;

// This is a main file and entry point.
fn main() {
    println!("\n \t Hello fellow Rustaceans! \n");
    println!("{}", sum(5, 9));
    println!("{}", res(5, 9));
    println!("{}", mult(5, 9));
    println!("{}", div(5, 9));

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");


    
    // Variables y constantes
    let mut my_string: &str = "...";
    let my_number: i32 = 2;
    println!("{my_string} {my_number}");
    my_string = "new date";
    println!("{my_string} {my_number}");


    let my_string_2: String = String::from("This is the new string now");
    println!("{my_string_2} {guess}");


    // New user input
    let mut name = String::new();

    println!("Enter your name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    println!("Hello, {}!", name.trim());

}


fn sum(a: i32, b: i32) -> i32{
    a+b
}

fn res(a: i32, b: i32) -> i32{
    a-b
}

fn mult(a: i32, b: i32) -> i32{
    a*b
}

fn div(a: i32, b: i32) -> i32{
    a/b
}