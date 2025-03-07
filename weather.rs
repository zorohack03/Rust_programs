fn average_temperature(temps: &[i32]) -> f32 {
    let sum: i32 = temps.iter().sum();
    sum as f32 / temps.len() as f32
}

fn main() {
    let week_temps = [30, 32, 33, 34, 35, 36, 37];
    let last_three_days = &week_temps[4..];

    println!("Last three days temperatures: {:?}", last_three_days);
    println!("Average temperature: {}", average_temperature(last_three_days));
}

