package main

import (
	"fmt"
	"math/rand"
	"time"

	mqtt "github.com/eclipse/paho.mqtt.golang"
)

const (
	// broker       = "tcp://broker.emqx.io:1883"
	broker       = "tcp://localhost:1883"
	topic        = "go/mqtt"
	waitSeconds  = 3
	retryLimit   = 5
	retryDelay   = 2
	clientPrefix = "mqtt-client"
)

func main() {
	rand.Seed(time.Now().UnixNano())

	readyCh := make(chan struct{})

	fmt.Println("Starting the permanent consumer...")
	go connectAndSubscribe("permanent-consumer", readyCh, true)

	<-readyCh
	fmt.Println("Permanent consumer setup completed.")

	fmt.Println("Starting the producer...")
	connectAndPublish("producer")

	fmt.Printf("Waiting %d seconds before starting the delayed consumer...\n", waitSeconds)
	time.Sleep(waitSeconds * time.Second)

	go connectAndSubscribe("delayed-consumer", make(chan struct{}), false)

	select {}
}

func connectAndSubscribe(clientPrefix string, readyCh chan struct{}, notifyWhenReady bool) {
	clientID := fmt.Sprintf("%s-%d", clientPrefix, rand.Intn(10000))
	opts := mqtt.NewClientOptions().AddBroker(broker).SetClientID(clientID)

	for attempt := 1; attempt <= retryLimit; attempt++ {
		client := mqtt.NewClient(opts)
		if token := client.Connect(); token.Wait() && token.Error() == nil {
			fmt.Printf("%s connected to broker\n", clientID)
			if token := client.Subscribe(topic, 1, func(client mqtt.Client, msg mqtt.Message) {
				fmt.Printf("%s received `%s` from `%s` topic\n", clientID, msg.Payload(), topic)
			}); token.Wait() && token.Error() == nil {
				fmt.Printf("%s subscribed to `%s`\n", clientID, topic)
				if notifyWhenReady {
					close(readyCh)
				}
				select {}
			} else {
				fmt.Printf("%s failed to subscribe: %s\n", clientID, token.Error())
				client.Disconnect(250)
			}
		} else {
			fmt.Printf("%s failed to connect: %s, attempt: %d\n", clientID, token.Error(), attempt)
		}
		time.Sleep(retryDelay * time.Second)
		if attempt == retryLimit {
			fmt.Printf("%s could not connect after %d attempts, giving up.\n", clientID, retryLimit)
			if notifyWhenReady {
				close(readyCh)
			}
			return
		}
	}
}

func connectAndPublish(clientPrefix string) {
	clientID := fmt.Sprintf("%s-%d", clientPrefix, rand.Intn(10000))
	opts := mqtt.NewClientOptions().AddBroker(broker).SetClientID(clientID)

	for attempt := 1; attempt <= retryLimit; attempt++ {
		client := mqtt.NewClient(opts)
		if token := client.Connect(); token.Wait() && token.Error() == nil {
			fmt.Printf("%s connected to broker\n", clientID)
			fmt.Println("Starting to publish messages...")
			for msgCount := 1; msgCount <= 5; msgCount++ {
				time.Sleep(1 * time.Second)
				message := fmt.Sprintf("message: %d", msgCount)
				fmt.Printf("%s publishing `message: %d` to topic `%s`\n", clientID, msgCount, topic)
				token := client.Publish(topic, 1, true, message) // Set retain flag to true
				token.Wait()
			}
			time.Sleep(1 * time.Second)
			client.Disconnect(250)
			fmt.Printf("%s stopped publishing messages.\n", clientID)
			return
		} else {
			fmt.Printf("%s failed to connect: %s, attempt: %d\n", clientID, token.Error(), attempt)
		}
		time.Sleep(retryDelay * time.Second)
		if attempt == retryLimit {
			fmt.Printf("%s could not connect after %d attempts, giving up.\n", clientID, retryLimit)
			return
		}
	}
}