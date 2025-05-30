use sqlx::{
    Pool, 
    Postgres
};
use tracing::{error, info};
use uuid::Uuid;
use cookie::Cookie;

use crate::{
    error::AppError, 
    modules::user::User
};

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
            ON CONFLICT (user_id) DO UPDATE
            SET 
                data = EXCLUDED.data,
                expires_at = CURRENT_TIMESTAMP + INTERVAL '7 days'
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

pub async fn get_user_by_session(
    session: String,
    pool: &Pool<Postgres>
) -> Result<User, AppError> {
    let user = sqlx::query_as::<_, User>(r#"
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
            users.id = (
                SELECT user_id FROM sessions
                WHERE 
                    sessions.data = $1 AND 
                    sessions.expires_at - CURRENT_TIMESTAMP > INTERVAL '0 days'
                LIMIT 1
            ) AND 
            users.state = 'active' ;
    "#)
        .bind(&session)
        .fetch_one(pool)
        .await;
    match user {
        Ok(data) => return Ok(data),
        Err(err) => match err {
            sqlx::Error::RowNotFound => return Err(AppError::NotFoundUser),
            other => {
                error!("{:#?}", other);
                return Err(AppError::InternalServer);
            }
        }
    };
}

pub async fn delete_session(
    user_id: i32,
    pool: &Pool<Postgres>
) -> Result<(), AppError> {
    let result = sqlx::query(r#"
        DELETE FROM sessions
        WHERE user_id = $1
    "#)
        .bind(user_id)
        .execute(pool)
        .await;
    match result {
        Ok(data) => {
            if data.rows_affected() > 0 {
                return Ok(());
            }
            return Err(AppError::NotFoundUser);
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
        .max_age(cookie::time::Duration::days(7)).to_string();
} 

pub fn build_deleted_cookie() -> String {
    return Cookie::build(("session", ""))
        .path("/")
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(0)).to_string();
}