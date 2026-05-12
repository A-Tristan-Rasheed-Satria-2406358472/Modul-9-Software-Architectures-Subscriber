use borsh::{BorshDeserialize, BorshSerialize};
use futures_util::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
    Connection, ConnectionProperties,
};
use std::thread;
use std::time::Duration;
use tokio_amqp::*;

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ten_millis = Duration::from_secs(1);

    let conn = Connection::connect(
        "amqp://guest:guest@localhost:5672/%2f",
        ConnectionProperties::default().with_tokio(),
    )
    .await?;
    let channel = conn.create_channel().await?;

    channel
        .queue_declare(
            "user_created",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let mut consumer = channel
        .basic_consume(
            "user_created",
            "subscriber_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery?;
        let message = UserCreatedEventMessage::try_from_slice(&delivery.data)?;
        thread::sleep(ten_millis);
        println!(
            "[Tristan Rasheed Satria - 2406358472] Message received: {:?}",
            message
        );
        delivery.ack(BasicAckOptions::default()).await?;
    }

    Ok(())
}
