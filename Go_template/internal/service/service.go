package service

import (
	"Go_template/internal/repository"
)

// Service represents the business service
type Service struct {
	repository *repository.Repository
}

// NewService creates a new service
func NewService(repo *repository.Repository) *Service {
	return &Service{
		repository: repo,
	}
}

// Add business logic methods here
// Example:
// func (s *Service) GetUser(id string) (*User, error) {
//     return s.repository.GetUser(id)
// }