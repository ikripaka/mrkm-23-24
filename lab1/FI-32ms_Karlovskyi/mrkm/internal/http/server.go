package http

import (
	"context"
	"fmt"
	"github.com/gin-gonic/gin"
	swaggerFiles "github.com/swaggo/files"
	ginSwagger "github.com/swaggo/gin-swagger"
	"go.opentelemetry.io/otel/sdk/trace"
	"go.uber.org/zap"
	"mrkm/docs"
	"mrkm/pkg/tracing"
	"net/http"
	"sync"
	"time"
)

type Server struct {
	wg     *sync.WaitGroup
	ctx    context.Context
	server *http.Server
	router *gin.Engine
	trace  *trace.TracerProvider
}

// New
// @title           API
// @version         2.0
// @description     This is a sample server.
// @termsOfService  http://swagger.io/terms/
// @contact.name   API Support
// @contact.url    http://www.swagger.io/support
// @contact.email  support@swagger.io
// @license.name Apache 2.0
// @license.url http://www.apache.org/licenses/LICENSE-2.0.html
func New(ctx context.Context, wg *sync.WaitGroup, cfg *Config, trace *trace.TracerProvider, handlers []Handler) *Server {
	docs.SwaggerInfo.Title = "API"
	docs.SwaggerInfo.Description = "This is a sample server CoinAMP server."
	docs.SwaggerInfo.Version = "2.0"
	docs.SwaggerInfo.Schemes = []string{"http", "https"}

	s := &Server{
		wg:  wg,
		ctx: ctx,
		server: &http.Server{
			Addr:              fmt.Sprintf("%s:%d", cfg.Host, cfg.Port),
			Handler:           nil,
			ReadHeaderTimeout: 10 * time.Second,
			ReadTimeout:       cfg.ReadTimeout,
			WriteTimeout:      cfg.WriteTimeout,
			IdleTimeout:       30 * time.Second,
		},
		router: gin.New(),
		trace:  trace,
	}

	s.router.Use(tracing.Middleware(cfg.ServiceName))
	s.router.GET("/docs/swagger/*any", ginSwagger.WrapHandler(swaggerFiles.Handler))
	api := s.router.Group("")

	s.registerHandlers(api, handlers...)

	return s
}

func (s *Server) registerHandlers(api *gin.RouterGroup, handlers ...Handler) {
	for _, h := range handlers {
		h.Register(api)
	}

	s.server.Handler = s.router
}

func (s *Server) Run() {
	s.wg.Add(1)
	zap.S().Infof("server listining: %s", s.server.Addr)

	if err := s.server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
		zap.S().Error(err.Error())
	}
}

func (s *Server) Shutdown(ctx context.Context) error {
	zap.S().Info("Shutdown server...")
	zap.S().Info("Stopping http server...")

	ctx, cancel := context.WithTimeout(s.ctx, 30*time.Second)
	defer func() {
		cancel()
		s.wg.Done()
	}()

	if err := s.server.Shutdown(ctx); err != nil {
		zap.S().Fatal("Server forced to shutdown:", zap.Error(err))
		return err
	}

	zap.S().Info("Server successfully stopped.")

	return nil
}
