mod app;
mod math;
mod processes;
mod ui;

use crate::app::App;

fn main() {
    if let Err(err) = ratatui::run(|term| App::new().run(term)) {
        eprintln!("{:?}", err);
    }

    println!("Goodbye!");
}
