package service

import (
	"testing"

	"Go_template/internal/repository"

	"github.com/stretchr/testify/assert"
)

func TestNewService(t *testing.T) {
	// Create a new service
	svc := NewService(nil) // Pass nil for repository in this test

	// Check that the service was created correctly
	assert.NotNil(t, svc)
	assert.Nil(t, svc.repository) // Should be nil since we passed nil
}

// Add more tests for business logic methods here