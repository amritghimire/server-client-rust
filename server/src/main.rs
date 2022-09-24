use clap::Parser;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;

use server::configuration::Settings;
use server::startup::app;

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log")]
    log_level: Option<String>,

    /// set the listen addr
    #[clap(short = 'a', long = "addr")]
    addr: Option<String>,

    /// set the listen port
    #[clap(short = 'p', long = "port")]
    port: Option<u16>,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir")]
    static_dir: Option<String>,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    let mut settings = Settings::new().expect("Failed to read configuration file");

    if let Some(log_level) = opt.log_level {
        settings.application.log_level = log_level;
    }

    if let Some(addr) = opt.addr {
        settings.application.addr = addr;
    }

    if let Some(port) = opt.port {
        settings.application.port = port;
    }

    if let Some(static_dir) = opt.static_dir {
        settings.application.static_dir = static_dir;
    }

    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!("{},hyper=info,mio=info", settings.application.log_level),
        )
    }

    // initialize tracing
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from((
        IpAddr::from_str(settings.application.addr.as_str())
            .unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        settings.application.port,
    ));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app(Some(settings.application.static_dir)).into_make_service())
        .await
        .unwrap();
}
