use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Enter the wavelength as argument to the program");
    } else {
        println!("{:?}", wavelength_to_rgb(args[1].trim().parse().unwrap()));
    }
}

const GAMMA: f32 = 0.80;
const INTENSITYMAX: f32 = 255.0;
fn wavelength_to_rgb(nm: i32) -> (u8, u8, u8) {
    let blue: f32;
    let factor: f32;
    let green: f32;
    let red: f32;

    match nm {
        380...439 => {
            red = -(nm - 440) as f32 / (440 - 380) as f32;
            green = 0.0;
            blue = 1.0;
        }
        440...489 => {
            red = 0.0;
            green = (nm - 440) as f32 / (490 - 440) as f32;
            blue = 1.0;
        }
        490...509 => {
            red = 0.0;
            green = 1.0;
            blue = -(nm - 510) as f32 / (510 - 490) as f32;
        }
        510...579 => {
            red = (nm - 510) as f32 / (580 - 510) as f32;
            green = 1.0;
            blue = 0.0;
        }
        580...644 => {
            red = 1.0;
            green = -(nm - 645) as f32 / (645 - 580) as f32;
            blue = 0.0;
        }
        645...780 => {
            red = 1.0;
            green = 0.0;
            blue = 0.0;
        }
        _ => {
            red = 0.0;
            green = 0.0;
            blue = 0.0;
        }
    }
    match nm {
        380...419 => {
            factor = 0.3 + 0.7 * (nm - 380) as f32 / (420 - 380) as f32;
        } 
        420...700 => {
            factor = 1.0;
        } 
        701...780 => {
            factor = 0.3 + 0.7 * (780 - nm) as f32 / (780 - 700) as f32;
        } 
        _ => {
            factor = 0.0;
        } 
    }
    (
        adjust(red, factor),
        adjust(green, factor),
        adjust(blue, factor),
    )
}

fn adjust(color: f32, factor: f32) -> u8 {
    let result: u8;
    if color == 0.0 {
        result = 0;
    } else {
        result = (INTENSITYMAX * (color * factor).powf(GAMMA)) as u8;
    }
    result
}
