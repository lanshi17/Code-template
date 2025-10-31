"""
Core service module.
"""

from .model import DataModel
from ...lib.utils import setup_logging
import logging


class CoreService:
    """
    Core service class that handles the main business logic.
    """
    
    def __init__(self):
        self.logger = setup_logging()
        self.model = DataModel()
    
    def process_data(self, data):
        """
        Process input data.
        
        Args:
            data: Input data to process
        
        Returns:
            Processed data
        """
        self.logger.info(f"Processing data: {data}")
        # Add your business logic here
        result = self.model.transform(data)
        self.logger.info(f"Data processed successfully: {result}")
        return result


def run_service():
    """
    Function to run the core service.
    """
    service = CoreService()
    
    # Example usage
    sample_data = {"key": "value", "number": 42}
    result = service.process_data(sample_data)
    
    print(f"Service result: {result}")