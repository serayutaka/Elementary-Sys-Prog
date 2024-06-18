fn main() {
    let args: Vec<String> = std::env::args().collect();

    let x_args = if args.len() < 2 { "" } else { &args[1] };
    let num: i64 = x_args.parse().unwrap_or(0);

    for i in 1..num+1 {
        for j in 0..i {
            print!("*");
        }
        print!("\n");
    }
}