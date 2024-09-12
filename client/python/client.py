import random
import time
from threading import Thread
from paho.mqtt import client as mqtt_client

# broker = 'broker.emqx.io'
broker = 'localhost'
port = 1883
topic = "python/mqtt"
client_id = f'client-{random.randint(0, 1000)}'
listener_client_id = f'listener-client-{random.randint(0, 1000)}'

wait_seconds = 3  # Global variable for number of seconds to wait between send and receive

# Global variable to store the last message received by the permanent listener
last_message_permanent_listener = None


def connect_mqtt(client_id):
    def on_connect(client, userdata, flags, rc):
        if rc == 0:
            print(f"Connected to MQTT Broker with client ID: {client_id}!")
        else:
            print(f"Failed to connect, return code {rc}\n")

    client = mqtt_client.Client(client_id)
    client.on_connect = on_connect
    client.connect(broker, port)
    return client


def clear_retained_messages(client):
    result = client.publish(topic, '', retain=True)
    status = result[0]
    if status == 0:
        print(f"Cleared retained messages on topic `{topic}`")
    else:
        print(f"Failed to clear retained messages on topic `{topic}`")


def publish(client):
    print("Starting to publish messages...")
    msg_count = 1
    while msg_count <= 5:
        time.sleep(1)
        msg = f"message: {msg_count}"
        result = client.publish(topic, msg, retain=True)
        status = result[0]
        if status == 0:
            print(f"Send `{msg}` to topic `{topic}`")
        else:
            print(f"Failed to send message to topic {topic}")
        msg_count += 1
    print("Stopped publishing messages.")


def subscribe(client, subscriber_name):
    def on_message(client, userdata, msg):
        global last_message_permanent_listener

        if subscriber_name == "Permanent Listener":
            if msg.payload.decode() == last_message_permanent_listener:
                print(f"{subscriber_name} Ignored duplicate message `{msg.payload.decode()}` from `{msg.topic}` topic")
                return
            last_message_permanent_listener = msg.payload.decode()

        print(f"{subscriber_name} Received `{msg.payload.decode()}` from `{msg.topic}` topic")

    client.subscribe(topic)
    client.on_message = on_message


def permanent_listener():
    print("Starting permanent listener...")
    listener_client = connect_mqtt(listener_client_id)
    listener_client.loop_start()
    subscribe(listener_client, "Permanent Listener")

    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        listener_client.loop_stop()


def run_subscriber():
    client = connect_mqtt(client_id)

    # Start the network loop in the background to allow publishing
    client.loop_start()

    # Publish messages
    publish(client)

    # Stop the loop after publishing
    client.loop_stop()
    print("Stopped network loop after publishing.")

    # Delay before subscribing
    print("---- Simulating an offline service by sending messages first and then receiving later ----")
    print(f"Waiting {wait_seconds} seconds before subscribing...")
    time.sleep(wait_seconds)

    # Start the network loop again in the background to allow subscribing
    print("Starting network loop for receiving messages...")
    client.loop_start()

    # Subscribe to the topic
    subscribe(client, "Delayed Subscriber")

    # Continue to run the network loop to process incoming messages
    time.sleep(15)

    # Stop the loop after finishing subscription
    client.loop_stop()
    print("Stopped network loop after receiving messages.")


def run():
    # Step 1: Connect client and clear previously retained messages
    client = connect_mqtt(client_id)
    client.loop_start()
    clear_retained_messages(client)
    time.sleep(1)  # Ensure retained message clearing is processed
    client.loop_stop()
    print("Stopped network loop after clearing retained messages.")

    # Add a slight delay to ensure old messages are processed
    time.sleep(2)

    # Step 2: Start the permanent listener
    listener_thread = Thread(target=permanent_listener)
    listener_thread.daemon = True
    listener_thread.start()

    # Step 3: Start the delayed subscriber
    run_subscriber()


if __name__ == '__main__':
    run()
