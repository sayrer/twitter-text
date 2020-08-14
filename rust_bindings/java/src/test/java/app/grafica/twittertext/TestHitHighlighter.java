package app.grafica.twittertext;

import static org.junit.Assert.*;
import org.junit.Test;

import java.io.IOException;
import java.util.List;
import java.util.Map;

public class TestHitHighlighter {
  static {
    TwitterTextJNILib.load();
  }

  @Test
  public void testConstructor() {
    HitHighlighter highlighter = new HitHighlighter();
    assertNotNull(highlighter);
  }

  @Test
  public void testConstructorParam() {
    HitHighlighter highlighter = new HitHighlighter("hmm");
    assertNotNull(highlighter);
  }

  private static final String KEY_HIGHLIGHT_HITS = "hits";

  private static Hits makeHits(List<List<Integer>> ints) {
    Hits hits = new Hits();
    for (List<Integer> list : ints) {
      Hit hit = new Hit();
      hit.setStart(list.get(0));
      hit.setEnd(list.get(1));
      hits.add(hit);
    }
    return hits;
  }

  @Test
  public void testYaml() throws IOException {
    HitHighlighter highlighter = new HitHighlighter();
    String path = "rust/conformance/tests/hit_highlighting.yml"; 
    List<Map> tests = Yaml.loadConformanceData(path, "plain_text");
    assertNotNull(tests);
    assert(tests.size() > 0);
    for (Map test : tests) {
      Hits hits = makeHits((List<List<Integer>>) test.get(KEY_HIGHLIGHT_HITS));
      assertEquals(test.get("expected"), highlighter.highlight(test.get("text").toString(), hits));
    }

    tests = Yaml.loadConformanceData(path, "with_links");
    assertNotNull(tests);
    assert(tests.size() > 0);
    for (Map test : tests) {
      Hits hits = makeHits((List<List<Integer>>) test.get(KEY_HIGHLIGHT_HITS));
      assertEquals(test.get("expected"), highlighter.highlight(test.get("text").toString(), hits));
    }
  }
}