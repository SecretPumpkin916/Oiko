use std::fs;
use std::io;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserConfig { 
    pub user: User,     
}

#[derive(Deserialize, Debug)]
pub struct User {       
    pub name: String,   
    pub password: String, 
}

pub fn readconfig() -> io::Result<UserConfig> {
    let contents = fs::read_to_string("config.toml")?;
    let config: UserConfig = toml::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    Ok(config) 
}