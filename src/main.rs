use kiss2d::clrs::*;
use kiss2d::{Canvas, Key, minifb};
use noise::{Perlin, NoiseFn, Fbm, Worley, OpenSimplex, Value, MultiFractal, RangeFunction};

use rand::{SeedableRng};
use rand::prelude::Distribution;

const WIDTH: usize = 400;
const HEIGHT: usize = 300;

static TITLE: &str = "Noise Test - Press ESC to exit";

fn color_selector(value: f64) -> u32 {
    if value <= -1.0 {
        return GRAY;
    } else if -1.0 < value && value <= -0.75 {
        return NAVY;
    } else if -0.75 < value && value <= -0.5 {
        return BLUE;
    } else if -0.5 < value && value <= -0.25 {
        return AQUA;
    } else if -0.25 < value && value <= 0.0 {
        return TEAL;
    } else if 0.0 < value && value <= 0.25 {
        return OLIVE;
    } else if 0.25 < value && value <= 0.5 {
        return GREEN;
    } else if 0.5 < value && value <= 0.75 {
        return LIME;
    } else if 0.75 < value && value <= 1.0 {
        return YELLOW;
    } else if value > 1.0 {
        return ORANGE;
    }
    RED
}

fn is_tree_zone(value: f64) -> bool {
    return value > 0.4;
}

// add poison disc sampling via own random numbers over multiple tiles, try poisson per tile and see if aritifacts are
// visible when rendering multiple tiles (poission via elimination of random points

fn main() -> minifb::Result<()> {
    let trees = Fbm::new().set_octaves(6).set_frequency(0.001).set_lacunarity(2.09).set_persistence(1.0);
//    let fbm = Fbm::new().set_octaves(6).set_frequency(1.0).set_lacunarity(2.09).set_persistence(1.0);
    let range = rand::distributions::Uniform::new(0.0, 1.0);
    let mut rng = rand::thread_rng();

    let worley = Worley::new().set_frequency(0.001).set_displacement(1.0).enable_range(true).set_range_function(RangeFunction::Manhattan);
    let mut canvas = Canvas::new(TITLE, WIDTH, HEIGHT)?;
    let mut zoom = 1.0;
    while canvas.is_open() && !canvas.is_keydown(Key::Escape) {
        for x in (0..WIDTH).step_by(10) {
            for y in (0..HEIGHT).step_by(10) {
                let point = [x as f64 / zoom, y as f64 / zoom];
                let tree_value = trees.get(point);
                if is_tree_zone(tree_value) {
                    let x_offset = range.sample(&mut rng);
                    let y_offset = range.sample(&mut rng);
                    let rotation = range.sample(&mut rng);
                    canvas.pixel(x, y, GREEN);
                }
            }
        }
        canvas.udpate();

        canvas.keys(|t| match t {
            Key::Z => zoom = zoom * 2.0,
            Key::X => zoom = zoom / 2.0,
            Key::P => {
                println!("FBM: frequency: {}, lacun: {}, octav: {}, persist: {}\n", trees.frequency, trees.lacunarity, trees.octaves, trees.persistence);
                println!("WORLEY: range_enabled: {}, frequency: {}, displacement: {}\n", worley.enable_range, worley.frequency, worley.displacement);
                println!("OTHER: zoom: {}", zoom);
            },
            _ => (),
        });

        canvas.redraw()?;
    }

    Ok(())
}

