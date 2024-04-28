use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseResult<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}
impl<T: Serialize> Responder for ResponseResult<T> {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
impl<T> ResponseResult<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        ResponseResult {
            code: 200,
            msg: "Success".to_string(),
            data: Some(data),
        }
    }
    pub fn fail(msg: T) -> Self
    where
        T: AsRef<str>,
    {
        ResponseResult {
            code: 400,
            msg: msg.as_ref().to_string(),
            data: None,
        }
    }
}
