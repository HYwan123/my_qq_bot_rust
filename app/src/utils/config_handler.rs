use crate::models::config::AppConfig;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::Result;
use toml::Value;
pub async fn get_config() -> Result<AppConfig> {
    let mut config_file = match File::open("./config.toml").await {
        Ok(file) => file,
        Err(_) => {
            let mut file = File::create("./config.toml").await?;
            file.write(toml::to_string(&AppConfig::new()).unwrap().as_bytes()).await?;
            panic!("未检测到config.toml,已自动生成,请配置")
        }
    };
    

    let mut contents = String::new();
    config_file.read_to_string(&mut contents).await?;
    let config: AppConfig = match toml::from_str(&contents) {
        Ok(config) => config,
        Err(_) => {

            let mut file = File::create("./config.toml").await?;
            let mut new_config = AppConfig::new();
            let config: Value = toml::from_str(&contents).unwrap();
            for (key, value) in config.as_table().unwrap() {
                match key.as_str() {
                    "api_key" => {
                        new_config.api_key = value.as_str().unwrap().to_string();
                    }
                    "bot_prompt" => {
                        new_config.bot_prompt = value.as_str().unwrap().to_string();
                    }
                    "manger_qq_id" => {
                        new_config.manger_qq_id = value.as_integer().unwrap() as u64;
                    }
                    "host" => {
                        new_config.host = value.as_str().unwrap().to_string();
                    }
                    "prot" => {
                        new_config.prot = value.as_integer().unwrap() as u64;
                    }
                    _ => {}
                }
            }
            
            
            file.write(toml::to_string(&new_config).unwrap().as_bytes()).await?;
            panic!("config.toml格式错误或更新,已自动生成,请配置")
        }
    };
    Ok(config)
}

