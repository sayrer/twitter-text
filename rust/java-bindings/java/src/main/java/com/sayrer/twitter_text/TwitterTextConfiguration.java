package com.sayrer.twitter_text;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Configuration for tweet text parsing and validation.
 * Provides a high-level Java API for loading and querying configurations.
 */
public final class TwitterTextConfiguration {

    private static final ObjectMapper MAPPER = new ObjectMapper();

    private final Configuration config;
    private final int version;
    private final int maxWeightedTweetLength;
    private final int scale;
    private final int defaultWeight;
    private final int transformedURLLength;
    private final List<TwitterTextWeightedRange> ranges;

    private TwitterTextConfiguration(Configuration config, String json) {
        this.config = config;
        this.version = config.getVersion();
        this.maxWeightedTweetLength = config.getMaxWeightedTweetLength();
        this.scale = config.getScale();
        this.defaultWeight = config.getDefaultWeight();
        this.transformedURLLength = config.getTransformedUrlLength();

        // Parse ranges from JSON
        this.ranges = parseRangesFromJson(json);
    }

    /**
     * Load configuration from a JSON string or resource file.
     *
     * @param pathOrJson either a path to a resource file (e.g., "v1.json", "v2.json")
     *                   or a JSON string
     * @param isResource if true, treat pathOrJson as a resource path; if false, as JSON
     * @return a TwitterTextConfiguration instance
     */
    public static TwitterTextConfiguration configurationFromJson(String pathOrJson, boolean isResource) {
        if (pathOrJson == null || pathOrJson.isEmpty()) {
            // Default to v2 config
            return configurationFromJson("v2.json", true);
        }

        String json;
        if (isResource) {
            // Load from resources
            try {
                json = loadResourceAsString(pathOrJson);
            } catch (IOException e) {
                // If resource not found, try v2.json as fallback
                try {
                    json = loadResourceAsString("v2.json");
                } catch (IOException e2) {
                    // Last resort: use default config
                    Configuration config = Configuration.createDefault();
                    String defaultJson = "{\"version\":3,\"maxWeightedTweetLength\":280,\"scale\":100,\"defaultWeight\":200,\"transformedURLLength\":23,\"ranges\":[]}";
                    return new TwitterTextConfiguration(config, defaultJson);
                }
            }
        } else {
            json = pathOrJson;
        }

        // Parse JSON using Configuration.fromJson
        Configuration config = Configuration.fromJson(json);
        if (config == null) {
            // Fallback to default
            config = Configuration.createDefault();
        }

        return new TwitterTextConfiguration(config, json);
    }

    private static String loadResourceAsString(String resourcePath) throws IOException {
        // Try to load from root (resource_strip_prefix removes "config/")
        String fullPath = "/" + resourcePath;
        InputStream is = TwitterTextConfiguration.class.getResourceAsStream(fullPath);

        if (is == null) {
            // Try without leading slash
            is = TwitterTextConfiguration.class.getResourceAsStream(resourcePath);
        }

        if (is == null) {
            // Try from config directory
            fullPath = "/config/" + resourcePath;
            is = TwitterTextConfiguration.class.getResourceAsStream(fullPath);
        }

        if (is == null) {
            throw new IOException("Resource not found: " + resourcePath);
        }

        byte[] bytes = is.readAllBytes();
        return new String(bytes, StandardCharsets.UTF_8);
    }

    /**
     * Parse ranges from JSON using Jackson.
     */
    private static List<TwitterTextWeightedRange> parseRangesFromJson(String json) {
        try {
            JsonNode root = MAPPER.readTree(json);
            JsonNode rangesNode = root.get("ranges");

            if (rangesNode == null || !rangesNode.isArray()) {
                return Collections.emptyList();
            }

            List<TwitterTextWeightedRange> result = new ArrayList<>();
            for (JsonNode rangeNode : rangesNode) {
                int start = rangeNode.get("start").asInt();
                int end = rangeNode.get("end").asInt();
                int weight = rangeNode.get("weight").asInt();
                result.add(new TwitterTextWeightedRange(new Range(start, end), weight));
            }

            return Collections.unmodifiableList(result);
        } catch (Exception e) {
            return Collections.emptyList();
        }
    }

    public int getVersion() {
        return version;
    }

    public int getMaxWeightedTweetLength() {
        return maxWeightedTweetLength;
    }

    public int getScale() {
        return scale;
    }

    public int getDefaultWeight() {
        return defaultWeight;
    }

    public int getTransformedURLLength() {
        return transformedURLLength;
    }

    public List<TwitterTextWeightedRange> getRanges() {
        return ranges;
    }

    Configuration getConfig() {
        return config;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (!(obj instanceof TwitterTextConfiguration)) return false;
        TwitterTextConfiguration other = (TwitterTextConfiguration) obj;
        return version == other.version &&
               maxWeightedTweetLength == other.maxWeightedTweetLength &&
               scale == other.scale &&
               defaultWeight == other.defaultWeight &&
               transformedURLLength == other.transformedURLLength &&
               Objects.equals(ranges, other.ranges);
    }

    @Override
    public int hashCode() {
        return Objects.hash(version, maxWeightedTweetLength, scale, defaultWeight, transformedURLLength, ranges);
    }

    /**
     * Represents a weighted range in the configuration.
     */
    public static final class TwitterTextWeightedRange {
        private final Range range;
        private final int weight;

        public TwitterTextWeightedRange(Range range, int weight) {
            this.range = range;
            this.weight = weight;
        }

        public Range getRange() {
            return range;
        }

        public int getWeight() {
            return weight;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (!(obj instanceof TwitterTextWeightedRange)) return false;
            TwitterTextWeightedRange other = (TwitterTextWeightedRange) obj;
            return weight == other.weight && Objects.equals(range, other.range);
        }

        @Override
        public int hashCode() {
            return Objects.hash(range, weight);
        }

        @Override
        public String toString() {
            return "WeightedRange{range=" + range + ", weight=" + weight + "}";
        }
    }
}
