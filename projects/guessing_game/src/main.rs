use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn printtypeof<T>(_: &T){
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret_number = {secret_number}");
    printtypeof(&secret_number);

    loop {
        println!("Please guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if (guess.starts_with("q")) {
            break;
        }

        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(e) => {
                println!("error: {e}");
                continue;
            },
        };
        // .expect("Please input a number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Bang on");
                break;
            }
        }
    }
}
