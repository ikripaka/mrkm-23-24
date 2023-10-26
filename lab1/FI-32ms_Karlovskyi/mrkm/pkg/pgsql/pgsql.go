package pgsql

import (
	"fmt"
	"github.com/pressly/goose/v3"
	"github.com/uptrace/opentelemetry-go-extra/otelgorm"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"time"
)

const (
	maxIDLETime            = 1 * time.Hour
	connectionMaxLifetime  = 24 * time.Hour
	maxIDLEConnectionCount = 10
	maxOpenConnectionCount = 20
)

var connection *gorm.DB

func NewPgsqlConnection(config *Config) (*gorm.DB, error) {
	dsn := fmt.Sprintf("host=%s port=%d user=%s password=%s dbname=%s sslmode=disable",
		config.Host, config.Port, config.User, config.Pass, config.Name)

	conn, err := gorm.Open(postgres.Open(dsn), &gorm.Config{})

	if err != nil {
		return nil, err
	}

	database, err := conn.DB()

	if err != nil {
		return nil, err
	}

	if err := conn.Use(otelgorm.NewPlugin()); err != nil {
		panic(err)
	}

	database.SetConnMaxIdleTime(maxIDLETime)
	database.SetConnMaxLifetime(connectionMaxLifetime)
	database.SetMaxIdleConns(maxIDLEConnectionCount)
	database.SetMaxOpenConns(maxOpenConnectionCount)
	connection = conn

	if _, err := goose.EnsureDBVersion(database); err != nil {
		return nil, err
	}

	if err := goose.Up(database, "./migrations", goose.WithAllowMissing()); err != nil {
		return nil, err
	}

	return connection, nil
}
