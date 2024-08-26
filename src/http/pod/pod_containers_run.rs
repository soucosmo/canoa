use bollard::container::{CreateContainerOptions, Config, StartContainerOptions};
use bollard::image;

//use futures_util::StreamExt;
use std::collections::HashMap;
use crate::types::AppState;
use actix_web::{web::Data, HttpResponse, Responder};
use super::HttpServer;
use std::sync::Mutex;
use bollard::Docker;
use futures_util::stream::StreamExt;

impl HttpServer {
    pub async fn pod_containers_run(app_state: Data<Mutex<AppState>>, docker: Data<Docker>) -> impl Responder {
        let state = app_state.lock().unwrap();

        for (pod_id, pod) in &state.pods {
            let pod_id = pod_id.to_string();

            let labels = HashMap::from([
                ("app", "canoa"),
                ("namespace", pod.namespace.as_str()),
                ("pod_id", pod_id.as_str()),
            ]);

            let image = pod.image.clone();
    
            let image = image.as_str();

            //let mut buf = bytes::Bytes::new();

            let image_options = image::CreateImageOptions {
                from_image: image,
                ..Default::default()
            };

            let mut imagepull = docker.create_image(Some(image_options), None, None);
            

            while let Some(response) = imagepull.next().await {
                match response {
                    Ok(res) => {
                        println!("{:?}", res.progress_detail);
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
        
    
            for i in 0..pod.replicas {
                let name = format!("{}-{}", pod.name, i);
    
                let options = CreateContainerOptions {
                    name: name.clone(),
                    //platform: Some("linux/amd64"),
                    platform: None,
                };

                let config = Config {
                    labels: Some(labels.clone()),
                    image: Some(image),
                   // cmd: Some(vec!["/bin/bash"]),
                   // shell: Some(vec!["RUN"]),
                    host_config: Some(bollard::models::HostConfig {
                        auto_remove: Some(true),
                        ..Default::default()
                    }),
                    attach_stdin: Some(true),
                    attach_stdout: Some(true),
                    attach_stderr: Some(true),
                    open_stdin: Some(true),
                    tty: Some(true),
                    
                    ..Config::default()
                };

                match docker.create_container(Some(options), config).await {
                    Ok(_) => {
                        println!("Sucesss");
                        let res = docker.start_container(name.as_str(), None::<StartContainerOptions<String>>).await;

                        match res {
                            Ok(_) => println!("Sucesso ao start"),
                            Err(e) =>  println!("erro start {}", e)
                        }


                    },
                    Err(e) => println!("Error: {e}"),
                }
            }
        }

        HttpResponse::Ok().finish()
    }
}