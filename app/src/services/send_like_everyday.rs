use anyhow::Result;
use reqwest::Client;
use serde_json::{Value, json};
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn start_task(client: Client, sched: JobScheduler) -> Result<()> {
    let job = Job::new_async("0 34 15 * * *", move |_uuid, _l| {
        let client = client.clone();
        Box::pin(async move {
            let users = get_all_users(client.clone()).await;
            for user_id in users {
                send_like(client.clone(), user_id).await.unwrap();
            }
        })
    })?;

    sched.add(job).await?;
    Ok(())
}

async fn send_like(client: Client, user_id: u64) -> Result<()> {
    client
        .post("http://192.168.2.118:3000/send_like")
        .header("Authorization", "Bearer Qqwe123123")
        .json(&json!({
            "user_id": user_id,
            "times": 10,
        }))
        .send()
        .await?
        .text()
        .await?;
    Ok(())
}

async fn get_all_users(client: Client) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let message = client
        .post("http://192.168.2.118:3000/get_friend_list")
        .header("Authorization", "Bearer Qqwe123123")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let struct_json: Value = serde_json::from_str(&message).unwrap();
    for data in struct_json["data"].as_array().unwrap().into_iter() {
        result.push(data["user_id"].as_u64().unwrap());
    }
    result
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::time::sleep;
    use tokio_cron_scheduler::JobScheduler;

    #[tokio::test]
    pub async fn test_task() {
        let sched = JobScheduler::new().await.unwrap();
        let _ = super::start_task(reqwest::Client::new(), sched.clone()).await;
        let _ = sched.start().await;
        sleep(Duration::from_hours(1)).await;
    }

    #[tokio::test]
    pub async fn test_send_like() {
        let _ = super::send_like(reqwest::Client::new(), 280179863).await;
    }

    #[tokio::test]
    pub async fn test_get_all_users() {
        let result = super::get_all_users(reqwest::Client::new()).await;
        print!("{:#?}", result)
    }
}
