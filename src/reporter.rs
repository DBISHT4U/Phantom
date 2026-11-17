use crate::checker::{CheckResult, ResultStatus};
use crate::sites;
use anyhow::Result;
use colored::*;
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct FoundRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Site")]
    site: String,
    #[tabled(rename = "Category")]
    category: String,
    #[tabled(rename = "URL")]
    url: String,
    #[tabled(rename = "Speed")]
    speed: String,
}

#[derive(Serialize)]
struct JsonReport {
    username: String,
    scan_time_secs: f64,
    total_sites: usize,
    found: usize,
    not_found: usize,
    errors: usize,
    results: Vec<CheckResult>,
}

pub fn print_results(username: &str, results: &[CheckResult], found_only: bool, plain: bool) {
    let found: Vec<&CheckResult> = results
        .iter()
        .filter(|r| r.status == ResultStatus::Found)
        .collect();

    if found.is_empty() {
        println!(
            "\n  {} No accounts found for '{}'",
            "😔".to_string(),
            username
        );
        return;
    }

    if plain {
        println!("\nFound accounts for '{}':", username);
        for (i, r) in found.iter().enumerate() {
            println!("  {}. [{}] {} — {}", i + 1, r.category, r.site, r.url);
        }
        return;
    }

    println!();

    // Table of found results
    let rows: Vec<FoundRow> = found
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let icon = sites::category_icon(&r.category);
            FoundRow {
                index: i + 1,
                site: r.site.clone(),
                category: format!("{} {}", icon, r.category),
                url: r.url.clone(),
                speed: format!("{}ms", r.response_time_ms),
            }
        })
        .collect();

    let table = Table::new(&rows).to_string();
    println!("{}", table);

    if !found_only {
        // Category breakdown
        let mut cat_counts: HashMap<String, usize> = HashMap::new();
        for r in &found {
            *cat_counts.entry(r.category.clone()).or_insert(0) += 1;
        }

        println!("\n  {}", "📂 By Category:".dimmed());
        let mut sorted: Vec<_> = cat_counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        for (cat, count) in &sorted {
            let icon = sites::category_icon(cat);
            print!("    {} {}:{} ", icon, cat, count.to_string().white().bold());
        }
        println!();
    }
}

pub fn print_summary(username: &str, results: &[CheckResult], elapsed: Duration) {
    let found = results
        .iter()
        .filter(|r| r.status == ResultStatus::Found)
        .count();
    let not_found = results
        .iter()
        .filter(|r| r.status == ResultStatus::NotFound)
        .count();
    let errors = results
        .iter()
        .filter(|r| matches!(r.status, ResultStatus::Error(_)))
        .count();

    let avg_speed = if !results.is_empty() {
        results.iter().map(|r| r.response_time_ms).sum::<u64>() / results.len() as u64
    } else {
        0
    };

    println!(
        "\n  {} {} scanned {} sites in {:.1}s",
        "⚡".to_string(),
        "PHANTOM".cyan().bold(),
        results.len().to_string().white().bold(),
        elapsed.as_secs_f64(),
    );
    println!(
        "  {} {} found  {} {} not found  {} {} errors  {} avg {}ms",
        "👻".to_string(),
        found.to_string().green().bold(),
        "│".dimmed(),
        not_found.to_string().dimmed(),
        "│".dimmed(),
        errors.to_string().yellow(),
        "│".dimmed(),
        avg_speed,
    );
}

pub fn export_json(
    username: &str,
    results: &[CheckResult],
    elapsed: Duration,
    path: &str,
) -> Result<()> {
    let found_count = results.iter().filter(|r| r.status == ResultStatus::Found).count();
    let not_found_count = results.iter().filter(|r| r.status == ResultStatus::NotFound).count();
    let error_count = results.iter().filter(|r| matches!(r.status, ResultStatus::Error(_))).count();

    let report = JsonReport {
        username: username.to_string(),
        scan_time_secs: elapsed.as_secs_f64(),
        total_sites: results.len(),
        found: found_count,
        not_found: not_found_count,
        errors: error_count,
        results: results.to_vec(),
    };

    let json = serde_json::to_string_pretty(&report)?;
    std::fs::write(path, json)?;
    Ok(())
}

pub fn export_csv(username: &str, results: &[CheckResult], path: &str) -> Result<()> {
    let mut csv = String::from("username,site,category,url,status,http_code,response_ms\n");

    for r in results {
        if r.status == ResultStatus::Found {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                username,
                r.site,
                r.category,
                r.url,
                r.status,
                r.http_code.unwrap_or(0),
                r.response_time_ms,
            ));
        }
    }

    std::fs::write(path, csv)?;
    Ok(())
}
