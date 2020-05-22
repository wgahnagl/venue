use actix::{System};
use dotenv::dotenv;
use models::env_config::ServerConfig;
use api::server;

pub fn main(){
    dotenv().ok();
    let config = ServerConfig::from_env().expect("failed to get server config");
    
    System::new("venue").block_on(server(config.bind_address)).expect("failed to start API");
    
}
