use crate::ditonic_sort::sequence::*;
use crate::ditonic_sort::find_sequences::*;
use crate::ditonic_sort::merge_left_left::*;
use crate::ditonic_sort::utils::group_by_pair::*;
use crate::debug_print;

pub fn merge_left_sequences(input: &mut Vec<i32>, l_sequences: Option<Vec<Sequence>>) -> Vec<Sequence> {
    debug_print!("Lets merge left sequences");

    let l_sequences = l_sequences.unwrap_or_else(
        || find_sequences(input).0
    );

    debug_print!("Sequences {:?}", l_sequences);

    let sequences_pairs = group_by_pair(&l_sequences);

    debug_print!("Pairs {:?}", sequences_pairs);

    for sequences_pair in sequences_pairs {
        if sequences_pair.len() == 2 {
            let sequence_a = &sequences_pair[0];
            let sequence_b = &sequences_pair[1];

            if sequence_a.len > sequence_b.len {
                merge_left_left(input, sequence_a, sequence_b);
            } else {
                merge_left_left(input, sequence_b, sequence_a);
            }
        }
    }

    l_sequences
}
