use actix_web::{App, HttpServer, web::{get, post, scope, Data}};
use std::sync::Mutex;
use bollard::Docker;
mod http_validate;
use actix::Actor;
mod types;
mod http;


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let docker = Docker::connect_with_local_defaults()
       .expect("Unable to connect to docker");

    let app_state = types::AppState::default();

    let background_actor = http::BackgroundActor {
        app_state: app_state.clone(),
        docker: docker.clone(),
        requesting_containers_run: false,
    }.start();

    let app_state = Data::new(Mutex::new(app_state));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(docker.clone()))
            .app_data(Data::clone(&app_state))
            .app_data(Data::new(background_actor.clone()))
            .route("/hello", get().to(http::HttpServer::hello))
            .service(
                scope("/pod")
                    .route("", get().to(http::HttpServer::pod_list))
                    .route("", post().to(http::HttpServer::pod_run))
                    .route("/containers-run", post().to(http::HttpServer::pod_containers_run))
            )
    })
    .bind(("127.0.0.1", 9797))?
    .run()
    .await
}
