import os
import pytest
import sys
import twitter_text
import yaml

def test_ctor():
    autolinker = twitter_text.Autolinker()
    assert autolinker is not None

def test_accessors():
    autolinker = twitter_text.Autolinker()

    assert autolinker.get_no_follow() is False
    autolinker.set_no_follow(True)
    assert autolinker.get_no_follow() is True

    assert autolinker.get_url_class() == ""
    autolinker.set_url_class("foo")
    assert autolinker.get_url_class() == "foo"

    assert autolinker.get_symbol_tag() == ""
    autolinker.set_symbol_tag("bar")
    assert autolinker.get_symbol_tag() == "bar"

    assert autolinker.get_text_with_symbol_tag() == ""
    autolinker.set_text_with_symbol_tag("qux")
    assert autolinker.get_text_with_symbol_tag() == "qux"

    assert autolinker.get_list_class() == "tweet-url list-slug"
    autolinker.set_list_class("tweet-url list-slug foo")
    assert autolinker.get_list_class() == "tweet-url list-slug foo"

    assert autolinker.get_username_class() == "tweet-url username"
    autolinker.set_username_class("tweet-url username bar")
    assert autolinker.get_username_class() == "tweet-url username bar"

    assert autolinker.get_hashtag_class() == "tweet-url hashtag"
    autolinker.set_hashtag_class("tweet-url hashtag baz")
    assert autolinker.get_hashtag_class() == "tweet-url hashtag baz"

    assert autolinker.get_cashtag_class() == "tweet-url cashtag"
    autolinker.set_cashtag_class("tweet-url cashtag qux")
    assert autolinker.get_cashtag_class() == "tweet-url cashtag qux"

    assert autolinker.get_username_url_base() == "https://twitter.com/"
    autolinker.set_username_url_base("https://example.com/")
    assert autolinker.get_username_url_base() == "https://example.com/"

    assert autolinker.get_hashtag_url_base() == "https://twitter.com/search?q=%23"
    autolinker.set_hashtag_url_base("https://example.com/search?q=%23")
    assert autolinker.get_hashtag_url_base() == "https://example.com/search?q=%23"

    assert autolinker.get_cashtag_url_base() == "https://twitter.com/search?q=%24"
    autolinker.set_cashtag_url_base("https://example.com/search?q=%24")
    assert autolinker.get_cashtag_url_base() == "https://example.com/search?q=%24"

    assert autolinker.get_invisible_tag_attrs() == "style='position:absolute;left:-9999px;'"
    autolinker.set_invisible_tag_attrs("")
    assert autolinker.get_invisible_tag_attrs() == ""

    assert autolinker.get_username_include_symbol() is False
    autolinker.set_username_include_symbol(True)
    assert autolinker.get_username_include_symbol() is True

def test_yaml():
    autolinker = twitter_text.Autolinker()

    with open (r"rust/conformance/tests/autolink.yml") as file:
        testmap = yaml.load(file, Loader=yaml.FullLoader)

        assert len(testmap["tests"]["usernames"]) > 0
        for test in testmap["tests"]["usernames"]:
            assert test["expected"] == autolinker.autolink_usernames_and_lists(test["text"])

        assert len(testmap["tests"]["lists"]) > 0
        for test in testmap["tests"]["lists"]:
            assert test["expected"] == autolinker.autolink_usernames_and_lists(test["text"])

        assert len(testmap["tests"]["hashtags"]) > 0
        for test in testmap["tests"]["hashtags"]:
            assert test["expected"] == autolinker.autolink_hashtags(test["text"])

        assert len(testmap["tests"]["urls"]) > 0
        for test in testmap["tests"]["urls"]:
            assert test["expected"] == autolinker.autolink_urls(test["text"])

        assert len(testmap["tests"]["cashtags"]) > 0
        for test in testmap["tests"]["cashtags"]:
            assert test["expected"] == autolinker.autolink_cashtags(test["text"])

        assert len(testmap["tests"]["all"]) > 0
        for test in testmap["tests"]["all"]:
            assert test["expected"] == autolinker.autolink(test["text"])

if __name__ == "__main__":
    raise SystemExit(pytest.main([__file__]))
