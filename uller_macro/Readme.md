# Uller Macro

# Crate provides a useful interface for the uller crate

# Qller (default)
## Macros for implementing `MakeLink` in query style using a struct as input
### Example
```rust
  use uller::prelude;
  #[derive(Qller)]
  #[url = "http://127.0.0.1:1234/"]
  struct Test {
      #[name = "f"] // rename to "f"
      f111: String,
      #[name = "v"] // rename to "v"
      #[pos = 0]    // move it to the first position
      v222: String,
  }
```
This will convert to `http://127.0.0.1:1234/?v={value}&f={value}`

Note: Positions start at 0, like an array.

# Juller (feature - juller)
## Macros for downloading `<T>` using a struct that implements `MakeLink` (`Qller`) and `JsonDownload`
### Example
```rust
  use uller::prelude;
  #[derive(Qller, Juller)]
  #[output = "TestOut"]
  #[url = "http://127.0.0.1:41112/"]
  struct Test {
      f: String,
      v: String,
  }
  #[derive(Deserialize, Debug)]
  struct TestOut {
      field: String,
  }
  async fn convert(st: &Test) -> TestOut {
      st.download().await.unwrap()
      // or
      st.download_verbose().await.unwrap()
  }
```

# Buller (feature - buller)
## Macros for downloading `Bytes` using a struct that implements `MakeLink` (`Qller`) and `BytesDownload`
### Example
```rust
  use uller::prelude;

  #[derive(Qller, Buller)]
  #[url = "http://127.0.0.1:41112/"]
  struct Test {
      f: String,
      v: String,
  }

  async fn convert(st: &Test) -> bytes::Bytes {
      st.download().await.unwrap()
      // or
      st.download_verbose().await.unwrap()
  }
```
