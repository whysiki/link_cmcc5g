use serde::{ Deserialize, Serialize };
use std::env;
use std::fs::{ File, OpenOptions };
use std::io::{ self, Read, Write };
pub(crate) mod adapter;
const BASE_URL: &str = "http://223.84.144.29:801/eportal/portal/login";
const CALLBACK: &str = "dr1004";
const LOGIN_METHOD: &str = "1";
const WLAN_AC_IP: &str = "218.204.128.10"; // Gateway IP
const JS_VERSION: &str = "4.1.3";
const TERMINAL_TYPE: &str = "1";
const LANG: &str = "zh-cn";
const V: &str = "2657";
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginConfig {
    pub user: String,
    pub pwd: String,
    pub base_url: String,
    pub callback: String,
    pub login_method: String,
    pub wlan_ac_ip: String,
    pub js_version: String,
    pub terminal_type: String,
    pub lang: String,
    pub v: String,
    pub wlan_user_ip: String,
}
fn read_config(file_path: &str) -> io::Result<LoginConfig> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: LoginConfig = serde_json::from_str(&contents)?;
    Ok(config)
}
pub fn write_config(file_path: &str, config: &LoginConfig) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(file_path)?;
    let contents = serde_json::to_string_pretty(config)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
pub fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn get_config_path() -> String {
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let mut config_path = exe_path.parent().expect("Failed to get parent directory").to_path_buf();
    config_path.push("config.json");
    config_path.to_str().expect("Failed to convert path to string").to_string()
}

pub fn get_login_config() -> LoginConfig {
    let binding = get_config_path();
    let file_path = binding.as_str();
    match read_config(file_path) {
        Ok(mut loaded_config) => {
            loaded_config.wlan_user_ip = adapter::wlan::get_ipv4_address().unwrap();
            println!("Loaded config: {:?}", loaded_config);
            loaded_config
        }
        Err(_) => {
            let user = get_input("Enter username:");
            let pwd = get_input("Enter password:");
            let config = LoginConfig {
                user,
                pwd,
                base_url: BASE_URL.to_string(),
                callback: CALLBACK.to_string(),
                login_method: LOGIN_METHOD.to_string(),
                wlan_ac_ip: WLAN_AC_IP.to_string(),
                wlan_user_ip: adapter::wlan::get_ipv4_address().unwrap(),
                js_version: JS_VERSION.to_string(),
                terminal_type: TERMINAL_TYPE.to_string(),
                lang: LANG.to_string(),
                v: V.to_string(),
            };
            write_config(file_path, &config).unwrap();
            println!("Configuration saved to: {}", file_path);
            config
        }
    }
}
