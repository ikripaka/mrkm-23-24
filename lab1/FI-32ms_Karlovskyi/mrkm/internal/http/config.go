package http

import "time"

type Config struct {
	ServiceName  string
	Host         string
	Port         int
	ReadTimeout  time.Duration
	WriteTimeout time.Duration
}
