use std::sync::Arc;
use axum::{extract::{Path, State}, Json, response::IntoResponse};
use crate::{app_error, db::SDB, blog::Payload};


type Response = Result<axum::response::Response, app_error::Error>;

pub async fn health_check() -> Response{
    Ok("All Systems Go!".into_response())
}

pub async fn create_blogs(State(db):State<Arc<SDB>>, Json(body):Json<Vec<Payload>>) -> Response { 

    let mut response = (0, 0);
    for blog in body {
        match db.create_blog(blog).await {
            Ok(_) => {response.0 += 1},
            Err(e) => {
                tracing::error!("Issue creating blog: {e:?}");
                response.1 += 1
            }
        }
    }
    Ok(format!("Create blog succeeded with {} blogs created and {} exceptions", response.0, response.1).into_response())
}

pub async fn get_blog(State(db):State<Arc<SDB>>, Path(blog_id): Path<u32>) -> Response { 
    let blog = db.get_blog(blog_id).await?;
    Ok(Json(blog).into_response())
}

pub async fn get_all_metadata(State(db):State<Arc<SDB>>) -> Response { 
    let metadata = db.get_all_blogs_metadata().await?;
    Ok(Json(metadata).into_response())
}

pub async fn delete_blogs(State(db):State<Arc<SDB>>) -> Response { 
    let deleted = db.delete_all_blogs().await?;
    Ok(Json(deleted).into_response())
}
