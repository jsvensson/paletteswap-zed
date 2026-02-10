; HCL basics
(string) @string
(number) @number
(bool) @boolean
(comment) @comment

; Block types - PaletteSwap specific
(block
  (identifier) @keyword
  (#match? @keyword "^(meta|palette|theme|syntax|ansi)$"))

; Hex color values
(string
  (template_literal) @string.special
  (#match? @string.special "^#[0-9a-fA-F]{6}$"))

; Function calls - brighten()
(function_call
  (identifier) @function
  (#match? @function "^brighten$"))

; Attribute names
(attribute
  (identifier) @property)

; Block identifiers
(block
  (identifier) @type)

; Operators
("=") @operator
("{") @punctuation.bracket
("}") @punctuation.bracket
("(") @punctuation.bracket
(")") @punctuation.bracket
