"""innertext - Pure Rust WHATWG-compliant text extraction from HTML.

This package re-exports functions from the compiled Rust extension.
"""

from ._innertext import inner_text, outer_text, text_content

__all__ = ["inner_text", "outer_text", "text_content"]
