use bollard::container::ListContainersOptions;
use actix_web::{web::Data, Responder};
use std::collections::HashMap;
use crate::types::AppState;
use std::sync::Mutex;
use super::HttpServer;
use bollard::Docker;

impl HttpServer {
    pub async fn pod_list(app_state: Data<Mutex<AppState>>, docker: Data<Docker>) -> impl Responder {
        let mut state = app_state.lock().unwrap();

        let filters = HashMap::from([
            ("label", vec!["app=canoa"])
        ]);

        let options = ListContainersOptions {
            all: false,
            filters,
            ..ListContainersOptions::default()
        };

        let containers = docker.list_containers(Some(options)).await;

        match containers {
            Ok(objs) => {
                for obj in objs {
                    println!("Container: {:?}", obj);
                }

                //"passou"
            },
            _ => println!("Error")
        }

        "Cosmo"
    }
}
