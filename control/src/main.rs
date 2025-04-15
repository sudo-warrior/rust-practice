fn main() {
   let  n = 32.0;
   let input = 'd';

    if input == 'd' {
        let f = degrees_to_ferenheit(n);
	println!("{n}°C = {f:.2}°F");
    } else if input == 'f' {
        let c = ferenheit_to_degrees(n);
	println!("{n}°C = {c:.2}°F");
    }
    println!("the way forward is back");
}

fn ferenheit_to_degrees(n: f64) -> f64 {
    (n - 32.0) * 5.0 / 9.0
}

fn degrees_to_ferenheit(n: f64) -> f64 {
    (n * 9.0 / 5.0) + 32.0
}
