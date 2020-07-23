import os
import pytest
import sys
import twitter_text
import yaml

def test_ctor():
    config = twitter_text.TwitterTextConfiguration()
    assert config is not None
    assert config.get_version() == 3

#def test_path():
#    path = "rust_bindings/cpp/test_data/test_config.json"
#    config = twitter_text.ConfigFactory.config_v3()
#    assert config is not None
#
#    assert config.get_version() == 3  

if __name__ == "__main__":
    raise SystemExit(pytest.main([__file__]))