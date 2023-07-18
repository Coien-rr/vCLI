pub fn convert_to_snake_case(input: &str) -> String {
    let mut output = String::new();

    for (i, c) in input.chars().enumerate() {
        if c == '/' || c == '.' {
            if i > 0 {
                output.push('_');
            }
        } else {
            output.push(c.to_ascii_lowercase());
        }
    }

    output
}
