use actix_web::{get, Responder};

use crate::utils::results::ResponseResult;

#[get("/")]
async fn healthcheck() -> impl Responder {
    ResponseResult::success("success")
}
