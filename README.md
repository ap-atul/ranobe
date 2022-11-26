## ranobe
A simple extensible scraper to read light novels from different sources.

### Api
`DOCS INCOMING ...`

### Add a new source
- Import the `Source` trait to implement

```rust
use crate::interface::Source;
use crate::model::*;
```

- Implement the functions of the trait

```rust
impl Source for SourceName {
    fn home(&self) -> Vec<Novel> {
        Vec::new()
    }

    // other functions
}
```

- Test it out and make a pull request
```shell
cargo build 
cargo run
cargo test
```
