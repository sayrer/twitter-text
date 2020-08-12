package app.grafica.twittertext;

import static org.junit.Assert.*;
import org.junit.Test;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class TestConfiguration {
  static {
    TwitterTextJNILib.load();
  }

  @Test
  public void testConstructor() {
    TwitterTextConfiguration ttc = new TwitterTextConfiguration();
    assertNotNull(ttc);
  }

  @Test
  public void testPath() {
    String path = "rust_bindings/cpp/test_data/test_config.json";
    TwitterTextConfiguration ttc = TwitterTextConfiguration.configurationFromPath(path);
    assertNotNull(ttc);
    assertEquals(ttc.getVersion(), 42);
    assertEquals(ttc.getMaxWeightedTweetLength(), 400);
    assertEquals(ttc.getScale(), 43);
    assertEquals(ttc.getDefaultWeight(), 213);
    assertEquals(ttc.getTransformedUrlLength(), 32);
    WeightedRangeList wrl = ttc.getRanges();
    assertEquals(wrl.size(), 1);
    WeightedRange wr = wrl.get(0);
    assertEquals(wr.getRange().getStart(), 0);
    assertEquals(wr.getRange().getEnd(), 4351);
    assertEquals(wr.getWeight(), 200);
  }

  @Test
  public void testJson() throws IOException {
    String path = "rust_bindings/cpp/test_data/test_config.json";
    byte[] content = Files.readAllBytes(Paths.get(path));
    TwitterTextConfiguration ttc = TwitterTextConfiguration.configurationFromJson(new String(content)); 
    assertNotNull(ttc);
    assertEquals(ttc.getVersion(), 42);
    assertEquals(ttc.getMaxWeightedTweetLength(), 400);
    assertEquals(ttc.getScale(), 43);
    assertEquals(ttc.getDefaultWeight(), 213);
    assertEquals(ttc.getTransformedUrlLength(), 32);
    WeightedRangeList wrl = ttc.getRanges();
    assertEquals(wrl.size(), 1);
    WeightedRange wr = wrl.get(0);
    assertEquals(wr.getRange().getStart(), 0);
    assertEquals(wr.getRange().getEnd(), 4351);
    assertEquals(wr.getWeight(), 200);
  }

  @Test
  public void testVersion() {
    TwitterTextConfiguration ttc = new TwitterTextConfiguration();
    assertEquals(ttc.getVersion(), 3);
    ttc.setVersion(199);
    assertEquals(ttc.getVersion(), 199);
  }

  @Test
  public void testMaxWeightedTweetLength() {
    TwitterTextConfiguration ttc = new TwitterTextConfiguration();
    assertEquals(ttc.getMaxWeightedTweetLength(), 280);
    ttc.setMaxWeightedTweetLength(199);
    assertEquals(ttc.getMaxWeightedTweetLength(), 199);
  }

  @Test
  public void testScale() {
    TwitterTextConfiguration ttc = new TwitterTextConfiguration();
    assertEquals(ttc.getScale(), 100);
    ttc.setScale(199);
    assertEquals(ttc.getScale(), 199);
  }

  @Test
  public void testDefaultWeight() {
    TwitterTextConfiguration ttc = new TwitterTextConfiguration();
    assertEquals(ttc.getDefaultWeight(), 200);
    ttc.setDefaultWeight(199);
    assertEquals(ttc.getDefaultWeight(), 199);
  }

  @Test
  public void testTransformedUrlLength() {
    TwitterTextConfiguration ttc = new TwitterTextConfiguration();
    assertEquals(ttc.getTransformedUrlLength(), 23);
    ttc.setTransformedUrlLength(199);
    assertEquals(ttc.getTransformedUrlLength(), 199);
  }

  @Test
  public void testEmojiParsingEnabled() {
    TwitterTextConfiguration ttc = new TwitterTextConfiguration();
    assertEquals(ttc.getEmojiParsingEnabled(), true);
    ttc.setEmojiParsingEnabled(false);
    assertEquals(ttc.getEmojiParsingEnabled(), false);
  }

  @Test
  public void testRanges() {
    TwitterTextConfiguration ttc = new TwitterTextConfiguration();
    WeightedRangeList wrl = ttc.getRanges();
    assertEquals(wrl.size(), 4);
    WeightedRange wr = wrl.get(0);
    assertEquals(wr.getRange().getStart(), 0);
    assertEquals(wr.getRange().getEnd(), 4351);
    assertEquals(wr.getWeight(), 100);
    wr = wrl.get(1);
    assertEquals(wr.getRange().getStart(), 8192);
    assertEquals(wr.getRange().getEnd(), 8205);
    assertEquals(wr.getWeight(), 100);
    wr = wrl.get(2);
    assertEquals(wr.getRange().getStart(), 8208);
    assertEquals(wr.getRange().getEnd(), 8223);
    assertEquals(wr.getWeight(), 100);
    wr = wrl.get(3);
    assertEquals(wr.getRange().getStart(), 8242);
    assertEquals(wr.getRange().getEnd(), 8247);
    assertEquals(wr.getWeight(), 100);
  }

  @Test
  public void testConfigV2() {
    TwitterTextConfiguration ttc = TwitterTextConfiguration.configV2();
    assertEquals(ttc.getVersion(), 2);
    assertEquals(ttc.getEmojiParsingEnabled(), false);
    WeightedRangeList wrl = ttc.getRanges();
    assertEquals(wrl.size(), 4);
  }

  @Test
  public void testConfigV1() {
    TwitterTextConfiguration ttc = TwitterTextConfiguration.configV1();
    assertEquals(ttc.getVersion(), 1);
    assertEquals(ttc.getEmojiParsingEnabled(), false);
    WeightedRangeList wrl = ttc.getRanges();
    assertEquals(wrl.size(), 0);
  }  
}