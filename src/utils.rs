use inquire::{validator::Validation, CustomUserError};

pub fn open_and_output_url(url: &str) {
    let open_result = open::that(url);
    match open_result {
        Ok(_) => println!("Opening {}", url),
        Err(_) => println!(
            "Failed to open search in your browser.\n Please visit {}",
            url
        )
    }
}

pub fn validate_not_empty_text(text: &str) -> Result<Validation, CustomUserError> {
    if text.len() < 1 {
        Ok(Validation::Invalid("Cannot search an empty string".into()))
    } else {
        Ok(Validation::Valid)
    }
}
