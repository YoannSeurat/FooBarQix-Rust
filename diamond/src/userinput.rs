pub fn user_input() -> char {
    let mut result;
    loop {
        println!("\nPlease enter a letter of the alphabet :");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to get input");
        result = buffer.trim().parse::<char>();
        match result {
            Ok(mut ok) => {
                ok = ok.to_ascii_uppercase();
                if 'A' <= ok && ok <= 'Z' {
                    return ok;
                }
                println!("Character should be a letter of the alphabet.");
            }
            Err(e) => {println!("Wrong number format ! {}", e);}
        }
    }
}