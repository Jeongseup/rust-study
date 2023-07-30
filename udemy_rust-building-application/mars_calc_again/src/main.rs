use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new();
    // some_fn(&mut input);

    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    // println!("Input: {}", input);

    let mars_wegith = calculate_weight_on_mars(100.0);
    println!("Weight on Mars: {}kg", mars_wegith);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

fn some_fn(s: &mut String) {
    s.push_str("a");
}
