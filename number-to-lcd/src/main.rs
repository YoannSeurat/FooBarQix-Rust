/*
https://codingdojo.org/kata/NumberToLCD/

Goal: write a program that displays LCD style numbers.

Part 1
Write a program that given a number (with arbitrary number of digits), converts it into LCD style numbers using the following format:

   _  _     _  _  _  _  _  
 | _| _||_||_ |_   ||_||_|  
 ||_  _|  | _||_|  ||_| _|  
(each digit is 3 lines high)

Yeah im not doing part 2

*/

const ZERO: &str = "
 _ 
| |
|_|";
const ONE: &str = "
   
  |
  |";
const TWO: &str = "
 _ 
 _|
|_ ";
const THREE: &str = "
_ 
_|
_|";
const FOUR: &str = "
   
|_|
  |";
const FIVE: &str = "
 _ 
|_ 
 _|";
const SIX: &str = "
 _ 
|_ 
|_|";
const SEVEN: &str = "
 _ 
  |
  |";
const EIGHT: &str = "
 _ 
|_|
|_|";
const NINE: &str = "
 _ 
|_|
 _|";
const DIGITS_LCD: [&str; 10] = [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

fn userinput() -> u32 {
    let mut result;
    loop {
        println!("Please enter a number : ");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to get input");
        result = buffer.trim().parse::<u32>();
        match result {
            Ok(ok) => {return ok;}
            Err(e) => {println!("Wrong number format ! {}\n", e);}
        }
    }
}

fn main() {
    loop {
        let number = userinput();
        for i in 1..4 {
            for d in number.to_string().split("") {
                if d != "" {
                    print!("{} ", DIGITS_LCD[d.parse::<usize>().unwrap()].to_string().split("\n").collect::<Vec<&str>>()[i]);
                }
            }
            println!();
        }
    }       
}