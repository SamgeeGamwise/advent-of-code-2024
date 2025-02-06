pub fn find_difference_between_lists(list1: [u32; 1000], list2: [u32; 1000]) -> u64 {
    let mut total_difference: u64 = 0;

    for i in 0..list1.len() {
        let difference: u64 = u64::from(list1[i].abs_diff(list2[i]));
        total_difference += difference;
    }

    total_difference
}

pub fn find_similarity_score(list1: [u32; 1000], list2: [u32; 1000]) -> u64 {
    let mut similarity_score = 0;
    let occurances = get_occurances_of_values(list2);

    for value in list1 {
        match occurances.iter().find(|&occurance| occurance.0 == value) {
        Some(occurance) => similarity_score += u64::from(value * occurance.1),
        None => continue,
        }
    }

    similarity_score
}

fn get_occurances_of_values(list: [u32; 1000]) -> Vec<(u32, u32)> {
    let mut occurances: Vec<(u32, u32)> = Vec::new();

    for value in list {
        match occurances.iter().position(|&occurance| occurance.0 == value) {
            Some(index) => occurances[index].1 += 1,
            None => occurances.push((value, 1)),
        };
    }

    occurances
}