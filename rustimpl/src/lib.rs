#![no_std]

pub fn split_string_try_op(value: &str, delimiter: &str) -> Option<(u16, u16, u16)> {
    let first_pos = value.find(delimiter)?;
    let first_substr = value.get(0..first_pos)?;
    let first = first_substr.parse::<i32>().ok()?;
    if first < 0 || first > 65535 {
        return None;
    }
    let first_remaining = value.get((first_pos + delimiter.len())..)?;

    let second_pos = first_remaining.find(delimiter)?;
    let second_substr = first_remaining.get(0..second_pos)?;
    let second = second_substr.parse::<i32>().ok()?;
    if second < 0 || second > 65535 {
        return None;
    }
    let second_remaining = first_remaining.get((second_pos + delimiter.len())..)?;

    let third = second_remaining.parse::<i32>().ok()?;
    if third < 0 || third > 65535 {
        return None;
    }
    // Range checking was done above, this static cast is valid
    return Some((first as u16, second as u16, third as u16));
}

pub fn split_string_like_cpp(value: &str, delimiter: &str) -> Option<(u16, u16, u16)> {
    let first_pos = value.find(delimiter);
    if first_pos.is_none() {
        return None;
    }
    let first_pos_index = first_pos.unwrap();

    let first_substr = value.get(0..first_pos_index).unwrap(); // "safe" because of first_pos.is_some()
    let first_conversation = first_substr.parse::<i32>();
    if first_conversation.is_err() {
        return None;
    }
    let first = first_conversation.unwrap();
    if first < 0 || first > 65535 {
        return None;
    }
    // Get a substring that is the remaining of the string with the first and the delimiter removed
    let first_remaining = value.get((first_pos_index + delimiter.len())..).unwrap(); // "safe" because of first_pos.is_some()

    let second_pos = first_remaining.find(delimiter);
    if second_pos.is_none() {
        return None;
    }
    let second_pos_index = second_pos.unwrap();

    let second_substr = first_remaining.get(0..second_pos_index).unwrap(); // "safe" because of second_pos.is_some()
    let second_conversation = second_substr.parse::<i32>();
    if second_conversation.is_err() {
        return None;
    }
    let second = second_conversation.unwrap();
    if second < 0 || second > 65535 {
        return None;
    }
    // Get a substring that is the remaining of the string with the second and the delimiter removed
    let second_remaining = first_remaining
        .get((second_pos_index + delimiter.len())..)
        .unwrap(); // "safe" because of second_pos.is_some()

    // Third is just a parse of the second remaining
    let third_conversation = second_remaining.parse::<i32>();
    if third_conversation.is_err() {
        return None;
    }
    let third = third_conversation.unwrap();
    if third < 0 || third > 65535 {
        return None;
    }

    // Range checking was done above, this static cast is valid
    return Some((first as u16, second as u16, third as u16));
}

pub fn split_string_iterator_combinator(value: &str, delimiter: &str) -> Option<(u16, u16, u16)> {
    let mut parts = value.split(delimiter).map(|v| v.parse::<u16>().ok());

    Some((
        parts.next()??, // Double ??, first ? unwraps the next, second ? unwraps the parse
        parts.next()??,
        parts.next().and_then(|v| parts.next().xor(Some(v)))??,
    ))
}

pub fn split_string_3(value: &str, delimiter: &str) -> Option<(u16, u16, u16)> {
    let mut parts = value.split(delimiter);

    let first = parts.next()?.parse::<u16>().ok()?;
    let second = parts.next()?.parse::<u16>().ok()?;
    let third = parts.next()?.parse::<u16>().ok()?;

    // should be no more in parts
    if parts.next().is_some() {
        return None;
    }
    
    Some((first, second, third))
}
