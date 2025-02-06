pub fn scroll_string_for_mul(memory: String) -> Vec<(u32, u32)> {
    let mut multiplies = Vec::new();
    let memory_length = memory.len();

    for cursor in 0..memory_length {
        // mul(XXX,XXX) <- seventh index after m
        if cursor + 7 < memory_length && &memory[cursor..cursor + 4] == "mul(" {
            for close_parentheses_cursor in cursor + 4 .. cursor + 12 {
                if &memory[close_parentheses_cursor..close_parentheses_cursor + 1] == ")" {
                    let numerics = &memory[cursor + 4 .. close_parentheses_cursor];

                    // println!("{}", numerics);
                    
                    let mut numbers  = match numerics.find(",") {
                        Some(index) => numerics.split_at(index),
                        None => break,
                    };

                    numbers.1 = &numbers.1[1..];

                    // println!("{},{}", numbers.0, numbers.1);
                    
                    let multiplicand= numbers.0.parse::<u32>();
                    let multiplier = numbers.1.parse::<u32>();
                    
                    if let (Ok(value1), Ok(value2)) = (multiplicand, multiplier) {
                        multiplies.push((value1, value2));
                    }
                }
            }
        }
    }

    multiplies
}