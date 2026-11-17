use anyhow::{Context, Result};
use colored::*;
use serde::Deserialize;
use std::collections::HashMap;

const EMBEDDED_SITES: &str = include_str!("../data/sites.json");

#[derive(Debug, Clone, Deserialize)]
pub struct Site {
    pub name: String,
    pub url: String,
    pub check: String,
    pub category: String,
    #[serde(default = "default_valid")]
    pub valid: u16,
    #[serde(default = "default_invalid")]
    pub invalid: u16,
    pub url_probe: Option<String>,
    pub body_match: Option<String>,
    pub body_absent: Option<String>,
}

fn default_valid() -> u16 { 200 }
fn default_invalid() -> u16 { 404 }

impl Site {
    /// Build the URL for a given username
    pub fn build_url(&self, username: &str) -> String {
        self.url.replace("{}", username)
    }

    /// Build the probe URL (for body-check sites that use an API)
    pub fn build_probe_url(&self, username: &str) -> String {
        if let Some(ref probe) = self.url_probe {
            probe.replace("{}", username)
        } else {
            self.build_url(username)
        }
    }
}

pub fn load_embedded() -> Result<Vec<Site>> {
    let sites: Vec<Site> = serde_json::from_str(EMBEDDED_SITES)
        .context("Failed to parse embedded sites database")?;
    Ok(sites)
}

pub fn load_from_file(path: &str) -> Result<Vec<Site>> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read sites file: {}", path))?;
    let sites: Vec<Site> = serde_json::from_str(&content)
        .with_context(|| "Failed to parse sites JSON")?;
    Ok(sites)
}

pub fn print_categories(sites: &[Site]) {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for site in sites {
        *counts.entry(site.category.clone()).or_insert(0) += 1;
    }

    println!("  {}\n", "📂 Available Categories".cyan().bold());

    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    for (cat, count) in &sorted {
        let icon = category_icon(cat);
        println!("    {} {:12} {} sites", icon, cat.white().bold(), count.to_string().dimmed());
    }
    println!("\n  Total: {} sites", sites.len().to_string().green().bold());
    println!("  Use: {} to filter", "--category social,dev".cyan());
}

pub fn print_sites(sites: &[Site]) {
    println!("  {}\n", "🌐 Supported Sites".cyan().bold());

    let mut by_category: HashMap<String, Vec<&Site>> = HashMap::new();
    for site in sites {
        by_category.entry(site.category.clone()).or_default().push(site);
    }

    let mut cats: Vec<_> = by_category.keys().cloned().collect();
    cats.sort();

    for cat in &cats {
        let icon = category_icon(cat);
        println!("  {} {}:", icon, cat.to_uppercase().white().bold());
        if let Some(cat_sites) = by_category.get(cat) {
            for site in cat_sites {
                println!("    {} {}", "•".dimmed(), site.name);
            }
        }
        println!();
    }
    println!("  Total: {} sites", sites.len().to_string().green().bold());
}

pub fn category_icon(cat: &str) -> &'static str {
    match cat {
        "social" => "👥",
        "dev" => "💻",
        "gaming" => "🎮",
        "media" => "🎵",
        "forum" => "💬",
        "creative" => "🎨",
        "security" => "🔒",
        _ => "🔗",
    }
}
