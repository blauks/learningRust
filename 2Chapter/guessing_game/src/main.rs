use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Gjett tallet!");

    let secret = rand::thread_rng().gen_range(1, 101);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Klarte ikke lese");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Du gjettet: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("For lite :("),
            Ordering::Greater => println!("For stort :/"),
            Ordering::Equal => {
                println!("Riktig! :)");
                break;
            }
        };
    }
}
