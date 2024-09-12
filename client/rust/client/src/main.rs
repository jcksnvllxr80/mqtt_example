use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Notify;
use tokio::time;
use rand::Rng;

// const BROKER: &str = "broker.emqx.io";
const BROKER: &str = "localhost";
const PORT: u16 = 1883;
const TOPIC: &str = "rust/mqtt";
const WAIT_SECONDS: u64 = 3;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    // Signal to notify that the permanent consumer is ready
    let permanent_consumer_ready = Arc::new(Notify::new());
    let permanent_consumer_ready_clone = permanent_consumer_ready.clone();

    // Start the permanent consumer and wait for it to be ready
    println!("Starting the permanent consumer...");

    let permanent_consumer_task = tokio::spawn(async move {
        let client_id = format!("permanent-consumer-{}", rand::thread_rng().gen::<u16>());
        let mqttoptions = MqttOptions::new(client_id.clone(), BROKER, PORT);
        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        client.subscribe(TOPIC, QoS::AtLeastOnce).await.unwrap();
        println!("Permanent consumer subscribed to `{}`", TOPIC);

        permanent_consumer_ready_clone.notify_one();
        println!("Permanent consumer is ready");

        while let Ok(event) = eventloop.poll().await {
            if let Event::Incoming(Packet::Publish(p)) = event {
                let payload = String::from_utf8(p.payload.to_vec()).unwrap();
                println!("Permanent consumer received `{}` from `{}` topic", payload, TOPIC);
            }
        }
    });

    // Wait until the permanent consumer is ready
    permanent_consumer_ready.notified().await;
    println!("Permanent consumer setup completed.");

    // Start the producer
    println!("Starting the producer...");

    let producer_task = tokio::spawn(async {
        let client_id = format!("producer-{}", rand::thread_rng().gen::<u16>());
        let mqttoptions = MqttOptions::new(client_id.clone(), BROKER, PORT);
        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        tokio::spawn(async move {
            while let Ok(event) = eventloop.poll().await {
                match event {
                    Event::Outgoing(_) | Event::Incoming(_) => {},
                }
            }
        });

        // Publish messages
        publish(client).await;
    });

    // Wait for the producer to finish
    producer_task.await.unwrap();

    // Start the delayed consumer after a delay
    println!("Waiting {} seconds before starting the delayed consumer...", WAIT_SECONDS);
    time::sleep(Duration::from_secs(WAIT_SECONDS)).await;

    let delayed_consumer_task = tokio::spawn(async move {
        let client_id = format!("delayed-consumer-{}", rand::thread_rng().gen::<u16>());
        let mqttoptions = MqttOptions::new(client_id.clone(), BROKER, PORT);
        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        client.subscribe(TOPIC, QoS::AtLeastOnce).await.unwrap();
        println!("Delayed consumer subscribed to `{}`", TOPIC);

        while let Ok(event) = eventloop.poll().await {
            if let Event::Incoming(Packet::Publish(p)) = event {
                let payload = String::from_utf8(p.payload.to_vec()).unwrap();
                println!("Delayed consumer received `{}` from `{}` topic", payload, TOPIC);
            }
        }
    });

    // Wait for both consumers to finish
    let _ = tokio::join!(permanent_consumer_task, delayed_consumer_task);

    println!("All tasks completed.");
}

async fn publish(client: AsyncClient) {
    println!("Starting to publish messages...");
    for msg_count in 1..=5 {
        time::sleep(Duration::from_secs(1)).await;
        let msg = format!("message: {}", msg_count);
        println!("Publishing `message: {}` to topic `{}`", msg_count, TOPIC);
        client.publish(TOPIC, QoS::AtLeastOnce, true, msg.as_bytes()).await.unwrap();
    }

    // Short delay to ensure the last message is received by the permanent consumer
    time::sleep(Duration::from_secs(1)).await;
    println!("Stopped publishing messages.");
}