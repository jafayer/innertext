"""
innertext - Pure Rust WHATWG-compliant text extraction from HTML

This module provides functions to extract text content from HTML documents
with full compliance to the WHATWG specification:
  - innerText: rendered text collection (respects CSS display/visibility/whitespace)
  - outerText: identical to innerText getter (outer text for element)
  - textContent: CSS-blind structural text concatenation
"""

def inner_text(html: str) -> str:
    """
    Extract innerText from HTML string.
    
    This implements the WHATWG innerText algorithm, which respects CSS
    properties like display, visibility, white-space, and text-transform.
    
    Args:
        html: HTML string to extract text from
        
    Returns:
        Rendered text content as a string
        
    Raises:
        ValueError: If HTML parsing fails
        
    Example:
        >>> innertext.inner_text("<div>Hello <span style='display:none'>hidden</span> World</div>")
        'Hello World'
    """
    ...

def outer_text(html: str) -> str:
    """
    Extract outerText from HTML string.
    
    Per WHATWG spec, outerText getter is identical to innerText getter.
    outerText setter is more complex and not implemented.
    
    Args:
        html: HTML string to extract text from
        
    Returns:
        Rendered text content as a string
        
    Raises:
        ValueError: If HTML parsing fails
    """
    ...

def text_content(html: str) -> str:
    """
    Extract textContent from HTML string.
    
    This performs a CSS-blind structural walk of the DOM tree, concatenating
    all text nodes without respecting any CSS properties.
    
    Args:
        html: HTML string to extract text from
        
    Returns:
        Structural text content as a string
        
    Raises:
        ValueError: If HTML parsing fails
        
    Example:
        >>> innertext.text_content("<div>Hello <span style='display:none'>hidden</span> World</div>")
        'Hello hidden World'
    """
    ...
