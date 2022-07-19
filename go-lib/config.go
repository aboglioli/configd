package configd

import (
	"time"
)

type Access struct {
	Source     string
	Instance   string
	Timestasmp time.Time
	Previous   *time.Time
}

type Config struct {
	SchemaId  string      `json:"schema_id"`
	Id        string      `json:"id"`
	Name      string      `json:"name"`
	Data      interface{} `json:"data"`
	Valid     bool        `json:"valid"`
	Checksum  string      `json:"checksum"`
	Accesses  []Access    `json:"accesses"`
	CreatedAt time.Time   `json:"created_at"`
	UpdatedAt time.Time   `json:"updated_at"`
	Version   int64       `json:"version"`
}
