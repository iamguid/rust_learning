pub fn swap_move(input: &mut Vec<i32>, from_pos: usize, before_pos: usize) {
    // Move to right
    if from_pos < before_pos {
        let mut i = from_pos;

        loop {
            if i >= before_pos - 1 {
                break;
            }
            
            input.swap(i, i + 1);

            i += 1;
        }
    }

    // Move to left
    else {
        let mut i = from_pos;

        loop {
            if i <= before_pos {
                break;
            }
            
            input.swap(i, i - 1);

            i -= 1;
        }
    }
}

#[cfg(test)]
mod swap_move_tests {
    use crate::ditonic_sort::utils::compare::*;
    use super::*;

    #[test]
    fn swap_move_should_be_move_element_to_end() {
        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        swap_move(&mut input, 0, 6);
        println!("{:?}", input);
        assert!(compare(&input, &vec![1, 2, 3, 4, 5, 0]));
    }

    #[test]
    fn swap_move_should_be_move_element_to_begin() {
        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        swap_move(&mut input, 5, 0);
        println!("{:?}", input);
        assert!(compare(&input, &vec![5, 0, 1, 2, 3, 4]));
    }

    #[test]
    fn swap_move_should_be_move_element_to_right() {
        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        swap_move(&mut input, 0, 2);
        println!("{:?}", input);
        assert!(compare(&input, &vec![1, 0, 2, 3, 4, 5]));
    }

    #[test]
    fn swap_move_should_be_move_element_to_left() {
        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        swap_move(&mut input, 5, 2);
        println!("{:?}", input);
        assert!(compare(&input, &vec![0, 1, 5, 2, 3, 4]));
    }
}