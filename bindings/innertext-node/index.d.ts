/**
 * innertext - Pure Rust WHATWG-compliant text extraction from HTML
 * 
 * Provides functions to extract text content from HTML documents:
 *   - innerText: rendered text collection (respects CSS display/visibility/whitespace)
 *   - outerText: identical to innerText getter per WHATWG spec
 *   - textContent: CSS-blind structural text concatenation
 */

/**
 * Extract innerText from HTML string.
 * 
 * Implements the WHATWG innerText algorithm, which respects CSS
 * properties like display, visibility, white-space, and text-transform.
 * 
 * @param html - HTML string to extract text from
 * @returns Rendered text content as a string
 * @throws Error if HTML parsing fails
 * 
 * @example
 * ```typescript
 * import { innerText } from 'innertext';
 * 
 * const html = "<div>Hello <span style='display:none'>hidden</span> World</div>";
 * console.log(innerText(html)); // "Hello World"
 * ```
 */
export function innerText(html: string): string;

/**
 * Extract outerText from HTML string.
 * 
 * Per WHATWG spec, outerText getter is identical to innerText getter.
 * outerText setter is more complex and not implemented.
 * 
 * @param html - HTML string to extract text from
 * @returns Rendered text content as a string
 * @throws Error if HTML parsing fails
 */
export function outerText(html: string): string;

/**
 * Extract textContent from HTML string.
 * 
 * Performs a CSS-blind structural walk of the DOM tree, concatenating
 * all text nodes without respecting any CSS properties.
 * 
 * @param html - HTML string to extract text from
 * @returns Structural text content as a string
 * @throws Error if HTML parsing fails
 * 
 * @example
 * ```typescript
 * import { textContent } from 'innertext';
 * 
 * const html = "<div>Hello <span style='display:none'>hidden</span> World</div>";
 * console.log(textContent(html)); // "Hello hidden World"
 * ```
 */
export function textContent(html: string): string;

/**
 * Parsed HTML document for text extraction.
 * 
 * Parsing is done once on initialization, and text extraction is then
 * performed multiple times if needed (efficient for multiple extractions).
 * 
 * @example
 * ```typescript
 * import { HtmlDocument } from 'innertext';
 * 
 * const doc = new HtmlDocument("<div>Hello <b>World</b></div>");
 * console.log(doc.innerText());    // "Hello World"
 * console.log(doc.textContent()); // "Hello World"
 * ```
 */
export class HtmlDocument {
  /**
   * Parse an HTML document.
   * 
   * @param html - HTML string to parse
   * @throws Error if HTML parsing fails
   */
  constructor(html: string);

  /**
   * Get rendered text content (respects CSS).
   * 
   * @returns Rendered text content
   * @throws Error if extraction fails
   */
  innerText(): string;

  /**
   * Get outer text (identical to innerText getter).
   * 
   * @returns Outer text content
   * @throws Error if extraction fails
   */
  outerText(): string;

  /**
   * Get structural text content (CSS-blind).
   * 
   * @returns Structural text content
   * @throws Error if extraction fails
   */
  textContent(): string;
}
