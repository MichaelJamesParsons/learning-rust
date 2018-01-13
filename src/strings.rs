
fn main () {
    // Indexed interpolation
    println!("It is {0} that {1} is {0}", true, 'x');

    // Non-indexed interpolation
    println!("It is {} that {} is {}", true, 'x', true);

    // Format float
    println!("{} rounded to two decimals is {:2}", 1.234, 1.234);

    // Convert to binary, hex, and oct
    println!("B: {:b} H: {:x} O: {:o}", 10, 10, 10);

    // Named args and whitespace formatting
    println!("{ten:>ws$}", ten=10, ws=5);

    // Pad with 0's
    println!("{ten:>0ws$}", ten=10, ws=5);
}