
fn main () {
    // initialize array
    let rand_array = [1,2,3];

    // access item
    println!("{}", rand_array[0]);

    // get length
    println!("{}", rand_array.len());

    // get slice
    println!("Second two items: {:?}", &rand_array[1..3]);
}