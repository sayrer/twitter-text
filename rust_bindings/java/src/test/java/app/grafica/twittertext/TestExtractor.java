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
/*
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
      List<Entity> list = extractor.extractMentionedScreennamesWithIndices(test.get("text").toString());
      int i = 0;
      for (Iterator<Entity> it = list.iterator(); it.hasNext(); i++) {

        assertEquals()
              ASSERT_EQ(std::string(entity.value), test.expected[index].screen_name);

      }
    }
  } */
}