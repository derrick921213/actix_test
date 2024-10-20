use crate::data_parser::{init_data, NodeManagementMap};
use flexi_logger::{Duplicate, FileSpec, Logger, WriteMode};
use log::info;

fn init_log() {
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default().directory("logs"))
        .duplicate_to_stdout(Duplicate::Info)
        .write_mode(WriteMode::BufferAndFlush)
        .rotate(
            flexi_logger::Criterion::Size(10 * 1024 * 1024),
            flexi_logger::Naming::Numbers,
            flexi_logger::Cleanup::KeepLogFiles(7),
        )
        .start()
        .unwrap();
    info!("Starting Actix Web server...");
}

pub fn init() -> std::io::Result<NodeManagementMap> {
    init_log();
    init_data()
}
