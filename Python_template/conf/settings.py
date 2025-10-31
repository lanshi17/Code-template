"""
Application settings and configuration.
"""

import os

# Get environment variable or default to development
ENV = os.getenv('ENVIRONMENT', 'development')

# Database configuration
DATABASE_URL = os.getenv('DATABASE_URL', 'sqlite:///app.db')

# Logging configuration
LOG_LEVEL = os.getenv('LOG_LEVEL', 'INFO')
LOG_FILE = os.getenv('LOG_FILE', '../log/application.log')

# API configuration
API_HOST = os.getenv('API_HOST', 'localhost')
API_PORT = int(os.getenv('API_PORT', 8000))

# Security settings
SECRET_KEY = os.getenv('SECRET_KEY', 'your-secret-key-here')