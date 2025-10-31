#!/usr/bin/env python3
"""
Main entry point for the application.
"""
import sys
import os

# Add the src directory to the path so we can import our modules
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

def main():
    """Main application entry point."""
    print("Starting the application...")
    
    # Import and run your main application logic here
    try:
        from core.service import run_service
        run_service()
    except ImportError as e:
        print(f"Error importing core.service: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"An error occurred: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()