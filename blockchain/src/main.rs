mod blockchain;
mod server;
mod sha;

use env_logger;
use log::{error, info, warn};
use server::server::start_server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server");

    start_server().await
}
