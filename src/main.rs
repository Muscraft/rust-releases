use chrono::NaiveDate;
use clap::Parser;
use comfy_table::{Cell, CellAlignment, Table};

use crate::cli::{Cli, Commands};

mod cli;
mod release;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Near { date, context }) => {
            let diff = release_diff(date);
            between(
                diff.saturating_sub_unsigned(context),
                diff.saturating_add_unsigned(context),
            );
        }
        Some(Commands::Next { amount }) => {
            next(amount);
        }
        Some(Commands::Since { date }) => {
            between(release_diff(date), 0);
        }
        Some(Commands::Until { date }) => {
            between(0, release_diff(date));
        }
        None => {
            next(9);
        }
    }
}

fn release_diff(date: NaiveDate) -> i64 {
    let stable = release::Release::new(0);
    date.signed_duration_since(stable.stable_on).num_weeks() / 6
}

fn between(start: i64, end: i64) {
    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("ΔS").set_alignment(CellAlignment::Center),
        Cell::new("Version").set_alignment(CellAlignment::Center),
        Cell::new("Stable Date").set_alignment(CellAlignment::Center),
        Cell::new("Δt").set_alignment(CellAlignment::Center),
        Cell::new("Branch Date").set_alignment(CellAlignment::Center),
        Cell::new("Δt").set_alignment(CellAlignment::Center),
    ]);
    for i in start..=end {
        let release = release::Release::new(i);
        table.add_row(vec![
            Cell::new(i).set_alignment(CellAlignment::Left),
            Cell::new(release.version.to_string()).set_alignment(CellAlignment::Center),
            Cell::new(release.stable_on.format("%b %d %Y").to_string())
                .set_alignment(CellAlignment::Right),
            Cell::new(release.time_stable()).set_alignment(CellAlignment::Right),
            Cell::new(release.branch_on.format("%b %d %Y").to_string())
                .set_alignment(CellAlignment::Right),
            Cell::new(release.time_branch()).set_alignment(CellAlignment::Right),
        ]);
    }
    println!("{table}");
}

fn next(amount: i64) {
    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("Channel").set_alignment(CellAlignment::Left),
        Cell::new("Version").set_alignment(CellAlignment::Center),
        Cell::new("Stable Date").set_alignment(CellAlignment::Center),
        Cell::new("Δt").set_alignment(CellAlignment::Center),
        Cell::new("Branch Date").set_alignment(CellAlignment::Center),
        Cell::new("Δt").set_alignment(CellAlignment::Center),
    ]);

    for i in 0..=amount {
        let release = release::Release::new(i);
        let name = match i {
            0 => "Stable".to_string(),
            1 => "Beta".to_string(),
            2 => "Nightly".to_string(),
            _ => format!("Nightly +{}", i - 2),
        };
        table.add_row(vec![
            Cell::new(name).set_alignment(CellAlignment::Left),
            Cell::new(release.version.to_string()).set_alignment(CellAlignment::Center),
            Cell::new(release.stable_on.format("%b %d %Y").to_string())
                .set_alignment(CellAlignment::Right),
            Cell::new(release.time_stable()).set_alignment(CellAlignment::Right),
            Cell::new(release.branch_on.format("%b %d %Y").to_string())
                .set_alignment(CellAlignment::Right),
            Cell::new(release.time_branch()).set_alignment(CellAlignment::Right),
        ]);
    }

    println!("{table}");
}
