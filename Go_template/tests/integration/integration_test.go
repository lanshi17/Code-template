package integration

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"Go_template/config"
	"Go_template/internal/app"
	"Go_template/internal/handler"

	"github.com/gin-gonic/gin"
	"github.com/stretchr/testify/assert"
)

func TestHealthCheckIntegration(t *testing.T) {
	// Create a test configuration
	cfg := &config.Config{
		Server: config.ServerConfig{
			Host: "localhost",
			Port: 8080,
		},
		Database: config.DatabaseConfig{
			Host:     "localhost",
			Port:     5432,
			User:     "user",
			Password: "password",
			Name:     "go_template",
		},
		Log: config.LogConfig{
			Level:  "info",
			Format: "text",
		},
		JWT: config.JWTConfig{
			Secret:          "test_secret",
			ExpirationHours: 24,
		},
	}

	// Create a new application
	appInstance, err := app.NewApp(cfg)
	assert.NoError(t, err)
	assert.NotNil(t, appInstance)

	// Create a test router
	gin.SetMode(gin.TestMode)
	router := gin.New()
	handler := handler.NewHandler(appInstance.Service)
	handler.RegisterRoutes(router)

	// Create a test request
	req, err := http.NewRequest("GET", "/health", nil)
	assert.NoError(t, err)

	// Create a response recorder
	w := httptest.NewRecorder()

	// Perform the request
	router.ServeHTTP(w, req)

	// Check the response
	assert.Equal(t, http.StatusOK, w.Code)
	assert.Contains(t, w.Body.String(), "OK")
	assert.Contains(t, w.Body.String(), "Service is healthy")
}