use serde::{
    Deserialize, 
    Serialize
};
use validator::{
    Validate, 
    ValidationError
};


#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub priority: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>
}

#[derive(Validate, Deserialize)]
pub struct CreateDto {
    #[validate(length(min=1, max=255, message="min=1, max=255"))]
    pub title: String,

    #[validate(length(min=1, max=6000, message="min=1, max=6000"))]
    pub body: Option<String>,

    #[validate(
        length(min=4, max=11, message="min=4, max=11"),
        custom(function = "state_validate")
    )]
    pub state: Option<String>,

    #[validate(
        length(min=3, max=6, message="min=3, max=6"),
        custom(function = "priority_validate")
    )]
    pub priority: Option<String>,
}

fn state_validate(state: &str) -> Result<(), ValidationError> {
    if  state != "TO_DO"       &&
        state != "IN_PROGRESS" &&
        state != "DONE" {
        return Err(
            ValidationError::new(
                "state must be one of ('TO_DO', 'IN_PROGRESS', 'DONE')"
            )
        );
    }
    return Ok(());
}

fn priority_validate(priority: &str) -> Result<(), ValidationError> {
    if  priority != "LOW"    &&
        priority != "MEDIUM" &&
        priority != "HIGH" {
        return Err(
            ValidationError::new(
                "priority must be one of ('LOW', 'MEDIUM', 'HIGH')"
            )
        );
    }
    return Ok(());
}