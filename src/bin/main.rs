use dice_mosaic::DiceMosaic;
use dice_mosaic::Resolution;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} FILE RESOLUTION", args[0]);
        eprintln!("Example: {} image.png low|medium|high", args[0]);
        std::process::exit(1);
    }

    let filepath = format!("{}", &args[1]);
    let resolution = match args.get(2) {
        Some(string) => match string.as_ref() {
            "high" => Resolution::High,
            "medium" => Resolution::Medium,
            "low" => Resolution::Low,
            _ => Resolution::Medium,
        }
        None => Resolution::Medium
    };

    let mut dm = DiceMosaic::new(&filepath, 50, resolution);
    dm.process();
    dm.stats();
}