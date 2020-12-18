use kiss2d::clrs::*;
use kiss2d::{Canvas, Key, minifb};
use noise::{Perlin, NoiseFn, Fbm, Worley, OpenSimplex, Value, MultiFractal};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

static TITLE: &str = "Noise Test - Press ESC to exit";

fn main() -> minifb::Result<()> {
    let noise = Fbm::new().set_octaves(6).set_frequency(0.001).set_lacunarity(2.09).set_persistence(1.0);
    let mut canvas = Canvas::new(TITLE, WIDTH, HEIGHT)?;
    let mut zoom = 1.0;
    let mut x_offset: isize = 0;
    let mut y_offset: isize = 0;
    while canvas.is_open() && !canvas.is_keydown(Key::Escape) {
        for x in (0..WIDTH).step_by(10) {
            for y in (0..HEIGHT).step_by(10) {
                let n = noise.get([(x_offset + x as isize) as f64 / zoom, (y_offset + y as isize) as f64 / zoom]);
                if -1.0 <= n && n < -0.75  {
                    canvas.circle((x as isize, y as isize), 3, NAVY);
                } else if -0.75 < n && n < -0.5 {
                    canvas.circle((x as isize, y as isize), 3, BLUE);
                } else if -0.5 < n && n < -0.25 {
                    canvas.circle((x as isize, y as isize), 3, AQUA);
                } else if -0.25 < n && n < 0.0 {
                    canvas.circle((x as isize, y as isize), 3, TEAL);
                } else if 0.0 < n && n < 0.25 {
                    canvas.circle((x as isize, y as isize), 3, OLIVE);
                } else if 0.25 < n && n < 0.5 {
                    canvas.circle((x as isize, y as isize), 3, GREEN);
                } else if 0.5 < n && n < 0.75 {
                    canvas.circle((x as isize, y as isize), 3, LIME);
                } else if 0.75 < n && n <= 1.0 {
                    canvas.circle((x as isize, y as isize), 3, YELLOW);
                }
                else {
                    canvas.circle((x as isize, y as isize), 3, RED);
                }
            }
        }
        canvas.udpate();

        canvas.keys(|t| match t {
            Key::Z => zoom = zoom * 10.0,
            Key::X => zoom = zoom / 10.0,
            Key::W => y_offset += 250,
            Key::S => y_offset -= 250,
            Key::D => x_offset += 250,
            Key::A => x_offset -= 250,
            Key::P => println!("frequency: {}, lacun: {}, octav: {}, persist: {}, zoom: {}\n", noise.frequency, noise.lacunarity, noise.octaves, noise.persistence, zoom),
            _ => (),
        });

        canvas.redraw()?;
    }

    Ok(())
}

