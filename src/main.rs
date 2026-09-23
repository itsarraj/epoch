use anyhow::{Context, Result};
use chrono::{DateTime, Local, TimeZone, Utc};
use clap::Parser;
use colored::*;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Unix timestamp or formatted date (if omitted, prints current time)
    input: Option<String>,

    /// Print output in UTC instead of local time
    #[arg(short, long)]
    utc: bool,
}

fn print_datetime<Tz: TimeZone>(dt: DateTime<Tz>, tz_name: &str)
where
    Tz::Offset: std::fmt::Display,
{
    println!("{}: {}", "RFC 3339".bold(), dt.to_rfc3339().cyan());
    println!("{}: {}", "RFC 2822".bold(), dt.to_rfc2822().cyan());
    println!(
        "{}: {} {}",
        "Human".bold(),
        dt.format("%Y-%m-%d %H:%M:%S").to_string().green(),
        tz_name.green()
    );
    println!(
        "{}: {}",
        "Unix Epoch".bold(),
        dt.timestamp().to_string().yellow()
    );
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(ref input) = args.input {
        // Try parsing as timestamp first
        if let Ok(ts) = input.parse::<i64>() {
            let dt_utc = Utc
                .timestamp_opt(ts, 0)
                .single()
                .context("Invalid timestamp")?;
            if args.utc {
                print_datetime(dt_utc, "UTC");
            } else {
                print_datetime(dt_utc.with_timezone(&Local), "Local");
            }
        } else {
            // Try parsing as RFC3339 or similar date
            let dt = DateTime::parse_from_rfc3339(input)
                .or_else(|_| DateTime::parse_from_rfc2822(input))
                .context("Could not parse input as timestamp or known date format")?;

            if args.utc {
                print_datetime(dt.with_timezone(&Utc), "UTC");
            } else {
                print_datetime(dt.with_timezone(&Local), "Local");
            }
        }
    } else {
        // Print current time
        let now_utc = Utc::now();
        if args.utc {
            print_datetime(now_utc, "UTC");
        } else {
            print_datetime(Local::now(), "Local");
        }
    }

    Ok(())
}
