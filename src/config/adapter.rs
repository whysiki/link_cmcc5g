pub mod wlan {
    use std::io::{ self, Write };
    use std::process::Command;
    use std::str;
    fn extract_ipv4_address(adapter: &str) -> Option<String> {
        adapter.lines().find_map(|line| {
            if line.trim().starts_with("IPv4 Address") || line.trim().starts_with("IPv4 地址") {
                Some(
                    line
                        .split(":")
                        .nth(1)?
                        .trim()
                        .trim_end_matches("(Preferred)")
                        .trim_end_matches("(preferred)")
                        .to_string()
                )
            } else {
                None
            }
        })
    }

    pub fn get_ipv4_address() -> Option<String> {
        let output = Command::new("cmd")
            .args(&["/C", "ipconfig /all"])
            .output()
            .expect("Failed to execute ipconfig command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        let adapters: Vec<&str> = output_str
            .split("Wireless LAN adapter")
            .filter(|s| s.contains("WLAN"))
            .collect();
        if adapters.is_empty() {
            println!("No Wireless LAN adapter WLAN found");
            return None;
        }
        if adapters.len() == 1 {
            return extract_ipv4_address(&adapters[0]);
        } else {
            println!("Found the following Wireless LAN adapters:");
            for (i, adapter) in adapters.iter().enumerate() {
                println!("{}. {}", i + 1, adapter);
            }
            print!("Please choose an adapter by number: ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read input");
            let choice: usize = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid selection");
                    return None;
                }
            };
            if choice < 1 || choice > adapters.len() {
                println!("Invalid selection");
                return None;
            }
            return extract_ipv4_address(&adapters[choice - 1]);
        }
    }
}
