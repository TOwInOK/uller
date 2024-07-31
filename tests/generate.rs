#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use uller::Buller;
    use uller::BytesDownload;
    use uller::JsonDownload;
    use uller::Juller;
    use uller::MakeLink;
    use uller::Qller;
    use url::Url;

    #[derive(Deserialize)]
    struct Test {
        t: String,
    }

    #[derive(Qller, Debug, Juller, Buller)]
    #[output = "Test"]
    #[url = "https://example.com"]
    struct Pancakes {
        #[name = "ident"]
        id: usize,
        name: String,
        #[name = "p"]
        #[pos = 1]
        price: f64,
    }

    #[test]
    fn it_works() {
        let pancake = Pancakes {
            id: 1,
            name: "t".to_string(),
            price: 5.99,
        };
        let url = pancake.url_generate();
        let expected_url = Url::parse_with_params(
            "https://example.com",
            &[("ident", "1"), ("p", "5.99"), ("name", "t")],
        )
        .unwrap();

        // panic!("{:#?}", &url.query());
        println!("{:#?}", pancake);
        assert_eq!(url.query(), expected_url.query());
    }
}
