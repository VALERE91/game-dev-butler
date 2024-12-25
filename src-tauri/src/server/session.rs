use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use serde::Deserialize;

use crate::{models::session::Session, server::internal_error};

#[derive(Debug, Clone, Deserialize)]
struct CreateSession {
    pub year: i32,
    pub name: String,
}

pub fn router() -> Router<deadpool_diesel::sqlite::Pool> {
    Router::new()
        .route("/", get(get_all_sessions))
        .route("/", post(add_session))
}

async fn get_all_sessions(
    State(pool): State<deadpool_diesel::sqlite::Pool>,
) -> Result<(StatusCode, Json<Vec<Session>>), (StatusCode, String)> {
    use crate::schema::session::dsl::*;
    let conn = pool.get().await.unwrap();
    let results = conn
        .interact(|conn| session.select(Session::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(results)))
}

async fn add_session(
    State(pool): State<deadpool_diesel::sqlite::Pool>,
    Json(payload): Json<CreateSession>,
) -> Result<(StatusCode, Json<Session>), (StatusCode, String)> {
    use crate::schema::session;

    let conn = pool.get().await.unwrap();
    let session = Session {
        id: 0,
        year: payload.year,
        name: payload.name.clone(),
    };

    let results = conn
        .interact(move |conn| {
            diesel::insert_into(session::table)
                .values(&session)
                .returning(Session::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(results)))
}
