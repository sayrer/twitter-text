// twitter-text.mjs - Wrapper providing the old twttr.txt API using WASM bindings
// This allows running the original JavaScript tests against the Rust implementation

import * as wasm from "../twitter_text_wasm_nodejs/twitter_text_wasm_nodejs.js";

// HTML escape function
function htmlEscape(text) {
  if (text === undefined || text === null) {
    return text;
  }
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");
}

// Split tags helper
function splitTags(text) {
  const result = [];
  let lastIndex = 0;
  const tagRegex = /<[^>]*>/g;
  let match;

  while ((match = tagRegex.exec(text)) !== null) {
    result.push(text.substring(lastIndex, match.index));
    result.push(match[0].substring(1, match[0].length - 1)); // Remove < and >
    lastIndex = match.index + match[0].length;
  }
  result.push(text.substring(lastIndex));
  return result;
}

// Configurations matching the old API
const configs = {
  version1: wasm.TwitterTextConfiguration.configV1(),
  version2: wasm.TwitterTextConfiguration.configV2(),
  version3: wasm.TwitterTextConfiguration.configV3(),
  defaults: wasm.TwitterTextConfiguration.configV3(),
};

// Extractor instance (reused)
const extractor = new wasm.Extractor();

// Extract functions
function extractMentions(text) {
  return Array.from(extractor.extractMentionedScreennames(text));
}

function extractMentionsWithIndices(text) {
  const entities = extractor.extractMentionedScreennamesWithIndices(text);
  return Array.from(entities).map((e) => ({
    screenName: e.value,
    indices: [e.start, e.end],
  }));
}

function extractMentionsOrListsWithIndices(text) {
  const entities = extractor.extractMentionsOrListsWithIndices(text);
  return Array.from(entities).map((e) => ({
    screenName: e.value,
    listSlug: e.listSlug || "",
    indices: [e.start, e.end],
  }));
}

function extractReplies(text) {
  const entity = extractor.extractReplyScreenname(text);
  if (entity) {
    return entity.value;
  }
  return null;
}

function extractUrls(text) {
  return Array.from(extractor.extractUrls(text));
}

function extractUrlsWithIndices(text) {
  const entities = extractor.extractUrlsWithIndices(text);
  return Array.from(entities).map((e) => ({
    url: e.value,
    indices: [e.start, e.end],
  }));
}

function extractHashtags(text) {
  return Array.from(extractor.extractHashtags(text));
}

function extractHashtagsWithIndices(text) {
  const entities = extractor.extractHashtagsWithIndices(text);
  return Array.from(entities).map((e) => ({
    hashtag: e.value,
    indices: [e.start, e.end],
  }));
}

function extractCashtags(text) {
  return Array.from(extractor.extractCashtags(text));
}

function extractCashtagsWithIndices(text) {
  const entities = extractor.extractCashtagsWithIndices(text);
  return Array.from(entities).map((e) => ({
    cashtag: e.value,
    indices: [e.start, e.end],
  }));
}

function extractEntitiesWithIndices(text) {
  const entities = extractor.extractEntitiesWithIndices(text);
  return Array.from(entities).map((e) => {
    if (e.isUrl()) {
      return { url: e.value, indices: [e.start, e.end] };
    } else if (e.isMention()) {
      return {
        screenName: e.value,
        listSlug: e.listSlug || "",
        indices: [e.start, e.end],
      };
    } else if (e.isHashtag()) {
      return { hashtag: e.value, indices: [e.start, e.end] };
    } else if (e.isCashtag()) {
      return { cashtag: e.value, indices: [e.start, e.end] };
    }
    return { value: e.value, indices: [e.start, e.end] };
  });
}

// Autolink functions
function createAutolinker(options = {}) {
  const autolinker = new wasm.Autolinker();

  if (options.urlClass) {
    autolinker.urlClass = options.urlClass;
  }
  if (options.hashtagClass) {
    autolinker.hashtagClass = options.hashtagClass;
  }
  if (options.usernameClass) {
    autolinker.usernameClass = options.usernameClass;
  }
  if (options.listClass) {
    autolinker.listClass = options.listClass;
  }
  if (options.cashtagClass) {
    autolinker.cashtagClass = options.cashtagClass;
  }
  if (options.usernameUrlBase) {
    autolinker.usernameUrlBase = options.usernameUrlBase;
  }
  if (options.hashtagUrlBase) {
    autolinker.hashtagUrlBase = options.hashtagUrlBase;
  }
  if (options.cashtagUrlBase) {
    autolinker.cashtagUrlBase = options.cashtagUrlBase;
  }
  if (options.listUrlBase) {
    autolinker.listUrlBase = options.listUrlBase;
  }
  if (options.usernameIncludeSymbol !== undefined) {
    autolinker.usernameIncludeSymbol = options.usernameIncludeSymbol;
  }
  if (!options.suppressNoFollow) {
    autolinker.noFollow = true;
  }
  // Enable data-screen-name by default (like old JS) unless suppressed
  if (!options.suppressDataScreenName) {
    autolinker.includeDataScreenName = true;
  }

  return autolinker;
}

function postProcessAutolink(html, options = {}) {
  let result = html;

  // Handle htmlEscapeNonEntities
  if (options.htmlEscapeNonEntities) {
    // This is complex - we need to escape text outside of tags
    // For now, this is a simplified version
  }

  return result;
}

function autoLink(text, options = {}) {
  const autolinker = createAutolinker(options);
  let result = autolinker.autolink(text);
  autolinker.free();
  return postProcessAutolink(result, options);
}

function autoLinkHashtags(text, options = {}) {
  const autolinker = createAutolinker(options);
  let result = autolinker.autolinkHashtags(text);
  autolinker.free();
  return postProcessAutolink(result, options);
}

function autoLinkCashtags(text, options = {}) {
  const autolinker = createAutolinker(options);
  let result = autolinker.autolinkCashtags(text);
  autolinker.free();
  return postProcessAutolink(result, options);
}

function autoLinkUrlsCustom(text, options = {}) {
  const autolinker = createAutolinker(options);
  let result = autolinker.autolinkUrls(text);
  autolinker.free();
  return postProcessAutolink(result, options);
}

function autoLinkUsernamesOrLists(text, options = {}) {
  const autolinker = createAutolinker(options);
  let result = autolinker.autolinkUsernamesAndLists(text);
  autolinker.free();
  return postProcessAutolink(result, options);
}

// Validation functions
const validator = new wasm.Validator();

function isValidTweetText(text, config) {
  // Fast path: no config or default config uses the simple validator
  if (!config || config === configs.defaults || config === configs.version3) {
    return validator.isValidTweet(text);
  }
  // The old API uses config objects, we need to use the appropriate config
  if (config === configs.version1) {
    return validator.isValidTweet(text);
  }
  // For other configs, parse and check validity
  const result = parseTweet(text, config);
  return result.isValid;
}

function isValidUsername(text) {
  return validator.isValidUsername(text);
}

function isValidList(text) {
  return validator.isValidList(text);
}

function isValidHashtag(text) {
  return validator.isValidHashtag(text);
}

function isValidUrl(
  text,
  unicodeDomainsAllowed = true,
  requireProtocol = true,
) {
  if (requireProtocol) {
    return validator.isValidUrl(text);
  } else {
    return validator.isValidUrlWithoutProtocol(text);
  }
}

// Parse tweet function
function parseTweet(text, config) {
  let wasmConfig;
  if (config === configs.version1) {
    wasmConfig = wasm.TwitterTextConfiguration.configV1();
  } else if (config === configs.version2) {
    wasmConfig = wasm.TwitterTextConfiguration.configV2();
  } else if (config === configs.version3) {
    wasmConfig = wasm.TwitterTextConfiguration.configV3();
  } else {
    wasmConfig = wasm.TwitterTextConfiguration.configV3();
  }

  const result = wasm.parseTweetWithConfig(text, wasmConfig);
  const parsed = {
    weightedLength: result.weightedLength,
    permillage: result.permillage,
    isValid: result.isValid,
    displayRangeStart: result.displayTextRange.start,
    displayRangeEnd: result.displayTextRange.end,
    validRangeStart: result.validTextRange.start,
    validRangeEnd: result.validTextRange.end,
  };
  result.free();
  wasmConfig.free();
  return parsed;
}

// Hit highlight function
function hitHighlight(text, hits, options = {}) {
  const tag = options.tag || "em";
  const highlighter = wasm.HitHighlighter.withTag(tag);
  const result = highlighter.highlight(text, hits);
  highlighter.free();
  return result;
}

// Index conversion functions (these work on entity arrays in-place)
function modifyIndicesFromUTF16ToUnicode(text, entities) {
  // Convert UTF-16 indices to Unicode code point indices
  let offset = 0;
  let utf16Index = 0;

  const codePoints = [...text];
  const utf16ToUnicode = new Map();

  for (let i = 0; i < codePoints.length; i++) {
    utf16ToUnicode.set(utf16Index, i);
    utf16Index += codePoints[i].length > 1 ? 2 : 1;
  }
  utf16ToUnicode.set(utf16Index, codePoints.length);

  for (const entity of entities) {
    if (entity.indices) {
      entity.indices[0] =
        utf16ToUnicode.get(entity.indices[0]) ?? entity.indices[0];
      entity.indices[1] =
        utf16ToUnicode.get(entity.indices[1]) ?? entity.indices[1];
    }
  }
}

function modifyIndicesFromUnicodeToUTF16(text, entities) {
  // Convert Unicode code point indices to UTF-16 indices
  const codePoints = [...text];
  const unicodeToUtf16 = new Map();
  let utf16Index = 0;

  for (let i = 0; i < codePoints.length; i++) {
    unicodeToUtf16.set(i, utf16Index);
    utf16Index += codePoints[i].length > 1 ? 2 : 1;
  }
  unicodeToUtf16.set(codePoints.length, utf16Index);

  for (const entity of entities) {
    if (entity.indices) {
      entity.indices[0] =
        unicodeToUtf16.get(entity.indices[0]) ?? entity.indices[0];
      entity.indices[1] =
        unicodeToUtf16.get(entity.indices[1]) ?? entity.indices[1];
    }
  }
}

// Tweet length functions
function getTweetLength(text, config) {
  const result = parseTweet(text, config || configs.defaults);
  return result.weightedLength;
}

function getUnicodeTextLength(text) {
  return [...text].length;
}

function hasInvalidCharacters(text) {
  // Check for invalid characters (control chars, etc.)
  return /[\uFFFE\uFEFF\uFFFF]|[\u202A-\u202E]/.test(text);
}

function isInvalidTweet(text, config) {
  const result = parseTweet(text, config || configs.defaults);
  if (result.isValid) {
    return false;
  }
  if (!text || text.length === 0) {
    return "empty";
  }
  if (hasInvalidCharacters(text)) {
    return "invalid_characters";
  }
  return "too_long";
}

// Export the twttr.txt compatible API
const txt = {
  // Extract functions
  extractMentions,
  extractMentionsWithIndices,
  extractMentionsOrListsWithIndices,
  extractReplies,
  extractUrls,
  extractUrlsWithIndices,
  extractHashtags,
  extractHashtagsWithIndices,
  extractCashtags,
  extractCashtagsWithIndices,
  extractEntitiesWithIndices,

  // Autolink functions
  autoLink,
  autoLinkHashtags,
  autoLinkCashtags,
  autoLinkUrlsCustom,
  autoLinkUsernamesOrLists,

  // Validation functions
  isValidTweetText,
  isValidUsername,
  isValidList,
  isValidHashtag,
  isValidUrl,

  // Parse function
  parseTweet,

  // Hit highlight
  hitHighlight,

  // Index conversion
  modifyIndicesFromUTF16ToUnicode,
  modifyIndicesFromUnicodeToUTF16,

  // Length functions
  getTweetLength,
  getUnicodeTextLength,
  hasInvalidCharacters,
  isInvalidTweet,

  // Utility functions
  htmlEscape,
  splitTags,

  // Configurations
  configs,
};

// For compatibility with the old global twttr.txt
export const twttr = { txt };
export default txt;
