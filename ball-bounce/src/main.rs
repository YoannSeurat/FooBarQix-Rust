use std::{io::Write, thread, time};

fn main() {    
    const WIDTH: usize = 16;
    const ONEHUNDRED: time::Duration = time::Duration::from_millis(100);

    let mut string = String::from("|");
    string.push_str(&std::iter::repeat(" ").take(WIDTH).collect::<String>().to_string());
    string.push('|');
    println!("{}", string);

    let mut i: usize = 0;
    loop {
        // (i // WIDTH) % 2 == 0 => index (i%WIDTH)+1
        // else => index WIDTH-(i%WIDTH)
        //   -> (+1 because of wall at index 0)
        string.replace_range(
            if i.div_euclid(WIDTH) % 2 == 0 {i%WIDTH+1..(i%WIDTH)+2} 
            else {WIDTH-(i%WIDTH)..WIDTH-(i%WIDTH)+1}, 
            "O"
        );
        println!("{}", string);
        string.replace_range(
            if i.div_euclid(WIDTH) % 2 == 0 {i%WIDTH+1..(i%WIDTH)+2} 
            else {WIDTH-(i%WIDTH)..WIDTH-(i%WIDTH)+1}, 
            " "
        );
        i += 1;
        thread::sleep(ONEHUNDRED);
    }
}
