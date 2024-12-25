use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use serde::Deserialize;

use crate::{models::class::Class, server::internal_error};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateClass {
    pub name: String,
    pub group: String,
    pub code: String,
    pub session_id: i32,
}

pub fn router() -> Router<deadpool_diesel::sqlite::Pool> {
    Router::new()
        .route("/", get(get_all_classes))
        .route("/", post(add_class))
}

async fn get_all_classes(
    State(pool): State<deadpool_diesel::sqlite::Pool>,
) -> Result<(StatusCode, Json<Vec<Class>>), (StatusCode, String)> {
    use crate::schema::classes::dsl::*;
    let conn = pool.get().await.unwrap();
    let results = conn
        .interact(|conn| classes.select(Class::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(results)))
}

async fn add_class(
    State(pool): State<deadpool_diesel::sqlite::Pool>,
    Json(payload): Json<CreateClass>,
) -> Result<(StatusCode, Json<Class>), (StatusCode, String)> {
    use crate::schema::classes;

    let conn = pool.get().await.unwrap();
    let class = Class {
        id: 0,
        code: payload.code.clone(),
        name: payload.name.clone(),
        group: payload.group.clone(),
        session_id: payload.session_id,
    };

    let results = conn
        .interact(move |conn| {
            diesel::insert_into(classes::table)
                .values(&class)
                .returning(Class::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(results)))
}
