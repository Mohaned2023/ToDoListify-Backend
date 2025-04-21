use argon2::{
    password_hash::{
        rand_core::OsRng, 
        SaltString
    }, 
    PasswordHasher,
    Argon2
};
use cookie::Cookie;
use sqlx::{Pool, Postgres};
use tracing::{error, info};
use uuid::Uuid;
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

pub async fn create_session(
    username: &str,
    user_id: i32,
    pool: &Pool<Postgres>
) -> Result<String, AppError> {
    let session = Uuid::new_v4().to_string();
    let rows = sqlx::query(
        r#"
            INSERT INTO sessions (user_id, data)
            VALUES ($1, $2)
            ON CONFLICT (user_id) DO NOTHING;
        "#
    )
        .bind(&user_id)
        .bind(&session)
        .execute(pool)
        .await;
    match rows {
        Ok(r) => {
            if r.rows_affected() > 0 {
                info!("create session for {}", username);
                return Ok(session);
            }
            error!("Can NOT create the session for {}", username);
            return Err(AppError::CanNotCreeateSession);
        }
        Err(e) => {
            error!("{:#?}", e);
            return Err(AppError::InternalServer);
        }
    }
}

pub fn build_cookie( session: String ) -> String {
    return Cookie::build(("session", session))
        .path("/")
        .http_only(true)
        .secure(true)
        .max_age(cookie::time::Duration::days(7)).to_string();
} 