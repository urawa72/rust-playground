use std::sync::Arc;

use anyhow::{Context, Result};
use futures::future::try_join_all;
use reqwest::header;
use serde_json::Value;
use tokio::task::JoinHandle;

async fn get_star_count(client: &reqwest::Client, repo: String) -> Result<u64> {
    let resp: Value = client
        .get(&format!("https://api.github.com/repos/{}", repo))
        .send()
        .await?
        .json()
        .await?;
    let count = resp
        .get("stargazers_count")
        .context("GitHub API error: stargazers_count is not found")?
        .as_u64()
        .context("GitHub API error: stargazers_count is not an integer")?;
    Ok(count)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    let client = reqwest::Client::builder()
        .user_agent("rust reqwest")
        .default_headers(headers)
        .build()?;

    let client = Arc::new(client);

    let repos = vec!["rust-lang/libc".to_string(), "rust-lang/log".to_string()];

    let handles: Vec<JoinHandle<Result<u64>>> = repos
        .iter()
        .map(|repo| {
            let client = client.clone();
            let repo = repo.clone();
            tokio::spawn(async move { get_star_count(&client, repo).await })
        })
        .collect::<Vec<_>>();

    let stars: Vec<u64> = try_join_all(handles)
        .await?
        .into_iter()
        .collect::<Result<Vec<u64>>>()?;

    for (repo, star) in repos.iter().zip(stars) {
        println!("{} has {} stars", repo, star);
    }

    Ok(())
}
