mod blockchain;
mod server;
mod sha;
use server::server::start_server;
use crate::blockchain::Chain;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    start_server().await
}
