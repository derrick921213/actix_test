// rust config file using yaml

// use config::{Config, ConfigError, File};
// use serde::{Deserialize, Serialize};
// use std::fs;
// #[derive(Debug, Serialize, Deserialize)]
// struct Settings {
//     debug: bool,
//     database: DatabaseSettings,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct DatabaseSettings {
//     url: String,
//     pool_size: u32,
// }

// impl Default for Settings {
//     fn default() -> Self {
//         Settings {
//             debug: true,
//             database: DatabaseSettings {
//                 url: "postgres://localhost/mydb".to_string(),
//                 pool_size: 5,
//             },
//         }
//     }
// }
// fn main() -> Result<(), ConfigError> {
//     let config_path = "config/config.yaml";
//     let config_dir = "config";
//     if !std::path::Path::new(config_dir).exists() {
//         fs::create_dir(config_dir).expect("Failed to create config directory");
//     }

//     if !std::path::Path::new(config_path).exists() {
//         let default_settings = Settings::default();
//         let default_settings_str =
//             serde_yaml::to_string(&default_settings).expect("Failed to serialize default settings");

//         fs::write(config_path, default_settings_str).expect("Failed to write default config");
//         println!("Default configuration file created at '{}'.", config_path);
//     }

//     let settings = Config::builder()
//         .add_source(File::with_name(config_path))
//         .add_source(config::Environment::with_prefix("APP"))
//         .build()?;
//     let settings: Settings = settings.try_deserialize()?;
//     println!("{:#?}", settings);
//     Ok(())
// }

// use actix_web::{error, web, App, HttpResponse, HttpServer, Responder};
// use flexi_logger::{Duplicate, FileSpec, Logger, WriteMode};
// use log::{error, info};
// use serde::Deserialize;
// use std::fs::File;
// use std::sync::Mutex;

// #[derive(Deserialize)]
// struct Info {
//     username: String,
// }

// /// deserialize `Info` from request's body, max payload size is 4kb
// async fn index(info: web::Json<Info>) -> impl Responder {
//     format!("Welcome {}!", info.username)
// }

// lazy_static::lazy_static! {
//     static ref LOG_FILE: Mutex<File> = Mutex::new(File::create("server.log").unwrap());
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     Logger::try_with_str("info")
//         .unwrap()
//         .log_to_file(FileSpec::default().directory("logs"))
//         .duplicate_to_stdout(Duplicate::Info)
//         .write_mode(WriteMode::BufferAndFlush)
//         .rotate(
//             flexi_logger::Criterion::Size(10 * 1024 * 1024),
//             flexi_logger::Naming::Numbers,
//             flexi_logger::Cleanup::KeepLogFiles(7),
//         )
//         .start()
//         .unwrap();
//     info!("Starting Actix Web server...");
//     HttpServer::new(|| {
//         let json_config = web::JsonConfig::default()
//             .limit(4096)
//             .error_handler(|err, _req| {
//                 error!("Error occurred: {:?}", err);
//                 let msg = err.to_string();
//                 error::InternalError::from_response(
//                     err,
//                     HttpResponse::Conflict().body(format!("Error: {}", msg)),
//                 )
//                 .into()
//             });

//         App::new().service(
//             web::resource("/")
//                 .app_data(json_config)
//                 .route(web::post().to(index)),
//         )
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

mod data;
mod data_parser;
mod handlers;
mod init;
mod routes;
use actix_web::{web, App, HttpServer};
use data_parser::NodeManagementMap;
use init::init;
use std::{
    fs::File,
    sync::{Arc, Mutex},
};
use tokio::select;
use tokio::signal::unix::{signal, SignalKind};

fn save_data_on_exit(data: Arc<Mutex<NodeManagementMap>>) {
    let data_map = data.lock().unwrap();
    if let Some(node_management) = data_map.get("NodeManagement") {
        let node_management = node_management.lock().unwrap();
        let mut file = File::create("data/NodeManagement.json").expect("Failed to create file");
        serde_json::to_writer_pretty(&mut file, &*node_management)
            .expect("Failed to write to file");

        println!("Data saved to 'data/NodeManagement.json'");
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = Arc::new(Mutex::new(init()?));
    let server_data = data.clone();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_data.clone()))
            .configure(routes::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    let server_handle = server.handle();
    let data_clone = data.clone();

    let handle = tokio::spawn(async move {
        let mut sigint = signal(SignalKind::interrupt()).expect("Failed to listen for SIGINT");
        let mut sigterm = signal(SignalKind::terminate()).expect("Failed to listen for SIGTERM");

        select! {
            _ = sigint.recv() => {
                println!("Received SIGINT, shutting down server...");
            }
            _ = sigterm.recv() => {
                println!("Received SIGTERM, shutting down server...");
            }
        }

        server_handle.stop(true).await;
        save_data_on_exit(data_clone);
    });

    server.await?;
    handle.await.expect("Failed to await handle");

    Ok(())
}
