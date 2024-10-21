use std::io;
fn main() {
    println!("Guess the number");
    println!("Please input the guess");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Falied to read the lines");

    print!("You have gussed {guess}")
}
