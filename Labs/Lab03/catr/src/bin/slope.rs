fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let x1: &f32 = &args[1].parse().unwrap_or(0.0);
        let y1: &f32 = &args[2].parse().unwrap_or(0.0);
        let x2: &f32 = &args[3].parse().unwrap_or(0.0);
        let y2: &f32 = &args[4].parse().unwrap_or(0.0);
    
        let m = (y2 - y1) / (x2 - x1);
    
        println!("The slope is : {}", m);
    }

    else {
        println!("Invalid input")
    }
}