use crate::{data::Node, data::NodeStatus, data_parser::NodeManagementMap};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
type Data = web::Data<Arc<Mutex<NodeManagementMap>>>;
#[derive(Deserialize)]
struct Info {
    username: String,
}
#[derive(Deserialize)]
struct NodeidRequest {
    id: String,
}
#[derive(Deserialize)]
#[allow(dead_code)]
struct NodeRequest {
    id: String,
    name: String,
    status: String,
    resource: NodeStatus,
}
#[post("/")]
async fn index(info: web::Json<Info>) -> impl Responder {
    HttpResponse::Ok().json(format!("Welcome {}!", info.username))
}
#[get("/readme")]
async fn readme() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../README.md"))
}

#[post("/add_node")]
#[allow(dead_code)]
async fn add_node(data: Data, node_request: web::Json<NodeRequest>) -> impl Responder {
    let mut data_map = data.lock().unwrap();

    if let Some(node_management) = data_map.get_mut("NodeManagement") {
        let mut node_management = node_management.lock().unwrap();
        node_management.add_node(
            Node::new(&node_request.id, &node_request.name, &node_request.status),
            NodeStatus::new(
                node_request.resource.get_cpu_usage(),
                node_request.resource.get_memory_usage(),
                node_request.resource.get_disk_usage(),
            ),
        );
        HttpResponse::Ok().json(&*node_management)
    } else {
        HttpResponse::NotFound().json("NodeManagement not found")
    }
}
#[post("/del_node")]
#[allow(dead_code)]
async fn del_node(data: Data, node_request: web::Json<NodeidRequest>) -> impl Responder {
    let mut data_map = data.lock().unwrap();

    if let Some(node_management) = data_map.get_mut("NodeManagement") {
        let mut node_management = node_management.lock().unwrap();
        node_management.remove_node(&node_request.id);
        HttpResponse::Ok().json(&*node_management)
    } else {
        HttpResponse::NotFound().json("NodeManagement not found")
    }
}
#[post("/get_node")]
#[allow(dead_code)]
async fn get_node(data: Data, node_request: web::Json<NodeidRequest>) -> impl Responder {
    let data_map = data.lock().unwrap();

    if let Some(node_management) = data_map.get("NodeManagement") {
        let node_management = node_management.lock().unwrap();
        println!("Requesting node id: {}", node_request.id);
        if let Some(node) = node_management.get_node(&node_request.id) {
            HttpResponse::Ok().json(node)
        } else {
            HttpResponse::NotFound().json("Node not found")
        }
    } else {
        HttpResponse::NotFound().json("NodeManagement not found")
    }
}
