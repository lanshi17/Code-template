"""
Unit tests for the application.
"""
import unittest


class TestCoreService(unittest.TestCase):
    """Unit tests for the CoreService class."""
    
    def setUp(self):
        """Set up test fixtures before each test method."""
        from src.core.service import CoreService
        self.service = CoreService()
    
    def test_process_data(self):
        """Test the process_data method."""
        input_data = {"test": "data"}
        result = self.service.process_data(input_data)
        self.assertIn('processed', result)
        self.assertTrue(result['processed'])


class TestDataModel(unittest.TestCase):
    """Unit tests for the DataModel class."""
    
    def setUp(self):
        """Set up test fixtures before each test method."""
        from src.core.model import DataModel
        self.model = DataModel()
    
    def test_add_and_get_item(self):
        """Test adding and getting an item."""
        self.model.add_item('key', 'value')
        self.assertEqual(self.model.get_item('key'), 'value')
    
    def test_remove_item(self):
        """Test removing an item."""
        self.model.add_item('key', 'value')
        self.assertTrue(self.model.remove_item('key'))
        self.assertIsNone(self.model.get_item('key'))
    
    def test_transform(self):
        """Test data transformation."""
        input_data = {"original": "data"}
        result = self.model.transform(input_data)
        self.assertIn('processed', result)
        self.assertTrue(result['processed'])


if __name__ == '__main__':
    unittest.main()