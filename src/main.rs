mod screen;
use screen::Screen;

fn main() -> Result<(), std::io::Error> {
    let mut screen = Screen::new();
    Ok(())
}
