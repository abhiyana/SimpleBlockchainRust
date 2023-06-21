use crate::blockchain::Chain;
use crate::blockchain::Transaction;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct TransactionRequest {
    sender: String,
    recipient: String,
    amount: u64,
}

#[derive(Serialize)]
struct TransactionResponse {
    message: String,
    transaction: Option<Transaction>,
}

#[derive(Serialize)]
struct ResolveResponse {
    message: String,
    chain: String,
}

#[derive(Serialize, Deserialize)]
struct NodeRequest {
    nodes: Vec<String>,
}

async fn index() -> impl Responder {
    HttpResponse::SeeOther()
        .header("Location", "/chain")
        .finish()
}

pub async fn chain_handler(chain: web::Data<Mutex<Chain>>) -> impl Responder {
    let chain = chain.lock().unwrap();
    HttpResponse::Ok().json(chain.to_json())
}

async fn mine(state: web::Data<Mutex<Chain>>) -> impl Responder {
    let mut chain = state.lock().unwrap();
    match chain.last_block() {
        Some(last_block) => {
            let last_proof = last_block.proof();
            let proof = Chain::proof_of_work(last_proof);
            let previous_hash = Chain::hash(last_block);
            let block = chain.new_block(Some(previous_hash.to_owned()), proof);

            let response = json!({
                "message": "New block forged",
                "proof": proof,
                "previous_hash": previous_hash,
                "index": block.0,
                "transactions": block.1,
            });

            HttpResponse::Ok().json(response)
        }
        None => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body("Unable to mine a new block"),
    }
}

async fn new_transaction(
    body: web::Json<TransactionRequest>,
    state: web::Data<Mutex<Chain>>,
) -> impl Responder {
    let mut chain = state.lock().unwrap();
    let index = chain.new_transaction(body.sender.clone(), body.recipient.clone(), body.amount);
    let transaction = chain.current_transactions().get(index as usize).cloned();

    let response = TransactionResponse {
        message: "Transaction added to the block".to_string(),
        transaction,
    };

    HttpResponse::Ok().json(response)
}

async fn resolve(state: web::Data<Mutex<Chain>>) -> impl Responder {
    let mut chain = state.lock().unwrap();
    let result = chain.resolve_conflicts();
    let message = if result {
        "Our chain was replaced"
    } else {
        "Our chain is authoritative"
    };

    let response = ResolveResponse {
        message: message.to_string(),
        chain: chain.to_json().to_string(),
    };

    HttpResponse::Ok().json(response)
}

async fn register_node(
    body: web::Json<NodeRequest>,
    state: web::Data<Mutex<Chain>>,
) -> impl Responder {
    let mut chain = state.lock().unwrap();
    for node in &body.nodes {
        chain.register_node(node);
    }

    let response = json!({
        "message": "New nodes have been added",
        "nodes": chain.nodes(),
    });

    HttpResponse::Ok().json(response)
}

async fn nodes(state: web::Data<Mutex<Chain>>) -> impl Responder {
    let chain = state.lock().unwrap();
    HttpResponse::Ok().json(chain.nodes())
}

pub async fn start_server() -> std::io::Result<()> {
    let chain = web::Data::new(Mutex::new(Chain::new()));

    HttpServer::new(move || {
        let chain = chain.clone();

        App::new()
            .route("/", web::get().to(index))
            .route("/chain", web::get().to(chain_handler)) // Use chain_handler as the route handler
            .route("/mine", web::get().to(mine))
            .route("/transactions/new", web::post().to(new_transaction))
            .route("/nodes/resolve", web::get().to(resolve))
            .route("/nodes/register", web::post().to(register_node))
            .route("/nodes", web::get().to(nodes))
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
