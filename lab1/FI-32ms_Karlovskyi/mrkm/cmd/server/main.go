package main

import (
	"context"
	"go.uber.org/zap"
	"mrkm/internal/constants"
	"mrkm/internal/container"
	"mrkm/internal/http"
	"mrkm/utils"
	"sync"
	"time"
)

func main() {
	now := time.Now()
	ctx := context.Background()
	wg := &sync.WaitGroup{}
	app := container.Build(ctx, wg)

	logger := app.Get(constants.LoggerName).(*zap.Logger)
	logger.Info("Starting application...")
	server := app.Get(constants.ServerName).(*http.Server)

	go server.Run()

	zap.S().Infof("Up and running (%s)", time.Since(now))
	zap.S().Infof("Got %s signal. Shutting down...", <-utils.WaitTermSignal())

	if err := app.Delete(); err != nil {
		zap.S().Errorf("Error stopping container: %s", err)
	}

	wg.Wait()
	zap.S().Info("Service stopped.")
}
