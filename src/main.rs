mod passage;
mod app;
mod ui;
mod color_the_words;
pub mod engine;

fn main() -> std::io::Result<()> {
    ratatui::run(app::run)
}