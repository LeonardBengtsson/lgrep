# lregex

Searches through text passed into stdin using a regex.

`lgrep <PATTERN>`

Use `|` to provide alternate patterns, `*` to match zero or more of a 
pattern, and `()` to group alternatives. `\` can be used to escape these 
special characters. NOTE: all other characters (including space) are 
interpreted as-is.

Example: Pattern `( *(King|Kong))*s` matches `Kings`, `King Kongs`, `Kong   Kong  s`, `s`

## Build

Build using `cargo build`.
