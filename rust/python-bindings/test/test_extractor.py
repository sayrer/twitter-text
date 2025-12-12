import os
import sys

import pytest
import twitter_text
import yaml


def test_extractor_ctor():
    extractor = twitter_text.Extractor()
    assert extractor is not None


def test_accessors():
    extractor = twitter_text.Extractor()

    assert extractor.get_extract_url_without_protocol() == True
    extractor.set_extract_url_without_protocol(False)
    assert extractor.get_extract_url_without_protocol() == False


def test_yaml():
    extractor = twitter_text.Extractor()
    with open(r"rust/conformance/tests/extract.yml") as file:
        testmap = yaml.load(file, Loader=yaml.FullLoader)

        assert len(testmap["tests"]["mentions"]) > 0
        for test in testmap["tests"]["mentions"]:
            mentions = extractor.extract_mentioned_screennames(test["text"])
            for index, s in enumerate(mentions):
                assert s == test["expected"][index]

        assert len(testmap["tests"]["mentions_with_indices"]) > 0
        for test in testmap["tests"]["mentions_with_indices"]:
            entities = extractor.extract_mentioned_screennames_with_indices(
                test["text"]
            )
            assert len(test["expected"]) == len(entities)
            for index, entity in enumerate(entities):
                assert entity.value == test["expected"][index]["screen_name"]
                assert entity.start == test["expected"][index]["indices"][0]
                assert entity.end == test["expected"][index]["indices"][1]

        assert len(testmap["tests"]["mentions_or_lists_with_indices"]) > 0
        for test in testmap["tests"]["mentions_or_lists_with_indices"]:
            entities = extractor.extract_mentions_or_lists_with_indices(test["text"])
            assert len(test["expected"]) == len(entities)
            for index, entity in enumerate(entities):
                assert entity.value == test["expected"][index]["screen_name"]
                assert entity.list_slug == test["expected"][index]["list_slug"]
                assert entity.start == test["expected"][index]["indices"][0]
                assert entity.end == test["expected"][index]["indices"][1]

        assert len(testmap["tests"]["replies"]) > 0
        for test in testmap["tests"]["replies"]:
            entity = extractor.extract_reply_screenname(test["text"])
            if entity is not None:
                assert entity.value == test["expected"]
            else:
                assert test["expected"] is None

        assert len(testmap["tests"]["urls"]) > 0
        for test in testmap["tests"]["urls"]:
            urls = extractor.extract_urls(test["text"])
            for index, s in enumerate(urls):
                assert s == test["expected"][index]

        assert len(testmap["tests"]["urls_with_indices"]) > 0
        for test in testmap["tests"]["urls_with_indices"]:
            entities = extractor.extract_urls_with_indices(test["text"])
            assert len(test["expected"]) == len(entities)
            for index, entity in enumerate(entities):
                assert entity.value == test["expected"][index]["url"]
                assert entity.start == test["expected"][index]["indices"][0]
                assert entity.end == test["expected"][index]["indices"][1]

        assert len(testmap["tests"]["urls_with_directional_markers"]) > 0
        for test in testmap["tests"]["urls_with_directional_markers"]:
            entities = extractor.extract_urls_with_indices(test["text"])
            assert len(test["expected"]) == len(entities)
            for index, entity in enumerate(entities):
                assert entity.value == test["expected"][index]["url"]
                assert entity.start == test["expected"][index]["indices"][0]
                assert entity.end == test["expected"][index]["indices"][1]

        assert len(testmap["tests"]["tco_urls_with_params"]) > 0
        for test in testmap["tests"]["tco_urls_with_params"]:
            entities = extractor.extract_urls_with_indices(test["text"])
            assert len(test["expected"]) == len(entities)
            for index, entity in enumerate(entities):
                assert entity.value == test["expected"][index]

        assert len(testmap["tests"]["hashtags"]) > 0
        for test in testmap["tests"]["hashtags"]:
            hashtags = extractor.extract_hashtags(test["text"])
            assert len(test["expected"]) == len(hashtags)
            for index, hashtag in enumerate(hashtags):
                assert hashtag == test["expected"][index]

        assert len(testmap["tests"]["hashtags_from_astral"]) > 0
        for test in testmap["tests"]["hashtags_from_astral"]:
            hashtags = extractor.extract_hashtags(test["text"])
            assert len(test["expected"]) == len(hashtags)
            for index, hashtag in enumerate(hashtags):
                assert hashtag == test["expected"][index]

        assert len(testmap["tests"]["hashtags_with_indices"]) > 0
        for test in testmap["tests"]["hashtags_with_indices"]:
            entities = extractor.extract_hashtags_with_indices(test["text"])
            assert len(test["expected"]) == len(entities)
            for index, entity in enumerate(entities):
                assert entity.value == test["expected"][index]["hashtag"]
                assert entity.start == test["expected"][index]["indices"][0]
                assert entity.end == test["expected"][index]["indices"][1]

        assert len(testmap["tests"]["cashtags"]) > 0
        for test in testmap["tests"]["cashtags"]:
            cashtags = extractor.extract_cashtags(test["text"])
            assert len(test["expected"]) == len(cashtags)
            for index, cashtag in enumerate(cashtags):
                assert cashtag == test["expected"][index]

        assert len(testmap["tests"]["cashtags_with_indices"]) > 0
        for test in testmap["tests"]["cashtags_with_indices"]:
            entities = extractor.extract_cashtags_with_indices(test["text"])
            assert len(test["expected"]) == len(entities)
            for index, entity in enumerate(entities):
                assert entity.value == test["expected"][index]["cashtag"]
                assert entity.start == test["expected"][index]["indices"][0]
                assert entity.end == test["expected"][index]["indices"][1]


def test_validating_extractor_ctor():
    extractor = twitter_text.ValidatingExtractor(
        twitter_text.TwitterTextConfiguration()
    )
    assert extractor is not None


def test_validating_extractor_accessors():
    extractor = twitter_text.ValidatingExtractor(
        twitter_text.TwitterTextConfiguration()
    )

    assert extractor.get_extract_url_without_protocol() == True
    extractor.set_extract_url_without_protocol(False)
    assert extractor.get_extract_url_without_protocol() == False

    assert extractor.get_normalize() == True
    extractor.set_normalize(False)
    assert extractor.get_normalize() == False


def test_validating_extractor_yaml():
    extractor = twitter_text.ValidatingExtractor(
        twitter_text.TwitterTextConfiguration()
    )
    with open(r"rust/conformance/tests/extract.yml") as file:
        testmap = yaml.load(file, Loader=yaml.FullLoader)

        assert len(testmap["tests"]["mentions_with_indices"]) > 0
        for test in testmap["tests"]["mentions_with_indices"]:
            result = extractor.extract_mentioned_screennames_with_indices(test["text"])
            assert len(test["expected"]) == len(result.entities)
            for index, entity in enumerate(result.entities):
                assert entity.value == test["expected"][index]["screen_name"]
                assert entity.start == test["expected"][index]["indices"][0]
                assert entity.end == test["expected"][index]["indices"][1]

        assert len(testmap["tests"]["mentions_or_lists_with_indices"]) > 0
        for test in testmap["tests"]["mentions_or_lists_with_indices"]:
            result = extractor.extract_mentions_or_lists_with_indices(test["text"])
            assert len(test["expected"]) == len(result.entities)
            for index, entity in enumerate(result.entities):
                assert entity.value == test["expected"][index]["screen_name"]
                assert entity.list_slug == test["expected"][index]["list_slug"]
                assert entity.start == test["expected"][index]["indices"][0]
                assert entity.end == test["expected"][index]["indices"][1]

        assert len(testmap["tests"]["replies"]) > 0
        for test in testmap["tests"]["replies"]:
            result = extractor.extract_reply_screenname(test["text"])
            if result.entity is not None:
                assert result.entity.value == test["expected"]
            else:
                assert test["expected"] is None

        assert len(testmap["tests"]["urls_with_indices"]) > 0
        for test in testmap["tests"]["urls_with_indices"]:
            result = extractor.extract_urls_with_indices(test["text"])
            assert len(test["expected"]) == len(result.entities)
            for index, entity in enumerate(result.entities):
                assert entity.value == test["expected"][index]["url"]
                assert entity.start == test["expected"][index]["indices"][0]
                assert entity.end == test["expected"][index]["indices"][1]

        assert len(testmap["tests"]["urls_with_directional_markers"]) > 0
        for test in testmap["tests"]["urls_with_directional_markers"]:
            result = extractor.extract_urls_with_indices(test["text"])
            assert len(test["expected"]) == len(result.entities)
            for index, entity in enumerate(result.entities):
                assert entity.value == test["expected"][index]["url"]
                assert entity.start == test["expected"][index]["indices"][0]
                assert entity.end == test["expected"][index]["indices"][1]

        assert len(testmap["tests"]["hashtags_with_indices"]) > 0
        for test in testmap["tests"]["hashtags_with_indices"]:
            result = extractor.extract_hashtags_with_indices(test["text"])
            assert len(test["expected"]) == len(result.entities)
            for index, entity in enumerate(result.entities):
                assert entity.value == test["expected"][index]["hashtag"]
                assert entity.start == test["expected"][index]["indices"][0]
                assert entity.end == test["expected"][index]["indices"][1]

        assert len(testmap["tests"]["cashtags_with_indices"]) > 0
        for test in testmap["tests"]["cashtags_with_indices"]:
            result = extractor.extract_cashtags_with_indices(test["text"])
            assert len(test["expected"]) == len(result.entities)
            for index, entity in enumerate(result.entities):
                assert entity.value == test["expected"][index]["cashtag"]
                assert entity.start == test["expected"][index]["indices"][0]
                assert entity.end == test["expected"][index]["indices"][1]


def test_federated_mentions():
    extractor = twitter_text.Extractor()

    # Simple federated mention
    mentions = extractor.extract_federated_mentions("Hello @user@mastodon.social!")
    assert len(mentions) == 1
    assert mentions[0] == "@user@mastodon.social"

    # With indices
    entities = extractor.extract_federated_mentions_with_indices(
        "Hello @user@mastodon.social!"
    )
    assert len(entities) == 1
    assert entities[0].value == "@user@mastodon.social"
    assert entities[0].start == 6
    assert entities[0].end == 27
    assert entities[0].entity_type == 4  # FEDERATEDMENTION

    # Multiple federated mentions
    mentions = extractor.extract_federated_mentions(
        "@alice@example.com and @bob@other.org"
    )
    assert len(mentions) == 2
    assert mentions[0] == "@alice@example.com"
    assert mentions[1] == "@bob@other.org"

    # Mixed with regular mentions (extract_federated_mentions includes both)
    mentions = extractor.extract_federated_mentions(
        "@regular and @federated@domain.com"
    )
    assert len(mentions) == 2
    assert mentions[0] == "regular"  # Regular mentions don't include @ prefix
    assert mentions[1] == "@federated@domain.com"


def test_entities_with_indices_federated():
    extractor = twitter_text.Extractor()

    # Test that extract_entities_with_indices_federated includes federated mentions
    entities = extractor.extract_entities_with_indices_federated(
        "Check @user@mastodon.social and https://example.com #hashtag $CASH"
    )
    assert len(entities) == 4

    # Find federated mention
    federated = next((e for e in entities if e.entity_type == 4), None)
    assert federated is not None
    assert federated.value == "@user@mastodon.social"

    # Regular extract_entities_with_indices should NOT include federated mentions
    regular_entities = extractor.extract_entities_with_indices(
        "Check @user@mastodon.social and https://example.com #hashtag $CASH"
    )
    federated_in_regular = next(
        (e for e in regular_entities if e.entity_type == 4), None
    )
    assert federated_in_regular is None


def test_validating_extractor_federated_mentions():
    extractor = twitter_text.ValidatingExtractor(
        twitter_text.TwitterTextConfiguration()
    )

    # Simple federated mention
    mentions = extractor.extract_federated_mentions("Hello @user@mastodon.social!")
    assert len(mentions) == 1
    assert mentions[0] == "@user@mastodon.social"

    # With indices
    result = extractor.extract_federated_mentions_with_indices(
        "Hello @user@mastodon.social!"
    )
    assert len(result.entities) == 1
    assert result.entities[0].value == "@user@mastodon.social"
    assert result.entities[0].entity_type == 4  # FEDERATEDMENTION


def test_validating_extractor_entities_with_indices_federated():
    extractor = twitter_text.ValidatingExtractor(
        twitter_text.TwitterTextConfiguration()
    )

    # Test that extract_entities_with_indices_federated includes federated mentions
    result = extractor.extract_entities_with_indices_federated(
        "Check @user@mastodon.social and https://example.com #hashtag"
    )

    # Find federated mention
    federated = next((e for e in result.entities if e.entity_type == 4), None)
    assert federated is not None
    assert federated.value == "@user@mastodon.social"


if __name__ == "__main__":
    raise SystemExit(pytest.main([__file__]))
