extern crate turtle;
use turtle::Turtle;
fn main() {
    let mut tur = Turtle::new();
    let sides: i32 = 4;
    let rotations: i32 = 20;
    let turn_angle: f64 = 360.0 / (sides as f64);
    let rotate_angle: f64 = 360.0 / (rotations as f64);
    for _ in 0..rotations {
        for _ in 0..sides {
            tur.forward(100.0);
            tur.right(turn_angle);
        }
        tur.right(rotate_angle as f64);
    }
}
