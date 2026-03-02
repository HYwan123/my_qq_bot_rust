use std::sync::Arc;

use anyhow::Result;
use app::{models::app_state::OneBotState, services::{onebot_handler::start_handler, send_like_everyday::start_task}};
use axum::{Router, extract::State, routing::get};
use tokio::net::TcpListener;
use tokio_cron_scheduler::JobScheduler;


async fn start_server() -> Result<()> {
    println!("启动!");
    let state: OneBotState = OneBotState::default_init().await?;
    let client = state.clone().reqwest_client.clone();
    let sched = JobScheduler::new().await?;
    start_task(client, sched.clone()).await?;
    sched.start().await?;
    
    tokio::spawn(start_handler(state.clone()));


    
    let app = Router::new()
        .route("/test", get("Ciallo World!"))
        .route("/get_state", get(return_state))
        .with_state(state.clone().into());
    let listener = TcpListener::bind(format!("{}:{}", state.config.host, state.config.prot)).await?;
    axum::serve(listener, app).await?;
    
    Ok(())

}

async fn return_state(app_state: State<Arc<OneBotState>>) -> String{
    return format!("用户\n{:#?}\n群组:\n{:#?}", app_state.users, app_state.groups);
}

#[tokio::main]
async fn main() {
    match start_server().await {
        Ok(_) => return,
        Err(msg) => println!("{msg}")
    }
}
