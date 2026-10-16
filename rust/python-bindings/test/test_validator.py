import os
import pytest
import sys
import twitter_text
import yaml

def test_ctor():
    validator = twitter_text.Validator()
    assert validator is not None

def test_accessors():
    validator = twitter_text.Validator()

    assert validator.get_max_tweet_length() == 280

    assert validator.get_short_url_length() == 23
    validator.set_short_url_length(42)
    assert validator.get_short_url_length() == 42

    assert validator.get_short_url_length_https() == 23
    validator.set_short_url_length_https(43)
    assert validator.get_short_url_length_https() == 43

def test_runic_weighting():
    # Runes weigh one unit each under v4, two under v3 and under the default.
    # https://github.com/twitter/twitter-text/issues/430
    runes = "\u16A0\u16A2\u16A6\u16A8\u16B1\u16B2"

    v4 = twitter_text.TwitterTextParser.parse(
        runes, twitter_text.TwitterTextConfiguration.config_v4(), True)
    assert v4.weighted_length == 6
    assert v4.is_valid

    v3 = twitter_text.TwitterTextParser.parse(
        runes, twitter_text.TwitterTextConfiguration.config_v3(), True)
    assert v3.weighted_length == 12

    default = twitter_text.TwitterTextParser.parse(
        runes, twitter_text.TwitterTextConfiguration(), True)
    assert default.weighted_length == 12


@pytest.mark.parametrize("name, text", [
    ("Latin Extended Additional", "\u1E00\u1E01\u1E02\u1E03\u1E04\u1E05"),
    ("Greek Extended", "\u1F00\u1F01\u1F02\u1F03\u1F04\u1F05"),
    ("Ethiopic", "\u1200\u1208\u1210\u1218\u1220\u1228"),
    ("Cherokee", "\u13A0\u13A1\u13A2\u13A3\u13A4\u13A5"),
    ("Khmer", "\u1780\u1781\u1782\u1783\u1784\u1785"),
    ("Arabic Presentation Forms-A", "\uFB50\uFB51\uFB52\uFB53\uFB54\uFB55"),
    ("Adlam", "\U0001E900\U0001E901\U0001E902\U0001E903\U0001E904\U0001E905"),
])
def test_alphabetic_scripts_weigh_like_latin_in_v4(name, text):
    # Six letters from an alphabetic block above U+10FF weigh six units under
    # v4 and twelve under v3 and under the default.
    assert len(text) == 6, name

    v4 = twitter_text.TwitterTextParser.parse(
        text, twitter_text.TwitterTextConfiguration.config_v4(), True)
    assert v4.weighted_length == 6, name
    assert v4.is_valid, name

    v3 = twitter_text.TwitterTextParser.parse(
        text, twitter_text.TwitterTextConfiguration.config_v3(), True)
    assert v3.weighted_length == 12, name

    default = twitter_text.TwitterTextParser.parse(
        text, twitter_text.TwitterTextConfiguration(), True)
    assert default.weighted_length == 12, name


def test_yaml():
    validator = twitter_text.Validator()

    with open (r"rust/conformance/tests/validate.yml") as file:
        testmap = yaml.load(file, Loader=yaml.FullLoader)

        assert len(testmap["tests"]["tweets"]) > 0
        for test in testmap["tests"]["tweets"]:
            assert test["expected"] == validator.is_valid_tweet(test["text"])

        assert len(testmap["tests"]["usernames"]) > 0
        for test in testmap["tests"]["usernames"]:
            assert test["expected"] == validator.is_valid_username(test["text"])

        assert len(testmap["tests"]["lists"]) > 0
        for test in testmap["tests"]["lists"]:
            assert test["expected"] == validator.is_valid_list(test["text"])

        assert len(testmap["tests"]["hashtags"]) > 0
        for test in testmap["tests"]["hashtags"]:
            assert test["expected"] == validator.is_valid_hashtag(test["text"])

        assert len(testmap["tests"]["urls"]) > 0
        for test in testmap["tests"]["urls"]:
            assert test["expected"] == validator.is_valid_url(test["text"])

        assert len(testmap["tests"]["urls_without_protocol"]) > 0
        for test in testmap["tests"]["urls_without_protocol"]:
            assert test["expected"] == validator.is_valid_url_without_protocol(test["text"])

def validate_weighting(tests, config):
    assert len(tests) > 0
    for test in tests:
        result = twitter_text.TwitterTextParser.parse(test["text"], config, True)
        assert test["expected"]["weightedLength"] == result.weighted_length
        assert test["expected"]["valid"] == result.is_valid
        assert test["expected"]["permillage"] == result.permillage
        assert test["expected"]["displayRangeStart"] == result.display_text_range.start
        assert test["expected"]["displayRangeEnd"] == result.display_text_range.end
        assert test["expected"]["validRangeStart"] == result.valid_text_range.start
        assert test["expected"]["validRangeEnd"] == result.valid_text_range.end

def test_weighted():
    validator = twitter_text.Validator()

    with open (r"rust/conformance/tests/validate.yml") as file:
        testmap = yaml.load(file, Loader=yaml.FullLoader)

        validate_weighting(testmap["tests"]["WeightedTweetsCounterTest"], twitter_text.TwitterTextConfiguration.config_v2())
        validate_weighting(testmap["tests"]["WeightedTweetsWithDiscountedEmojiCounterTest"], twitter_text.TwitterTextConfiguration.config_v3())
        validate_weighting(testmap["tests"]["UnicodeDirectionalMarkerCounterTest"], twitter_text.TwitterTextConfiguration.config_v3())

if __name__ == "__main__":
    raise SystemExit(pytest.main([__file__]))