package config

import (
	"github.com/spf13/viper"
	"mrkm/internal/http"
	"mrkm/pkg/pgsql"
	"mrkm/pkg/tracing"
	"sync"
)

var config *Config
var once sync.Once

type Config struct {
	PgSQLConfig   *pgsql.Config
	ServerConfig  *http.Config
	TracingConfig *tracing.Config
}

func New() (*Config, error) {
	once.Do(func() {
		config = &Config{}

		viper.AddConfigPath(".")
		viper.SetConfigName("config")

		if err := viper.ReadInConfig(); err != nil {
			panic(err)
		}

		databaseConfig := viper.Sub("database")
		serverConfig := viper.Sub("server")
		tracingConfig := viper.Sub("tracing")

		if err := databaseConfig.Unmarshal(&config.PgSQLConfig); err != nil {
			panic(err)
		}

		if err := serverConfig.Unmarshal(&config.ServerConfig); err != nil {
			panic(err)
		}

		if err := tracingConfig.Unmarshal(&config.TracingConfig); err != nil {
			panic(err)
		}
	})

	return config, nil
}
