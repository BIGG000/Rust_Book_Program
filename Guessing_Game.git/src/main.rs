extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess the number!");
	let secert_number = rand::thread_rng().gen_range(1,101);
	loop { 
	println!("Please input your Guess.");
	let mut guess = String::new();
	io::stdin().read_line(&mut guess).expect("failed to read line");
	let guess: u32 = match guess.trim().parse()
	  {
	Ok(num) => num,
	Err(_) => continue,
	  };
	println!("your Guessed: {}", guess);
	match guess.cmp(&secert_number) {
	Ordering::Less  => println!("Too Small!"),
	Ordering::Greater => println!("TOo Big!"),
	Ordering::Equal => {
	println!("You WIN!");
break;
	  }
	}
      }
}
