## Rules

The rules for the `.env` mostly follow those of the [dotenv node module](https://www.npmjs.com/package/dotenv), but do differ in certain ways

- `BASIC=basic` becomes `{ BASIC: "basic" }`
- Empty lines are skipped
- Lines beginning with `#` are treated as comments and therefore skipped
- `#` marks the beginning of a comment, unless surrounded by quotes. NOTE: Inline comments are currently unsupported
- Empty values become empty strings
- Inner quotes are maintained (`JSON={"foo": "bar"}` becomes `{ JSON:"{\"foo\": \"bar\" }"`)
- Whitespace is removed from both ends of unquoted values
- Single and double quoted values are escaped (`SINGLE_QUOTE='quoted'` becomes `{ SINGLE_QUOTE: "quoted" }`)
- Single and double quoted values maintain whitespace from both ends (`FOO=" some value "` becomes `{ FOO: ' some value '}` )
- All control characters are ignored, and multiline values are not supported
- Language literals are ignored and will be converted to strings (`IS_SECRET=true` becomes `{ IS_SECRET: "true" }`)
- Assignments may be prefixed with `export` but it is neither required nor advised
- Any breaking from the above rules, or general rules of variable syntax will result in compile errors
