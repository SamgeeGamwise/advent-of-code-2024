use std::time::Instant;
mod read_file;


fn main() {
    let start = Instant::now();

    let rules = read_file::file_to_rules("rules.txt");
    let pages: [Vec<u8>; 193] = read_file::file_to_pages("pages.txt");
    let mut count: u64= 0;

    for page in pages {
        count += u64::from(check_numbers_against_rules(page, rules));
    }

    println!("{}", count);
    println!("Time elapsed is: {:?}", start.elapsed());
}


fn check_numbers_against_rules(mut numbers: Vec<u8>, rules: [(u8, u8); read_file::RULES_LENGTH]) -> u8 {
    let mut incorrect = false;

    loop {
        let mut change = false;

        for i in 0..numbers.len() {
            for rule in rules {
                if numbers[i] == rule.0 {
                    for j in 0..i {
                        if numbers[j] == rule.1 {
                            change = true;
                            incorrect = true;
                            let removed = numbers.remove(j);
                            numbers.insert(i, removed);
                        }
                    }
                }
            }
        }

        if !change {
            break;
        }
    }


    if incorrect {
        let middle = *numbers.get(numbers.len() / 2).unwrap();
        return middle;
    }

    return 0;
}