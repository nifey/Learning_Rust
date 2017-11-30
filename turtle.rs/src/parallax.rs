extern crate turtle;
extern crate rand;

use std::collections::VecDeque;
use self::rand::Rng;
use self::turtle::Turtle;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 600.0;
const MAX_BUILDING_WIDTH: f64 = 120.0;
const MIN_BUILDING_WIDTH: f64 = 50.0;

struct Building {
    pub building_width: f64,
    pub building_height: f64,
}

impl Building {
    fn new(min: f64, max: f64) -> Self {
        Building {
            building_width: rand::thread_rng().gen_range(
                MIN_BUILDING_WIDTH,
                MAX_BUILDING_WIDTH + 1.0,
            ) as f64,
            building_height: rand::thread_rng().gen_range(min, max + 1.0) as f64,
        }
    }
}

struct Buildings {
    building_color: String,
    building_speed: f64,
    buildings: VecDeque<Building>,
    height_min: f64,
    height_max: f64,
}

impl Buildings {
    fn new(color: &str, speed: f64, min: f64, max: f64) -> Self {
        let mut b: VecDeque<Building> = VecDeque::new();
        while total_width(&b) < WIDTH {
            b.push_back(Building::new(min, max));
        }
        Buildings {
            building_color: color.to_string(),
            building_speed: speed,
            buildings: b,
            height_min: min,
            height_max: max,
        }
    }
    fn display(&self, turtle: &mut Turtle) {
        turtle.go_to([-WIDTH / 2.0, -HEIGHT / 2.0]);
        turtle.set_fill_color(&self.building_color[..]);
        for b in self.buildings.iter() {
            turtle.begin_fill();
            turtle.forward(b.building_height);
            turtle.right(90.0);
            turtle.forward(b.building_width);
            turtle.right(90.0);
            turtle.forward(b.building_height);
            turtle.end_fill();
            turtle.right(180.0);
        }
    }
    fn update(&mut self) {
        let mut update_value = self.building_speed;
        let mut number_to_remove = 0;
        for b in self.buildings.iter_mut() {
            if b.building_width < update_value {
                update_value -= b.building_width;
                b.building_width = 0.0;
                number_to_remove += 1;
            } else {
                b.building_width -= update_value;
                break;
            }
        }
        for _ in 0..number_to_remove {
            self.buildings.pop_front();
        }
        while total_width(&self.buildings) < WIDTH {
            self.buildings.push_back(Building::new(
                self.height_min,
                self.height_max,
            ));
        }
    }
}

fn total_width(vec: &VecDeque<Building>) -> f64 {
    let mut len: f64 = 0.0;
    for b in vec.iter() {
        len += b.building_width;
    }
    len
}

pub fn parallax_main() {
    let mut turtle = Turtle::new();
    turtle.set_pen_size(0.0);
    turtle.hide();
    turtle.set_speed("instant");

    let mut background = Buildings::new("#C6E2FF", 3.0, 300.0, 500.0);
    let mut middle = Buildings::new("light blue", 6.0, 200.0, 400.0);
    let mut foreground = Buildings::new("blue", 13.0, 50.0, 300.0);

    loop {
        background.display(&mut turtle);
        middle.display(&mut turtle);
        foreground.display(&mut turtle);
        turtle.wait(0.05);
        turtle.clear();
        background.update();
        middle.update();
        foreground.update();
    }
}
