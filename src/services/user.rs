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
use tracing::{error, info};
use crate::{
    error::AppError, 
    modules::user::{
        CreateDto, 
        DeleteDto, 
        LoginDto, 
        UpdateInformationDto, 
        UpdatePasswordDto, 
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
                username = $1 AND
                state    = 'active' 
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

pub async fn update_information(
    update_info_dto: UpdateInformationDto,
    user: User,
    pool: &Pool<Postgres>
) -> Result<User, AppError> {
    let mut _email: String = user.email.clone();
    let mut _name: String = user.name.clone();
    let mut _username: String = user.username.clone();

    if update_info_dto.email.is_some() {
        _email = update_info_dto.email.unwrap();
    }
    if update_info_dto.name.is_some() {
        _name = update_info_dto.name.unwrap();
    }
    if update_info_dto.username.is_some() {
        _username = update_info_dto.username.unwrap();
    }

    if  _name     == user.name  &&
        _email    == user.email &&
        _username == user.username {
        return Err(AppError::BadRequest);
    }

    let user = sqlx::query_as::<_, User>(r#"
        UPDATE users
        SET 
            email     = $1,
            name      = $2,
            username  = $3,
            update_at = CURRENT_TIMESTAMP
        WHERE
            id = $4
        RETURNING 
            id, 
            name, 
            email, 
            username,
            password,
            to_char(create_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as create_at, 
            to_char(update_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as update_at
    "#)
        .bind(_email)
        .bind(_name)
        .bind(_username)
        .bind(user.id)
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

pub async fn update_password(
    update_pass_dto: UpdatePasswordDto,
    user: User,
    pool: &Pool<Postgres>
) -> Result<(), AppError> {
    if let Ok(parsed_hash) = PasswordHash::new(&user.password) {
        let result = parsed_hash
            .verify_password(&[&Argon2::default()], update_pass_dto.old_password);
        if result.is_err() {
            return Err(AppError::Unauthorized);
        }
    }
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(
            &update_pass_dto.password.as_bytes(),
            &salt
        )
        .unwrap();
    let result = sqlx::query(r#"
        UPDATE users
        SET
            password  = $1,
            salt      = $2,
            update_at = CURRENT_TIMESTAMP
        WHERE
            id = $3
    "#)
        .bind(hash.to_string())
        .bind(salt.to_string())
        .bind(user.id)
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

pub async fn delete(
    delete_dto: DeleteDto,
    user: User,
    pool: &Pool<Postgres>
) -> Result<(), AppError> {
    match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => {
            let result = parsed_hash
                .verify_password(&[&Argon2::default()], delete_dto.password);
            if result.is_err() {
                return Err(AppError::Unauthorized);
            }
        }
        Err(e) => {
            error!("{:#?}", e);
            return Err(AppError::InternalServer);
        }
    }
    let result = sqlx::query(r#"
        DELETE FROM users
        WHERE id = $1;
    "#)
        .bind(user.id)
        .execute(pool)
        .await;
    match result {
        Ok(_) => {
            info!("user '{}' has been deleted.", user.name);
            return Ok(())
        }
        Err(e) => {
            error!("{:#?}", e);
            return Err(AppError::InternalServer);
        }
    }
}