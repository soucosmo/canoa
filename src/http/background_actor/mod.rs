use actix::prelude::*;
use crate::types::AppState;
use std::time::Duration;
use bollard::Docker;
use minreq::post;

// Define um Actor que executa uma tarefa em segundo plano
pub struct BackgroundActor {
    pub app_state: AppState,
    pub requesting_containers_run: bool,
    pub docker: Docker,
}


impl Actor for BackgroundActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(5), |actor, f| {
            if actor.requesting_containers_run {
               return; 
            }

            actor.requesting_containers_run = true;

            // This logic will change in the future, I'm just doing it this way to get an idea of ​​how it works.
            match post("http://localhost:9797/pod/containers-run").send() {
                Ok(res) => {
                    println!("Status code [{}]", res.status_code);

                    actor.requesting_containers_run = false;
                },
                Err(e) => {
                    actor.requesting_containers_run = false;
                }
            }
        });
    }
}
