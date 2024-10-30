use set_partitions::{HashSubsets, VecSetPartition};
use std::collections::BTreeSet;

// Values of each input or output
pub type Set = Vec<u64>;
pub type Partition = Vec<Set>;

fn filter(
    set: &Set,
    input_len: usize,
    partitions: &mut VecSetPartition<u32, HashSubsets<u32>>,
) -> BTreeSet<Vec<u32>> {
    let mut filtered_set = BTreeSet::new();

    loop {
        let subset = partitions.subsets();
        for offset in subset.subsets().iter() {
            // skip if its all inputs or all outputs
            let inputs = offset
                .iter()
                .filter(|o| **o <= input_len as u32)
                .collect::<Vec<_>>();
            let outputs = offset
                .iter()
                .filter(|o| **o > input_len as u32)
                .collect::<Vec<_>>();
            if inputs.len() == 0 || outputs.len() == 0 {
                continue;
            }
            // If the sum of the values in the inputs is greater than the sum of the values in the outputs
            let input_sum = inputs.iter().map(|o| set[**o as usize]).sum::<u64>();
            let output_sum = outputs.iter().map(|o| set[**o as usize]).sum::<u64>();
            // TODO need to account for fees
            if input_sum == output_sum {
                filtered_set.insert(offset.iter().cloned().collect());
            }
        }
        if !partitions.increment() {
            break;
        }
    }

    filtered_set
}

/// Get all the input/output partitions of a given set and filter out the ones that don't meet the input/output sum criteria    
pub fn get_input_output_partitions(set: &Set, input_len: usize) -> BTreeSet<Vec<u32>> {
    let elements = set
        .iter()
        .enumerate()
        .map(|(i, v)| i as u32)
        .collect::<Vec<_>>();

    let mut partitions: VecSetPartition<u32, HashSubsets<u32>> = {
        let res = VecSetPartition::try_from_repr(elements.iter().cloned());
        res.unwrap()
    };
    partitions.reset();

    let filtered_set = filter(&set, input_len, &mut partitions);

    filtered_set
}

#[cfg(test)]
mod tests {
    // TODO: migrate tests from partion crate in cja
    //
    use super::*;

    #[test]
    fn it_works() {
        let set: Set = vec![100, 200, 300];
        // Indexing at 0
        let input_len = 1;
        let partitions = get_input_output_partitions(&set, input_len);
        println!("{:?}", partitions);

        assert_eq!(partitions.len(), 1);

        let set: Set = vec![300, 200, 200, 300];
        let input_len = 1;
        let partitions = get_input_output_partitions(&set, input_len);
        println!("{:?}", partitions);
        assert_eq!(partitions.len(), 3);

        let set: Set = vec![100, 100, 100, 100, 300, 100];
        let input_len = 3;
        let partitions = get_input_output_partitions(&set, input_len);
        println!("{:?}", partitions);
        assert_eq!(partitions.len(), 10);
    }
}
