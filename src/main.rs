use actix_web::{App, HttpServer, web::{get, post, scope, Data}};
use std::sync::Mutex;
use bollard::Docker;
mod http_validate;
mod types;
mod http;


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let docker = Docker::connect_with_local_defaults()
       .expect("Unable to connect to docker");

    let app_state = types::AppState::default();

    let app_state = Data::new(Mutex::new(app_state));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(docker.clone()))
            .app_data(Data::clone(&app_state))
            .route("/hello", get().to(http::HttpServer::hello))
            .service(
                scope("/pod")
                    .route("", get().to(http::HttpServer::pod_list))
                    .route("", post().to(http::HttpServer::pod_run))
            )
    })
    .bind(("127.0.0.1", 9797))?
    .run()
    .await
}
