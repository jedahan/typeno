mod screen;
use screen::Screen;

fn main() -> Result<(), std::io::Error> {
    let _screen = Screen::new();
    Ok(())
}
