use actix_web::{web, Scope};

use crate::api::healthcheck;

pub struct HealthcheckRouter {}

impl HealthcheckRouter {
    pub fn get_router() -> Scope {
        web::scope("/healthcheck")
            .service(healthcheck::healthcheck)
    }
}
