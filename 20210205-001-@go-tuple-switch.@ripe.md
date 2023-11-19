# Go pattern: tuple switch

An idiom I discovered a few years ago; I think about it as "poor man's pattern
matching" in Go:

```go
func f(foo, bar, baz bool) {
	type tuple struct{ foo, bar, baz bool }
	switch (tuple{foo, bar, baz}) {
	case tuple{true, true, true}:
		// ...
	case tuple{true, true, false}:
		// ...
	case tuple{true, false, true},
		tuple{true, false, false},
		tuple{false, true, true}:
		// ...
	}
}
```

This works also for [some types other than bool][cmp], notably including
`string`, and they can be mixed in the `tuple` type when needed.

[cmp]: https://golang.org/ref/spec#Comparison_operators
