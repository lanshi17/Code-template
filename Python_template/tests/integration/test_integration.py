"""
Integration tests for the application.
"""
import unittest


class TestServiceIntegration(unittest.TestCase):
    """Integration tests for service components."""
    
    def test_service_model_integration(self):
        """Test integration between service and model."""
        from src.core.service import CoreService
        from src.core.model import DataModel
        
        service = CoreService()
        model = DataModel()
        
        # Add data to model
        model.add_item('test_key', 'test_value')
        
        # Process data through service
        test_data = {'input': 'data'}
        result = service.process_data(test_data)
        
        # Verify the transformation was applied
        self.assertIn('processed', result)
        self.assertTrue(result['processed'])


if __name__ == '__main__':
    unittest.main()