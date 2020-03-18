# rusting-interpreters
## 2.4: Scanning
- The original intent was to stay pretty close to the book - use functions and structs with the same names. However, it turns out idiomatic Rust doesn't look much like idiomatic Java. I also just personally disagree with some of the author's naming choices - I prefer descriptive function names to short ones, it's 2020, everyone has autocomplete. There are comments in the code in some of the places I've diverged discussing why.
- The challenges mention scanners that don't discard whitespace and comments. I've actually done that here for comments, because by the time one is detected the Scanner is committed to returning a token. The best alternative I came up with was to recurse and return the next token, but for long sequences of single line comments that would be a very dangerous idea. Simpler to consume the comment, return it as a token, and filter it out before parsing.