package com.innertext;

/**
 * Pure Rust WHATWG-compliant text extraction from HTML.
 * 
 * Provides static methods to extract text content from HTML documents:
 *   - innerText: rendered text collection (respects CSS display/visibility/whitespace)
 *   - outerText: identical to innerText getter per WHATWG spec
 *   - textContent: CSS-blind structural text concatenation
 */
public class InnerText {
    static {
        System.loadLibrary("innertext");
    }

    /**
     * Extract innerText from HTML string.
     * 
     * Implements the WHATWG innerText algorithm, which respects CSS
     * properties like display, visibility, white-space, and text-transform.
     * 
     * @param html HTML string to extract text from
     * @return Rendered text content as a string
     * @throws IllegalArgumentException if HTML parsing fails
     * 
     * @example
     * String html = "<div>Hello <span style='display:none'>hidden</span> World</div>";
     * String text = InnerText.innerText(html);  // "Hello World"
     */
    public static native String innerText(String html);

    /**
     * Extract outerText from HTML string.
     * 
     * Per WHATWG spec, outerText getter is identical to innerText getter.
     * 
     * @param html HTML string to extract text from
     * @return Rendered text content as a string
     * @throws IllegalArgumentException if HTML parsing fails
     */
    public static native String outerText(String html);

    /**
     * Extract textContent from HTML string.
     * 
     * Performs a CSS-blind structural walk of the DOM tree, concatenating
     * all text nodes without respecting any CSS properties.
     * 
     * @param html HTML string to extract text from
     * @return Structural text content as a string
     * @throws IllegalArgumentException if HTML parsing fails
     * 
     * @example
     * String html = "<div>Hello <span style='display:none'>hidden</span> World</div>";
     * String text = InnerText.textContent(html);  // "Hello hidden World"
     */
    public static native String textContent(String html);
}
