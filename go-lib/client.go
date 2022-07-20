package configd

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"time"
)

var (
	ErrEmptyUrl        = errors.New("empty url")
	ErrEmptySource     = errors.New("empty source")
	ErrEmptyInstance   = errors.New("empty instance")
	ErrEmptySchemaId   = errors.New("schema_id is required")
	ErrEmptyConfigId   = errors.New("config_id is required")
	ErrInvalidInterval = errors.New("interval must be between 1 second and 1 minute")
)

type ConfigHandler func(*Config, error) error

type ConfigdClient struct {
	url      string
	source   string
	instance string

	httpClient *http.Client
	errCh      chan error
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
		http.DefaultClient,
		make(chan error),
	}, nil
}

func (c *ConfigdClient) GetConfig(
	ctx context.Context,
	schemaId string,
	configId string,
	interval time.Duration,
	configHandler ConfigHandler,
) error {
	if schemaId == "" {
		return ErrEmptySchemaId
	}

	if configId == "" {
		return ErrEmptyConfigId
	}

	if interval < time.Second {
		return ErrInvalidInterval
	}

	if interval > time.Minute {
		return ErrInvalidInterval
	}

	go func() {
		tick := time.NewTicker(interval)

		for {
			select {
			case <-ctx.Done():
				c.notifyErr(nil)
			case <-tick.C:
				config, err := c.fetchConfig(ctx, schemaId, configId)
				if err != nil {
					c.notifyErr(err)
					return
				}

				if err := configHandler(config, err); err != nil {
					c.notifyErr(err)
					return
				}
			}
		}
	}()

	return nil
}

func (c *ConfigdClient) Wait() error {
	return <-c.errCh
}

func (c *ConfigdClient) fetchConfig(
	ctx context.Context,
	schemaId string,
	configId string,
) (*Config, error) {
	url := fmt.Sprintf(
		"%s/schemas/%s/configs/%s",
		c.url,
		schemaId,
		configId,
	)

	req, err := http.NewRequestWithContext(ctx, "GET", url, nil)
	req.Header.Set("X-Configd-Source", c.source)
	req.Header.Set("X-Configd-Instance", c.instance)

	res, err := c.httpClient.Do(req)
	if err != nil {
		return nil, err
	}
	defer res.Body.Close()

	var config Config
	if err := json.NewDecoder(res.Body).Decode(&config); err != nil {
		return nil, err
	}

	return &config, nil
}

func (c *ConfigdClient) notifyErr(err error) {
	select {
	case c.errCh <- err:
	default:
	}
}
