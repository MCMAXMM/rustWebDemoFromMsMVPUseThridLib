use actix_web::{web,App,HttpServer};
use std::io;
use std::sync::Mutex;
#[path="../handlers.rs"]
mod handlers;

#[path="../routers.rs"]
mod routers;

#[path="../state.rs"]
mod state;

#[path="../models.rs"]
mod models;

use routers::*;
use state::AppState;
#[actix_rt::main]
async fn main()->io::Result<()>{
    //创建web data 方便handler注入依赖
    let shared_data=web::Data::new(AppState{
        health_check_response:"I'm OK.".to_string(),
        visit_count:Mutex::new(0),
        courses:Mutex::new(Vec::new()),
    });
    //配置数据
    let app=move||{
        App::new()
        .app_data(shared_data.clone())
        .configure(general_routes)
        .configure(course_routes)
    };
    HttpServer::new(app).bind("localhost:3000")?.run().await
}