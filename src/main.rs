use std::env;
mod koch;
mod parallax;
mod pattern;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!(
            "Usage: give program numbers as arguements to the program\n
                 1.Koch curve generation
                 2.Pattern generation
                 3.Parallax scroll effect"
        );
    } else {
        let mut num: u32;
        for i in 1..args.len() {
            num = args[i].trim().parse().unwrap();
            match num {
                1 => {

                    use koch::koch_main;
                    koch_main();
                }
                2 => {
                    use parallax::parallax_main;
                    parallax_main();
                }
                3 => {
                    use pattern::pattern_main;
                    pattern_main();
                }
                _ => println!("Enter valid number"),     
            }
        }
    }
}
