//! config.rs
//!
//! //! https://github.com/olegccc/esp32-axum-ws/blob/main/.cargo/config.toml

#[derive(Debug)]
pub struct Config {
    // pub wifi_ssid: &'static str,
    // pub wifi_pass: &'static str,
    pub wifi_ssid: String,
    pub wifi_pass: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Config> {

        // run time
        let wifi_ssid = std::env::var("WIFI_SSID")?;
        let wifi_pass = std::env::var("WIFI_PASS")?;

        Ok(Self {
            wifi_ssid,
            wifi_pass,

            // compile time
//            wifi_ssid: env!("WIFI_SSID"),
//            wifi_pass: env!("WIFI_PASS"),
        })
    }
}
