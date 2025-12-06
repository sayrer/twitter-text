import os
import pytest
import sys
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
    with open (r"rust/conformance/tests/extract.yml") as file:
        testmap = yaml.load(file, Loader=yaml.FullLoader)

        assert len(testmap["tests"]["mentions"]) > 0
        for test in testmap["tests"]["mentions"]:
            mentions = extractor.extract_mentioned_screennames(test["text"])
            for index, s in enumerate(mentions):
                assert s == test["expected"][index]
        
        assert len(testmap["tests"]["mentions_with_indices"]) > 0
        for test in testmap["tests"]["mentions_with_indices"]:
            entities = extractor.extract_mentioned_screennames_with_indices(test["text"])
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
    extractor = twitter_text.ValidatingExtractor(twitter_text.TwitterTextConfiguration())
    assert extractor is not None

def test_validating_extractor_accessors():
    extractor = twitter_text.ValidatingExtractor(twitter_text.TwitterTextConfiguration())

    assert extractor.get_extract_url_without_protocol() == True
    extractor.set_extract_url_without_protocol(False)
    assert extractor.get_extract_url_without_protocol() == False

    assert extractor.get_normalize() == True
    extractor.set_normalize(False)
    assert extractor.get_normalize() == False

def test_validating_extractor_yaml():
    extractor = twitter_text.ValidatingExtractor(twitter_text.TwitterTextConfiguration())
    with open (r"rust/conformance/tests/extract.yml") as file:
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

if __name__ == "__main__":
    raise SystemExit(pytest.main([__file__]))
