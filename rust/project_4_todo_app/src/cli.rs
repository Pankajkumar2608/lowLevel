use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustodo")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        title: String,
        #[arg(short, long, default_value_t = 1)]
        priority: u8,
    },
    List,
    Done {
        id: String,
    },
    Delete {
        id: String,
    },
}