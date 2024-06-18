fn main() {
    let args: Vec<String> = std::env::args().collect();
    let x_arg = if args.len() < 2 { "" } else { &args[1] };

    let x: f32 = x_arg.parse().unwrap_or(0.0);

    let c = (3.1416) * x * 2.0;
    println!("The circumference of a circle is {}", c);
}