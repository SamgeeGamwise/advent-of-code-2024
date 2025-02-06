pub fn scroll_string_for_mul(memory: String) -> Vec<(u32, u32)> {
    let mut multiplies = Vec::new();
    let memory_length = memory.len();
    let mut enabled: bool = true;

    for cursor in 0..memory_length {
        
        
        // If currently enabled, there is room for a don't command, and the next 7 characters are 'don't()'
        if enabled && cursor + 6 < memory_length && &memory[cursor..cursor + 7] == "don't()" {
            enabled = false;
        // If not enabled, there is room for the do command, and the next 4 characters are 'do()'
        } else if !enabled && cursor + 3 < memory_length && &memory[cursor..cursor + 4] == "do()" {
            enabled = true;
        }
        
        
        
        
        // if enabled, there is room for at least 'mul(X,X)', and the next 4 characters are 'mul('
        if enabled && cursor + 7 < memory_length && &memory[cursor..cursor + 4] == "mul(" {

            // Search for closing parentheses in range of valid command
            for close_parentheses_cursor in cursor + 4 .. cursor + 12 {

                // If closing parentheses is found, look for comma and attempt to parse into u32
                if &memory[close_parentheses_cursor..close_parentheses_cursor + 1] == ")" {
                    let numerics = &memory[cursor + 4 .. close_parentheses_cursor];

                    let mut numbers  = match numerics.find(",") {
                        Some(index) => numerics.split_at(index),
                        None => break,
                    };

                    numbers.1 = &numbers.1[1..];
                    
                    let multiplicand= numbers.0.parse::<u32>();
                    let multiplier = numbers.1.parse::<u32>();
                    
                    // If numbers are parsed successfully as numbers, we have a valid command
                    // Add to multiplies
                    if let (Ok(value1), Ok(value2)) = (multiplicand, multiplier) {
                        multiplies.push((value1, value2));
                    }
                }
            }
        }
    }

    multiplies
}