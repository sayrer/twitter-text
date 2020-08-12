package app.grafica.twittertext;

import com.fasterxml.jackson.dataformat.yaml.YAMLMapper;

import java.io.BufferedReader;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.Reader;
import java.util.List;
import java.util.Map;

public class Yaml {

  @SuppressWarnings("unchecked")
  static List<Map> loadConformanceData(String yamlFile, String testType) throws IOException {
    final InputStream stream = new FileInputStream(yamlFile);
    final Reader reader = new BufferedReader(new InputStreamReader(stream));
    final Map fullConfig = new YAMLMapper().readValue(reader, Map.class);
    final Map testConfig = (Map) fullConfig.get("tests");
    return (List<Map>) testConfig.get(testType);
  }
}