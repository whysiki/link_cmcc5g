use tokio;
mod config;
use std::env;
mod connect;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let clear = args.len() > 1 && args[1] == "clear";
    let config_path_string = config::get_config_path();
    let config_path = config_path_string.as_str();
    if clear && std::path::Path::new(config_path).exists() {
        let prompt =
            format!("Are you sure you want to delete the configuration file at {}? (y/n)", config_path);
        let user_input = config::get_input(&prompt);
        if user_input == "y" {
            std::fs::remove_file(config_path)?;
            println!("Configuration file deleted");
        } else {
            println!("Operation cancelled");
        }
        return Ok(());
    } else if clear {
        println!("The configuration file is not located at: {}", config_path);
        return Ok(());
    }
    connect::connect_to_cmcc().await?;
    Ok(())
}
