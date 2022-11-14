# Semantic Version tools

Provides tools to determine new versions based on `semantic version` comments.

Refer to docs for more information, run `make doc`.

### To compile

Run: 
```bash
make release
```

Use binary named `getver`.

### To use

```bash
getver --current-version v2.1.4 --comment "feat: this is a feature"
# Output:
v2.2.0
```
