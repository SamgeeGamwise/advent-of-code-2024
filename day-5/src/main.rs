use read_file::Rule;

mod read_file;


fn main() {
    let mut rules = read_file::file_to_rules("rules.txt");
    let mut master_rules: Vec<u8> = Vec::new();

    rules.sort_by(|a, b| a.before.cmp(&b.before));

    loop {
        // Loop through each rule
        // Check before against every after in the vector
        // if a before does not exist as an after, add before to master_rules
        // Remove all rules that contain before as a rule
        for i in (0..rules.len()).rev() {

            if master_rules.contains(&rules[i].before) {
                rules.remove(i);
                continue;
            }

            let mut unique = true;

            for rule_after in &rules {
                if rules[i].before == rule_after.after {
                    unique = false;
                    continue;
                }
            }

            if unique {
                master_rules.push(rules[i].before);
                rules.remove(i);
            }
        }

        if rules.len() == 0 {
            break;
        }
    }

    println!("{:?}", master_rules);
}
