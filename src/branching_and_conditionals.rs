
fn main () {
    println!("!true = {}", !true);
    println!("true && false = {}", true && false);
    println!("true || false = {}", true || false);
    println!("true != false : {}", true != false);

    let age = 6;
    if age == 5 {
        println!("Go to kindergarten");
    } else if age > 5 && age <= 18 {
        println!("Go to grade {}", age - 5);
    } else if  age > 18 && age <= 25 {
        println!("go to college")
    } else {
        println!("Do what you want.")
    }

    // loop
    let mut x = 1;
    loop {
        if x % 2 == 0 {
            println!("LOOP {}", x);
            x +=1;
            continue;
        }

        if x > 10 {
            break;
        }

        x += 1;
    }

    // while loop
    let mut y = 1;
    while y <= 10 {
        println!("WHILE {}", y);
        y += 1;
    }

    // for loop
    for z in 1..10 {
        println!("FOR: {}", z)
    }
}