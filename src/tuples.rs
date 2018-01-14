
fn main () {
    let rand_tuple = ("Michael", 22);
    let rand_tuple_2: (&str, i8) = ("Michael", 22);

    println!("Name: {}", rand_tuple.0);
    println!("Name: {}", rand_tuple_2.0)
}