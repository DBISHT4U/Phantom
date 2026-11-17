use crate::sites::Site;
use colored::*;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize)]
pub struct CheckResult {
    pub site: String,
    pub url: String,
    pub category: String,
    pub status: ResultStatus,
    pub http_code: Option<u16>,
    pub response_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum ResultStatus {
    Found,
    NotFound,
    Error(String),
}

impl std::fmt::Display for ResultStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResultStatus::Found => write!(f, "FOUND"),
            ResultStatus::NotFound => write!(f, "NOT FOUND"),
            ResultStatus::Error(e) => write!(f, "ERROR: {}", e),
        }
    }
}

pub async fn check_username(
    username: &str,
    sites: &[Site],
    timeout_secs: u64,
    max_concurrent: usize,
    proxy: Option<&str>,
    verbose: bool,
) -> Vec<CheckResult> {
    // Build HTTP client
    let mut client_builder = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .danger_accept_invalid_certs(false)
        .redirect(reqwest::redirect::Policy::limited(5))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36");

    if let Some(proxy_url) = proxy {
        if let Ok(p) = reqwest::Proxy::all(proxy_url) {
            client_builder = client_builder.proxy(p);
        }
    }

    let client = Arc::new(client_builder.build().unwrap_or_else(|_| Client::new()));

    // Progress bar
    let pb = ProgressBar::new(sites.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("  {spinner:.cyan} [{bar:40.cyan/dim}] {pos}/{len} ({percent}%) {msg}")
            .unwrap()
            .progress_chars("█▓░"),
    );
    pb.enable_steady_tick(Duration::from_millis(80));

    let username = username.to_string();

    // Run all checks concurrently with semaphore
    let results: Vec<CheckResult> = stream::iter(sites.iter().cloned())
        .map(|site| {
            let client = Arc::clone(&client);
            let pb = pb.clone();
            let username = username.clone();
            let verbose = verbose;

            async move {
                let result = check_site(&client, &site, &username).await;

                // Live output for found results
                if result.status == ResultStatus::Found {
                    pb.suspend(|| {
                        println!(
                            "  {} {} {}  ({}ms)",
                            "👻 FOUND".green().bold(),
                            result.site.white().bold(),
                            result.url.blue().underline(),
                            result.response_time_ms,
                        );
                    });
                } else if verbose {
                    match &result.status {
                        ResultStatus::NotFound => {
                            pb.suspend(|| {
                                println!(
                                    "  {} {}",
                                    "   miss".dimmed(),
                                    result.site.dimmed(),
                                );
                            });
                        }
                        ResultStatus::Error(e) => {
                            pb.suspend(|| {
                                println!(
                                    "  {} {} — {}",
                                    "  error".red(),
                                    result.site.dimmed(),
                                    e.dimmed(),
                                );
                            });
                        }
                        _ => {}
                    }
                }

                pb.inc(1);
                result
            }
        })
        .buffer_unordered(max_concurrent)
        .collect()
        .await;

    pb.finish_and_clear();
    results
}

async fn check_site(client: &Client, site: &Site, username: &str) -> CheckResult {
    let url = site.build_url(username);
    let probe_url = site.build_probe_url(username);
    let start = std::time::Instant::now();

    let response = match client.get(&probe_url).send().await {
        Ok(r) => r,
        Err(e) => {
            let elapsed = start.elapsed().as_millis() as u64;
            let err_msg = if e.is_timeout() {
                "timeout".to_string()
            } else if e.is_connect() {
                "connection failed".to_string()
            } else {
                format!("{}", e).chars().take(50).collect()
            };
            return CheckResult {
                site: site.name.clone(),
                url,
                category: site.category.clone(),
                status: ResultStatus::Error(err_msg),
                http_code: None,
                response_time_ms: elapsed,
            };
        }
    };

    let elapsed = start.elapsed().as_millis() as u64;
    let status_code = response.status().as_u16();

    let result_status = match site.check.as_str() {
        "status" => {
            if status_code == site.valid {
                ResultStatus::Found
            } else {
                ResultStatus::NotFound
            }
        }
        "body_contains" => {
            match response.text().await {
                Ok(body) => {
                    if let Some(ref pattern) = site.body_match {
                        if body.contains(pattern) {
                            ResultStatus::Found
                        } else {
                            ResultStatus::NotFound
                        }
                    } else {
                        ResultStatus::NotFound
                    }
                }
                Err(_) => ResultStatus::Error("body read failed".to_string()),
            }
        }
        "body_absent" => {
            match response.text().await {
                Ok(body) => {
                    if let Some(ref absent_text) = site.body_absent {
                        if body.to_lowercase().contains(&absent_text.to_lowercase()) {
                            ResultStatus::NotFound
                        } else if status_code >= 400 {
                            ResultStatus::NotFound
                        } else {
                            ResultStatus::Found
                        }
                    } else {
                        if status_code == site.valid {
                            ResultStatus::Found
                        } else {
                            ResultStatus::NotFound
                        }
                    }
                }
                Err(_) => ResultStatus::Error("body read failed".to_string()),
            }
        }
        _ => {
            if status_code == site.valid {
                ResultStatus::Found
            } else {
                ResultStatus::NotFound
            }
        }
    };

    CheckResult {
        site: site.name.clone(),
        url,
        category: site.category.clone(),
        status: result_status,
        http_code: Some(status_code),
        response_time_ms: elapsed,
    }
}
