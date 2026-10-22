mod passage;
mod app;
mod ui;
pub mod engine;

fn main() -> std::io::Result<()> {
    ratatui::run(app::run)
}