package handler

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"Go_template/internal/service"

	"github.com/gin-gonic/gin"
	"github.com/stretchr/testify/assert"
)

func TestHealthCheck(t *testing.T) {
	// Create a new handler
	svc := service.NewService(nil) // Pass nil for repository in this test
	handler := NewHandler(svc)

	// Create a test context
	gin.SetMode(gin.TestMode)
	router := gin.New()
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