extern crate librespot;
extern crate tokio;

use tokio_core::reactor::Core;
use librespot::core::config::SessionConfig;


#[tokio::main]
async fn main() {
    std::env::env_logger::init();
    let mut core = Core::new().unwrap();

    let session_config = SessionConfig::Default();



    println!("{:?}", session_config);
}
