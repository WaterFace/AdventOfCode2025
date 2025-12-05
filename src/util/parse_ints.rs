pub fn parse_ints_u64(s: &str) -> Result<Vec<u64>, std::num::ParseIntError> {
    let mut buf = Vec::new();
    parse_ints_buf_u64(s, &mut buf)?;
    Ok(buf)
}

pub fn parse_ints_buf_u64(s: &str, buf: &mut Vec<u64>) -> Result<(), std::num::ParseIntError> {
    parse_ints_radix_buf_u64(s, 10, buf)
}

pub fn parse_ints_radix_buf_u64(
    s: &str,
    radix: u32,
    buf: &mut Vec<u64>,
) -> Result<(), std::num::ParseIntError> {
    let parsed = buf;

    let mut char_indices = s.char_indices();

    let mut current = None;

    loop {
        let next = char_indices.next();
        if let Some((i, c)) = next
            && c.is_digit(radix)
        {
            current = match current {
                // We're currently in the middle of a digit string,
                // so continue it
                Some((start, _)) => Some((start, i)),
                // We're not in the middle of a digit string, so start a new one
                None => Some((i, i)),
            };
            continue;
        }

        if let Some((start, end)) = current {
            let n_str = &s[start..=end];
            let n = u64::from_str_radix(n_str, radix)?;
            parsed.push(n);
            current = None;
        }

        if next.is_none() {
            break;
        }
    }

    Ok(())
}

pub fn parse_ints_u32(s: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    let mut buf = Vec::new();
    parse_ints_buf_u32(s, &mut buf)?;
    Ok(buf)
}

pub fn parse_ints_buf_u32(s: &str, buf: &mut Vec<u32>) -> Result<(), std::num::ParseIntError> {
    parse_ints_radix_buf_u32(s, 10, buf)
}

pub fn parse_ints_radix_buf_u32(
    s: &str,
    radix: u32,
    buf: &mut Vec<u32>,
) -> Result<(), std::num::ParseIntError> {
    let parsed = buf;

    let mut char_indices = s.char_indices();

    let mut current = None;

    loop {
        let next = char_indices.next();
        if let Some((i, c)) = next
            && c.is_digit(radix)
        {
            current = match current {
                // We're currently in the middle of a digit string,
                // so continue it
                Some((start, _)) => Some((start, i)),
                // We're not in the middle of a digit string, so start a new one
                None => Some((i, i)),
            };
            continue;
        }

        if let Some((start, end)) = current {
            let n_str = &s[start..=end];
            let n = u32::from_str_radix(n_str, radix)?;
            parsed.push(n);
            current = None;
        }

        if next.is_none() {
            break;
        }
    }

    Ok(())
}

pub fn parse_ints(s: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    let mut buf = Vec::new();
    parse_ints_buf(s, &mut buf)?;
    Ok(buf)
}

pub fn parse_ints_buf(s: &str, buf: &mut Vec<i32>) -> Result<(), std::num::ParseIntError> {
    parse_ints_radix_buf(s, 10, buf)
}

pub fn parse_ints_radix_buf(
    s: &str,
    radix: u32,
    buf: &mut Vec<i32>,
) -> Result<(), std::num::ParseIntError> {
    let parsed = buf;

    let mut char_indices = s.char_indices();

    let mut current = None;

    loop {
        let next = char_indices.next();
        if let Some((i, c)) = next {
            if c.is_digit(radix) {
                current = match current {
                    // We're currently in the middle of a digit string,
                    // so continue it
                    Some((start, _)) => Some((start, i)),
                    // We're not in the middle of a digit string, so start a new one
                    None => Some((i, i)),
                };
                continue;
            } else if c == '-' {
                current = Some((i, i)); // Either way, we need to start a new string here
                continue;
            }
        }

        if let Some((start, end)) = current {
            let n_str = &s[start..=end];
            let n = i32::from_str_radix(n_str, radix)?;
            parsed.push(n);
            current = None;
        }

        if next.is_none() {
            break;
        }
    }

    Ok(())
}

pub fn parse_ints_i64(s: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    parse_ints_radix_i64(s, 10)
}

pub fn parse_ints_radix_i64(s: &str, radix: u32) -> Result<Vec<i64>, std::num::ParseIntError> {
    let mut parsed = Vec::new();

    let mut char_indices = s.char_indices();

    let mut current = None;

    loop {
        let next = char_indices.next();
        if let Some((i, c)) = next {
            if c.is_digit(radix) {
                current = match current {
                    // We're currently in the middle of a digit string,
                    // so continue it
                    Some((start, _)) => Some((start, i)),
                    // We're not in the middle of a digit string, so start a new one
                    None => Some((i, i)),
                };
                continue;
            } else if c == '-' {
                current = Some((i, i)); // Either way, we need to start a new string here
                continue;
            }
        }

        if let Some((start, end)) = current {
            let n_str = &s[start..=end];
            let n = i64::from_str_radix(n_str, radix)?;
            parsed.push(n);
            current = None;
        }

        if next.is_none() {
            break;
        }
    }

    Ok(parsed)
}
