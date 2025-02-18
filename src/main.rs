use application::Application;
fn main() {
    let mut app = Application::new();
    app.run();
}
mod application;
mod data_collection;
mod models;
mod enums;
mod traits;
