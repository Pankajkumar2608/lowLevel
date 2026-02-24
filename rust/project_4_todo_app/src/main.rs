mod cli;
mod task;
mod storage;
mod app;

use clap::Parser;
use cli::{Cli, Commands};
use app::TodoApp;
use uuid::Uuid;

fn main() {
    let cli = Cli::parse();
    let mut app = TodoApp::new();

    match cli.command {
        Commands::Add { title, priority } => {
            app.add(title, priority);
        }
        Commands::List => {
            for task in app.tasks {
                println!("{:?}", task);
            }
        }
        Commands::Done { id } => {
            let uuid = Uuid::parse_str(&id).unwrap();
            app.complete(uuid);
        }
        Commands::Delete { id } => {
            let uuid = Uuid::parse_str(&id).unwrap();
            app.delete(uuid);
        }
    }
}