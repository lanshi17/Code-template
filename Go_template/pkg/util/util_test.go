package util

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestGenerateRandomString(t *testing.T) {
	// Test generating a random string
	str, err := GenerateRandomString(16)
	assert.NoError(t, err)
	assert.Len(t, str, 16)

	// Test generating a random string with odd length
	str, err = GenerateRandomString(15)
	assert.NoError(t, err)
	assert.Len(t, str, 16) // Should be rounded up to even length
}

func TestStringInSlice(t *testing.T) {
	// Test checking if a string is in a slice
	slice := []string{"apple", "banana", "cherry"}

	assert.True(t, StringInSlice("apple", slice))
	assert.True(t, StringInSlice("banana", slice))
	assert.True(t, StringInSlice("cherry", slice))
	assert.False(t, StringInSlice("orange", slice))
	assert.False(t, StringInSlice("apple", []string{}))
}