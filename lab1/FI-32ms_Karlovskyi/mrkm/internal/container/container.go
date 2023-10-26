package container

import (
	"context"
	"errors"
	"fmt"
	"github.com/sarulabs/di"
	"go.opentelemetry.io/otel/sdk/trace"
	"go.uber.org/zap"
	"mrkm/internal/config"
	"mrkm/internal/constants"
	"mrkm/internal/http"
	"mrkm/pkg/pgsql"
	"mrkm/pkg/tracing"
	"sync"
	"time"
)

var container di.Container
var once sync.Once

func Build(ctx context.Context, wg *sync.WaitGroup) di.Container {
	once.Do(func() {
		builder, _ := di.NewBuilder()
		defs := []di.Def{
			{
				Name: constants.LoggerName,
				Build: func(ctn di.Container) (interface{}, error) {
					logger, err := zap.NewDevelopment()

					if err != nil {
						return nil, errors.New(fmt.Sprintf("can't initialize zap logger: %v", err))
					}

					zap.ReplaceGlobals(logger)
					return logger, nil
				},
			},
			{
				Name: constants.ConfigName,
				Build: func(ctn di.Container) (interface{}, error) {
					return config.New()
				},
			},
			{
				Name: constants.PgSQLName,
				Build: func(ctn di.Container) (interface{}, error) {
					cfg := ctn.Get(constants.ConfigName).(*config.Config)

					return pgsql.NewPgsqlConnection(cfg.PgSQLConfig)
				},
			},
			{
				Name: constants.ServerName,
				Build: func(ctn di.Container) (interface{}, error) {
					cfg := ctn.Get(constants.ConfigName).(*config.Config)
					tr := ctn.Get(constants.TracingName).(*trace.TracerProvider)
					handlers := []http.Handler{
						ctn.Get(constants.SignHandlerName).(http.Handler),
					}

					return http.New(ctx, wg, cfg.ServerConfig, tr, handlers), nil
				},
				Close: func(obj interface{}) error {
					return obj.(*http.Server).Shutdown(ctx)
				},
			},
			{
				Name: constants.TracingName,
				Build: func(ctn di.Container) (interface{}, error) {
					cfg := ctn.Get(constants.ConfigName).(*config.Config)

					return tracing.NewTracingProvider(cfg.TracingConfig, cfg.ServerConfig.ServiceName)
				},
				Close: func(obj interface{}) error {
					timeout, cancel := context.WithTimeout(ctx, time.Second*5)
					defer cancel()

					return obj.(*trace.TracerProvider).Shutdown(timeout)
				},
			},
		}

		defs = append(defs, BuildServices()...)
		defs = append(defs, BuildRepositories()...)
		defs = append(defs, BuildHandlers()...)

		if err := builder.Add(defs...); err != nil {
			panic(err)
		}

		container = builder.Build()
	})

	return container
}
