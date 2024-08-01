#[cfg(test)]
mod tests {
    use uller::prelude::*;
    #[test]
    fn qller() {
        #[derive(Qller)]
        #[url = "https://example.com"]
        struct Test {
            #[name = "ident"]
            id: usize,
            name: String,
            #[name = "p"]
            #[pos = 1]
            price: f64,
        }
        let pancake = Test {
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

        assert_eq!(url.query(), expected_url.query());
    }

    #[cfg(feature = "juller")]
    #[tokio::test]
    async fn juller() {
        use serde::Deserialize;
        #[derive(Qller, Juller, Debug)]
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
        }

        let opts = mockito::ServerOpts {
            host: "127.0.0.1",
            port: 41112,
            ..Default::default()
        };
        let mut server = mockito::Server::new_with_opts(opts);
        let mock = server
            .mock("GET", "/?f=v&v=v")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                  "field": "val"
                }
                "#,
            )
            .create();
        let test = Test {
            f: "v".to_string(),
            v: "v".to_string(),
        };
        let out_convert = convert(&test).await;
        let expected = TestOut {
            field: "val".to_string(),
        };
        mock.assert();

        assert_eq!(expected.field, out_convert.field);
    }

    #[cfg(feature = "buller")]
    #[tokio::test]
    async fn buller() {
        #[derive(Qller, Buller, Debug)]
        #[url = "http://127.0.0.1:41111/"]
        struct Test {
            f: String,
            v: String,
        }

        let opts = mockito::ServerOpts {
            host: "127.0.0.1",
            port: 41111,
            ..Default::default()
        };

        let mut server = mockito::Server::new_with_opts(opts);
        let mock = server
            .mock("GET", "/?f=v&v=v")
            .with_status(200)
            .with_body(b"123")
            .create();
        let test = Test {
            f: "v".to_string(),
            v: "v".to_string(),
        };
        let out_convert = test.download_verbose().await.unwrap();
        let expected = bytes::Bytes::from_static(b"123");
        mock.assert();

        assert_eq!(&expected[..], out_convert);
    }
}
