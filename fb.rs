fn fb(n : i32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz ".to_string(),
        (0, _) => "Fizz ".to_string(),
        (_, 0) => "Buzz ".to_string(),
        (_, _) => format!("{} ", n),
    }
}

pub fn fb_range(
    start   : i32,
    stop    : i32
) -> String {
    (start..=stop)
        .map(fb)
        .collect::<String>()
}
