use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:");
        println!("  <temperature> <unit>");
        println!("Example:");
        println!("  10.0 C");
        println!("  -40 F");
        return;
    }

    let mut val: f64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("The temperature must be a number");
            return;
        }
    };

    if args[2].eq("C") {
        val = val * 9.0 / 5.0 + 32.0;
        println!("{:.2}° F", val);
    } else if args[2].eq("F") {
        val = (val - 32.0) * 5.0 / 9.0;
        println!("{:.2}° C", val);
    } else {
        println!("Unknown unit");
        return;
    }
}
