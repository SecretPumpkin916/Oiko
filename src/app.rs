use std::io::{self, Write};
use crate::ipfs;
use crate::catalog::{Catalog, Project};

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
                println!("  upload <file_path>              - Upload a file to IPFS");
                println!("  download <cid> <output_path>    - Download a file from IPFS");
                println!("  publish <name> <ver> <desc>     - Publish a project to catalog");
                println!("  list                            - List all available projects");
                println!("  pull <name>                     - Download a project by name");
                println!("  unpublish <name>                - Remove a project from catalog");
                println!("  clear                           - Clear the screen");
                println!("  exit                            - Exit the application");
            }
            "exit" => {
                println!("Goodbye!"); 
                std::process::exit(0);
            }
            "clear" => {
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush().unwrap();
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

            // Publish a project to the catalog
            cmd if cmd.starts_with("publish ") => {
                let parts: Vec<&str> = cmd.strip_prefix("publish ").unwrap().trim().splitn(3, ' ').collect();
                if parts.len() != 3 {
                    println!("Usage: publish <name> <version> <description>");
                    println!("Example: publish oiko 1.0.0 A decentralized CLI tool");
                    continue;
                }
                
                println!("Please upload the project files first, then provide the CID.");
                print!("Enter IPFS CID: ");
                io::stdout().flush().unwrap();
                let mut cid = String::new();
                io::stdin().read_line(&mut cid).unwrap();
                let cid = cid.trim().to_string();

                let mut catalog = Catalog::load();
                let project = Project {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    description: parts[2].to_string(),
                    cid,
                    author: "SecretPumpkin".to_string(), // You can link this to login later
                };

                match catalog.add_project(project) {
                    Ok(()) => println!("Project published successfully!"),
                    Err(e) => println!("Publish failed: {}", e),
                }
            }

            // List all available projects
            "list" => {
                let catalog = Catalog::load();
                if catalog.projects.is_empty() {
                    println!("No projects found in the catalog.");
                } else {
                    println!("{:<15} {:<10} {:<45} {}", "NAME", "VERSION", "CID", "DESCRIPTION");
                    println!("-------------------------------------------------------------------------------------");
                    for p in catalog.projects {
                        println!("{:<15} {:<10} {:<45} {}", p.name, p.version, p.cid, p.description);
                    }
                }
            }

            // Pull (Install) a project by name
            cmd if cmd.starts_with("pull ") => {
                let name = cmd.strip_prefix("pull ").unwrap().trim();
                let catalog = Catalog::load();
                
                if let Some(project) = catalog.projects.iter().find(|p| p.name == name) {
                    println!("Pulling {} v{} from IPFS...", project.name, project.version);
                    let output_path = format!("./{}", project.name);
                    
                    match ipfs::download_from_ipfs(&project.cid, &output_path).await {
                        Ok(()) => println!("Successfully installed {}!", project.name),
                        Err(e) => println!("Failed to pull project: {}", e),
                    }
                } else {
                    println!("Project '{}' not found. Type 'list' to see available projects.", name);
                }
            }

            // Unpublish a project
            cmd if cmd.starts_with("unpublish ") => {
                let name = cmd.strip_prefix("unpublish ").unwrap().trim();
                let mut catalog = Catalog::load();
                
                match catalog.remove_project(name) {
                    Ok(()) => println!("Project '{}' has been unpublished.", name),
                    Err(e) => println!("Unpublish failed: {}", e),
                }
            }
            
            _ => println!("Unknown command: '{}'. Type 'help' for a list of commands.", input),
        }
    }
}