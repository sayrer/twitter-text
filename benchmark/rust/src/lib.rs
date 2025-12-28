#[cfg(test)]
mod tests {
    use twitter_text::autolinker::Autolinker;
    use twitter_text::extractor::Extractor;
    use twitter_text::validator::Validator;
    use twitter_text::ParserBackend;
    use twitter_text_config::Configuration;

    // Same texts used in the benchmark
    const SIMPLE_TEXT: &str = "Hello world!";
    const TEXT_WITH_MENTION: &str = "Hello @jack how are you?";
    const TEXT_WITH_HASHTAG: &str = "Check out #rustlang for great programming content";
    const TEXT_WITH_URL: &str = "Check out https://github.com/rust-lang/rust for more info";
    const TEXT_WITH_CASHTAG: &str = "Stock $AAPL is looking good today";
    const COMPLEX_TEXT: &str = "Hey @elonmusk check out #AI and #ML trends at https://example.com/ai - $TSLA looking strong! @OpenAI is doing great work. Visit https://t.co/abc123 for more #tech news.";

    const TEXTS: &[&str] = &[
        SIMPLE_TEXT,
        TEXT_WITH_MENTION,
        TEXT_WITH_HASHTAG,
        TEXT_WITH_URL,
        TEXT_WITH_CASHTAG,
        COMPLEX_TEXT,
    ];

    #[test]
    fn backends_produce_same_extracted_urls() {
        for text in TEXTS {
            let nom = Extractor::with_parser_backend(ParserBackend::Nom);
            let pest = Extractor::with_parser_backend(ParserBackend::Pest);
            assert_eq!(
                nom.extract_urls(text),
                pest.extract_urls(text),
                "URLs differ for: {}",
                text
            );
        }
    }

    #[test]
    fn backends_produce_same_extracted_hashtags() {
        for text in TEXTS {
            let nom = Extractor::with_parser_backend(ParserBackend::Nom);
            let pest = Extractor::with_parser_backend(ParserBackend::Pest);
            assert_eq!(
                nom.extract_hashtags(text),
                pest.extract_hashtags(text),
                "Hashtags differ for: {}",
                text
            );
        }
    }

    #[test]
    fn backends_produce_same_extracted_mentions() {
        for text in TEXTS {
            let nom = Extractor::with_parser_backend(ParserBackend::Nom);
            let pest = Extractor::with_parser_backend(ParserBackend::Pest);
            assert_eq!(
                nom.extract_mentioned_screennames(text),
                pest.extract_mentioned_screennames(text),
                "Mentions differ for: {}",
                text
            );
        }
    }

    #[test]
    fn backends_produce_same_extracted_cashtags() {
        for text in TEXTS {
            let nom = Extractor::with_parser_backend(ParserBackend::Nom);
            let pest = Extractor::with_parser_backend(ParserBackend::Pest);
            assert_eq!(
                nom.extract_cashtags(text),
                pest.extract_cashtags(text),
                "Cashtags differ for: {}",
                text
            );
        }
    }

    #[test]
    fn backends_produce_same_validation() {
        for text in TEXTS {
            let nom = Validator::with_parser_backend(ParserBackend::Nom);
            let pest = Validator::with_parser_backend(ParserBackend::Pest);
            assert_eq!(
                nom.is_valid_tweet(text),
                pest.is_valid_tweet(text),
                "Validation differs for: {}",
                text
            );
        }
    }

    #[test]
    fn backends_produce_same_autolink() {
        for text in TEXTS {
            let nom = Autolinker::with_parser_backend(false, ParserBackend::Nom);
            let pest = Autolinker::with_parser_backend(false, ParserBackend::Pest);
            assert_eq!(
                nom.autolink(text),
                pest.autolink(text),
                "Autolink differs for: {}",
                text
            );
        }
    }

    #[test]
    fn backends_produce_same_parse_results() {
        let config = Configuration::default();
        for text in TEXTS {
            let nom_result =
                twitter_text::parse_with_parser_backend(text, &config, true, ParserBackend::Nom);
            let pest_result =
                twitter_text::parse_with_parser_backend(text, &config, true, ParserBackend::Pest);
            assert_eq!(
                nom_result.weighted_length, pest_result.weighted_length,
                "Weighted length differs for: {}",
                text
            );
            assert_eq!(
                nom_result.is_valid, pest_result.is_valid,
                "Validity differs for: {}",
                text
            );
            assert_eq!(
                nom_result.display_text_range, pest_result.display_text_range,
                "Display text range differs for: {}",
                text
            );
            assert_eq!(
                nom_result.valid_text_range, pest_result.valid_text_range,
                "Valid text range differs for: {}",
                text
            );
        }
    }
}
