<div align="center">
    <h3>Ranobe</h3>
    <p>A simple extensible scraper to read light novels from different sources.</p>
</div>

------------

### Community

<a href="https://discord.gg/96wsWZ6M" title="Join Discord">
    <img width="40" height="40" src="https://assets-global.website-files.com/6257adef93867e50d84d30e2/636e0a69f118df70ad7828d4_icon_clyde_blurple_RGB.svg" alt="Join Discord" title="Join Discord">
</a>

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

```
cargo build 
cargo run
cargo test
```
