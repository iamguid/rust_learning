use crate::ditonic_sort::sequence::Sequence;

pub fn group_by_pair(input: &Vec<Sequence>) -> Vec<&[Sequence]> {
    let mut result: Vec<&[Sequence]> = Vec::new();
    let mut i = 0;

    while i + 1 < input.len() {
        result.push(&input[i..i+2]);
        i += 2;
    }

    result
}