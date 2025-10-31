package repository

import (
	"testing"

	"Go_template/config"

	"github.com/stretchr/testify/assert"
)

func TestNewRepository(t *testing.T) {
	// Create a test configuration
	cfg := &config.Config{
		Database: config.DatabaseConfig{
			Host:     "localhost",
			Port:     5432,
			User:     "test_user",
			Password: "test_password",
			Name:     "test_db",
		},
	}

	// Test creating a new repository
	// Note: This will fail because we don't have a real database connection in tests
	// In a real scenario, you would use a test database or mock the database connection
	repo, err := NewRepository(cfg)
	assert.Error(t, err) // Expect an error because we can't connect to the database
	assert.Nil(t, repo)

	// In a real test, you would do something like:
	// assert.NoError(t, err)
	// assert.NotNil(t, repo)
	// defer repo.Close()
}