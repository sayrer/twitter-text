package app.grafica.twittertext;

import static org.junit.Assert.*;
import org.junit.Test;

import java.io.IOException;
import java.util.List;
import java.util.Map;

public class TestExtractor {
  static {
    TwitterTextJNILib.load();
  }

  @Test
  public void testConstructor() {
    Extractor extractor = new Extractor();
    assertNotNull(extractor);
  }

  @Test
  public void testAccessors() {
    Extractor extractor = new Extractor();
    assertNotNull(extractor);

    assertEquals(extractor.getExtractUrlWithoutProtocol(), true);
    extractor.setExtractUrlWithoutProtocol(false);
    assertEquals(extractor.getExtractUrlWithoutProtocol(), false);
  }

  @Test
  public void testYaml() throws IOException {
    Extractor extractor = new Extractor();
    assertNotNull(extractor);

    String path = "rust/conformance/tests/extract.yml"; 
    List<Map> tests = Yaml.loadConformanceData(path, "mentions");
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), extractor.extractMentionedScreennames(test.get("text").toString()));
    }

    tests = Yaml.loadConformanceData(path, "mentions_with_indices");
    assert(tests.size() > 0);
    for (Map test : tests) {
      List<Entity> actual = extractor.extractMentionedScreennamesWithIndices(test.get("text").toString());
      List<Entity> expected = Yaml.getExpectedEntities(test, "screen_name", null);
      assert(actual.size() > 0);
      assertEquals(actual.size(), expected.size());
      for (int i = 0; i < actual.size(); i++) {
        assertEquals(expected.get(i).getStart(), actual.get(i).getStart());
        assertEquals(expected.get(i).getEnd(), actual.get(i).getEnd());
        assertEquals(expected.get(i).getValue(), actual.get(i).getValue());
        assertEquals(expected.get(i).getListSlug(), actual.get(i).getListSlug());
      }
    }

    tests = Yaml.loadConformanceData(path, "mentions_or_lists_with_indices");
    assert(tests.size() > 0);
    for (Map test : tests) {
      List<Entity> actual = extractor.extractMentionsOrListsWithIndices(test.get("text").toString());
      List<Entity> expected = Yaml.getExpectedEntities(test, "screen_name", "list_slug");
      assert(actual.size() > 0);
      assertEquals(actual.size(), expected.size());
      for (int i = 0; i < actual.size(); i++) {
        assertEquals(expected.get(i).getStart(), actual.get(i).getStart());
        assertEquals(expected.get(i).getEnd(), actual.get(i).getEnd());
        assertEquals(expected.get(i).getValue(), actual.get(i).getValue());
        assertEquals(expected.get(i).getListSlug(), actual.get(i).getListSlug());
      }
    }
    
    tests = Yaml.loadConformanceData(path, "replies");
    assert(tests.size() > 0);
    for (Map test : tests) {
      Entity e = extractor.extractReplyScreenname(test.get("text").toString());
      assertEquals(test.get("expected"), e != null ? e.getValue() : null);
    }

    tests = Yaml.loadConformanceData(path, "urls");
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), extractor.extractUrls(test.get("text").toString()));
    }

    tests = Yaml.loadConformanceData(path, "urls_with_indices");
    assert(tests.size() > 0);
    for (Map test : tests) {
      List<Entity> actual = extractor.extractUrlsWithIndices(test.get("text").toString());
      List<Entity> expected = Yaml.getExpectedEntities(test, "url", null);
      assert(actual.size() > 0);
      assertEquals(actual.size(), expected.size());
      for (int i = 0; i < actual.size(); i++) {
        assertEquals(expected.get(i).getStart(), actual.get(i).getStart());
        assertEquals(expected.get(i).getEnd(), actual.get(i).getEnd());
        assertEquals(expected.get(i).getValue(), actual.get(i).getValue());
        assertEquals(expected.get(i).getListSlug(), actual.get(i).getListSlug());
      }
    }

    tests = Yaml.loadConformanceData(path, "urls_with_directional_markers");
    assert(tests.size() > 0);
    for (Map test : tests) {
      List<Entity> actual = extractor.extractUrlsWithIndices(test.get("text").toString());
      List<Entity> expected = Yaml.getExpectedEntities(test, "url", null);
      assert(actual.size() > 0);
      assertEquals(actual.size(), expected.size());
      for (int i = 0; i < actual.size(); i++) {
        assertEquals(expected.get(i).getStart(), actual.get(i).getStart());
        assertEquals(expected.get(i).getEnd(), actual.get(i).getEnd());
        assertEquals(expected.get(i).getValue(), actual.get(i).getValue());
        assertEquals(expected.get(i).getListSlug(), actual.get(i).getListSlug());
      }
    }

    tests = Yaml.loadConformanceData(path, "tco_urls_with_params");
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), extractor.extractUrls(test.get("text").toString()));
    }

    tests = Yaml.loadConformanceData(path, "hashtags");
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), extractor.extractHashtags(test.get("text").toString()));
    }

    tests = Yaml.loadConformanceData(path, "hashtags_from_astral");
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), extractor.extractHashtags(test.get("text").toString()));
    }

    tests = Yaml.loadConformanceData(path, "hashtags_with_indices");
    assert(tests.size() > 0);
    for (Map test : tests) {
      List<Entity> actual = extractor.extractHashtagsWithIndices(test.get("text").toString());
      List<Entity> expected = Yaml.getExpectedEntities(test, "hashtag", null);
      assert(actual.size() > 0);
      assertEquals(actual.size(), expected.size());
      for (int i = 0; i < actual.size(); i++) {
        assertEquals(expected.get(i).getStart(), actual.get(i).getStart());
        assertEquals(expected.get(i).getEnd(), actual.get(i).getEnd());
        assertEquals(expected.get(i).getValue(), actual.get(i).getValue());
        assertEquals(expected.get(i).getListSlug(), actual.get(i).getListSlug());
      }
    }

    tests = Yaml.loadConformanceData(path, "cashtags");
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), extractor.extractCashtags(test.get("text").toString()));
    }

    tests = Yaml.loadConformanceData(path, "cashtags_with_indices");
    assert(tests.size() > 0);
    for (Map test : tests) {
      List<Entity> actual = extractor.extractCashtagsWithIndices(test.get("text").toString());
      List<Entity> expected = Yaml.getExpectedEntities(test, "cashtag", null);
      assert(actual.size() > 0);
      assertEquals(actual.size(), expected.size());
      for (int i = 0; i < actual.size(); i++) {
        assertEquals(expected.get(i).getStart(), actual.get(i).getStart());
        assertEquals(expected.get(i).getEnd(), actual.get(i).getEnd());
        assertEquals(expected.get(i).getValue(), actual.get(i).getValue());
        assertEquals(expected.get(i).getListSlug(), actual.get(i).getListSlug());
      }
    }
  }

  @Test
  public void tesValidatingConstructor() {
    ValidatingExtractor extractor = new ValidatingExtractor(TwitterTextConfiguration.configV3());
    assertNotNull(extractor);
  }


  @Test
  public void testValidatingAccessors() {
    ValidatingExtractor extractor = new ValidatingExtractor(TwitterTextConfiguration.configV3());
    assertNotNull(extractor);

    assertEquals(extractor.getNormalize(), true);
    extractor.setNormalize(false);
    assertEquals(extractor.getNormalize(), false);

    assertEquals(extractor.getExtractUrlWithoutProtocol(), true);
    extractor.setExtractUrlWithoutProtocol(false);
    assertEquals(extractor.getExtractUrlWithoutProtocol(), false);
  }
  @Test
  public void testValidatingYaml() throws IOException {
    ValidatingExtractor extractor = new ValidatingExtractor(new TwitterTextConfiguration());
    assertNotNull(extractor);

    String path = "rust/conformance/tests/extract.yml"; 
    List<Map> tests = Yaml.loadConformanceData(path, "mentions_with_indices");
    assert(tests.size() > 0);
    for (Map test : tests) {
      ExtractResult er = extractor.extractMentionedScreennamesWithIndices(test.get("text").toString());
      assert(er.getParseResults().getIsValid());
      List<Entity> actual = er.getEntities();
      List<Entity> expected = Yaml.getExpectedEntities(test, "screen_name", null);
      assert(actual.size() > 0);
      assertEquals(actual.size(), expected.size());
      for (int i = 0; i < actual.size(); i++) {
        assertEquals(expected.get(i).getStart(), actual.get(i).getStart());
        assertEquals(expected.get(i).getEnd(), actual.get(i).getEnd());
        assertEquals(expected.get(i).getValue(), actual.get(i).getValue());
        assertEquals(expected.get(i).getListSlug(), actual.get(i).getListSlug());
      }
    }
  
    tests = Yaml.loadConformanceData(path, "mentions_or_lists_with_indices");
    assert(tests.size() > 0);
    for (Map test : tests) {
      ExtractResult er = extractor.extractMentionsOrListsWithIndices(test.get("text").toString());
      assert(er.getParseResults().getIsValid());
      List<Entity> actual = er.getEntities();
      List<Entity> expected = Yaml.getExpectedEntities(test, "screen_name", "list_slug");
      assert(actual.size() > 0);
      assertEquals(actual.size(), expected.size());
      for (int i = 0; i < actual.size(); i++) {
        assertEquals(expected.get(i).getStart(), actual.get(i).getStart());
        assertEquals(expected.get(i).getEnd(), actual.get(i).getEnd());
        assertEquals(expected.get(i).getValue(), actual.get(i).getValue());
        assertEquals(expected.get(i).getListSlug(), actual.get(i).getListSlug());
      }
    }

    tests = Yaml.loadConformanceData(path, "replies");
    assert(tests.size() > 0);
    for (Map test : tests) {
      MentionResult mr = extractor.extractReplyScreenname(test.get("text").toString());
      Entity e = mr.getEntity();
      assertEquals(test.get("expected"), e != null ? e.getValue() : null);
    }

    tests = Yaml.loadConformanceData(path, "urls_with_indices");
    assert(tests.size() > 0);
    for (Map test : tests) {
      ExtractResult er = extractor.extractUrlsWithIndices(test.get("text").toString());
      assert(er.getParseResults().getIsValid());
      List<Entity> actual = er.getEntities();
      List<Entity> expected = Yaml.getExpectedEntities(test, "url", null);
      assert(actual.size() > 0);
      assertEquals(actual.size(), expected.size());
      for (int i = 0; i < actual.size(); i++) {
        assertEquals(expected.get(i).getStart(), actual.get(i).getStart());
        assertEquals(expected.get(i).getEnd(), actual.get(i).getEnd());
        assertEquals(expected.get(i).getValue(), actual.get(i).getValue());
        assertEquals(expected.get(i).getListSlug(), actual.get(i).getListSlug());
      }
    }

    tests = Yaml.loadConformanceData(path, "urls_with_directional_markers");
    assert(tests.size() > 0);
    for (Map test : tests) {
      ExtractResult er = extractor.extractUrlsWithIndices(test.get("text").toString());
      assert(er.getParseResults().getIsValid());
      List<Entity> actual = er.getEntities();
      List<Entity> expected = Yaml.getExpectedEntities(test, "url", null);
      assert(actual.size() > 0);
      assertEquals(actual.size(), expected.size());
      for (int i = 0; i < actual.size(); i++) {
        assertEquals(expected.get(i).getStart(), actual.get(i).getStart());
        assertEquals(expected.get(i).getEnd(), actual.get(i).getEnd());
        assertEquals(expected.get(i).getValue(), actual.get(i).getValue());
        assertEquals(expected.get(i).getListSlug(), actual.get(i).getListSlug());
      }
    }

    tests = Yaml.loadConformanceData(path, "hashtags_with_indices");
    assert(tests.size() > 0);
    for (Map test : tests) {
      ExtractResult er = extractor.extractHashtagsWithIndices(test.get("text").toString());
      assert(er.getParseResults().getIsValid());
      List<Entity> actual = er.getEntities();
      List<Entity> expected = Yaml.getExpectedEntities(test, "hashtag", null);
      assert(actual.size() > 0);
      assertEquals(actual.size(), expected.size());
      for (int i = 0; i < actual.size(); i++) {
        assertEquals(expected.get(i).getStart(), actual.get(i).getStart());
        assertEquals(expected.get(i).getEnd(), actual.get(i).getEnd());
        assertEquals(expected.get(i).getValue(), actual.get(i).getValue());
        assertEquals(expected.get(i).getListSlug(), actual.get(i).getListSlug());
      }
    }

    tests = Yaml.loadConformanceData(path, "cashtags_with_indices");
    assert(tests.size() > 0);
    for (Map test : tests) {
      ExtractResult er = extractor.extractCashtagsWithIndices(test.get("text").toString());
      assert(er.getParseResults().getIsValid());
      List<Entity> actual = er.getEntities();
      List<Entity> expected = Yaml.getExpectedEntities(test, "cashtag", null);
      assert(actual.size() > 0);
      assertEquals(actual.size(), expected.size());
      for (int i = 0; i < actual.size(); i++) {
        assertEquals(expected.get(i).getStart(), actual.get(i).getStart());
        assertEquals(expected.get(i).getEnd(), actual.get(i).getEnd());
        assertEquals(expected.get(i).getValue(), actual.get(i).getValue());
        assertEquals(expected.get(i).getListSlug(), actual.get(i).getListSlug());
      }
    }
  }
}