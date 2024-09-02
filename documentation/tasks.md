# Tasks

## Add a post domain model and implement

```rust
// domain/post/model.rs
pub struct Post {
    id: PostId,
    title: Title,
    content: Content,
    // ... other fields
}
```

## Add an application layer to support multiple interfaces