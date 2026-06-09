pub fn user_input(maxi:usize, text:&str, ) -> usize {
    let mut result;
    loop {
        println!("\n{}", text);
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to get input");
        result = buffer.trim().parse::<usize>();
        match result {
            Ok(ok) => {
                if ok <= maxi {
                    return ok;
                }
                println!("Number should be between 0 and {}.", maxi);
            }
            Err(e) => {println!("Wrong number format ! {}", e);}
        }
    }
}