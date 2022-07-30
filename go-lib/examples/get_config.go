package main

import (
	"context"
	"fmt"
	"math/rand"
	"time"

	"github.com/aboglioli/configd/go-lib"
)

var letters = []rune("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")

func randSeq(n int) string {
	b := make([]rune, n)
	for i := range b {
		b[i] = letters[rand.Intn(len(letters))]
	}
	return string(b)
}

type Database struct {
	Name string `json:"name"`
	Host string `json:"host"`
	Port int64  `json:"port"`
}

type MyConfig struct {
	Env       string     `json:"env"`
	Databases []Database `json:"databases"`
	RateLimit float64    `json:"rate_limit"`
}

func main() {
	rand.Seed(time.Now().UnixNano())

	client, err := configd.NewConfigdClient(configd.ConfigdConfig{
		Url:      "http://localhost:8080",
		Source:   "Example",
		Instance: randSeq(10),
		Password: "passwd123",
	})
	if err != nil {
		panic(err)
	}

	configHandler := func(c *configd.Config, err error) error {
		if err != nil {
			return err
		}

		var myConfig MyConfig
		if err := c.Unmarshal(&myConfig); err != nil {
			return err
		}

		valid := "INVALID"
		if c.Valid {
			valid = "VALID"
		}

		fmt.Printf("##### %s (%s) - %s\n", c.Name, c.Id, valid)
		fmt.Printf("· Checksum: %s\n", c.Checksum)
		fmt.Printf("· Accesses: %d\n", len(c.Accesses))
		fmt.Printf("· Updated at: %s\n", c.UpdatedAt)
		fmt.Printf("· Version: %d\n", c.Version)
		fmt.Printf("· Data: %#v\n", myConfig)

		return nil
	}

	// ctx, _ := context.WithTimeout(context.Background(), 10*time.Second)

	if err := client.GetConfig(
		context.Background(),
		"custom-schema",
		"dev",
		2*time.Second,
		configHandler,
	); err != nil {
		panic(err)
	}

	if err := client.Wait(); err != nil {
		panic(err)
	}

	fmt.Println("DONE")
}
