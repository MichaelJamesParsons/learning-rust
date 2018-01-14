
fn main () {
    let mut vector1 = vec![1,2,3,4,5];

    println!("Item 2: {}", vector1[1]);

    for i in &vector1 {
        println!("Vect: {}", i);
    }

    vector1.push(6);
    vector1.pop();
}