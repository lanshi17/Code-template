package repository

import (
	"database/sql"
	"fmt"

	"Go_template/config"

	// PostgreSQL driver
	_ "github.com/lib/pq"
)

// Repository represents the database repository
type Repository struct {
	db *sql.DB
}

// NewRepository creates a new repository
func NewRepository(cfg *config.Config) (*Repository, error) {
	// Create connection string
	connStr := fmt.Sprintf(
		"host=%s port=%d user=%s password=%s dbname=%s sslmode=disable",
		cfg.Database.Host,
		cfg.Database.Port,
		cfg.Database.User,
		cfg.Database.Password,
		cfg.Database.Name,
	)

	// Open database connection
	db, err := sql.Open("postgres", connStr)
	if err != nil {
		return nil, fmt.Errorf("failed to open database: %w", err)
	}

	// Test the connection
	if err := db.Ping(); err != nil {
		return nil, fmt.Errorf("failed to ping database: %w", err)
	}

	return &Repository{
		db: db,
	}, nil
}

// Close closes the database connection
func (r *Repository) Close() error {
	if r.db != nil {
		return r.db.Close()
	}
	return nil
}

// Add database access methods here
// Example:
// func (r *Repository) GetUser(id string) (*User, error) {
//     // Implementation here
// }