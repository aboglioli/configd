package configd

import (
	"encoding/json"
	"time"
)

type Access struct {
	Source     string
	Instance   string
	Timestasmp time.Time
	Previous   *time.Time
}

type Config struct {
	SchemaId  string          `json:"schema_id"`
	Id        string          `json:"id"`
	Name      string          `json:"name"`
	Data      json.RawMessage `json:"data"`
	Valid     bool            `json:"valid"`
	Checksum  string          `json:"checksum"`
	Accesses  []Access        `json:"accesses"`
	CreatedAt time.Time       `json:"created_at"`
	UpdatedAt time.Time       `json:"updated_at"`
	Version   int64           `json:"version"`
}

func (c *Config) Unmarshal(v interface{}) error {
	return json.Unmarshal(c.Data, v)
}
