# Lexer Test Coverage

This document tracks the implementation and test coverage of the Excel formula lexer based on the specification.

## Token Types in "Table I. Lexical tokens used in the grammar"

- **BOOL** - Boolean literal `TRUE | FALSE`

- **NUMBER** - Integer, floating point, or scientific notation `[0-9]+ .? [0-9]* (e [0-9]+)?`

- **STRING** - String literal `" ([^ "] | "")* "`

- **ERROR** - Error literals `#NULL! | #DIV/0! | #VALUE! | #NAME? | #NUM! | #N/A`

- **ERROR-REF** - Reference error literal `#REF!`

### To be implemented

- **CELL** - Cell reference `$? [A-Z]+ $? [1-9][0-9]*`
  - [ ] Simple cell reference (e.g., A1)
  - [ ] Absolute row (e.g., A$1)
  - [ ] Absolute column (e.g., $A1)
  - [ ] Fully absolute (e.g., $A$1)
  - [ ] Multi-letter columns (e.g., AA1, ZZ100)

- **DDECALL** - Dynamic Data Exchange link `' ([^ '] | ")+ '`
  - [ ] Basic DDE calls
  - [ ] DDE with quotes

- **EXCEL-FUNCTION** - Excel built-in function name followed by `(`
  - [ ] Common functions (SUM, AVERAGE, IF, etc.)
  - [ ] Function call opening parenthesis

- **FILE** - External file reference `\[ [0-9]+ \]`
  - [ ] File references

- **HORIZONTAL-RANGE** - Range of rows `$? [0-9]+ : $? [0-9]+`
  - [ ] Simple row range (e.g., 1:5)
  - [ ] Absolute row ranges

- **VERTICAL-RANGE** - Range of columns `$? [A-Z]+ : $? [A-Z]+`
  - [ ] Simple column range (e.g., A:C)
  - [ ] Absolute column ranges

- **NR** - Named range `[A-Z_\\★1][★4]*`
  - [ ] Simple named ranges
  - [ ] Named ranges with underscores
  - [ ] Named ranges with extended characters

- **NR-COMBINATION** - Named range starting with string that could be another token
  - [ ] Named ranges starting with TRUE/FALSE
  - [ ] Named ranges starting with cell-like strings

- **SR-COLUMN** - Column definition in structured references `[\w\.]+`
  - [ ] Structured reference columns

- **REF-FUNCTION** - Excel built-in reference-returning function `(INDEX | OFFSET | INDIRECT)\(`
  - [ ] INDEX function
  - [ ] OFFSET function
  - [ ] INDIRECT function

- **REF-FUNCTION-COND** - Excel built-in conditional reference function `(IF | CHOOSE)\(`
  - [ ] IF function (when used with references)
  - [ ] CHOOSE function (when used with references)

- **RESERVED-NAME** - Excel reserved name `_xlnm\. [A-Z_]+`
  - [ ] Reserved names

- **SHEET** - Worksheet name `★2+ !`
  - [ ] Simple sheet references
  - [ ] Sheet names with spaces

- **SHEET-QUOTED** - Sheet reference in single quotes `' (★3 | ")* ' !`
  - [ ] Quoted sheet references
  - [ ] Sheets with special characters

- **MULTIPLE-SHEETS** - Reference to multiple sheets `★2+ : ★2+ !`
  - [ ] Multiple sheet ranges

- **MULTIPLE-SHEETS-QUOTED** - Multiple sheets reference in single quotes
  - [ ] Quoted multiple sheet ranges

- **UDF** - User Defined Function `(_xll\.)? [A-Z_\][A-Z0-9_\\.★1]* \(`
  - [ ] Simple UDFs
  - [ ] XLL functions

## Other Token Types

- **Array Delimiters**

- **Unary Operators**

### To be implemented

- **Binary Operators**
  - [ ] `+` (addition)
  - [ ] `-` (subtraction)
  - [ ] `*` (multiplication)
  - [ ] `/` (division)
  - [ ] `^` (exponentiation)
  - [ ] `&` (concatenation)
  - [ ] `<` (less than)
  - [ ] `>` (greater than)
  - [ ] `=` (equals)
  - [ ] `<=` (less than or equal)
  - [ ] `>=` (greater than or equal)
  - [ ] `<>` (not equal)

- **Other Delimiters**
  - [ ] `(` (left parenthesis)
  - [ ] `)` (right parenthesis)
  - [ ] `:` (range operator)
  - [ ] `!` (sheet separator)
  - [ ] ` ` (space - intersection operator)
  - [ ] `%` (percent - postfix operator)

## Notes

### Placeholder Characters (★)

- **★1** - Extended characters: Non-control Unicode characters x80 and up
- **★2** - Sheet characters: Any character except `' * [ ] \ : / ? ( ) ; { } # " = < > & + - * / ^ % ,`
- **★3** - Enclosed sheet characters: Any character except `' * [ ] \ : / ?`
- **★4** - Valid named range characters: `A-Z0-9\\_.?★1`

