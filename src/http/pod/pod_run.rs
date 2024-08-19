use actix_web::{web::Data, Responder, HttpResponse, web::Json};
use crate::http_validate::PodRunValidate;
use crate::types::{AppState, Pod, Resources};
use validify::Validify;
use super::HttpServer;
use std::sync::Mutex;
use serde_json::json;
use uuid::Uuid;


impl HttpServer {
    pub async fn pod_run(app_state: Data<Mutex<AppState>>, mut data: Json<PodRunValidate>) -> impl Responder {
        match data.validify() {
            Ok(_) => {
                let mut state = app_state.lock().unwrap();

                state.namespaces.insert(data.namespace.clone());

                let resources: Resources;

                if let Some(ref mut r) = data.resources {
                    if let Err(e) = r.validify() {
                        return HttpResponse::BadRequest().json(e);
                    }

                    resources = Resources {
                        cpu: r.cpu.clone(),
                        memory: r.memory.clone(),
                    }
                } else {
                    resources = Resources {
                        cpu: "0.001".to_string(),
                        memory: "5m".to_string()
                    }
                }

                let pod_id = Uuid::new_v4();

                let pod = Pod {
                    name: data.name.to_string(),
                    namespace: data.namespace.to_string(),
                    replicas: data.replicas,
                    image: data.image.clone(),
                    containers: vec![],
                    resources,
                };

                state.pods.insert(pod_id.clone(), pod);

                HttpResponse::Accepted().json(json!({
                    "id": pod_id,
                    "state": state.clone()
                }))
            },
            Err(e) => {

                println!("error: {:?}", e);

                HttpResponse::BadRequest().json(e)
            }
        }
    }
}
