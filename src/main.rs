mod app;
mod processes;
mod ui;
mod utils;

use crate::app::App;

fn main() {
    if let Err(err) = ratatui::run(|term| App::new().run(term)) {
        eprintln!("{:?}", err);
    }

    println!("Goodbye!");
}
