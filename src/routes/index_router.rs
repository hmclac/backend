use crate::controllers::index_controller;
use actix_web::web::{self, ServiceConfig};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index_controller::greet));
}
