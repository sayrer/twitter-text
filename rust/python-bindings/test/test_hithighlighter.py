import os
import pytest
import sys
import twitter_text
import yaml

def test_ctor():
    hit_highlighter = twitter_text.HitHighlighter()
    assert hit_highlighter is not None

def test_ctor_with_string():
    hit_highlighter = twitter_text.HitHighlighter("hmm")
    assert hit_highlighter is not None

def array_to_hits(arr):
    hits = twitter_text.Hits()
    for hit in arr:
        h = twitter_text.Hit()
        h.start = hit[0]
        h.end = hit[1]
        hits.append(h)

    return hits

def test_yaml():
    hit_highlighter = twitter_text.HitHighlighter()
    with open (r"rust/conformance/tests/hit_highlighting.yml") as file:
        testmap = yaml.load(file, Loader=yaml.FullLoader)

        assert len(testmap["tests"]["plain_text"]) > 0
        for test in testmap["tests"]["plain_text"]:
            assert test["expected"] == hit_highlighter.highlight(test["text"], array_to_hits(test["hits"]))

        assert len(testmap["tests"]["with_links"]) > 0
        for test in testmap["tests"]["with_links"]:
            assert test["expected"] == hit_highlighter.highlight(test["text"], array_to_hits(test["hits"]))

if __name__ == "__main__":
    raise SystemExit(pytest.main([__file__]))
