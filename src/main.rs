mod error;

use bradis::{Addr, Server};
use clap::Parser;
use error::Error;
use tokio::net::{TcpListener, ToSocketAddrs};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(version, about)]
struct Options {
    #[arg(short, long, default_value_t = 3333)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let options = Options::parse();
    serve(("127.0.0.1", options.port)).await
}

async fn serve<T: ToSocketAddrs>(addr: T) -> Result<(), Error> {
    let server = Server::default();
    let listener = TcpListener::bind(addr).await?;

    loop {
        match listener.accept().await {
            Ok((stream, peer)) => {
                let local = stream.local_addr().unwrap();
                info!("Accepted {}", peer);
                server.connect(stream, Addr { local, peer });
            }
            Err(error) => {
                info!("{}", error);
            }
        }
    }
}
