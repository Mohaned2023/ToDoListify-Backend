use argon2::{
    password_hash::{
        rand_core::OsRng, 
        SaltString
    }, 
    PasswordHasher,
    Argon2
};
use sqlx::{Pool, Postgres};
use crate::{
    error::AppError, 
    modules::user::{
        CreateDto,
        User
    }
};

pub async fn create(
    create_dto: CreateDto,
    pool: &Pool<Postgres>
) -> Result<User, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(
            &create_dto.password.as_bytes(),
            &salt
        )
        .unwrap();
    let user = sqlx::query_as::<_, User>(
        r#"
            INSERT INTO users (name, email, username, password, salt)
            VALUES ( $1, $2, $3, $4, $5 )
            RETURNING 
                id, 
                name, 
                email, 
                username, 
                to_char(create_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as create_at, 
                to_char(update_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as update_at
        "#
    )
        .bind(&create_dto.name)
        .bind(&create_dto.email)
        .bind(&create_dto.username)
        .bind(&hash.to_string())
        .bind(&salt.to_string())
        .fetch_one(pool)
        .await;
    match user {
        Ok(data) => return Ok(data),
        Err(e) => match e {
            sqlx::Error::Database(db_err) => {
                if let Some(err_code) = db_err.code() {
                    if err_code == "23505" {
                        return Err(AppError::UserFound);
                    }
                }
                return Err(AppError::InternalServer);
            }
            _ => return Err(AppError::InternalServer),
        }
    }
}