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

    // pub fn get_ssid() -> Result<String, String> {
    //     let output = Command::new("cmd")
    //         .args(["/C", "netsh wlan show interfaces"])
    //         .output()
    //         .map_err(|err| format!("Failed to execute netsh command: {}", err))?;
    //     let output_str = String::from_utf8_lossy(&output.stdout);
    //     let ssid = output_str.lines().find_map(|line| {
    //         if line.trim().starts_with("SSID") {
    //             line.split(":")
    //                 .nth(1)
    //                 .map(|s| s.trim().to_string())
    //         } else {
    //             None
    //         }
    //     });
    //     ssid.ok_or_else(|| "SSID not found".to_string())
    // }

    pub fn get_ssid() -> Result<String, String> {
        let output = Command::new("powershell")
            .args([
                "-Command",
                "(Get-NetAdapter | Where-Object {$_.Status -eq 'Up'}).Name | ForEach-Object { Get-NetConnectionProfile -InterfaceAlias $_ }",
            ])
            .output()
            .map_err(|err| format!("Failed to execute PowerShell command: {}", err))?;
        // let output_str = String::from_utf8_lossy(&output.stdout);
        let output_str = String::from_utf8_lossy(&output.stdout);
        // let output_str = str
        //     ::from_utf8(&output.stdout)
        //     .map_err(|err| format!("Failed to parse PowerShell output: {}", err))?;
        // // .map_err(|err| format!("Failed to parse PowerShell output: {}", err))?;

        // println!("{}", output_str);

        let mut ssid = None;
        for line in output_str.lines() {
            if line.trim().starts_with("InterfaceAlias") {
                if line.contains("WLAN") {
                    continue;
                } else {
                    break;
                }
            }
            if line.trim().starts_with("Name") {
                ssid = line
                    .split(":")
                    .nth(1)
                    .map(|s| s.trim().to_string());
                break;
            }
        }
        ssid.ok_or_else(|| "SSID not found".to_string())
    }

    pub fn get_ipv4_address() -> Option<String> {
        let output = Command::new("cmd")
            .args(["/C", "ipconfig /all"])
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
            extract_ipv4_address(adapters[0])
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
            extract_ipv4_address(adapters[choice - 1])
        }
    }
}
