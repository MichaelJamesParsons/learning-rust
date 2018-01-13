
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

    // String length
    let my_string = "I am a random string.";
    println!("Length: {}", my_string.len());

    // String split
    let (first, second) = my_string.split_at(6);
    println!("First: \"{}\" Second: \"{}\".", first, second);

    // Count characters
    let count = my_string.chars().count();
    println!("count: {}", count);

    // Iterate over characters in string
    let mut chars = my_string.chars();
    let mut indiv_char = chars.next();

    loop {
        match indiv_char {
            Some(x) => println!("{}", x),
            None => break
        }
        indiv_char = chars.next()
    }

    // Iterate over words in string
    let mut iter = my_string.split_whitespace();
    let mut indiv_word = iter.next();

    loop {
        match indiv_word {
            Some(x) => println!("{}", x),
            None => break
        }

        indiv_word = iter.next();
    }
}