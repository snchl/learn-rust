fn main() {
    println!("18°C equal {}°F", celsius_to_fahrenheit(18.0));
    println!("32°F equal {}°C", fahrenheit_to_celsius(32.0));
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    (temp * 9.0/5.0) + 32.0
}

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0/9.0
}
