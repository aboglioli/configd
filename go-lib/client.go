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
	password string

	httpClient *http.Client
	errCh      chan error

	lastConfig *Config
}

type ConfigdConfig struct {
	Url      string
	Source   string
	Instance string
	Password string
}

func NewConfigdClient(
	cfg ConfigdConfig,
) (*ConfigdClient, error) {
	if cfg.Url == "" {
		return nil, ErrEmptyUrl
	}

	if cfg.Source == "" {
		return nil, ErrEmptySource
	}

	if cfg.Instance == "" {
		return nil, ErrEmptyInstance
	}

	return &ConfigdClient{
		cfg.Url,
		cfg.Source,
		cfg.Instance,
		cfg.Password,
		http.DefaultClient,
		make(chan error),
		nil,
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

				if c.lastConfig != nil {
					if c.lastConfig.Checksum == config.Checksum &&
						c.lastConfig.Version == config.Version {
						continue
					}
				}

				c.lastConfig = config

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
	if len(c.password) > 0 {
		req.Header.Set("X-Configd-Password", c.password)
	}

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
