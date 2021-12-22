use crate::ditonic_sort::merge_left_sequences::*;
use crate::ditonic_sort::merge_right_sequences::*;
use crate::ditonic_sort::final_merge::*;

pub fn sort(input: &mut Vec<i32>) {
    let mut r_sequences = merge_right_sequences(input, None);
    let mut l_sequences = merge_left_sequences(input, None);

    while r_sequences.len() > 1 || l_sequences.len() > 1 {
        r_sequences = merge_right_sequences(input, None);
        l_sequences = merge_left_sequences(input, None);
    }

    if r_sequences.len() == 0 && l_sequences.len() == 0 {
        return
    }

    if r_sequences.len() == 1 && l_sequences.len() == 0 {
        return
    }

    if r_sequences.len() == 0 && l_sequences.len() == 1 {
        input.reverse();
        return
    }

    if r_sequences.len() == 1 && l_sequences.len() == 1 {
        let a_sequence = &r_sequences[0];
        let b_sequence = &l_sequences[0];

        final_merge(input, a_sequence, b_sequence);

        return
    }
}

#[cfg(test)]
mod sort_tests {
    use crate::ditonic_sort::utils::compare::*;
    use super::*;

    #[test]
    fn bug_1() {
        let input: Vec<i32> = vec![7, 7, 9, 8, 3];
        let mut copy = input.clone();
        sort(&mut copy);
        assert!(compare(&copy, &[3, 7, 7, 8, 9]));
    }

    #[test]
    fn bug_2() {
        let input: Vec<i32> = vec![9, 4, 7, 7, 9];
        let mut copy = input.clone();
        sort(&mut copy);
        assert!(compare(&copy, &[4, 7, 7, 9, 9]));
    }

    #[test]
    fn bug_3() {
        let input: Vec<i32> = vec![6, 2, 0, 5, 8, 6, 6, 8, 5, 3];
        let mut copy = input.clone();
        sort(&mut copy);
        assert!(compare(&copy, &[0, 2, 3, 5, 5, 6, 6, 6, 8, 8]));
    }

    #[test]
    fn bug_4() {
        let input: Vec<i32> = vec![16, 94, 34, 76, 23, 16, 23, 0, 16, 18];
        let mut copy = input.clone();
        sort(&mut copy);
        assert!(compare(&copy, &[0, 16, 16, 16, 18, 23, 23, 34, 76, 94]));
    }

    #[test]
    fn bug_5() {
        let input: Vec<i32> = vec![54, 86, 32, 63, 76, 88, 95, 15, 63, 9];
        let mut copy = input.clone();
        sort(&mut copy);
        assert!(compare(&copy, &[9, 15, 32, 54, 63, 63, 76, 86, 88, 95]));
    }

    #[test]
    fn bug_6() {
        let input: Vec<i32> = vec![92, 83, 70, 22, 92, 30, 14, 67, 49, 41];
        let mut copy = input.clone();
        sort(&mut copy);
        assert!(compare(&copy, &[14, 22, 30, 41, 49, 67, 70, 83, 92, 92]));
    }
}
