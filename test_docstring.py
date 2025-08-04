"""
Mathematical utilities module.

This module provides mathematical functions and classes for various
computational tasks, including recursive algorithms and basic arithmetic.
"""

def fibonacci(n):
    """Calculate the nth Fibonacci number using recursion.
    
    Args:
        n: The position in the Fibonacci sequence (non-negative integer)
        
    Returns:
        The nth Fibonacci number
        
    Example:
        >>> fibonacci(5)
        5
    """
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

class Calculator:
    """A simple calculator class for basic arithmetic operations."""
    
    def add(self, a, b):
        """Add two numbers together.
        
        Args:
            a: First number
            b: Second number
            
        Returns:
            Sum of a and b
        """
        return a + b