extern crate turtle;
use self::turtle::Turtle;
pub fn koch_main() {
    //Create a new turtle
    let mut turtle = Turtle::new();

    //Set koch curve generation and lenght of side
    let mut generation = 0;
    let side: f64 = 300.0;

    //Set pen size, speed and visibility of turtle
    turtle.set_pen_size(1.0);
    turtle.set_speed("instant");
    turtle.hide();

    //Moving to initial position
    turtle.pen_up();
    turtle.go_to([0.0, 1.732 * (side / 4.0)]);
    turtle.right(150.0);
    turtle.pen_down();

    while generation < 7 {
        turtle.clear();
        for _ in 0..3 {
            koch(&mut turtle, side, generation);
            turtle.right(120.0);
        }
        turtle.wait_for_click();
        generation += 1;
    }
}
fn koch(mut turtle: &mut Turtle, side: f64, generation: i32) {
    if generation == 0 {
        turtle.forward(side);
    } else {
        koch(&mut turtle, side / 3.0, generation - 1);
        turtle.left(60.0);
        koch(&mut turtle, side / 3.0, generation - 1);
        turtle.right(120.0);
        koch(&mut turtle, side / 3.0, generation - 1);
        turtle.left(60.0);
        koch(&mut turtle, side / 3.0, generation - 1);
    }
}
