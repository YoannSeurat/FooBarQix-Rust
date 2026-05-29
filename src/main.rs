use std::io;

fn main() {
    println!("Enter max number :");
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).expect("error");
    let max_i: u8 = buff.trim().parse().unwrap();

    for i in 0..max_i {
        let mut output = String::from("");

        if i % 3 == 0 {
            output.push_str("Foo");
        }
        if i % 5 == 0 {
            output.push_str("Bar");
        }
        if i % 7 == 0 {
            output.push_str("Qix");
        }

        for c in i.to_string().chars() {
            if c == '3' {
                output.push_str("Foo");
            } else if c == '5' {
                output.push_str("Bar");
            } else if c == '7' {
                output.push_str("Qix");
            }
        }

        if output == "" {
            output.push_str(&i.to_string());
        }

        println!("{} => {}", i, output);
    }
}
