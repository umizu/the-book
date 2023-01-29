// fn main() {
//     let mut s = String::from("hello");

//     s.push_str("fsdg");

//     let r1 = &mut s;

//     r1.push_str("fsdg");
//     // let r2 = &mut s;

//     println!("{}", r1);

//     // let s1 = String::from("hello");

//     // let len = calculate_length(&s1);

//     // println!("The length of '{}' is {}.", s1, len);
// }

// // fn calculate_length(s: &String) -> usize {
// //     s.len()
// // }


fn main() {
    let mut gg = String::from("gg");
    let reference_to = dangle(&mut gg);

    println!("{}", reference_to)
}

fn dangle(g: &mut String) -> &String {
    g.push_str(", yoo");
    g
}
