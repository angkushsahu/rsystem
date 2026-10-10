mod app;

use crate::app::App;

fn main() {
    // todo
    if let Err(err) = ratatui::run(|term| App::new().run(term)) {
        println!("{:?}", err);
    }

    println!("Goodbye!");
}
