use std::{io::Write, time::Instant};

use escape_json::{two_pass_replace, two_pass_search_one_pass_copy};

fn main() {
    let INPUT: String = format!("\"{}{}\"", "something".repeat(1000), "  ".repeat(100));
    // let mut file = std::fs::File::create("./escape.json").unwrap();
    // file.write_all(INPUT.as_bytes()).unwrap();
    let array = include_str!("../assets/array.json");
    let big = include_str!("../assets/big.json");
    let large = include_str!("../assets/large.json");
    let package = include_str!("../assets/package.json");
    let test = include_str!("../assets/test.json");
    bench(array, "array");
    bench(big, "big");
    bench(large, "large");
    bench(package, "package");
    bench(test, "test");
    bench(&INPUT, "escape");
}

fn bench(input: &str, suite_name: &str) {
    let pass = 1;
    let start = Instant::now();
    for i in 0..pass {
        let _ = two_pass_replace(input);
    }
    println!("`{}` replace: {:?}", suite_name, start.elapsed());

    let start = Instant::now();
    for i in 0..pass {
        let _ = two_pass_search_one_pass_copy(input);
    }
    println!("`{}` merged copy: {:?}", suite_name, start.elapsed());

    let start = Instant::now();
    for i in 0..pass {
        let _ = escape_json::escape_json(input);
    }
    println!("`{}` one iteration: {:?}", suite_name, start.elapsed());
}
