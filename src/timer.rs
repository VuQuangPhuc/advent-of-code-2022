use std::time::Instant;

pub fn timeit<F: Fn() -> T, T>(f: F) -> T {
    let before = Instant::now();
    let result = f();
    println!("Time elapsed: {:.2?}", before.elapsed());
    result
}
