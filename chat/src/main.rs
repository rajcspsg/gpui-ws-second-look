mod chat;
mod ownership;

use gpui::*;

fn main() {
    let app = gpui::App::new();
    chat::run_app(app);
    //ownership::run_app(app);
}
