use std::io::{self, Write};
use crate::ipfs;

pub async fn app() {
    let orange_bold = "\x1b[1m\x1b[38;5;208m";
    let reset = "\x1b[0m";   
    
    println!("{orange_bold}");
    println!("  ██████╗ ██╗██╗  ██╗ ██████╗ ");
    println!(" ██╔═══██╗██║██║ ██╔╝██╔═══██╗");
    println!(" ██║   ██║██║█████╔╝ ██║   ██║");
    println!(" ██║   ██║██║██╔═██╗ ██║   ██║");
    println!(" ╚██████╔╝██║██║  ██╗╚██████╔╝");
    println!("  ╚═════╝ ╚═╝╚═╝  ╚═╝ ╚═════╝   ");
    println!("{reset}");
    println!("Welcome to Oiko! 🎃\n");
    
    loop {
        // Print prompt
        print!("Oiko > ");
        io::stdout().flush().unwrap(); 
        
        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim(); 
        
        // Handle commands
        match input {
            "help" => {
                println!("Available commands:");
                println!("  upload <file_path>   - Upload a file to IPFS");
                println!("  download <cid> <out> - Download a file from IPFS");
                println!("  help                 - Show this help message");
                println!("  exit                 - Exit the application");
            }
            "exit" => {
                println!("Goodbye!"); 
                std::process::exit(0);
            }
            "" => continue, // Skip empty input
            
            // Handle upload command
            cmd if cmd.starts_with("upload ") => {
                let path = cmd.strip_prefix("upload ").unwrap().trim();
                println!("Uploading {} to IPFS...", path);
                
                match ipfs::upload_to_ipfs(path).await {
                    Ok(cid) => {
                        // Automatically pin the file to keep it on the network
                        ipfs::pin_file(&cid).await.ok();
                    }
                    Err(e) => println!("Upload failed: {}", e),
                }
            }
            
            // Handle download command
            cmd if cmd.starts_with("download ") => {
                let parts: Vec<&str> = cmd.strip_prefix("download ").unwrap().trim().splitn(2, ' ').collect();
                if parts.len() != 2 {
                    println!("Usage: download <cid> <output_path>");
                    continue;
                }
                let cid = parts[0];
                let output_path = parts[1];
                
                println!("Downloading {} from IPFS...", cid);
                match ipfs::download_from_ipfs(cid, output_path).await {
                    Ok(()) => println!("Done!"),
                    Err(e) => println!("Download failed: {}", e),
                }
            }
            
            _ => println!("Unknown command: '{}'. Type 'help' for a list of commands.", input),
        }
    }
}