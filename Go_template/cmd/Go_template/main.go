package main

import (
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"

	"Go_template/config"
	"Go_template/internal/app"
)

func main() {
	// Load configuration
	cfg, err := config.LoadConfig()
	if err != nil {
		log.Fatalf("Failed to load config: %v", err)
	}

	// Create application
	application, err := app.NewApp(cfg)
	if err != nil {
		log.Fatalf("Failed to create app: %v", err)
	}

	// Run application
	go func() {
		if err := application.Run(); err != nil {
			log.Fatalf("Failed to run app: %v", err)
		}
	}()

	// Wait for interrupt signal to gracefully shutdown the server
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit

	// Shutdown application
	log.Println("Shutting down server...")
	if err := application.Shutdown(); err != nil {
		log.Fatalf("Server forced to shutdown: %v", err)
	}

	fmt.Println("Server exiting")
}