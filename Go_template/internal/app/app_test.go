package app

import (
	"testing"

	"Go_template/config"

	"github.com/stretchr/testify/assert"
)

func TestNewApp(t *testing.T) {
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

	// Test creating a new application
	app, err := NewApp(cfg)
	assert.NoError(t, err)
	assert.NotNil(t, app)
	assert.NotNil(t, app.repository)
	assert.NotNil(t, app.service)
	assert.NotNil(t, app.handler)
}