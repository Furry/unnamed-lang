pub fn is_numeric(input: String) -> bool {
    // See if the input is numeric.
    let mut numeric = true;
    let mut index = 0;
    if input.len() == 0 {
        return false;
    }
    let chars = input.chars().into_iter();
    for c in chars {
        if (index == 0 && c == '-') || c.is_numeric() {
            index += 1;
        } else {
            numeric = false;
            break;
        }
    }

    return numeric;
}

pub fn is_alphabetic(input: String) -> bool {
    // See if the input is alphabetic.
    let mut alphabetic = true;
    let mut index = 0;
    if input.len() == 0 {
        return false;
    }

    let chars = input.chars().into_iter();
    for c in chars {
        if c.is_alphabetic() {
            index += 1;
        } else {
            alphabetic = false;
            break;
        }
    }

    return alphabetic;
}