use config::LoginConfig;
use log::{ error, info };
use reqwest::header::{ HeaderMap, HeaderValue, HOST, REFERER, USER_AGENT };
use tokio;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = reqwest::Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert(REFERER, HeaderValue::from_static("http://223.84.144.29/"));
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36"
        )
    );
    headers.insert(HOST, HeaderValue::from_static("223.84.144.29:801"));

    let login_config: LoginConfig = config::get_login_config();

    let url = format!(
        "{}?callback={}&login_method={}&user_account={}%40lan&user_password={}&wlan_user_ip={}&wlan_user_ipv6=&wlan_user_mac=000000000000&wlan_ac_ip={}&wlan_ac_name=&jsVersion={}&terminal_type={}&lang={}&v={}&lang=zh",
        login_config.base_url,
        login_config.callback,
        login_config.login_method,
        login_config.user,
        login_config.pwd,
        login_config.wlan_user_ip,
        login_config.wlan_ac_ip,
        login_config.js_version,
        login_config.terminal_type,
        login_config.lang,
        login_config.v
    );

    info!("Sending request to URL: {}", url);

    let request = client.get(&url).headers(headers);

    match request.send().await {
        Ok(response) =>
            match response.text().await {
                Ok(body) => println!("{}", body),
                Err(e) => error!("Failed to read response body: {}", e),
            }
        Err(e) => error!("Request failed: {}", e),
    }

    Ok(())
}
