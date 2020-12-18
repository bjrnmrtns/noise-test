use kiss2d::clrs::*;
use kiss2d::{Canvas, Key, minifb};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

static TITLE: &str = "Noise Test - Press ESC to exit";

fn main() -> minifb::Result<()> {
    let mut canvas = Canvas::new(TITLE, WIDTH, HEIGHT)?;
    while canvas.is_open() && !canvas.is_keydown(Key::Escape) {
        for x in (0..WIDTH).step_by(10) {
            for y in (0..HEIGHT).step_by(10) {
                canvas.circle((x as isize, y as isize), 3, RED);
            }
        }
        canvas.udpate();

        canvas.keys(|t| match t {
            Key::W => println!("holding w!"),
            _ => (),
        });

        canvas.redraw()?;
    }

    Ok(())
}

