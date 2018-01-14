
fn main () {
    let vect1 = vec![1,2,3];
    let vect2 = vect1;

    // Would throw "use of moved value" exception, because
    // the resource reference for vect1 how exists in vect2.
    //
    // Error will not occur for primitive values.
    //
    // println!("vect1[0] : {}", vect1[0]);

    // works
    println!("vect1[0] : {}", vect2[0]);

    let vect2 = vec![1,2,3];
    sum_vects(&vect2);
    println!("Sum of vect: {:?}", &vect2)
}

fn sum_vects(v1: &Vec<i32>) -> i32 {
    let sum = v1.iter().fold(0,|mut sum, &x| { sum += x; sum});

    return sum
}