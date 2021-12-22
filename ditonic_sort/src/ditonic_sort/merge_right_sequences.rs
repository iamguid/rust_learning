use crate::ditonic_sort::sequence::*;
use crate::ditonic_sort::merge_right_right::*;
use crate::ditonic_sort::find_sequences::*;
use crate::ditonic_sort::utils::group_by_pair::*;
use crate::debug_print;

pub fn merge_right_sequences(input: &mut Vec<i32>, r_sequences: Option<Vec<Sequence>>) -> Vec<Sequence> {
    debug_print!("Lets merge right sequences");

    let r_sequences = r_sequences.unwrap_or_else(
        || find_sequences(input).1
    );

    debug_print!("Sequences {:?}", r_sequences);

    let sequences_pairs = group_by_pair(&r_sequences);

    debug_print!("Pairs {:?}", sequences_pairs);

    for sequences_pair in sequences_pairs {
        if sequences_pair.len() == 2 {
            let sequence_a = &sequences_pair[0];
            let sequence_b = &sequences_pair[1];

            if sequence_a.len > sequence_b.len {
                merge_right_right(input, sequence_a, sequence_b);
            } else {
                merge_right_right(input, sequence_b, sequence_a);
            }
        }
    }

    r_sequences
}