use sqlx::{
    Pool, 
    Postgres
};
use tracing::error;

use crate::{
    modules::task::{
        CreateDto,
        Task,
        UpdateDto
    },
    error::AppError
};

pub async fn create(
    create_dto: CreateDto,
    user_id: i32,
    pool: &Pool<Postgres>
) -> Result<Task, AppError> {
    let result = sqlx::query_as::<_, Task>(r#"
        INSERT INTO tasks (user_id, title, body, state, priority)
        VALUES ( $1, $2, $3, $4, $5 )
        RETURNING
            id,
            user_id,
            title,
            body,
            state,
            priority,
            to_char(created_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at, 
            to_char(updated_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as updated_at
    "#)
        .bind(user_id)
        .bind(create_dto.title)
        .bind( & if create_dto.body.is_some() {create_dto.body.unwrap()} else {"".to_string()} )
        .bind( & if create_dto.state.is_some() {create_dto.state.unwrap()} else {"TO_DO".to_string()} )
        .bind( & if create_dto.priority.is_some() {create_dto.priority.unwrap()} else {"MEDIUM".to_string()} )
        .fetch_one(pool)
        .await;
    match result {
        Ok(task) => return Ok(task),
        Err(e) => {
            error!("{:#?}", e);
            return Err(AppError::InternalServer);
        }
    }
}

pub async fn get_all(
    user_id: i32,
    pool: &Pool<Postgres>
) -> Result<Vec<Task>, AppError> {
    let result = sqlx::query_as::<_, Task>(r#"
        SELECT 
            id,
            user_id,
            title,
            body,
            state,
            priority,
            to_char(created_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at, 
            to_char(updated_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as updated_at
        FROM tasks
        WHERE user_id = $1;
    "#)
        .bind(user_id)
        .fetch_all(pool)
        .await;
    match result {
        Ok(tasks) => return Ok(tasks),
        Err(e) => match e {
            sqlx::Error::RowNotFound => return Err(AppError::NotFoundData),
            other => {
                error!("{:#?}", other);
                return Err(AppError::InternalServer);
            }
        }
    }
}

pub async fn udpate(
    update_dto: UpdateDto,
    id: i32,
    user_id: i32,
    pool: &Pool<Postgres>
) -> Result<Task, AppError> {
    let is_update: bool = update_dto.title.is_some() ||
        update_dto.body.is_some()  ||
        update_dto.state.is_some() ||
        update_dto.priority.is_some() ;
    if !is_update {
        return Err(AppError::BadRequest);
    }

    let result = sqlx::query_as::<_, Task>(r#"
        UPDATE tasks
        SET 
            title = CASE
                WHEN LENGTH($1) > 0 THEN $1
                ELSE title
            END,
            body  = CASE
                WHEN LENGTH($2) > 0 THEN $2
                ELSE body
            END,
            state  = CASE
                WHEN LENGTH($3) > 0 THEN $3
                ELSE state
            END,
            priority = CASE
                WHEN LENGTH($4) > 0 THEN $4
                ELSE priority
            END,
            updated_at = CASE
                WHEN $5 THEN CURRENT_TIMESTAMP
                ELSE updated_at
            END
        WHERE
            user_id = $6 AND
            id      = $7
        RETURNING
            id,
            user_id,
            title,
            body,
            state,
            priority,
            to_char(created_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at, 
            to_char(updated_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as updated_at
    "#)
        .bind( & if update_dto.title.is_some() { update_dto.title.unwrap() } else { "".to_string() })
        .bind( & if update_dto.body.is_some() { update_dto.body.unwrap() } else { "".to_string() })
        .bind( & if update_dto.state.is_some() { update_dto.state.unwrap() } else { "".to_string() })
        .bind( & if update_dto.priority.is_some() { update_dto.priority.unwrap() } else { "".to_string() })
        .bind( true )
        .bind(user_id)
        .bind(id)
        .fetch_one(pool)
        .await;
    match result {
        Ok(task) => return Ok(task),
        Err(e) => match e {
            sqlx::Error::RowNotFound => return Err(AppError::NotFoundData),
            other => {
                error!("{:#?}", other);
                return Err(AppError::InternalServer);
            }
        }
    }
}

pub async fn delete(
    id: i32,
    user_id: i32,
    pool: &Pool<Postgres>
) -> Result<(), AppError> {
    let result = sqlx::query(r#"
        DELETE FROM tasks
        WHERE
            id      = $1 AND
            user_id = $2
    "#)
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await;
    match result {
        Ok(data) => {
            if data.rows_affected() > 0 {
                return Ok(());
            }
            return Err(AppError::NotFoundData);
        }
        Err(e) => {
            error!("{:#?}", e);
            return Err(AppError::InternalServer);
        }
    }
}