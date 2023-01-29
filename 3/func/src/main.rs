fn main() {

    let my_array = [1, 2, 3, 4, 5];
    for element in my_array.iter() {
        println!("the value is: {element}");
    }
    do_something("yoo");

    println!("{0}, {1}", five(), seven());

    let x = {
        let x = 3;
        println!("x inside the block: {x}");
        x + 1
    };
    println!("x inside outside the block: {x}");

    for number in (1..=3).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn do_something(a: &str) {
    println!("a: {a}");
}

fn five() -> i8 {
    5
}
fn seven() -> i8 {
    7
}
