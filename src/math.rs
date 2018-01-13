
fn main () {
    println!("5 + 4 = {}", 5 + 4);
    println!("5 - 4 = {}", 5 - 4);
    println!("5 * 4 = {}", 5 * 4);
    println!("5 / 4 = {}", 5 / 4);
    println!("abs(-4) = {}", -4i32.abs());
    println!("4 ^ 6 = {}", 4i32.pow(6));
    println!("sqrt 9 = {}", 9f64.sqrt());
    println!("cbrt 9 = {}", 9f64.cbrt());
    println!("round 1.2345 = {}", 1.2345f64.round());
    println!("floor 1.2345 = {}", 1.2345f64.floor());
    println!("ceil 1.2345 = {}", 1.2345f64.ceil());
    println!("e ^ 2 = {}", 2f64.exp());
    println!("log(2) = {}", 2f64.ln());
    println!("log10(2) = {}", 2f64.log10());
    println!("90 to radians = {}", 90f64.to_radians());
    println!("PI to degrees = {}", std::f64::consts::PI.to_degrees());
    println!("Max 4,5 = {}", 4f64.max(5f64));
    println!("Min 4,5 = {}", 4f64.min(5f64));
    println!("Sin 3.14 = {}", 3.14f64.sin());
}