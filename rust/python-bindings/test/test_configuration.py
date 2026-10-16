import os
import sys

import pytest
import twitter_text
import yaml

def test_ctor():
    config = twitter_text.TwitterTextConfiguration()
    assert config is not None
    # The default configuration is v3; v4 is opt-in.
    assert config.get_version() == 3


def test_path():
    path = "rust/cpp-bindings/test_data/test_config.json"
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
    with open(r"rust/cpp-bindings/test_data/test_config.json") as file:
        config = twitter_text.TwitterTextConfiguration.configuration_from_json(
            file.read()
        )
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
    # The default configuration is v3; v4 is opt-in.
    config = twitter_text.TwitterTextConfiguration()
    assert config.get_version() == 3
    ranges = config.get_ranges()
    assert len(ranges) == 4
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


def test_v4():
    config = twitter_text.TwitterTextConfiguration.config_v4()
    assert config.get_version() == 4
    assert config.get_emoji_parsing_enabled() == True
    ranges = config.get_ranges()
    v3_ranges = twitter_text.TwitterTextConfiguration.config_v3().get_ranges()
    # v4 is v3 plus the alphabetic scripts above U+10FF.
    assert len(ranges) > len(v3_ranges)
    for v3_range in v3_ranges:
        assert any(r.range.start == v3_range.range.start
                   and r.range.end == v3_range.range.end
                   and r.weight == v3_range.weight for r in ranges)

    def covers(start, end):
        return any(r.range.start <= start and r.range.end >= end and r.weight == 100
                   for r in ranges)

    assert covers(0x16A0, 0x16FF)    # Runic
    assert covers(0x1200, 0x137F)    # Ethiopic
    assert covers(0x1780, 0x17FF)    # Khmer
    assert covers(0x1E00, 0x1EFF)    # Latin Extended Additional
    assert covers(0x1F00, 0x1FFF)    # Greek Extended
    assert covers(0xFB50, 0xFDFF)    # Arabic Presentation Forms-A
    assert covers(0x1E900, 0x1E95F)  # Adlam
    assert not covers(0x4E00, 0x9FFF)    # CJK Unified Ideographs
    assert not covers(0xAC00, 0xD7AF)    # Hangul Syllables
    assert not covers(0x1D400, 0x1D7FF)  # Mathematical Alphanumeric Symbols


def test_v3():
    config = twitter_text.TwitterTextConfiguration.config_v3()
    assert config.get_version() == 3
    assert config.get_emoji_parsing_enabled() == True
    ranges = config.get_ranges()
    assert len(ranges) == 4


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

    ranges = config.get_ranges()
    assert len(ranges) == 0

if __name__ == "__main__":
    raise SystemExit(pytest.main([__file__]))
