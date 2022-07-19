package main

import (
	"context"
	"fmt"
	"time"

	"github.com/aboglioli/configd/go-lib"
)

func main() {
	client, err := configd.NewConfigdClient(
		"http://localhost:8080",
		"Example",
		"instance#01",
	)
	if err != nil {
		panic(err)
	}

	config := client.GetConfig(
		context.Background(),
		"custom-schema",
		"dev",
		2*time.Second,
	)

	for config := range config {
		fmt.Println(config)
	}
}
