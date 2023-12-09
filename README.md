I choose to use Advent Of Code 2023 to brush up on rust. The following is a log of my experinces:

# Dec 1

This execise was used to reacquint myself with the basics of the rust syntac. I also explore useful iterators on string like `chars` and on that `rev` and `enumerate`. Finally I explored how to pass an iterator as an argument, by telling rust that an argument implements a trait:

```rust
fn find_first_digit(chars: impl Iterator<Item=char>) -> Option<u32>{
    ...
}
```

# Dec 2

