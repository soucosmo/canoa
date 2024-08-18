use bollard::image::ListImagesOptions;
use bollard::container::{StatsOptions, CreateContainerOptions, ListContainersOptions};
use actix_web::{web::Data, Responder, HttpResponse};
use futures_util::stream::TryStreamExt;
use std::collections::HashMap;
use crate::types::AppState;
use std::sync::Mutex;
use serde_json::json;
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

    pub async fn pod_run(app_state: Data<Mutex<AppState>>) -> impl Responder {
        //app_state.docker.create_container(options, config)

        let mut state = app_state.lock().unwrap();


        return HttpResponse::Created().json(
            json!({
                "nome": "Cosmo",
                "pods": state.pods,
            })
        )
        /*
        
        let images = &app_state.docker.list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        })).await.unwrap();
    
    for image in images {
        println!("-> {:?}", image);
    }
    
    let stats = &app_state.docker.stats("postgres", Some(StatsOptions {
        stream: true,
        ..Default::default()
    })).try_collect::<Vec<_>>().await.unwrap();

for stat in stats {
    println!("{} - mem total: {:?} | mem usage: {:?}",
                 stat.name,
                 stat.memory_stats.max_usage,
                 stat.memory_stats.usage);
                }
                "Running pod"
                */
    }
}
        