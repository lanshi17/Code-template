package errors

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNew(t *testing.T) {
	// Test creating a new error
	err := New("test error")
	assert.NotNil(t, err)
	assert.Equal(t, "test error", err.Error())
}

func TestError_Error(t *testing.T) {
	// Test the Error method
	err := &Error{message: "test error"}
	assert.Equal(t, "test error", err.Error())
}

func TestWrap(t *testing.T) {
	// Test wrapping an error
	originalErr := fmt.Errorf("original error")
	wrappedErr := Wrap(originalErr, "wrapped message")
	assert.NotNil(t, wrappedErr)
	assert.Contains(t, wrappedErr.Error(), "wrapped message")
	assert.Contains(t, wrappedErr.Error(), "original error")
}