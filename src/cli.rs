use chrono::{NaiveDate, Utc};
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Near {
        #[arg(short, long, value_parser = parse_date)]
        date: NaiveDate,

        #[arg(short, long, default_value_t = 2)]
        context: u64,
    },
    Next {
        #[arg(value_parser = clap::value_parser!(i64).range(1..))]
        amount: i64,
    },
    Since {
        #[arg(short, long, value_parser = before_today)]
        date: NaiveDate,
    },
    Until {
        #[arg(short, long, value_parser = after_today)]
        date: NaiveDate,
    },
}

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| e.to_string())
}

fn before_today(s: &str) -> Result<NaiveDate, String> {
    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| e.to_string())?;
    let today = Utc::now().naive_utc().date();
    if date >= today {
        Err(format!(
            "date must be before today (`{}`)",
            today.format("%Y-%m-%d")
        ))
    } else {
        Ok(date)
    }
}

fn after_today(s: &str) -> Result<NaiveDate, String> {
    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| e.to_string())?;
    let today = Utc::now().naive_utc().date();
    if date <= today {
        Err(format!(
            "date must be after today (`{}`)",
            today.format("%Y-%m-%d")
        ))
    } else {
        Ok(date)
    }
}
