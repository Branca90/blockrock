use rocket::{Rocket, Build};
use prometheus::Registry;

pub fn init_metrics(rocket: Rocket<Build>) -> Rocket<Build> {
    let registry = Registry::new();
    rocket.manage(registry)
}
