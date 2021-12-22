use crate::ditonic_sort::sequence::*;
use crate::ditonic_sort::utils::swap_move::*;
use crate::debug_print;

pub fn merge_right_right(input: &mut Vec<i32>, sequence_a: &Sequence, sequence_b: &Sequence) {
    let begin_a = sequence_a.begin;
    let begin_b = sequence_b.begin;
    let end_a = sequence_a.begin + sequence_a.len;
    let end_b = sequence_b.begin + sequence_b.len;
    let is_a_behind_b = begin_a > begin_b;
    let sequences_end = if is_a_behind_b { end_a } else { end_b };
    let mut a = begin_a;
    let mut moves = 0;

    debug_print!("Lets merge right right from b {:?} to a {:?}", sequence_b, sequence_a);
    debug_print!("{:?}", input);

    loop {
        let from_pos = if is_a_behind_b {
            begin_b
        } else {
            if begin_b + moves >= end_b - 1 {
                end_b - 1
            } else {
                begin_b + moves
            }
        };

        if moves >= end_b - begin_b {
            debug_print!("moves {}", moves);
            break;
        }

        debug_print!("from_pos, {}", from_pos);

        loop {
            if a >= sequences_end {
                a = sequences_end;
                break;
            }

            if input[a] >= input[from_pos] {
                break;
            }

            if input[a] < input[from_pos] {
                a += 1;
            }
        }

        debug_print!("move from {} to {}", from_pos, a);
        swap_move(input, from_pos, a);
        debug_print!("{:?}", input);
        moves += 1;
    }
}

#[cfg(test)]
mod merge_right_right_tests {
    use crate::ditonic_sort::sequence::{Sequence, SequenceType};
    use crate::ditonic_sort::utils::compare::*;
    use super::*;

    #[test]
    fn sequences_has_equal_length() {
        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5];

        let sequence_a = Sequence {
            sequence_type: SequenceType::Right,
            begin: 0,
            len: 6,
        };

        let sequence_b = Sequence {
            sequence_type: SequenceType::Right,
            begin: 6,
            len: 6,
        };

        merge_right_right(&mut input, &sequence_a, &sequence_b);

        assert!(compare(&input, &vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]));
    }

    #[test]
    fn sequences_has_equal_length_rev() {
        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5];

        let sequence_a = Sequence {
            sequence_type: SequenceType::Right,
            begin: 0,
            len: 6,
        };

        let sequence_b = Sequence {
            sequence_type: SequenceType::Right,
            begin: 6,
            len: 6,
        };

        merge_right_right(&mut input, &sequence_b, &sequence_a);

        assert!(compare(&input, &vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]));
    }

    #[test]
    fn a_sequence_length_is_greater_then_b() {
        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 0, 1, 2];

        let sequence_a = Sequence {
            sequence_type: SequenceType::Right,
            begin: 0,
            len: 6,
        };

        let sequence_b = Sequence {
            sequence_type: SequenceType::Right,
            begin: 6,
            len: 3,
        };

        merge_right_right(&mut input, &sequence_a, &sequence_b);

        assert!(compare(&input, &vec![0, 0, 1, 1, 2, 2, 3, 4, 5]));
    }

    #[test]
    fn a_sequence_length_is_greater_then_b_rev() {
        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 0, 1, 2];

        let sequence_a = Sequence {
            sequence_type: SequenceType::Right,
            begin: 0,
            len: 6,
        };

        let sequence_b = Sequence {
            sequence_type: SequenceType::Right,
            begin: 6,
            len: 3,
        };

        merge_right_right(&mut input, &sequence_b, &sequence_a);

        assert!(compare(&input, &vec![0, 0, 1, 1, 2, 2, 3, 4, 5]));
    }

    #[test]
    fn a_sequence_length_is_less_then_b() {
        let mut input: Vec<i32> = vec![0, 1, 2, 0, 1, 2, 3, 4, 5];

        let sequence_a = Sequence {
            sequence_type: SequenceType::Right,
            begin: 0,
            len: 3,
        };

        let sequence_b = Sequence {
            sequence_type: SequenceType::Right,
            begin: 3,
            len: 6,
        };

        merge_right_right(&mut input, &sequence_a, &sequence_b);

        assert!(compare(&input, &vec![0, 0, 1, 1, 2, 2, 3, 4, 5]));
    }

    #[test]
    fn a_sequence_length_is_less_then_b_rev() {
        let mut input: Vec<i32> = vec![0, 1, 2, 0, 1, 2, 3, 4, 5];

        let sequence_a = Sequence {
            sequence_type: SequenceType::Right,
            begin: 0,
            len: 3,
        };

        let sequence_b = Sequence {
            sequence_type: SequenceType::Right,
            begin: 3,
            len: 6,
        };

        merge_right_right(&mut input, &sequence_b, &sequence_a);

        assert!(compare(&input, &vec![0, 0, 1, 1, 2, 2, 3, 4, 5]));
    }

    #[test]
    fn a_sequence_is_less_then_b() {
        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 3, 4, 5];

        let sequence_a = Sequence {
            sequence_type: SequenceType::Right,
            begin: 0,
            len: 6,
        };

        let sequence_b = Sequence {
            sequence_type: SequenceType::Right,
            begin: 6,
            len: 3,
        };

        merge_right_right(&mut input, &sequence_a, &sequence_b);

        assert!(compare(&input, &vec![0, 1, 2, 3, 3, 4, 4, 5, 5]));
    }

    #[test]
    fn a_sequence_is_less_then_b_rev() {
        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 3, 4, 5];

        let sequence_a = Sequence {
            sequence_type: SequenceType::Right,
            begin: 0,
            len: 6,
        };

        let sequence_b = Sequence {
            sequence_type: SequenceType::Right,
            begin: 6,
            len: 3,
        };

        merge_right_right(&mut input, &sequence_b, &sequence_a);

        assert!(compare(&input, &vec![0, 1, 2, 3, 3, 4, 4, 5, 5]));
    }

    #[test]
    fn sequences_with_padding() {
        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 0, 0, 0, 1, 2, 3, 4, 5];

        let sequence_a = Sequence {
            sequence_type: SequenceType::Right,
            begin: 0,
            len: 6,
        };

        let sequence_b = Sequence {
            sequence_type: SequenceType::Right,
            begin: 8,
            len: 6,
        };

        merge_right_right(&mut input, &sequence_a, &sequence_b);

        assert!(compare(&input, &vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 0, 0]));
    }

    #[test]
    fn sequences_with_padding_rev() {
        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 0, 0, 0, 1, 2, 3, 4, 5];

        let sequence_a = Sequence {
            sequence_type: SequenceType::Right,
            begin: 0,
            len: 6,
        };

        let sequence_b = Sequence {
            sequence_type: SequenceType::Right,
            begin: 8,
            len: 6,
        };

        merge_right_right(&mut input, &sequence_b, &sequence_a);

        assert!(compare(&input, &vec![0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]));
    }
}
