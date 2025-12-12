pub fn check_title(title: &str) -> Result<(), &'static str> {
    if !(1..=50).contains(&title.len()) {
        return Err("Title must be 1-50 characters long");
    }

    Ok(())
}

pub fn check_description(description: &str) -> Result<(), &'static str> {
    if description.len() > 500 {
        return Err("Description is too long");
    }

    Ok(())
}

pub fn validate_post(
    title: &str, description: &str
) -> Result<(), &'static str> {
    check_title(title)?;
    check_description(description)?;

    Ok(())
}