pub mod configuration;
pub mod error;
pub mod routes;
pub mod startup;
pub mod telemetry;

use axum::extract::rejection::JsonRejection;
use axum::Json;
use clap::Parser;
use sqlx::PgPool;

pub type JsonPayload<T> = Result<Json<T>, JsonRejection>;
pub type JsonResponse<T> = Result<Json<T>, error::AppError>;

pub struct State {
    pub database: PgPool,
}

// Setup the command line interface with clap.
#[derive(Parser, Debug, Default)]
#[clap(name = "server", about = "A server for our wasm project!")]
pub struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log")]
    pub log_level: Option<String>,

    /// set the listen addr
    #[clap(short = 'a', long = "addr")]
    pub addr: Option<String>,

    /// set the listen port
    #[clap(short = 'p', long = "port")]
    pub port: Option<u16>,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir")]
    pub static_dir: Option<String>,
}
