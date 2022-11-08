use std::{error::Error, fs};

use serde::{Deserialize, Serialize};


// Server端配置
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub listen_address: ListenAddress,
}

// 监听地址
#[derive(Debug, Serialize, Deserialize)]
pub struct ListenAddress {
    pub addr: String,
}

impl ServerConfig {
    // 加载Server端配置文件
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let config = fs::read_to_string(path)?;
        let server_conf: Self = toml::from_str(&config)?;
        Ok(server_conf)
    }
}
