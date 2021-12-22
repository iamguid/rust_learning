use std::vec::Vec;

mod calc;
mod pig;
mod test_interface;

fn main() {
    let vector1: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let vector2: Vec<i32> = vec![1, 3, 5, 5, 7, 10, 200];

    println!("mean of vector1: {}", calc::mean(&vector1));
    println!("mean of vector2: {}", calc::mean(&vector2));

    println!("median of vector1: {}", calc::median(&vector1));
    println!("median of vector2: {}", calc::median(&vector2));

    println!("mode of vector1: {}", calc::mode(&vector1));
    println!("mode of vector2: {}", calc::mode(&vector2));

    let string1 = String::from("first");
    let string2 = String::from("apple");
    let string3 = String::from("first apple");

    println!("pig latin of string \"{}\" is \"{}\"", string1, pig::convert(&string1));
    println!("pig latin of string \"{}\" is \"{}\"", string2, pig::convert(&string2));
    println!("pig latin of string \"{}\" is \"{}\"", string3, pig::convert(&string3));

    let ti = test_interface::TextIterface::new();

    ti.parse("Add TestEmplyee to TestDepartment")
}
