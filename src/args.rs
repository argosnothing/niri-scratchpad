use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Action {
    Create { scratchpad_number: i32 },
    Delete { scratchpad_number: i32 },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}
