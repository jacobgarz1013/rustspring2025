const FAH_FREEZE: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f-FAH_FREEZE)*5.0/9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c*9.0/5.0) + FAH_FREEZE
}

fn main() {
    let mut temp_f = 32.0; 

    println!("{:.1}°F is {:.1}°C", temp_f, fahrenheit_to_celsius(temp_f));

    for i in 1..=5 {
        let next_temp_f = temp_f + i as f64;
        println!("{:.1}°F is {:.1}°C", next_temp_f, fahrenheit_to_celsius(next_temp_f));
    }
}
