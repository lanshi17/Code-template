package errors

import (
	"fmt"
)

// Custom error types
var (
	ErrUserNotFound    = New("user not found")
	ErrInvalidPassword = New("invalid password")
	ErrUnauthorized    = New("unauthorized")
)

// Error represents a custom error
type Error struct {
	message string
}

// New creates a new error with the specified message
func New(message string) *Error {
	return &Error{
		message: message,
	}
}

// Error returns the error message
func (e *Error) Error() string {
	return e.message
}

// Wrap wraps an existing error with a new message
func Wrap(err error, message string) *Error {
	return &Error{
		message: fmt.Sprintf("%s: %v", message, err),
	}
}