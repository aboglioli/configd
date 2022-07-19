package main

import (
	"context"
	"fmt"
	"time"

	"github.com/aboglioli/configd/go-lib"
)

type Database struct {
	Name string `json:"name"`
	Host string `json:"host"`
	Port int64 `json:"port"`
}

type MyConfig struct {
	Env string `json:"env"`
	Databases []Database `json:"databases"`
}

func main() {
	client, err := configd.NewConfigdClient(
		"http://localhost:8080",
		"Example",
		"instance#01",
	)
	if err != nil {
		panic(err)
	}

	configHandler := func(c *configd.Config, err error) error {
		if err != nil {
			return err
		}

		data := c.Data.(*MyConfig)

		fmt.Printf("#####\n")
		fmt.Printf("· Name: %s\n", c.Name)
		fmt.Printf("· Valid: %t\n", c.Valid)
		fmt.Printf("· Checksum: %s\n", c.Checksum)
		fmt.Printf("· Accesses: %d\n", len(c.Accesses))
		fmt.Printf("· Updated at: %s\n", c.UpdatedAt)
		fmt.Printf("· Version: %d\n", c.Version)
		fmt.Printf("· Data: %#v\n", data)

		return nil
	}

	if err := client.GetConfig(
		context.Background(),
		"custom-schema",
		"dev",
		2 * time.Second,
		&MyConfig{},
		configHandler,
	); err != nil {
		panic(err)
	}

	if err := client.Wait(); err != nil {
		panic(err)
	}

	fmt.Println("DONE")
}
