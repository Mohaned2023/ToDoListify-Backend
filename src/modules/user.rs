use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    pub create_at: Option<String>,
    pub update_at: Option<String>,
}

#[derive(Validate, Deserialize)]
pub struct CreateDto {
    #[validate(length(min=2, max=255, message="min=2 && max=255"))]
    pub name: String,

    #[validate(custom(function = "username_validate"))]
    pub username: String,

    #[validate(
        length(min=5, max=255, message="min=2 && max=255"),
        email
    )]
    pub email: String,

    #[validate(custom( function = "password_validate"))]
    pub password: String,

    #[validate(must_match(other="password", message="Invalid password confirmation!"))]
    pub confirmation: String,
}

#[derive(Validate, Deserialize)]
pub struct LoginDto {
    #[validate(custom(function = "username_validate"))]
    pub username: String,

    #[validate(custom(function = "password_validate"))]
    pub password: String
}

#[derive(Validate, Deserialize)]
pub struct UpdateInformationDto {
    #[validate(length(min=2, max=255, message="min=2 && max=255"))]
    pub name: Option<String>,

    #[validate(custom(function = "username_validate"))]
    pub username: Option<String>,

    #[validate(
        length(min=5, max=255, message="min=2 && max=255"),
        email
    )]
    pub email: Option<String>
}

#[derive(Validate, Deserialize)]
pub struct UpdatePasswordDto {
    #[validate(custom( function = "password_validate"))]
    pub old_password: String,

    #[validate(custom( function = "password_validate"))]
    pub password: String,

    #[validate(must_match(other="password", message="Invalid password confirmation!"))]
    pub confirmation: String,
}

#[derive(Validate, Deserialize)]
pub struct DeleteDto {
    #[validate(custom( function = "password_validate"))]
    pub password: String,
}

fn username_validate(username: &str) -> Result<(), ValidationError> {
    if username.len() < 3 || username.len() > 255 {
        return Err(ValidationError::new("min=8 && max=512"));
    }
    let error_match: Result<(), ValidationError> = Err(ValidationError::new("User name not matches!"));
    let pattren: Regex = Regex::new(r"([a-z0-9_]+)").unwrap();
    let check_result = pattren.captures(username);
    if check_result.is_none() {
        return error_match;
    }
    let get_match_result = check_result.unwrap().get(0);
    if get_match_result.is_none() || get_match_result.unwrap().as_str().len() != username.len() {
        return error_match;
    }
    Ok(())
}

fn password_validate(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 || password.len() > 512 {
        return Err(ValidationError::new("min=8 && max=512"));
    }
    let lower_letter_pat = Regex::new(r"([a-z])").unwrap();
    let uper_letter_pat = Regex::new(r"([A-Z])").unwrap();
    let numbers_pat = Regex::new(r"([0-9])").unwrap();
    if 
        !lower_letter_pat.is_match(password) ||
        !uper_letter_pat.is_match(password) ||
        !numbers_pat.is_match(password) {
            return Err(ValidationError::new("password is to wake!!!"));
        }
    Ok(())
}
