package app

import (
	"context"
	"fmt"
	"net/http"
	"time"

	"Go_template/config"
	"Go_template/internal/handler"
	"Go_template/internal/repository"
	"Go_template/internal/service"

	"github.com/gin-gonic/gin"
)

// App represents the application
type App struct {
	cfg        *config.Config
	server     *http.Server
	repository *repository.Repository
	service    *service.Service
	handler    *handler.Handler
}

// NewApp creates a new application
func NewApp(cfg *config.Config) (*App, error) {
	// Create repository
	repo, err := repository.NewRepository(cfg)
	if err != nil {
		return nil, fmt.Errorf("failed to create repository: %w", err)
	}

	// Create service
	svc := service.NewService(repo)

	// Create handler
	h := handler.NewHandler(svc)

	return &App{
		cfg:        cfg,
		repository: repo,
		service:    svc,
		handler:    h,
	}, nil
}

// Run starts the application
func (a *App) Run() error {
	// Set Gin to release mode in production
	if a.cfg.Log.Level != "debug" {
		gin.SetMode(gin.ReleaseMode)
	}

	// Create router
	router := gin.New()
	router.Use(gin.Logger())
	router.Use(gin.Recovery())

	// Register routes
	a.handler.RegisterRoutes(router)

	// Create server
	a.server = &http.Server{
		Addr:    fmt.Sprintf("%s:%d", a.cfg.Server.Host, a.cfg.Server.Port),
		Handler: router,
	}

	// Start server
	fmt.Printf("Server is running on %s:%d\n", a.cfg.Server.Host, a.cfg.Server.Port)
	return a.server.ListenAndServe()
}

// Shutdown gracefully shuts down the application
func (a *App) Shutdown() error {
	if a.server != nil {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()

		if err := a.server.Shutdown(ctx); err != nil {
			return fmt.Errorf("server shutdown failed: %w", err)
		}
	}

	if a.repository != nil {
		if err := a.repository.Close(); err != nil {
			return fmt.Errorf("repository close failed: %w", err)
		}
	}

	return nil
}