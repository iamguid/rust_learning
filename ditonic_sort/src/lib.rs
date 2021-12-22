mod ditonic_sort;

use crate::ditonic_sort::sort::sort;

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

pub fn ditonic_sort(input: &mut Vec<i32>) {
    sort(input);
}