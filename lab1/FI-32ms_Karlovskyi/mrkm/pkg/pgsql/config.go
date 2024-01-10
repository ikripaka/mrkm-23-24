package pgsql

import "time"

type Config struct {
	Host              string
	Port              uint16
	Name              string
	User              string
	Pass              string
	ConnectionTimeout time.Duration
	Compression       string
	PingInterval      time.Duration
	MinConnections    int
	MaxConnections    int
}
