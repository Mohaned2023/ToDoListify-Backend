use argon2::{
    password_hash::{
        rand_core::OsRng, 
        SaltString
    }, 
    Argon2, 
    PasswordHash, 
    PasswordHasher
};
use sqlx::{Pool, Postgres};
use tracing::error;
use crate::{
    error::AppError, 
    modules::user::{
        CreateDto, 
        LoginDto, 
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
                error!("{:#?}", db_err);
                return Err(AppError::InternalServer);
            }
            _ => {
                error!("{:#?}", e);
                return Err(AppError::InternalServer)
            },
        }
    }
}

pub async fn login(
    login_dto: LoginDto,
    pool: &Pool<Postgres>
) -> Result<User, AppError> {
    let user = sqlx::query_as::<_, User> (
        r#"
            SELECT 
                id, 
                name, 
                email, 
                username, 
                password,
                to_char(create_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as create_at, 
                to_char(update_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as update_at
            FROM users
            WHERE 
                username = $1 
        "#,
    )
        .bind(&login_dto.username)
        .bind(&login_dto.password)
        .fetch_one(pool)
        .await;
    match user {
        Ok(data) => {
            if let Ok(parsed_hash) = PasswordHash::new(&data.password) {
                let result = parsed_hash.verify_password(&[&Argon2::default()], login_dto.password);
                if result.is_ok() {
                    return Ok(data);
                }
            }
            return Err(AppError::Unauthorized);
        },
        Err(e) => match e {
            sqlx::Error::RowNotFound => return Err(AppError::NotFoundUser),
            other => {
                error!("{:#?}", other);
                return Err(AppError::InternalServer)
            }
        }
    }
}
