package app.grafica.twittertext;

import static org.junit.Assert.*;
import org.junit.Test;

import java.io.IOException;
import java.util.List;
import java.util.Map;

public class TestValidator {
  static {
    TwitterTextJNILib.load();
  }

  @Test
  public void testConstructor() {
    Validator validator = new Validator();
    assertNotNull(validator);
  }

  @Test
  public void testAccessors() {
    Validator validator = new Validator();
    assertNotNull(validator);

    assertEquals(validator.getMaxTweetLength(), 280);
    assertEquals(validator.getShortUrlLength(), 23);
    validator.setShortUrlLength(42);
    assertEquals(validator.getShortUrlLength(), 42);

    assertEquals(validator.getShortUrlLengthHttps(), 23);
    validator.setShortUrlLengthHttps(42);
    assertEquals(validator.getShortUrlLengthHttps(), 42);
  }

  @Test
  public void testYaml() throws IOException {
    Validator validator = new Validator();
    assertNotNull(validator);

    String path = "rust/conformance/tests/validate.yml"; 
    List<Map> tests = Yaml.loadConformanceData(path, "tweets");
    assertNotNull(tests);
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), validator.isValidTweet(test.get("text").toString()));
    }

    tests = Yaml.loadConformanceData(path, "usernames");
    assertNotNull(tests);
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), validator.isValidUsername(test.get("text").toString()));
    }

    tests = Yaml.loadConformanceData(path, "lists");
    assertNotNull(tests);
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), validator.isValidList(test.get("text").toString()));
    }

    tests = Yaml.loadConformanceData(path, "hashtags");
    assertNotNull(tests);
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), validator.isValidHashtag(test.get("text").toString()));
    }

    tests = Yaml.loadConformanceData(path, "urls");
    assertNotNull(tests);
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), validator.isValidUrl(test.get("text").toString()));
    }

    tests = Yaml.loadConformanceData(path, "urls_without_protocol");
    assertNotNull(tests);
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), validator.isValidUrlWithoutProtocol(test.get("text").toString()));
    }
  }

  public void validateWeighting(List<Map> tests, TwitterTextConfiguration config) {
    assert(tests.size() > 0);
    for (Map test : tests) {
      TwitterTextParseResults result = TwitterTextParser.parse(test.get("text").toString(), config, true);
      Map<String, Object> expected = (Map<String, Object>) test.get("expected");
      assertEquals(expected.get("weightedLength"), result.getWeightedLength());
      assertEquals(expected.get("valid"), result.getIsValid());
      assertEquals(expected.get("permillage"), result.getPermillage());
      assertEquals(expected.get("displayRangeStart"), result.getDisplayTextRange().getStart());
      assertEquals(expected.get("displayRangeEnd"), result.getDisplayTextRange().getEnd());
      assertEquals(expected.get("validRangeStart"), result.getValidTextRange().getStart());
      assertEquals(expected.get("validRangeEnd"), result.getValidTextRange().getEnd());
    }
  }

  @Test
  public void testWeighted() throws IOException {
    Validator validator = new Validator();
    assertNotNull(validator);

    String path = "rust/conformance/tests/validate.yml"; 
    List<Map> tests = Yaml.loadConformanceData(path, "WeightedTweetsCounterTest");
    validateWeighting(tests, TwitterTextConfiguration.configV2());
    
    tests = Yaml.loadConformanceData(path, "WeightedTweetsWithDiscountedEmojiCounterTest");
    validateWeighting(tests, TwitterTextConfiguration.configV3());

    tests = Yaml.loadConformanceData(path, "UnicodeDirectionalMarkerCounterTest");
    validateWeighting(tests, TwitterTextConfiguration.configV3());
  }  
}