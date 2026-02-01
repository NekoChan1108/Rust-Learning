use std::io;
use std::cmp;
use rand::Rng;

fn main() {
    let x = rand::rng().random_range(1..=10);
    println!("The secret number is {x}");
    println!("Guess the number!");
    loop {

        let mut guess = String::new();
         _ = io::stdin().read_line(&mut guess).unwrap();
        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                println!("Please input a number!");
                continue
            },
        };
        match guess.cmp(&x){
            cmp::Ordering::Less => println!("Too small!"),
            cmp::Ordering::Greater => println!("Too big!"),
            cmp::Ordering::Equal => {
                println!("You win!");
                break
            },
        }
    }
}
