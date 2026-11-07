use crate::readconfig::readconfig; 
use std::io::{self, Write}; 

pub fn login() {
    let config = match readconfig() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("Failed to read config: {}", e);
            return; 
        }
    };

    loop {
        print!("Enter username: ");
        io::stdout().flush().unwrap(); 
        let mut input_name = String::new();
        io::stdin().read_line(&mut input_name).unwrap();
        let input_name = input_name.trim(); 

        print!("Enter password: ");
        io::stdout().flush().unwrap();
        let mut input_pwd = String::new();
        io::stdin().read_line(&mut input_pwd).unwrap();
        let input_pwd = input_pwd.trim();

        if input_name == config.user.name && input_pwd == config.user.password {
            println!("Login successful! Welcome, {}!\n", input_name);
            break; 
        } else {
            println!("Invalid credentials. Please try again.\n");
        }
    }
}