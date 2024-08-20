use super::BackgroundActor;
use crate::types::AppState;
use actix::prelude::*;

// Defina uma mensagem que inclui uma referência ao `app_data`
pub struct BackgroundMessage {
    pub app_state: AppState,
}

impl Message for BackgroundMessage {
    type Result = Result<String, ()>;
}

impl Handler<BackgroundMessage> for BackgroundActor {
    type Result = Result<String, ()>;

    fn handle(&mut self, msg: BackgroundMessage, _: &mut Self::Context) -> Self::Result {
        self.app_state = msg.app_state.clone();
        // Acessa o `app_data` dentro do `Actor`
        let data = &msg.app_state.namespaces;

        println!("handleee");
        Ok(format!("Dados recebidos no Actor: {:?}", data))
    }
}
