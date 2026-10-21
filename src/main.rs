mod passage;
mod app;
mod ui;

fn main() -> std::io::Result<()> {
    ratatui::run(app::run)
}

// i should prolly make a mod file 