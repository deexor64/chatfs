/*
 * Parses a lines string like "1-5", "3-*", "*-10"
 * Returns a 0-based start index and end index (exclusive)
 * Also enforces a maximum number of lines
 */
pub fn parse_lines(lines: &str, total_lines: usize, max_lines: usize) -> (usize, usize, String) {
    let parts: Vec<&str> = lines.split('-').collect();

    // Parse start
    let start = if parts[0] == "*" {
        0
    } else {
        parts[0]
            .parse::<usize>()
            .unwrap_or(1)
            .saturating_sub(1)
    };

    // Parse end
    let mut end = if parts[1] == "*" {
        total_lines
    } else {
        parts[1]
            .parse::<usize>()
            .unwrap_or(total_lines)
    };

    let mut note = String::new();

    // Ensure end >= start
    if end < start {
        // ISSUE: This doesn't work
        end = start;
        note.push_str("End line less than start, but this may not what you want - adjusted as end = start\n");
    }

    // Clamp end to total lines
    if end > total_lines {
        end = total_lines;
        note.push_str("End line exceeded total lines - clamped as end = total_lines\n");
    }

    // Limit max number of lines returned
    if end.saturating_sub(start) > max_lines {
        end = start + max_lines;
        note.push_str("Requested range exceeded max lines - truncated as end = start + max_lines\n");
    }

    (start, end, note)
}
