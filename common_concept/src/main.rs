//!
//! This is the answer for the exercise https://doc.rust-lang.org/book/ch03-05-control-flow.html
//!

///
/// Convert  Fahrenheit temperatures to Celsius temperatures.
/// # Arguments
///  * f: f64 the temperature in Fahrenheit
/// # Returns
/// * f64: the temperature in Celsius
/// # Examples
/// ```
/// let c = fahrenheit_to_celsius(32.0);
/// assert_eq!(c, 0.0);
/// ```
///
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

///
/// Convert Celsius temperatures to Fahrenheit  Celsius.
/// # Arguments
///  * c: f64 the temperature in Celsius
/// #  Returns
/// * f64: the temperature in Fahrenheit
/// # Examples
/// ```
/// let f = celsius_to_fahrenheit(0.0);
/// assert_eq!(f, 32.0);
/// ```
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + 32.0
}

///
/// Generate the nth Fibonacci number.
/// # Arguments
///  * n: u64 the nth Fibonacci number to generate
/// # Returns
/// * i64: the nth Fibonacci number
/// # Examples
/// ```
/// let fib_5 = generate_nth_fibonacci(5);
/// assert_eq!(fib_5, 5);
/// ```
fn generate_nth_fibonacci(n: u64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => generate_nth_fibonacci(n - 1) + generate_nth_fibonacci(n - 2),
    }
}

///
/// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
///
fn print_christmas_carol() {
    let gifts = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Gold Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];
    for day in 1..=12 {
        println!("On the {} day of Christmas, my true love sent to me", day);
        // reverse the array to print the gifts in reverse
        for i in (0..day).rev() {
            println!("{}", gifts[i]);
        }
    }
}
fn main() {
    println!(
        "Fahrenheit 32.0 to Celsius: {}",
        fahrenheit_to_celsius(32.0)
    );
    println!("Celsius 0.0 to Fahrenheit: {}", celsius_to_fahrenheit(0.0));
    println!("Fibonacci 5: {}", generate_nth_fibonacci(5));
    print_christmas_carol();
}
