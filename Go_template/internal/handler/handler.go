package handler

import (
	"net/http"

	"Go_template/internal/service"

	"github.com/gin-gonic/gin"
)

// Handler represents the HTTP handler
type Handler struct {
	service *service.Service
}

// NewHandler creates a new handler
func NewHandler(service *service.Service) *Handler {
	return &Handler{
		service: service,
	}
}

// RegisterRoutes registers the HTTP routes
func (h *Handler) RegisterRoutes(router *gin.Engine) {
	// Health check endpoint
	router.GET("/health", h.HealthCheck)

	// Add more routes here
	// Example:
	// router.POST("/users", h.CreateUser)
	// router.GET("/users/:id", h.GetUser)
}

// HealthCheck handles the health check endpoint
func (h *Handler) HealthCheck(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"status": "OK",
		"message": "Service is healthy",
	})
}