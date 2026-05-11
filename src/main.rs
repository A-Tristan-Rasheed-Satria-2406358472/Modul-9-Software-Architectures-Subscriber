use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn get_handler_action(&self) -> String {
        "process_user_created".to_owned()
    }

    fn handle(
        &self,
        message: Box<UserCreatedEventMessage>,
    ) -> Result<(), HandleError> {
        println!(
            "[Tristan Rasheed Satria - 2406358472] Message received: {:?}",
            message
        );
        Ok(())
    }
}

fn main() {
    let listener = CrosstownBus::new_queue_listener(
        "amqp://guest:guest@localhost:5672".to_owned(),
    )
    .unwrap();

    let _ = listener.listen(
        "user_created".to_owned(),
        UserCreatedHandler {},
        crosstown_bus::QueueProperties {
            auto_delete: false,
            durable: false,
            use_dead_letter: true,
        },
    );

    loop {}
}
