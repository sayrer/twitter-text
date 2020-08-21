package app.grafica.twittertext;

import com.fasterxml.jackson.dataformat.yaml.YAMLMapper;

import java.io.BufferedReader;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.Reader;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;

public class Yaml {
  static List<Entity> getExpectedEntities(Map testCase, String key, String listSlugKey) {
    @SuppressWarnings("unchecked") final List<Map<String, Object>> expectedConfig =
        (List<Map<String, Object>>) testCase.get("expected");
    final List<Entity> expected = new LinkedList<>();
    for (Map<String, Object> configEntry : expectedConfig) {
      @SuppressWarnings("unchecked") final List<Integer> indices =
          (List<Integer>) configEntry.get("indices");
      final String listSlug = listSlugKey != null ? configEntry.get(listSlugKey).toString() : "";
      Entity e = new Entity();
      e.setStart(indices.get(0));
      e.setEnd(indices.get(1));
      e.setValue(configEntry.get(key).toString());
      e.setListSlug(listSlug.isEmpty() ? null : listSlug);
      expected.add(e);
    }
    return expected;
  }

  @SuppressWarnings("unchecked")
  static List<Map> loadConformanceData(String yamlFile, String testType) throws IOException {
    final InputStream stream = new FileInputStream(yamlFile);
    final Reader reader = new BufferedReader(new InputStreamReader(stream));
    final Map fullConfig = new YAMLMapper().readValue(reader, Map.class);
    final Map testConfig = (Map) fullConfig.get("tests");
    return (List<Map>) testConfig.get(testType);
  }
}