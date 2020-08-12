import os
import pytest
import sys
import twitter_text
import yaml

def test_ctor():
    config = twitter_text.TwitterTextConfiguration()
    assert config is not None
    assert config.get_version() == 3

def test_path():
    path = "rust_bindings/cpp/test_data/test_config.json"
    config = twitter_text.TwitterTextConfiguration.configuration_from_path(path)
    assert config is not None
    assert config.get_version() == 42
    assert config.get_max_weighted_tweet_length() == 400
    assert config.get_scale() == 43
    assert config.get_default_weight() == 213
    assert config.get_transformed_url_length() == 32
    ranges = config.get_ranges()
    assert len(ranges) == 1
    wr = ranges[0]
    assert wr.range.start == 0
    assert wr.range.end == 4351
    assert wr.weight == 200

def test_json():
    with open(r"rust_bindings/cpp/test_data/test_config.json") as file:
        config = twitter_text.TwitterTextConfiguration.configuration_from_json(file.read())
        assert config is not None
        assert config.get_version() == 42
        assert config.get_max_weighted_tweet_length() == 400
        assert config.get_scale() == 43
        assert config.get_default_weight() == 213
        assert config.get_transformed_url_length() == 32
        ranges = config.get_ranges()
        assert len(ranges) == 1
        wr = ranges[0]
        assert wr.range.start == 0
        assert wr.range.end == 4351
        assert wr.weight == 200

def test_accessors():
    config = twitter_text.TwitterTextConfiguration()

    assert config.get_version() == 3
    config.set_version(199)
    assert config.get_version() == 199

    assert config.get_max_weighted_tweet_length() == 280
    config.set_max_weighted_tweet_length(199)
    assert config.get_max_weighted_tweet_length() == 199

    assert config.get_scale() == 100
    config.set_scale(199)
    assert config.get_scale() == 199

    assert config.get_default_weight() == 200
    config.set_default_weight(199)
    assert config.get_default_weight() == 199

    assert config.get_transformed_url_length() == 23
    config.set_transformed_url_length(199)
    assert config.get_transformed_url_length() == 199

    assert config.get_emoji_parsing_enabled() == True
    config.set_emoji_parsing_enabled(False)
    assert config.get_emoji_parsing_enabled() == False

def test_ranges():
    config = twitter_text.TwitterTextConfiguration()
    ranges = config.get_ranges()
    assert ranges[0].range.start == 0
    assert ranges[0].range.end == 4351
    assert ranges[0].weight == 100

    assert ranges[1].range.start == 8192
    assert ranges[1].range.end == 8205
    assert ranges[1].weight == 100

    assert ranges[2].range.start == 8208
    assert ranges[2].range.end == 8223
    assert ranges[2].weight == 100

    assert ranges[3].range.start == 8242
    assert ranges[3].range.end == 8247
    assert ranges[3].weight == 100

def test_v2():
    config = twitter_text.TwitterTextConfiguration.config_v2()
    assert config.get_version() == 2
    assert config.get_emoji_parsing_enabled() == False
    ranges = config.get_ranges()
    assert len(ranges) == 4

def test_v1():
    config = twitter_text.TwitterTextConfiguration.config_v1()
    assert config.get_version() == 1
    assert config.get_emoji_parsing_enabled() == False
    ranges = config.get_ranges()
    assert len(ranges) == 0

if __name__ == "__main__":
    raise SystemExit(pytest.main([__file__]))