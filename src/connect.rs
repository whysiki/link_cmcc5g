use log::{ error, info };
use reqwest::header::{ HeaderMap, HeaderValue, HOST, REFERER, USER_AGENT };
use tokio::time::{ sleep, Duration };
use crate::config;
async fn link_cmcc(client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(REFERER, HeaderValue::from_static("http://223.84.144.29/"));
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36"
        )
    );
    headers.insert(HOST, HeaderValue::from_static("223.84.144.29:801"));

    let login_config: config::LoginConfig = config::get_login_config();
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
        Ok(response) => {
            match response.text().await {
                Ok(body) => println!("{}", body),
                Err(e) => error!("Failed to read response body: {}", e),
            }
        }
        Err(e) => error!("Request failed: {}", e),
    }

    Ok(())
}

async fn try_link_cmcc(
    client: &reqwest::Client,
    max_retries: u32,
    retry_interval: Duration
) -> Result<(), Box<dyn std::error::Error>> {
    let mut attempt = 0;

    while attempt < max_retries {
        attempt += 1;
        match link_cmcc(client).await {
            Ok(_) => {
                println!("Link attempt {} succeeded", attempt);
                sleep(retry_interval).await;
                match link_cmcc(client).await {
                    Ok(_) => {
                        println!("Second link attempt succeeded");
                        return Ok(());
                    }
                    Err(err) => {
                        error!("Second link attempt failed: {}, retrying...", err);
                    }
                }
            }
            Err(err) => {
                error!("Link attempt {} failed: {}, retrying...", attempt, err);
            }
        }

        sleep(retry_interval).await;
    }

    Err(format!("Failed to link after {} attempts", max_retries).into())
}

pub async fn connect_to_cmcc() -> Result<(), Box<dyn std::error::Error>> {
    let retry_times = 40;
    let retry_interval = Duration::from_secs(15);
    let client = reqwest::Client::new();

    for attempt in 1..=retry_times {
        if let Some(ssid) = config::adapter::wlan::get_ssid().ok() {
            let ssid_trimmed = ssid.trim();
            if ssid_trimmed == "CMCC-5G" {
                println!("Matched SSID: {}, attempting to connect...", ssid_trimmed);

                match try_link_cmcc(&client, 3, Duration::from_secs(2)).await {
                    Ok(_) => {
                        println!("Successfully linked to CMCC-5G");
                        return Ok(());
                    }
                    Err(_) => {
                        println!("Failed to link to CMCC-5G, retrying...");
                    }
                }
            } else {
                println!(
                    "Not Matched SSID: {}, waiting {} seconds before retrying... {}/{}",
                    ssid_trimmed,
                    retry_interval.as_secs(),
                    attempt,
                    retry_times
                );
            }
        } else {
            println!(
                "SSID not found, waiting {} seconds before retrying... {}/{}",
                retry_interval.as_secs(),
                attempt,
                retry_times
            );
        }

        sleep(retry_interval).await;
    }

    Err("Failed to connect after 40 attempts.".into())
}
