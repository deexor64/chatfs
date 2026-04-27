use crate::core::types::Line;


/*
 * Parses a lines string like "1-5", "3-*", "*-10"
 */
const PATTERN_ERR: &str = "Lines must follow 'start-end' pattern (e.g. 2-2, 1-5, 1-*, *-10, *-*)";

pub fn parse_lines(lines: String) -> Result<(Line, Line), String> {
    // Enforce 'start-end' pattern
    let mut parts = lines.split('-');

    let start = parts.next().ok_or_else(|| PATTERN_ERR.to_string())?;
    let end = parts.next().ok_or_else(|| PATTERN_ERR.to_string())?;

    // Reject extra '-' characters
    if parts.next().is_some() {
        return Err(PATTERN_ERR.to_string());
    }

    // Reject empty start or end
    if start.is_empty() || end.is_empty() {
        return Err(PATTERN_ERR.to_string());
    }

    // Reject invalid line numbers
    let start = match start {
        "*" => Ok(Line::Num(0)),
        _ => start.parse::<usize>()
            .map(|v| Line::Num(v))
            .map_err(|_| PATTERN_ERR.to_string())
    }?;

    let end = match end {
        "*" => Ok(Line::All),
        _ => end.parse::<usize>()
            .map(|v| Line::Num(v))
            .map_err(|_| PATTERN_ERR.to_string())
    }?;

    Ok((start, end))
}
