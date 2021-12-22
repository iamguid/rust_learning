use crate::ditonic_sort::sequence::*;
use crate::ditonic_sort::utils::swap_move::*;
use crate::debug_print;

pub fn final_merge(input: &mut Vec<i32>, a_sequence: &Sequence, b_sequence: &Sequence) {
    debug_print!("Lets do final merge {:?}", input);

    let is_a_behind_b = a_sequence.begin > b_sequence.begin;
    let is_piramid = if is_a_behind_b { a_sequence.sequence_type == SequenceType::Left } else { a_sequence.sequence_type == SequenceType::Right };
    let base_index = if is_a_behind_b { a_sequence.begin } else { b_sequence.begin };
    let begin_index = if is_a_behind_b { b_sequence.begin } else { a_sequence.begin };
    let end_index =  if is_a_behind_b { a_sequence.begin + a_sequence.len } else { b_sequence.begin + b_sequence.len };
    let first_len = begin_index + base_index;
    let second_len = end_index - base_index;
    let merge_first_to_second = first_len < second_len;

    debug_print!("is_a_behind_b {}", is_a_behind_b);
    debug_print!("is_piramid {}", is_piramid);
    debug_print!("base_index {}", base_index);
    debug_print!("begin_index {}", begin_index);
    debug_print!("end_index {}", end_index);
    debug_print!("first_len {}", first_len);
    debug_print!("second_len {}", second_len);
    debug_print!("merge_first_to_second {}", merge_first_to_second);

    let mut moves = 0;

    if merge_first_to_second {
        let mut i = base_index - 1;
        let mut j = base_index + 1;

        loop {
            if moves >= first_len {
                break;
            }

            if is_piramid {
                if j < end_index && input[i] <= input[j] {
                    j += 1;
                    continue;
                }
            } else {
                if j < end_index && input[i] >= input[j] {
                    j += 1;
                    continue;
                }
            }

            debug_print!("move from {} to {}", i, j);
            swap_move(input, i, j);
            debug_print!("{:?}", input);
            moves += 1;

            if i > begin_index {
                i -= 1;
            }
        }

        if is_piramid {
            input.reverse();
        }
    } else {
        let mut i = begin_index;
        let j = end_index - 1;

        loop {
            if moves >= second_len - 1 {
                break;
            }

            if is_piramid {
                if input[i] <= input[j] {
                    i += 1;
                    continue;
                }
            } else {
                if input[i] >= input[j] {
                    i += 1;
                    continue;
                }
            }

            debug_print!("move from {} to {}", j, i);
            swap_move(input, j, i);
            debug_print!("{:?}", input);
            moves += 1;
        }

        if !is_piramid {
            input.reverse();
        }
    }

}