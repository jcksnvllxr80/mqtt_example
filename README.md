# MQTT broker with producer/consumer client

mqtt example. mqtt only holds the last message to a given topic so if clients are offline for an extended period, 
they will likely lose messages.

## python output

```text
Cleared retained messages on topic `python/mqtt`
Connected to MQTT Broker with client ID: client-642!
Stopped network loop after clearing retained messages.
Starting permanent listener...
Starting to publish messages...
Connected to MQTT Broker with client ID: listener-client-31!
Connected to MQTT Broker with client ID: client-642!
Send `message: 1` to topic `python/mqtt`
Permanent Listener Received `message: 1` from `python/mqtt` topic
Send `message: 2` to topic `python/mqtt`
Permanent Listener Received `message: 2` from `python/mqtt` topic
Send `message: 3` to topic `python/mqtt`
Permanent Listener Received `message: 3` from `python/mqtt` topic
Send `message: 4` to topic `python/mqtt`
Permanent Listener Received `message: 4` from `python/mqtt` topic
Send `message: 5` to topic `python/mqtt`
Stopped publishing messages.
Permanent Listener Received `message: 5` from `python/mqtt` topic
Stopped network loop after publishing.
---- Simulating an offline service by sending messages first and then receiving later ----
Waiting 3 seconds before subscribing...
Starting network loop for receiving messages...
Delayed Subscriber Received `message: 5` from `python/mqtt` topic
Stopped network loop after receiving messages.

Process finished with exit code 0
```

## rust output 

```text
Starting the permanent consumer...
Permanent consumer subscribed to `rust/mqtt`
Permanent consumer is ready
Permanent consumer setup completed.
Starting the producer...
Starting to publish messages...
Publishing `message: 1` to topic `rust/mqtt`
Permanent consumer received `message: 1` from `rust/mqtt` topic
Publishing `message: 2` to topic `rust/mqtt`
Permanent consumer received `message: 2` from `rust/mqtt` topic
Publishing `message: 3` to topic `rust/mqtt`
Permanent consumer received `message: 3` from `rust/mqtt` topic
Publishing `message: 4` to topic `rust/mqtt`
Permanent consumer received `message: 4` from `rust/mqtt` topic
Publishing `message: 5` to topic `rust/mqtt`
Permanent consumer received `message: 5` from `rust/mqtt` topic
Stopped publishing messages.
Waiting 3 seconds before starting the delayed consumer...
Delayed consumer subscribed to `rust/mqtt`
Delayed consumer received `message: 5` from `rust/mqtt` topic
```

## go output

```text
Starting the permanent consumer...
permanent-consumer-1192 connected to broker
permanent-consumer-1192 subscribed to `go/mqtt`
Permanent consumer setup completed.
Starting the producer...
producer-9233 connected to broker
Starting to publish messages...
producer-9233 publishing `message: 1` to topic `go/mqtt`
permanent-consumer-1192 received `message: 1` from `go/mqtt` topic
producer-9233 publishing `message: 2` to topic `go/mqtt`
permanent-consumer-1192 received `message: 2` from `go/mqtt` topic
producer-9233 publishing `message: 3` to topic `go/mqtt`
permanent-consumer-1192 received `message: 3` from `go/mqtt` topic
producer-9233 publishing `message: 4` to topic `go/mqtt`
permanent-consumer-1192 received `message: 4` from `go/mqtt` topic
producer-9233 publishing `message: 5` to topic `go/mqtt`
permanent-consumer-1192 received `message: 5` from `go/mqtt` topic
producer-9233 stopped publishing messages.
Waiting 3 seconds before starting the delayed consumer...
delayed-consumer-36 connected to broker
delayed-consumer-36 subscribed to `go/mqtt`
delayed-consumer-36 received `message: 5` from `go/mqtt` topic
```