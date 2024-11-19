use daemon::Daemon;

mod config;
mod daemon;
mod ipc;
mod niri_ipc;
mod niri_tracker;
mod rules_common;
mod window_rules;

use clap::Parser;

// TODO:
// * Handle parse and send commands
// * Add pid window matching

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = false)]
    daemon: bool,

    #[arg(long)]
    in_current_column: Option<bool>,

    #[arg(long)]
    in_column: Option<u64>,

    #[arg(long)]
    column: Option<u64>,
}

fn main() {
    let args = Args::parse();

    if args.daemon {
        Daemon::run();
        return;
    }

    if let Some(true) = args.in_current_column {
        println!("Should send command to open in current column");
    } else if let Some(column) = args.in_column {
        println!("Should send command to open in column {}", column);
    } else if let Some(column) = args.column {
        println!("Should send command to open at column {}", column);
    }
}
