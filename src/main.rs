use std::{os::unix::process::CommandExt, process::Command};

use daemon::Daemon;

mod config;
mod daemon;
mod ipc;
mod niri_ipc;
mod niri_tracker;
mod process;
mod rules_common;
mod window_rules;

use clap::Parser;
use ipc::Ipc;
use rules_common::RuleLifetime;
use window_rules::WindowRule;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = false)]
    daemon: bool,

    #[arg(long)]
    in_current_column: bool,

    #[arg(long)]
    in_column: Option<u64>,

    #[arg(long)]
    column: Option<u64>,

    #[arg(long)]
    close: bool,

    #[arg(long)]
    fixed_width: Option<i32>,

    command: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let pid = std::process::id();

    if args.daemon {
        Daemon::run();
        return;
    }

    let mut window_rule = WindowRule::default();
    window_rule.pid = Some(pid);
    window_rule.rule_lifetime = Some(RuleLifetime::Matches(1));

    if args.in_current_column {
        window_rule.in_current_column = Some(args.in_current_column);
    }
    if args.in_column.is_some() {
        window_rule.in_column = args.in_column;
    }
    if args.column.is_some() {
        window_rule.column = args.column;
    }
    if args.close {
        window_rule.close = Some(args.close);
    }
    if args.fixed_width.is_some() {
        window_rule.fixed_width = args.fixed_width;
    }

    Ipc::register_window_rule(window_rule);

    if let Some(command) = args.command.first() {
        let _ = Command::new(command)
            .args(args.command.into_iter().skip(1))
            .exec();
        println!("Failed to execute command")
    }
}
