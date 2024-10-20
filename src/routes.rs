use crate::handlers::*;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(readme);
    cfg.service(add_node);
    cfg.service(del_node);
    cfg.service(get_node);
}
