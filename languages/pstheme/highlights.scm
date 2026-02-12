; HCL basics
(string_lit) @string
(numeric_lit) @number
(bool_lit) @boolean
(comment) @comment

; Block types - PaletteSwap specific
(block
  (identifier) @keyword
  (#match? @keyword "^(meta|palette|theme|syntax|ansi)$"))

; Hex color values
(string_lit
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

; Variable expressions (namespaces like 'palette' in 'palette.base')
(variable_expr
  (identifier) @namespace)

; Attribute access (properties like 'base', 'text' in 'palette.base')
(get_attr
  (identifier) @property)

; Operators
("=") @operator
("{") @punctuation.bracket
("}") @punctuation.bracket
("(") @punctuation.bracket
(")") @punctuation.bracket
