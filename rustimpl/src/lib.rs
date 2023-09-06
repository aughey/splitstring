#![no_std]

pub fn split_string_like_cpp(value: &str, delimiter: &str) -> Option<(u16, u16, u16)> {
    let first_pos = value.find(delimiter); // find the index of the delimiter
    if first_pos.is_none() {
        // fail if didn't find
        return None;
    }
    let first_pos_index = first_pos.unwrap(); // "safe" because of first_pos.is_some()

    let first_substr = value.get(0..first_pos_index).unwrap(); // create a string of what's before the delimiter
    let first_conversation = first_substr.parse::<i32>(); // parse that string to an i32
    if first_conversation.is_err() {
        // fail if the parse failed
        return None;
    }
    let first = first_conversation.unwrap(); // "safe" because of first_conversation.is_ok()
    if first < 0 || first > 65535 {
        // Range checking to make sure it will fit in a u16
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

pub fn split_string_like_cpp_i_need_help(value: &str, delimiter: &str) -> Option<(u16, u16, u16)> {
    let first_pos = value.find(delimiter); // find the index of the delimiter
    if let Some(first_pos_index) = first_pos {
        if let Some(first_substr) = value.get(0..first_pos_index) {
            let first_conversation = first_substr.parse::<i32>(); // parse that string to an i32
            if let Ok(first) = first_conversation {
                if first < 0 || first > 65535 {
                    // Range checking to make sure it will fit in a u16
                    return None;
                }
                // Get a substring that is the remaining of the string with the first and the delimiter removed

                if let Some(first_remaining) = value.get((first_pos_index + delimiter.len())..) {
                    let second_pos = first_remaining.find(delimiter);
                    if let Some(second_pos_index) = second_pos {
                        if let Some(second_substr) = first_remaining.get(0..second_pos_index) {
                            let second_conversation = second_substr.parse::<i32>();
                            if let Ok(second) = second_conversation {
                                if second < 0 || second > 65535 {
                                    return None;
                                }
                                // Get a substring that is the remaining of the string with the second and the delimiter removed
                                if let Some(second_remaining) =
                                    first_remaining.get((second_pos_index + delimiter.len())..)
                                {
                                    // Third is just a parse of the second remaining
                                    let third_conversation = second_remaining.parse::<i32>();
                                    if let Ok(third) = third_conversation {
                                        if third < 0 || third > 65535 {
                                            return None;
                                        }

                                        // Range checking was done above, this static cast is valid
                                        return Some((first as u16, second as u16, third as u16));
                                    } else {
                                        return None; // couldn't convert third value
                                    }
                                } else {
                                    return None; // couldn't get second remaining
                                }
                            } else {
                                return None; // couldn't convert second value
                            }
                        } else {
                            return None; // couldn't get second substring
                        }
                    } else {
                        return None; // couldn't find second delimiter
                    }
                } else {
                    return None; // couldn't get first remaining
                }
            } else {
                return None; // couldn't convert first value
            }
        } else {
            return None; // couldn't get first substring
        }
    } else {
        return None; // couldn't find first delimiter
    }
}

pub fn split_string_like_cpp_no_unwrap(value: &str, delimiter: &str) -> Option<(u16, u16, u16)> {
    let first_pos = value.find(delimiter); // find the index of the delimiter
    let first_pos_index = if let Some(i) = first_pos {
        i
    } else {
        // fail if didn't find
        return None;
    };

    let first_substr = if let Some(s) = value.get(0..first_pos_index) {
        s
    } else {
        return None;
    };
    let first_conversation = first_substr.parse::<i32>(); // parse that string to an i32
    let first = if let Ok(value) = first_conversation {
        value
    } else {
        // fail if the parse failed
        return None;
    };
    if first < 0 || first > 65535 {
        // Range checking to make sure it will fit in a u16
        return None;
    }
    // Get a substring that is the remaining of the string with the first and the delimiter removed
    let first_remaining = if let Some(remaining) = value.get((first_pos_index + delimiter.len())..)
    {
        remaining
    } else {
        return None;
    };

    let second_pos = first_remaining.find(delimiter);
    let second_pos_index = if let Some(i) = second_pos {
        i
    } else {
        return None;
    };

    let second_substr = if let Some(s) = first_remaining.get(0..second_pos_index) {
        s
    } else {
        return None;
    };
    let second_conversation = second_substr.parse::<i32>();
    let second = if let Ok(value) = second_conversation {
        value
    } else {
        return None;
    };
    if second < 0 || second > 65535 {
        return None;
    }
    // Get a substring that is the remaining of the string with the second and the delimiter removed
    let second_remaining =
        if let Some(s) = first_remaining.get((second_pos_index + delimiter.len())..) {
            s
        } else {
            return None;
        };

    // Third is just a parse of the second remaining
    let third_conversation = second_remaining.parse::<i32>();
    let third = if let Ok(value) = third_conversation {
        value
    } else {
        return None;
    };
    if third < 0 || third > 65535 {
        return None;
    }

    // Range checking was done above, this static cast is valid
    return Some((first as u16, second as u16, third as u16));
}

pub fn split_string_like_cpp_i_need_help(value: &str, delimiter: &str) -> Option<(u16, u16, u16)> {
    let first_pos = value.find(delimiter); // find the index of the delimiter
    if let Some(first_pos_index) = first_pos {
        if let Some(first_substr) = value.get(0..first_pos_index) {
            let first_conversation = first_substr.parse::<i32>(); // parse that string to an i32
            if let Ok(first) = first_conversation {
                if first < 0 || first > 65535 {
                    // Range checking to make sure it will fit in a u16
                    return None;
                }
                // Get a substring that is the remaining of the string with the first and the delimiter removed

                if let Some(first_remaining) = value.get((first_pos_index + delimiter.len())..) {
                    let second_pos = first_remaining.find(delimiter);
                    if let Some(second_pos_index) = second_pos {
                        if let Some(second_substr) = first_remaining.get(0..second_pos_index) {
                            let second_conversation = second_substr.parse::<i32>();
                            if let Ok(second) = second_conversation {
                                if second < 0 || second > 65535 {
                                    return None;
                                }
                                // Get a substring that is the remaining of the string with the second and the delimiter removed
                                if let Some(second_remaining) =
                                    first_remaining.get((second_pos_index + delimiter.len())..)
                                {
                                    // Third is just a parse of the second remaining
                                    let third_conversation = second_remaining.parse::<i32>();
                                    if let Ok(third) = third_conversation {
                                        if third < 0 || third > 65535 {
                                            return None;
                                        }

                                        // Range checking was done above, this static cast is valid
                                        return Some((first as u16, second as u16, third as u16));
                                    } else {
                                        return None; // couldn't convert third value
                                    }
                                } else {
                                    return None; // couldn't get second remaining
                                }
                            } else {
                                return None; // couldn't convert second value
                            }
                        } else {
                            return None; // couldn't get second substring
                        }
                    } else {
                        return None; // couldn't find second delimiter
                    }
                } else {
                    return None; // couldn't get first remaining
                }
            } else {
                return None; // couldn't convert first value
            }
        } else {
            return None; // couldn't get first substring
        }
    } else {
        return None; // couldn't find first delimiter
    }
}

pub fn split_string_try_op(value: &str, delimiter: &str) -> Option<(u16, u16, u16)> {
    let first_pos = value.find(delimiter)?; // Find the index of the delimiter
    let first_substr = value.get(0..first_pos)?; // extract the substring of the first item
    let first = first_substr.parse::<i32>().ok()?; // parse it to an i32
    if first < 0 || first > 65535 {
        // range check to make sure it will fit in a u16
        return None;
    }
    let first_remaining = value.get((first_pos + delimiter.len())..)?; // Get the remaining string

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

pub fn split_string_try_op_no_conditionals(
    value: &str,
    delimiter: &str,
) -> Option<(u16, u16, u16)> {
    let first_pos = value.find(delimiter)?; // The the index of the delimiter
    let first_substr = value.get(0..first_pos)?; // extract the substring of the first item
    let first = first_substr
        .parse::<i32>() // parse it to an i32
        .ok()
        .filter(|v| *v >= 0 && *v <= 65535)?; // range check
    let first_remaining = value.get((first_pos + delimiter.len())..)?; // Get the remaining string

    let second_pos = first_remaining.find(delimiter)?;
    let second_substr = first_remaining.get(0..second_pos)?;
    let second = second_substr
        .parse::<i32>()
        .ok()
        .filter(|v| *v >= 0 && *v <= 65535)?;
    let second_remaining = first_remaining.get((second_pos + delimiter.len())..)?;

    let third = second_remaining
        .parse::<i32>()
        .ok()
        .filter(|v| *v >= 0 && *v <= 65535)?;

    // Range checking was done above, this static cast is valid
    return Some((first as u16, second as u16, third as u16));
}

pub fn split_string_try_op_one_line(value: &str, delimiter: &str) -> Option<(u16, u16, u16)> {
    let first_pos = value.find(delimiter)?;
    let first = value
        .get(0..first_pos)? // extract substring
        .parse::<i32>() // convert to i32
        .ok() // convert parse failure to to Option<i32>
        .filter(|v| *v >= 0 && *v <= 65535)?; // range checking
    let first_remaining = value.get((first_pos + delimiter.len())..)?;

    let second_pos = first_remaining.find(delimiter)?;
    let second = first_remaining
        .get(0..second_pos)?
        .parse::<i32>()
        .ok()
        .filter(|v| *v >= 0 && *v <= 65535)?;
    let second_remaining = first_remaining.get((second_pos + delimiter.len())..)?;

    let third = second_remaining
        .parse::<i32>()
        .ok()
        .filter(|v| *v >= 0 && *v <= 65535)?;

    // Range checking was done above, this static cast is valid
    return Some((first as u16, second as u16, third as u16));
}

pub fn split_string_3(value: &str, delimiter: &str) -> Option<(u16, u16, u16)> {
    let mut parts = value.split(delimiter);

    // Extract and parse each value
    let first = parts.next()?.parse::<u16>().ok()?;
    let second = parts.next()?.parse::<u16>().ok()?;
    let third = parts.next()?.parse::<u16>().ok()?;

    // should be no more in parts
    if parts.next().is_some() {
        return None;
    }

    Some((first, second, third))
}

pub fn split_string_3_iterparse(value: &str, delimiter: &str) -> Option<(u16, u16, u16)> {
    // Create an iterator that emits parsed u16 values
    let mut parts = value.split(delimiter).map(|v| v.parse::<u16>().ok());

    let first = parts.next()??; // Double ??, first ? unwraps the next, second ? unwraps the parse
    let second = parts.next()??;
    let third = parts.next()??;

    // should be no more in parts
    if parts.next().is_some() {
        return None;
    }

    Some((first, second, third))
}

pub fn split_string_iterator_combinator(value: &str, delimiter: &str) -> Option<(u16, u16, u16)> {
    let mut parts = value.split(delimiter).map(|v| v.parse::<u16>().ok());

    Some((
        parts.next()??, // Double ??, first ? unwraps the next, second ? unwraps the parse
        parts.next()??,
        parts.next().and_then(|v| parts.next().xor(Some(v)))??, // witchcraft get last item and fail if there is more
    ))
}
