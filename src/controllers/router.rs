use actix_web::web::{ServiceConfig,self};

use super::healthcheck::HealthcheckRouter;

pub fn register_router(sc:&mut ServiceConfig){
   sc.service(
       web::scope("")
           .service(HealthcheckRouter::get_router()) 
   );
}
