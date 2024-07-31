#[cfg(test)]
mod tests {
    use uller::MakeLink;
    use uller::Qller;
    use url::Url;

    #[derive(Qller)]
    #[url = "https://example.com"]
    struct Pancakes {
        #[name = "ident"]
        id: usize,
        name: String,
        #[name = "p"]
        #[pos = 0]
        price: f64,
        description: String,
    }

    #[test]
    fn it_works() {
        let pancake = Pancakes {
            id: 1,
            name: "Blueberry Pancake".to_string(),
            price: 5.99,
            description: "Delicious blueberry pancake".to_string(),
        };
        let url = pancake.url_generate();
        let expected_url = Url::parse_with_params(
            "https://example.com",
            &[
                ("ident", "1"),
                ("name", "Blueberry Pancake"),
                ("p", "5.99"),
                ("description", "Delicious blueberry pancake"),
            ],
        )
        .unwrap();

        // panic!("{:#?}", &url.query());
        assert_eq!(url, expected_url);
    }
}
