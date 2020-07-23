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

if __name__ == "__main__":
    raise SystemExit(pytest.main([__file__]))