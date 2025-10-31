"""
Data model module.
"""

from typing import Any, Dict, List
import json


class DataModel:
    """
    A simple data model class for the application.
    """
    
    def __init__(self):
        self.data: Dict[str, Any] = {}
    
    def add_item(self, key: str, value: Any) -> None:
        """
        Add an item to the model.
        
        Args:
            key: Key for the item
            value: Value for the item
        """
        self.data[key] = value
    
    def get_item(self, key: str) -> Any:
        """
        Get an item from the model.
        
        Args:
            key: Key of the item to retrieve
        
        Returns:
            Value of the item or None if not found
        """
        return self.data.get(key)
    
    def remove_item(self, key: str) -> bool:
        """
        Remove an item from the model.
        
        Args:
            key: Key of the item to remove
        
        Returns:
            True if item was removed, False if not found
        """
        if key in self.data:
            del self.data[key]
            return True
        return False
    
    def list_items(self) -> List[str]:
        """
        Get a list of all item keys.
        
        Returns:
            List of all keys in the model
        """
        return list(self.data.keys())
    
    def transform(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """
        Transform input data according to business rules.
        
        Args:
            data: Input data to transform
        
        Returns:
            Transformed data
        """
        # Example transformation: add a processed flag
        transformed = data.copy()
        transformed['processed'] = True
        return transformed
    
    def to_json(self) -> str:
        """
        Convert the model data to JSON string.
        
        Returns:
            JSON string representation of the model
        """
        return json.dumps(self.data)
    
    @classmethod
    def from_json(cls, json_str: str) -> 'DataModel':
        """
        Create a DataModel from a JSON string.
        
        Args:
            json_str: JSON string to deserialize
        
        Returns:
            New DataModel instance
        """
        instance = cls()
        instance.data = json.loads(json_str)
        return instance