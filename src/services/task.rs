use sqlx::{
    Pool, 
    Postgres
};
use tracing::error;

use crate::{
    modules::task::{
        CreateDto,
        Task
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