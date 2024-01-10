package main

import (
	"context"
	"fmt"
	"go.uber.org/zap"
	"mrkm/internal/cli"
	"mrkm/internal/constants"
	"mrkm/internal/container"
	"sync"
)

func main() {
	ctx := context.Background()
	wg := &sync.WaitGroup{}
	app := container.Build(ctx, wg)

	logger := app.Get(constants.LoggerName).(*zap.Logger)
	logger.Info("Starting application...")

	cliManager := app.Get(constants.CLIName).(*cli.Manager)

	if err := cliManager.Run(); err != nil {
		fmt.Println(err)
	}

	if err := app.Delete(); err != nil {
		zap.S().Errorf("Error stopping container: %s", err)
	}

	wg.Wait()
	zap.S().Info("Service stopped.")
}
