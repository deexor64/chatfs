use crate::core::types::Line;


/*
 * Returns a 0-based start index and end index
 */
pub fn safe_lines(lines: (Line, Line), total_lines: usize) -> Result<(usize, usize), String> {
    // Normalize start/end to actual line numbers
    let (start, end) = lines;

    let start: usize = match start {
        Line::Num(s) => s,
        Line::All => 1,
    };

    let end: usize = match end {
        Line::Num(e) => e,
        Line::All => total_lines,
    };

    // Ensure safe line range
    if start < 1 {
        return Err("Start line lower than 1 (should be 1-based)\n".to_string());
    }

    if end > total_lines {
        return Err("End line exceeds total lines\n".to_string());
    }

    if end < start {
        return Err("End line less than start (start index must be less than or equal to end index)\n".to_string());
    }

    // Convert to 0-based index range
    Ok((start - 1, end))
}
