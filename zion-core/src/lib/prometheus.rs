use prometheus::{register_counter, register_gauge, Counter, Gauge, TextEncoder};
use rocket::{get, routes, Rocket, Build};

lazy_static::lazy_static! {
    static ref TX_COUNTER: Counter = register_counter!(
        "blockrock_transactions_total",
        "Numero totale di transazioni"
    ).unwrap();
    static ref ACTIVE_PEERS: Gauge = register_gauge!(
        "blockrock_active_peers",
        "Numero di peer attivi"
    ).unwrap();
}

pub fn update_metrics(tx_count: u64, peer_count: f64) {
    TX_COUNTER.inc_by(tx_count as f64);
    ACTIVE_PEERS.set(peer_count);
}

#[get("/metrics")]
async fn metrics() -> String {
    let encoder = TextEncoder::new();
    let mut buffer = String::new();
    encoder.encode_utf8(&prometheus::gather(), &mut buffer).unwrap();
    buffer
}

pub fn init_metrics(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/", routes![metrics])
}
