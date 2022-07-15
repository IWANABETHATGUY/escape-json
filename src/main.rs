use std::time::Instant;

use escape_json::{two_pass_replace, two_pass_search_one_pass_copy};

fn main() {
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
}

fn bench(input: &str, suite_name: &str) {
    let pass = 100;
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
}
