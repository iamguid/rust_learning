use crate::ditonic_sort::sequence::*;
use crate::debug_print;

pub fn find_sequences(input: &Vec<i32>) -> (Vec<Sequence>, Vec<Sequence>) {
    debug_print!("Lets find sequences in input {:?}", input);

    let mut left_sequences: Vec<Sequence> = Vec::new();
    let mut right_sequences: Vec<Sequence> = Vec::new();

    let length = input.len() - 1;

    let mut i = 0;
    let mut j = 1;

    loop {
        if j > length {
            break;
        }

        if input[i] == input[j] {
            debug_print!("== i {} j {}", i, j);
            j += 1;
            continue;
        }

        if input[i] < input[j] {
            debug_print!("Lets find right sequence");
            let r_sequence = find_right_sequence(input, i, j);
            i += r_sequence.len - 1;
            j = i + 1;
            right_sequences.push(r_sequence);
            continue;
        }

        if input[i] > input[j] {
            debug_print!("Lets find left sequence");
            let l_sequence = find_left_sequence(input, i, j);
            i += l_sequence.len - 1;
            j = i + 1;
            left_sequences.push(l_sequence);
            continue;
        }
    }

    (left_sequences, right_sequences)
}

fn find_right_sequence(input: &Vec<i32>, i: usize, j: usize) -> Sequence {
    let length = input.len() - 1;
    let sequence_type = SequenceType::Right;
    let sequence_begin = i;
    let mut sequence_len = j - i + 1;

    let mut i = j;
    let mut j = i + 1;

    loop {
        debug_print!(">> i {} j {}", i, j);
        if j > length || input[i] > input[j]  {
            debug_print!(">> end, begin {}, len {}", sequence_begin, sequence_len);
            return Sequence { sequence_type, begin: sequence_begin, len: sequence_len };
        }

        sequence_len += 1;
        i += 1;
        j += 1;
    }
}

fn find_left_sequence(input: &Vec<i32>, i: usize, j: usize) -> Sequence {
    let length = input.len() - 1;
    let sequence_type = SequenceType::Left;
    let sequence_begin = i;
    let mut sequence_len = j - i + 1;

    let mut i = j;
    let mut j = i + 1;

    loop {
        debug_print!("<< i {} j {}", i, j);
        if j > length || input[i] < input[j] {
            debug_print!("<< end, begin {}, len {}", sequence_begin, sequence_len);
            return Sequence { sequence_type, begin: sequence_begin, len: sequence_len };
        }

        sequence_len += 1;
        i += 1;
        j += 1;
    }
}

#[cfg(test)]
mod find_sequences {
    use crate::ditonic_sort::utils::compare::*;
    use crate::ditonic_sort::sequence::SequenceType;
    use super::*;

    #[test]
    fn right_sequence() {
        let input: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        let sequence_a = Sequence { sequence_type: SequenceType::Right, begin: 0, len: 6 };
        let (left_sequences, right_sequences) = find_sequences(&input);
        assert!(compare(&left_sequences, &[]));
        assert!(compare(&right_sequences, &[sequence_a]));
    }

    #[test]
    fn left_sequence() {
        let input: Vec<i32> = vec![5, 4, 3, 2, 1, 0];
        let sequence_a = Sequence { sequence_type: SequenceType::Left, begin: 0, len: 6 };
        let (left_sequences, right_sequences) = find_sequences(&input);
        assert!(compare(&left_sequences, &[sequence_a]));
        assert!(compare(&right_sequences, &[]));
    }

    #[test]
    fn equals_sequence() {
        let input: Vec<i32> = vec![0, 0, 0, 0, 0];
        let (left_sequences, right_sequences) = find_sequences(&input);
        assert!(compare(&left_sequences, &[]));
        assert!(compare(&right_sequences, &[]));
    }

    #[test]
    fn right_left_sequence() {
        let input: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 0];

        let sequence_a = Sequence { sequence_type: SequenceType::Right, begin: 0, len: 6 };
        let sequence_b = Sequence { sequence_type: SequenceType::Left, begin: 5, len: 6 };

        let (left_sequences, right_sequences) = find_sequences(&input);

        assert!(compare(&left_sequences, &[sequence_b]));
        assert!(compare(&right_sequences, &[sequence_a]));
    }

    #[test]
    fn left_right_sequence() {
        let input: Vec<i32> = vec![5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5];

        let sequence_a = Sequence { sequence_type: SequenceType::Left, begin: 0, len: 6 };
        let sequence_b = Sequence { sequence_type: SequenceType::Right, begin: 5, len: 6 };

        let (left_sequences, right_sequences) = find_sequences(&input);

        assert!(compare(&left_sequences, &[sequence_a]));
        assert!(compare(&right_sequences, &[sequence_b]));
    }

    #[test]
    fn right_left_right_sequences() {
        let input: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5];

        let sequence_a = Sequence { sequence_type: SequenceType::Right, begin: 0, len: 6 };
        let sequence_b = Sequence { sequence_type: SequenceType::Left, begin: 5, len: 6 };
        let sequence_c = Sequence { sequence_type: SequenceType::Right, begin: 10, len: 6 };

        let (left_sequences, right_sequences) = find_sequences(&input);

        assert!(compare(&left_sequences, &[sequence_b]));
        assert!(compare(&right_sequences, &[sequence_a, sequence_c]));
    }

    #[test]
    fn left_right_left_sequences() {
        let input: Vec<i32> = vec![5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 0];

        let sequence_a = Sequence { sequence_type: SequenceType::Left, begin: 0, len: 6 };
        let sequence_b = Sequence { sequence_type: SequenceType::Right, begin: 5, len: 6 };
        let sequence_c = Sequence { sequence_type: SequenceType::Left, begin: 10, len: 6 };

        let (left_sequences, right_sequences) = find_sequences(&input);

        assert!(compare(&left_sequences, &[sequence_a, sequence_c]));
        assert!(compare(&right_sequences, &[sequence_b]));
    }

    #[test]
    fn equals_left_right_sequences() {
        let input: Vec<i32> = vec![5, 5, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4];

        let sequence_a = Sequence { sequence_type: SequenceType::Left, begin: 0, len: 8 };
        let sequence_b = Sequence { sequence_type: SequenceType::Right, begin: 7, len: 5 };

        let (left_sequences, right_sequences) = find_sequences(&input);

        assert!(compare(&left_sequences, &[sequence_a]));
        assert!(compare(&right_sequences, &[sequence_b]));
    }

    #[test]
    fn left_right_equals_sequences() {
        let input: Vec<i32> = vec![4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 5, 5];

        let sequence_a = Sequence { sequence_type: SequenceType::Left, begin: 0, len: 5 };
        let sequence_b = Sequence { sequence_type: SequenceType::Right, begin: 4, len: 8 };

        let (left_sequences, right_sequences) = find_sequences(&input);

        assert!(compare(&left_sequences, &[sequence_a]));
        assert!(compare(&right_sequences, &[sequence_b]));
    }

    #[test]
    fn by_two_sequences() {
        let input: Vec<i32> = vec![1, 2, 1, 2, 1, 2, 1];

        let sequence_a = Sequence { sequence_type: SequenceType::Right, begin: 0, len: 2 };
        let sequence_b = Sequence { sequence_type: SequenceType::Left, begin: 1, len: 2 };
        let sequence_c = Sequence { sequence_type: SequenceType::Right, begin: 2, len: 2 };
        let sequence_d = Sequence { sequence_type: SequenceType::Left, begin: 3, len: 2 };
        let sequence_e = Sequence { sequence_type: SequenceType::Right, begin: 4, len: 2 };
        let sequence_f = Sequence { sequence_type: SequenceType::Left, begin: 5, len: 2 };

        let (left_sequences, right_sequences) = find_sequences(&input);

        assert!(compare(&left_sequences, &[sequence_b, sequence_d, sequence_f]));
        assert!(compare(&right_sequences, &[sequence_a, sequence_c, sequence_e]));
    }

    #[test]
    fn three_elements_right_left_sequences() {
        let input: Vec<i32> = vec![1, 2, 1];

        let sequence_a = Sequence { sequence_type: SequenceType::Right, begin: 0, len: 2 };
        let sequence_b = Sequence { sequence_type: SequenceType::Left, begin: 1, len: 2 };

        let (left_sequences, right_sequences) = find_sequences(&input);

        assert!(compare(&left_sequences, &[sequence_b]));
        assert!(compare(&right_sequences, &[sequence_a]));
    }

    #[test]
    fn three_elements_left_right_sequences() {
        let input: Vec<i32> = vec![2, 1, 2];

        let sequence_a = Sequence { sequence_type: SequenceType::Left, begin: 0, len: 2 };
        let sequence_b = Sequence { sequence_type: SequenceType::Right, begin: 1, len: 2 };

        let (left_sequences, right_sequences) = find_sequences(&input);

        assert!(compare(&left_sequences, &[sequence_a]));
        assert!(compare(&right_sequences, &[sequence_b]));
    }

    #[test]
    fn two_elements_right_sequence() {
        let input: Vec<i32> = vec![1, 2];

        let sequence_a = Sequence { sequence_type: SequenceType::Right, begin: 0, len: 2 };

        let (left_sequences, right_sequences) = find_sequences(&input);

        assert!(compare(&left_sequences, &[]));
        assert!(compare(&right_sequences, &[sequence_a]));
    }

    #[test]
    fn two_elements_left_sequences() {
        let input: Vec<i32> = vec![2, 1];

        let sequence_a = Sequence { sequence_type: SequenceType::Left, begin: 0, len: 2 };

        let (left_sequences, right_sequences) = find_sequences(&input);

        assert!(compare(&left_sequences, &[sequence_a]));
        assert!(compare(&right_sequences, &[]));
    }

    #[test]
    fn bug_1() {
        let input: Vec<i32> = vec![6, 6, 6, 2, 7];

        let sequence_a = Sequence { sequence_type: SequenceType::Left, begin: 0, len: 4 };
        let sequence_b = Sequence { sequence_type: SequenceType::Right, begin: 3, len: 2 };

        let (left_sequences, right_sequences) = find_sequences(&input);

        assert!(compare(&left_sequences, &[sequence_a]));
        assert!(compare(&right_sequences, &[sequence_b]));
    }
}