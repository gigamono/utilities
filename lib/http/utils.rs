use crate::result::Result;

pub fn parse_url_path_number(path: &str) -> Result<usize> {
    // Trim one or more "/" from the prefix.
    let trimmed_path = path.trim_start_matches("/");

    // Get the index of the next "/".
    let index = trimmed_path.find("/").unwrap_or(0);

    // Get the substring up until "/".
    let sub = &trimmed_path[..index];

    // Parse substring.
    let num = sub.parse::<usize>()?;

    Ok(num)
}
