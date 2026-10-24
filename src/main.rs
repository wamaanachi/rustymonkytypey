mod passage;
mod app;
mod ui;
mod color_the_words;
mod results;
mod theme;
mod themes;
pub mod engine;

fn main() -> std::io::Result<()> {
    ratatui::run(app::run)
}