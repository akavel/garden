# Go pattern: less — comparator for prioritized sorting

This pattern should work in basically any programming language
where "less than" functions are used
to customize sorting/collections.
It has the benefit of having a regular, structured form,
and thus making it easy to see if it is written correctly
(in other words, easy to avoid bugs and easy to review).

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

<!-- TODO: when I implement "tree tags", possibly move this to @programming -->

[[TODO]] write with `if`s, then for Go explain a `switch` refactoring as better

[[TODO]] add a runnable link to sample code using it in Go playground
