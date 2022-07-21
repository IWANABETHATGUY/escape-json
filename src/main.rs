use std::{io::Write, time::Instant};

use escape_json::{two_pass_replace, two_pass_search_one_pass_copy, regex_replace, regex_iter_replace, memchr3_replace};

fn main() {
    println!("{}", 10 >> 3 == 30);
    let INPUT: String = format!("\"{}{}\"", "something".repeat(1000), "test".repeat(10000));
    // let INPUT: String = format!("\"{}{}\"", "something".repeat(1000), "  ".repeat(100));
    // let mut file = std::fs::File::create("./escape.json").unwrap();
    // file.write_all(INPUT.as_bytes()).unwrap();
    // for byte  in '\u{2028}'.to_string().bytes() {
    //     println!("{:b}", byte);
    // }
    // println!("{}", 0b10000000 >> 7);

    // println!("{}", );
    // println!("{}", 0b000000000010000000101000);;
    // println!("{}", r#"\\u2028"#.len());
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

    let start = Instant::now();
    for i in 0..pass {
        let _ = escape_json::escape_json(input);
    }
    println!("`{}` one iteration: {:?}", suite_name, start.elapsed());

    let start = Instant::now();
    for i in 0..pass {
        let _ = regex_replace(input);
    }
    println!("`{}` regex_replace: {:?}", suite_name, start.elapsed());
    
    let start = Instant::now();
    for i in 0..pass {
        let _ = regex_iter_replace(input);
    }
    println!("`{}` regex_iter: {:?}", suite_name, start.elapsed());

    let start = Instant::now();
    for i in 0..pass {
        let _ = memchr3_replace(input);
    }
    println!("`{}` memchr3: {:?}", suite_name, start.elapsed());
}
