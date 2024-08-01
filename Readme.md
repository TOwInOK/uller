# Uller

## About
This crate give you:
* MakeLink
  - Interface for making `Url` from structure
* JsonDownload<T>
  - Interface for fetching data from generated `Url` by `MakeLink` and convert to ``<T>`` structure
* BytesDownload
  - Interface for download some data as `bytes` from `MakeLink` convert

### info under is [uller-macro crate examples](https://crates.io/crates/uller_macro)

# Example (future - macro)

## Qller (default)
### Macros for implement `MakeLink` in query style by using struct as prompt
#### Example
```rust
  use uller::prelude;
  #[derive(Qller)]
  #[url = "http://127.0.0.1:1234/"]
  struct Test {
      #[name = "f"] // rename to "f"
      f111: String,
      #[name = "v"] // rename to "v"
      #[pos = 0] // move it in first position
      v222: String
  }
```
will convert to http://127.0.0.1:1234/?v={value}&f={value}
note: position starts with 0 pos like an array.

# Example (future - juller)

## Juller (feature - juller)
### Macros for download ``<T>`` using struct which implement `MakeLink` (`Qller`) and `JsonDownload`
#### Example
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

# Example (future - buller)

## Buller (feature - buller)
### Macros for download `Bytes` using struct which implement `MakeLink` (`Qller`) and `BytesDownload`
#### Example
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
