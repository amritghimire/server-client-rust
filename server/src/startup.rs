use crate::configuration::Settings;
use crate::routes::{health_check, subscriptions};
use crate::telemetry::{get_subscriber, init_subscriber};
use crate::{Opt, State};

use axum::http::Request;
use axum::routing::get_service;
use axum::{http::StatusCode, routing::get, Extension, Router};

use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestId, RequestId},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};
use tracing::Level;
use uuid::Uuid;

#[derive(Clone)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string();

        Some(RequestId::new(request_id.parse().unwrap()))
    }
}

pub async fn app(settings: Settings, db: Option<PgPool>) -> Router {
    let connection = if let Some(database) = db {
        database
    } else {
        PgPool::connect(settings.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres")
    };

    sqlx::migrate!()
        .run(&connection)
        .await
        .expect("Failed to perform migration");

    let mut router = Router::new()
        .route("/api/health_check", get(health_check))
        .merge(subscriptions::router());
    if !settings.application.static_dir.is_empty() {
        router = router.fallback(
            Router::new().nest(
                "/",
                get_service(ServeDir::new(&settings.application.static_dir).fallback(
                    ServeFile::new(Path::new(&settings.application.static_dir).join("index.html")),
                ))
                .handle_error(|error| async move {
                    tracing::error!(?error, "failed serving static file");
                    StatusCode::INTERNAL_SERVER_ERROR
                })
                .layer(TraceLayer::new_for_http()),
            ),
        )
    }

    let shared_settings = Arc::new(settings);
    let shared_database = Arc::new(State {
        database: connection,
    });

    router
        .layer(Extension(shared_database))
        .layer(Extension(shared_settings))
        .layer(
            // from https://docs.rs/tower-http/0.2.5/tower_http/request_id/index.html#using-trace
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(
                            DefaultMakeSpan::new()
                                .include_headers(true)
                                .level(Level::INFO),
                        )
                        .on_response(DefaultOnResponse::new().include_headers(true)),
                )
                .propagate_x_request_id(),
        )
}

pub async fn start(opt: Opt, mut settings: Settings) {
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
    let sub = get_subscriber("server-app".into(), "info".into(), std::io::stdout);
    init_subscriber(sub);

    let addr = SocketAddr::from((
        IpAddr::from_str(settings.application.addr.as_str())
            .unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        settings.application.port,
    ));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app(settings, None).await.into_make_service())
        .await
        .unwrap();
}
