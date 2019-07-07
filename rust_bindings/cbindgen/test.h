#ifndef twitter_text_ffi_h
#define twitter_text_ffi_h

/* Generated with cbindgen:0.9.0 */

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

namespace twittertext {
namespace ffi {

static const int32_t MAX_TWEET_LENGTH = 280;

static const uintptr_t MAX_URL_LENGTH = 4096;

struct Configuration;

struct Range;

///  A struct that represents a parsed tweet containing the length of the tweet,
///  its validity, display ranges etc. The name mirrors Twitter's Java implementation.
struct TwitterTextParseResults {
  /// The weighted length is the number used to determine the tweet's length for the purposes of Twitter's limit of 280. Most characters count
  /// for 2 units, while a few ranges (like ASCII and Latin-1) count for 1. See [Twitter's blog post](https://blog.twitter.com/official/en_us/topics/product/2017/Giving-you-more-characters-to-express-yourself.html).
  int32_t weighted_length;
  /// The weighted length expressed as a number relative to a limit of 1000.
  /// This value makes it easier to implement UI like Twitter's tweet-length meter.
  int32_t permillage;
  /// Whether the tweet is valid: its weighted length must be under the configured limit, it must
  /// not be empty, and it must not contain invalid characters.
  bool is_valid;
  /// The display range expressed in UTF-16.
  Range display_text_range;
  /// The valid display range expressed in UTF-16. After the end of the valid range, clients
  /// typically stop highlighting entities, etc.
  Range valid_text_range;

  TwitterTextParseResults(int32_t const& weighted_length,
                          int32_t const& permillage,
                          bool const& is_valid,
                          Range const& display_text_range,
                          Range const& valid_text_range)
    : weighted_length(weighted_length),
      permillage(permillage),
      is_valid(is_valid),
      display_text_range(display_text_range),
      valid_text_range(valid_text_range)
  {}

  bool operator==(const TwitterTextParseResults& other) const {
    return weighted_length == other.weighted_length &&
           permillage == other.permillage &&
           is_valid == other.is_valid &&
           display_text_range == other.display_text_range &&
           valid_text_range == other.valid_text_range;
  }
  bool operator!=(const TwitterTextParseResults& other) const {
    return weighted_length != other.weighted_length ||
           permillage != other.permillage ||
           is_valid != other.is_valid ||
           display_text_range != other.display_text_range ||
           valid_text_range != other.valid_text_range;
  }
};

extern "C" {

///  Produce a [TwitterTextParseResults] struct from a [str]. If extract_urls is true, the weighted
///  length will give all URLs the weight supplied in [Configuration](twitter_text_configuration::Configuration),
///  regardless of their length.
///  This function will allocate an NFC-normalized copy of the input string. If the text is already
///  NFC-normalized, [ValidatingExtractor::new_with_nfc_input] will be more efficient.
TwitterTextParseResults parse(const str *text,
                              const Configuration *config,
                              bool extract_urls);

} // extern "C"

} // namespace ffi
} // namespace twittertext

#endif // twitter_text_ffi_h
