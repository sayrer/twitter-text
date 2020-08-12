package app.grafica.twittertext;

import static org.junit.Assert.*;
import org.junit.Test;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.Map;


public class TestAutolinker {
  static {
    TwitterTextJNILib.load();
  }

  @Test
  public void testConstructor() {
    Autolinker autolinker = new Autolinker();
    assertNotNull(autolinker);
  }

  @Test
  public void testAccessors() {
    Autolinker autolinker = new Autolinker();

    assertEquals(autolinker.getNoFollow(), false);
    autolinker.setNoFollow(true);
    assertEquals(autolinker.getNoFollow(), true);
   
    assertEquals(autolinker.getUrlClass(), "");
    autolinker.setUrlClass("foo");
    assertEquals(autolinker.getUrlClass(), "foo");

    assertEquals(autolinker.getUrlTarget(), "");
    autolinker.setUrlTarget("bar");
    assertEquals(autolinker.getUrlTarget(), "bar");

    assertEquals(autolinker.getSymbolTag(), "");
    autolinker.setSymbolTag("baz");
    assertEquals(autolinker.getSymbolTag(), "baz");

    assertEquals(autolinker.getTextWithSymbolTag(), "");
    autolinker.setTextWithSymbolTag("qux");
    assertEquals(autolinker.getTextWithSymbolTag(), "qux");

    assertEquals(autolinker.getListClass(), "tweet-url list-slug");
    autolinker.setListClass("tweet-url list-slug foo");
    assertEquals(autolinker.getListClass(), "tweet-url list-slug foo");

    assertEquals(autolinker.getUsernameClass(), "tweet-url username");
    autolinker.setUsernameClass("tweet-url username bar");
    assertEquals(autolinker.getUsernameClass(), "tweet-url username bar");

    assertEquals(autolinker.getHashtagClass(), "tweet-url hashtag");
    autolinker.setHashtagClass("tweet-url hashtag baz");
    assertEquals(autolinker.getHashtagClass(), "tweet-url hashtag baz");

    assertEquals(autolinker.getCashtagClass(), "tweet-url cashtag");
    autolinker.setCashtagClass("tweet-url cashtag qux");
    assertEquals(autolinker.getCashtagClass(), "tweet-url cashtag qux");

    assertEquals(autolinker.getUsernameUrlBase(), "https://twitter.com/");
    autolinker.setUsernameUrlBase("https://example.com/");
    assertEquals(autolinker.getUsernameUrlBase(), "https://example.com/");

    assertEquals(autolinker.getListUrlBase(), "https://twitter.com/");
    autolinker.setListUrlBase("https://example.com/");
    assertEquals(autolinker.getListUrlBase(), "https://example.com/");

    assertEquals(autolinker.getHashtagUrlBase(), "https://twitter.com/search?q=%23");
    autolinker.setHashtagUrlBase("https://example.com/search?q=%23");
    assertEquals(autolinker.getHashtagUrlBase(), "https://example.com/search?q=%23");

    assertEquals(autolinker.getCashtagUrlBase(), "https://twitter.com/search?q=%24");
    autolinker.setCashtagUrlBase("https://example.com/search?q=%24");
    assertEquals(autolinker.getCashtagUrlBase(), "https://example.com/search?q=%24");

    assertEquals(autolinker.getInvisibleTagAttrs(), "style='position:absolute;left:-9999px;'");
    autolinker.setInvisibleTagAttrs("");
    assertEquals(autolinker.getInvisibleTagAttrs(), "");

    assertEquals(autolinker.getUsernameIncludeSymbol(), false);
    autolinker.setUsernameIncludeSymbol(true);
    assertEquals(autolinker.getUsernameIncludeSymbol(), true);
  }

  // This tests whether the Java to CPP/Rust string conversion works.
  //
  // If it doesn't, cxx will crash like this:
  //
  // - terminating with uncaught exception of type 
  // std::invalid_argument: data for rust::String is not utf-8
  //
  @Test
  public void testEmoji() {
    Autolinker autolinker = new Autolinker();

    assertEquals(autolinker.getUrlClass(), "");
    autolinker.setUrlClass("foo 👳🏿‍♀️");
    assertEquals(autolinker.getUrlClass(), "foo 👳🏿‍♀️");
  }

  @Test
  public void testYaml() throws IOException {
    Autolinker autolinker = new Autolinker();
    String path = "rust/conformance/tests/autolink.yml"; 
    List<Map> tests = Yaml.loadConformanceData(path, "usernames");
    assertNotNull(tests);
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), autolinker.autolinkUsernamesAndLists(test.get("text").toString()));
    }

    tests = Yaml.loadConformanceData(path, "lists");
    assertNotNull(tests);
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), autolinker.autolinkUsernamesAndLists(test.get("text").toString()));
    }

    tests = Yaml.loadConformanceData(path, "hashtags");
    assertNotNull(tests);
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), autolinker.autolinkHashtags(test.get("text").toString()));
    }

    tests = Yaml.loadConformanceData(path, "urls");
    assertNotNull(tests);
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), autolinker.autolinkUrls(test.get("text").toString()));
    }

    tests = Yaml.loadConformanceData(path, "cashtags");
    assertNotNull(tests);
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), autolinker.autolinkCashtags(test.get("text").toString()));
    }

    tests = Yaml.loadConformanceData(path, "all");
    assertNotNull(tests);
    assert(tests.size() > 0);
    for (Map test : tests) {
      assertEquals(test.get("expected"), autolinker.autolink(test.get("text").toString()));
    }
  }
}