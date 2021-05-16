use crate::reserve::{Reserve, Reserves};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use serde_json::json;


#[get("/reserves")]
async fn list() -> Result<HttpResponse, CustomError>{
    let res = Reserves::list()?;
    Ok(HttpResponse::Ok().json(
        res
    ))
}

#[post("/reserve/create")]
async fn create_res(reserve: web::Json<Reserve>) -> Result<HttpResponse, CustomError>{
    let res = Reserves::create(reserve.into_inner())?;
    Ok(HttpResponse::Ok().json(
        res
    ))
}

pub fn reserve_routes(config: &mut web::ServiceConfig){
    config.service(create_res);
    config.service(list);
}