#![allow(unused_imports, dead_code, clippy::needless_return)]

mod checker;
mod reporter;
mod sites;

use anyhow::Result;
use clap::Parser;
use colored::*;
use std::time::Instant;

#[derive(Parser)]
#[command(
    name = "phantom",
    version,
    about = "👻 Phantom — Hunt usernames across 400+ sites at the speed of Rust",
    long_about = "Blazing-fast username OSINT tool. Find anyone's digital footprint\nacross social media, dev platforms, gaming, forums, and more.\n\nBuilt with Rust for maximum speed — scans 400+ sites in seconds.",
    after_help = "EXAMPLES:\n  phantom john_doe\n  phantom john_doe jane_doe --category social\n  phantom john_doe --timeout 10 --output results.json\n  phantom john_doe --category dev,gaming --csv\n  phantom john_doe --verbose --proxy socks5://127.0.0.1:9050"
)]
struct Cli {
    /// Username(s) to hunt (space-separated)
    #[arg(required = true)]
    usernames: Vec<String>,

    /// Filter by category: social, dev, gaming, media, forum, creative, security, other
    #[arg(short, long, value_delimiter = ',')]
    category: Option<Vec<String>>,

    /// Request timeout in seconds
    #[arg(short, long, default_value = "8")]
    timeout: u64,

    /// Max concurrent requests
    #[arg(long, default_value = "50")]
    workers: usize,

    /// Export results to JSON file
    #[arg(short, long)]
    output: Option<String>,

    /// Export results to CSV
    #[arg(long)]
    csv: Option<String>,

    /// Show only found results (hide errors/not-found)
    #[arg(short, long)]
    found_only: bool,

    /// Show detailed request info
    #[arg(short, long)]
    verbose: bool,

    /// HTTP/SOCKS5 proxy URL
    #[arg(long)]
    proxy: Option<String>,

    /// Path to custom sites JSON file
    #[arg(long)]
    sites: Option<String>,

    /// Show results as plain text (no colors/table)
    #[arg(long)]
    plain: bool,

    /// List all available categories and site counts
    #[arg(long)]
    list_categories: bool,

    /// List all supported sites
    #[arg(long)]
    list_sites: bool,
}

fn print_banner() {
    let banner = r#"
    ██████╗ ██╗  ██╗ █████╗ ███╗   ██╗████████╗ ██████╗ ███╗   ███╗
    ██╔══██╗██║  ██║██╔══██╗████╗  ██║╚══██╔══╝██╔═══██╗████╗ ████║
    ██████╔╝███████║███████║██╔██╗ ██║   ██║   ██║   ██║██╔████╔██║
    ██╔═══╝ ██╔══██║██╔══██║██║╚██╗██║   ██║   ██║   ██║██║╚██╔╝██║
    ██║     ██║  ██║██║  ██║██║ ╚████║   ██║   ╚██████╔╝██║ ╚═╝ ██║
    ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝   ╚═╝    ╚═════╝ ╚═╝     ╚═╝
    "#;
    println!("{}", banner.cyan().bold());
    println!(
        "    {} {}\n",
        "👻 Hunt usernames across the internet".white().bold(),
        format!("v{}", env!("CARGO_PKG_VERSION")).dimmed()
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if !cli.plain {
        print_banner();
    }

    // Load sites database
    let all_sites = if let Some(ref path) = cli.sites {
        sites::load_from_file(path)?
    } else {
        sites::load_embedded()?
    };

    // List categories
    if cli.list_categories {
        sites::print_categories(&all_sites);
        return Ok(());
    }

    // List all sites
    if cli.list_sites {
        sites::print_sites(&all_sites);
        return Ok(());
    }

    // Filter by category
    let filtered_sites = if let Some(ref cats) = cli.category {
        all_sites
            .into_iter()
            .filter(|s| cats.iter().any(|c| c.eq_ignore_ascii_case(&s.category)))
            .collect()
    } else {
        all_sites
    };

    let site_count = filtered_sites.len();

    // Process each username
    for username in &cli.usernames {
        println!(
            "  {} Hunting {} across {} sites...\n",
            "🔍".to_string(),
            username.green().bold(),
            site_count.to_string().cyan().bold()
        );

        let start = Instant::now();

        let results = checker::check_username(
            username,
            &filtered_sites,
            cli.timeout,
            cli.workers,
            cli.proxy.as_deref(),
            cli.verbose,
        )
        .await;

        let elapsed = start.elapsed();

        // Report
        reporter::print_results(username, &results, cli.found_only, cli.plain);
        reporter::print_summary(username, &results, elapsed);

        // Export JSON
        if let Some(ref path) = cli.output {
            reporter::export_json(username, &results, elapsed, path)?;
            println!(
                "\n  {} JSON saved to {}",
                "📁".to_string(),
                path.white().bold()
            );
        }

        // Export CSV
        if let Some(ref path) = cli.csv {
            reporter::export_csv(username, &results, path)?;
            println!(
                "  {} CSV saved to {}",
                "📁".to_string(),
                path.white().bold()
            );
        }

        println!();
    }

    Ok(())
}
