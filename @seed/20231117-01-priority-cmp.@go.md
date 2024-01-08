# Universal comparator/less pattern for prioritized sorting

```go
func Less(a, b T) bool {
  switch {
  case a.foo != b.foo:
    return a.foo < b.foo
  case a.bar != b.bar:
    return a.bar < b.bar
  default:
    return a.bing < b.bing
  }
}
```

[[TODO]] explain why this is valuable - easy to understand, clear pattern, "less" clearly corresponds to "<" in body

[[TODO]] how to tag this? @go? @programming (with tree tags)? -> maybe for now @go, and think more later?

[[TODO]] write with `if`s, then for Go explain a `switch` refactoring as better
