mod ditonic_sort;

use crate::ditonic_sort::sort::sort;
use crate::ditonic_sort::utils::compare::compare;

use rand::Rng;

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug_print {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug_print {
    ($( $args:expr ),*) => {}
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut vector: Vec<i32> = (0..100000).map(|_| rng.gen_range(0..1000)).collect();
    sort(&mut vector);
}
