use crate::models::node::Node;
use crate::models::response::ResponseBody;
use crate::AppState;
use actix_web::{web, HttpResponse, Result};

pub async fn insert(data: web::Data<AppState>, node: web::Json<Node>) -> Result<HttpResponse> {
    let mut nodes = data.nodes.lock().await;
    nodes.push(node.0);
    Ok(HttpResponse::Ok().json(ResponseBody::new("OK", "")))
}

pub async fn find_all(data: web::Data<AppState>) -> Result<HttpResponse> {
    let nodes = &*data.nodes.lock().await;
    Ok(HttpResponse::Ok().json(ResponseBody::new("OK", nodes)))
}