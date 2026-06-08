use rand::Rng;

fn userinput() -> u32 {
    let mut result;
    loop {
        println!("Please enter a number : ");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to get input");
        result = buffer.trim().parse::<u32>();
        match result {
            Ok(ok) => {
                if ok <= 1000 {
                    return ok;
                }
                println!("Number should be between 0 and 1000.");
            }
            Err(e) => {println!("Wrong number format ! {}", e);}
        }
    }
}

fn main() {
    loop {
        println!("A random number between 0 and 1000 has been chosen ! You only have 10 guesses so be smart");
        let random = rand::thread_rng().gen_range(0..=1000);
        let mut found = false;
        let mut count = 0;
        while !found {
            let guess = userinput();
            count += 1;
            if guess == random {
                println!("You found it, well done !\n");
                found = true;
            } else if random < guess {
                println!("Less. You have {} guesses left", 10-count);
            } else {
                println!("More. You have {} guesses left", 10-count);
            }
            if !found && count >= 10 {
                println!("You have reached your guessing limit... Try again.\n");
                found = true;
            }
        }
    }
}
