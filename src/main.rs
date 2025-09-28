
fn main() -> typ::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = typ::App::new()?;

    let result = app.run(&mut terminal);

    ratatui::restore();
    result
}
