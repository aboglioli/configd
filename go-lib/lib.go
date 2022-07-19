package configd

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io/ioutil"
	"net/http"
	"time"
)

var (
	ErrEmptyUrl       = errors.New("empty url")
	ErrEmptySource    = errors.New("empty source")
	ErrEmptyInstance  = errors.New("empty instance")
	ErrFetchingConfig = errors.New("fetching config")
)

type ConfigdClient struct {
	url      string
	source   string
	instance string
}

func NewConfigdClient(
	url string,
	source string,
	instance string,
) (*ConfigdClient, error) {
	if url == "" {
		return nil, ErrEmptyUrl
	}

	if source == "" {
		return nil, ErrEmptySource
	}

	if instance == "" {
		return nil, ErrEmptyInstance
	}

	return &ConfigdClient{
		url,
		source,
		instance,
	}, nil
}

func (c *ConfigdClient) GetConfig(
	ctx context.Context,
	schemaId string,
	configId string,
	interval time.Duration,
) <-chan *Config {
	configCh := make(chan *Config)

	go func() {
		tick := time.NewTicker(interval)

		for {
			select {
			case <-tick.C:
				config, err := c.fetchConfig(schemaId, configId)
				if err != nil {
					panic(err)
				}

				configCh <- config
			}
		}
	}()

	return configCh
}

func (c *ConfigdClient) fetchConfig(schemaId, configId string) (*Config, error) {
	res, err := http.Get(
		fmt.Sprintf(
			"%s/schemas/%s/configs/%s",
			c.url,
			schemaId,
			configId,
		),
	)
	if err != nil {
		return nil, err
	}

	body, err := ioutil.ReadAll(res.Body)
	if err != nil {
		return nil, err
	}

	var config Config
	if err := json.Unmarshal(body, &config); err != nil {
		return nil, err
	}

	return &config, nil
}
