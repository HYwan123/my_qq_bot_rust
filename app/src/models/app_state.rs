use std::sync::Arc;

use crate::{
    models::{config::AppConfig, qq_group::QQGroup, qq_user::QQUser},
    utils::{config_handler},
};
use anyhow::Result;
use dashmap::DashMap;
use iflow_client::iflow_client::IflowClient;
use reqwest::Client;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct OneBotState {
    pub config: Arc<AppConfig>,
    pub reqwest_client: Client,
    pub iflow_client: Arc<IflowClient>,
    pub users: Arc<DashMap<u64, Arc<RwLock<QQUser>>>>,
    pub groups: Arc<DashMap<u64, Arc<RwLock<QQGroup>>>>,
}

impl OneBotState {
    pub async fn default_init() -> Result<Self> {
        let client = reqwest::Client::new();
        let init_config = config_handler::get_config().await?;
        Ok(Self {
            iflow_client: Arc::new(IflowClient::new(client.clone(), &init_config.api_key)),
            config: Arc::new(init_config),
            reqwest_client: client.clone(),
            users: Arc::new(DashMap::new()),
            groups: Arc::new(DashMap::new()),
        })
    }
}
