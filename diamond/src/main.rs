/*
https://codingdojo.org/kata/Diamond/

Diamond

About this Kata
Alistair Cockburn wrote a blog post about this kata, in response to the Seb Rose kata proposition.

Problem Description
Given a letter, print a diamond starting with ‘A’ with the supplied letter at the widest point.

For example: print-diamond ‘C’ prints

  A
 B B
C   C
 B B
  A
*/

mod userinput;

fn diamond(letter:usize) {
    let delta = letter-65; // distance in alphabet between letter and 'A'
    for reverse in [false, true] {
        for mut i in 0..=delta {
            if reverse {
                if i == delta { break; }
                i = delta-i-1;
            }
            let mut string = String::new();
            string.push_str(&" ".repeat(delta-i));
            string.push((65+i as u8) as char);
            if i != 0 {
                string.push_str(&" ".repeat(2*(i-1)+1));
                string.push((65+i as u8) as char);
            }
            println!("{}", string);
        }
    }
}

fn main() {
    loop {
        diamond(userinput::user_input() as usize);   
    }
}
