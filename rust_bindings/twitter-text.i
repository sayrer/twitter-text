#ifdef SWIGJAVA
%module TwitterText
#elif SWIGPYTHON
%module twitter_text
#elif SWIGRUBY
%module twittertext
#endif
%{
/* Includes the header in the wrapper code */
#include "rust/twitter-text/twitter-text.h"
#include "rust_bindings/cpp/twitter.h"
%}

%include <stdint.i>
%include <std_string.i>
%include <std_vector.i>

namespace std {
    %template(WeightedRangeList) vector<::twitter_text::WeightedRange>;
    %template(Hits) vector<twitter_text::Hit>;
    %template(Entities) vector<twitter_text::Entity>;
}

#ifdef SWIGPYTHON

%rename("%(undercase)s", %$isfunction) "";

namespace rust {
    class String;
    %typemap(out) String {
      $result = PyUnicode_FromStringAndSize($1.data(), $1.size());
    }

    %typemap(out) String* {
      $result = PyUnicode_FromStringAndSize($1->data(), $1->size());
    }
}

// see tf_session.i for more TODO here

#endif

#ifdef SWIGRUBY

%rename("%(undercase)s", %$isfunction) "";

namespace rust {
    class String;
    %typemap(out) String {
      $result = rb_utf8_str_new($1.data(), $1.size());
    }

    %typemap(out) String* {
      $result = rb_utf8_str_new($1->data(), $1->size());
    }
}

namespace std {
  %typemap(out) vector<twitter_text::ExtractorString> {
    VALUE list = rb_ary_new2($1.size());
    std::vector<twitter_text::ExtractorString>* estrings = &$1;
    for (size_t i = 0; i < $1.size(); i++) {
      VALUE s = rb_utf8_str_new(estrings->at(i).s.data(), estrings->at(i).s.size());
      rb_ary_push(list, s);
    }
    $result = list;
  }
}

#endif

#ifdef SWIGJAVA

%rename (RustString) String;
%rename("%(lowercamelcase)s", %$not %$isclass) "";
%rename (ExtractResult) SwigExtractResult;
%rename (MentionResult) SwigMentionResult;

namespace rust {
    class String;
    %typemap(jni) String, String* "jbyteArray"

  %typemap(in) String {
    jsize length = jenv->GetArrayLength($input);
    char* buf = (char *) jenv->GetPrimitiveArrayCritical($input, NULL);
    // TODO: null/error check
    $1 = ::rust::String(buf, length);
    jenv->ReleasePrimitiveArrayCritical($input, buf, 0);
  }

    %typemap(out) String {
      const size_t len = $1.size();
      $result = jenv->NewByteArray(len);
      // TODO: check that this succeeded
      jenv->SetByteArrayRegion($result, 0, len, (const jbyte*)$1.data());
    }

    %typemap(jtype) String "byte[]"
    %typemap(jstype) String "String"
    %typemap(javaout) String {
      System.out.println("Test decode?");
      String s = java.nio.charset.StandardCharsets.UTF_8.decode(java.nio.ByteBuffer.wrap($jnicall)).toString();
      System.out.println("decoded: " + s);
      return s;
    }
    %typemap(javain,
    pre= "byte[] temp$javainput = $javainput.getBytes(java.nio.charset.StandardCharsets.UTF_8);"
  ) String 
    "temp$javainput"


  %typemap(in) String* {
    jsize length = jenv->GetArrayLength($input);
    char* buf = (char *) jenv->GetPrimitiveArrayCritical($input, NULL);
    // TODO: null/error check
    $1 = new ::rust::String(buf, length);
    jenv->ReleasePrimitiveArrayCritical($input, buf, 0);
  }

    %typemap(out) String* {
      const size_t len = $1->size();
      $result = jenv->NewByteArray(len);
      // TODO: check that this succeeded
      jenv->SetByteArrayRegion($result, 0, len, (const jbyte*)$1->data());
    }

    %typemap(jtype) String* "byte[]"
    %typemap(jstype) String* "String"
    %typemap(javaout) String* {
      System.out.println("Test decode? 2");
      return java.nio.charset.StandardCharsets.UTF_8.decode(java.nio.ByteBuffer.wrap($jnicall)).toString();
    }
    %typemap(javain,
    pre= "byte[] temp$javainput = new byte[0]; if ($javainput != null) temp$javainput = $javainput.getBytes(java.nio.charset.StandardCharsets.UTF_8);"
  ) String* 
    "temp$javainput"
}

namespace std {
  %typemap(jni) string "jbyteArray"
  %typemap(in) string {
    jsize length = jenv->GetArrayLength($input);
    char* buf = (char *) jenv->GetPrimitiveArrayCritical($input, NULL);
    // TODO: null/error check
    $1 = std::string(buf, length);
    jenv->ReleasePrimitiveArrayCritical($input, buf, 0);
  }
  %typemap(jtype) string "byte[]"
  %typemap(jstype) string "String"
  %typemap(out) string {
    const size_t len = $1.size();
    $result = jenv->NewByteArray(len);
    // TODO: check that this succeeded
    jenv->SetByteArrayRegion($result, 0, len, (const jbyte*)$1.data());
  }
  %typemap(javaout) string {
    System.out.println("Calling javaout");
    return java.nio.charset.StandardCharsets.UTF_8.decode(java.nio.ByteBuffer.wrap($jnicall)).toString();
  }
  %typemap(javain,
    pre= "byte[] temp$javainput = $javainput.getBytes(java.nio.charset.StandardCharsets.UTF_8);"
  ) string 
    "temp$javainput"

  %typemap(jni) string* "jbyteArray"
  %typemap(in) string* {
    jsize length = jenv->GetArrayLength($input);
    char* buf = (char *) jenv->GetPrimitiveArrayCritical($input, NULL);
    // TODO: null/error check
    $1 = std::string(buf, length);
    jenv->ReleasePrimitiveArrayCritical($input, buf, 0);
  }
  %typemap(jtype) string* "byte[]"
  %typemap(jstype) string* "String"
  %typemap(out) string* {
    const size_t len = $1.size();
    $result = jenv->NewByteArray(len);
    // TODO: check that this succeeded
    jenv->SetByteArrayRegion($result, 0, len, (const jbyte*)$1.data());
  }
  %typemap(javaout) string* {
    System.out.println("Calling javaout");
    return java.nio.charset.StandardCharsets.UTF_8.decode(java.nio.ByteBuffer.wrap($jnicall)).toString();
  }
  %typemap(javain,
    pre= "byte[] temp$javainput = $javainput.getBytes(java.nio.charset.StandardCharsets.UTF_8);"
  ) string* 
    "temp$javainput"

  %typemap(jstype) vector<twitter_text::ExtractorString> "java.util.AbstractList<String>"
  %typemap(javaout) vector<twitter_text::ExtractorString> {
    System.out.println("Test decode? 3");
    ExtractorStrings es = new ExtractorStrings($jnicall, true);
    java.util.ArrayList list = new java.util.ArrayList(es.size());
    for (int i = 0; i < es.size(); i++) {
      list.add(es.get(i).getS());
    }
    return list;
  }
}

#endif

%ignore config_v1;
%ignore config_v2;
%ignore config_v3;
%ignore default_config;
%ignore get_config_weighted_ranges;
%ignore configuration_from_path;
%ignore configuration_from_json;
%ignore autolink_default_config;
%ignore autolink_all;
%ignore make_extractor;
%ignore extract_entities_with_indices;
%ignore extract_mentioned_screennames;
%ignore extract_mentioned_screennames_with_indices;
%ignore extract_mentions_or_lists_with_indices;
%ignore extract_reply_username;
%ignore extract_urls;
%ignore extract_urls_with_indices;
%ignore extract_hashtags;
%ignore extract_hashtags_with_indices;
%ignore extract_cashtags;
%ignore extract_cashtags_with_indices;
%ignore make_validating_extractor;
%ignore extract_entities_with_indices_validated;
%ignore extract_mentioned_screennames_with_indices_validated;
%ignore extract_mentions_or_lists_with_indices_validated;
%ignore extract_reply_username_validated;
%ignore extract_urls_with_indices_validated;
%ignore extract_hashtags_with_indices_validated;
%ignore extract_cashtags_with_indices_validated;
%ignore make_default_highlighter;
%ignore make_highlighter;
%ignore make_default_validator;
%ignore autolink_usernames_and_lists;
%ignore autolink_hashtags;
%ignore autolink_urls;
%ignore autolink_cashtags;
%ignore get_extract_url_without_protocol;
%ignore set_extract_url_without_protocol;
%ignore get_extract_url_without_protocol_validated;
%ignore set_extract_url_without_protocol_validated;
%ignore get_normalize;
%ignore set_normalize;
%ignore hit_highlight;
%ignore is_valid_tweet;
%ignore is_valid_username;
%ignore is_valid_list;
%ignore is_valid_hashtag;
%ignore is_valid_url;
%ignore is_valid_url_without_protocol;
%ignore get_max_tweet_length;
%ignore get_short_url_length;
%ignore set_short_url_length;
%ignore get_short_url_length_https;
%ignore set_short_url_length_https;
%ignore parse_ffi;
%ignore pretty_print;

%ignore AutolinkerConfig;
%ignore Configuration;
%ignore ExtractResult;
%ignore MentionResult;

%ignore Box;

/* Parse the header file to generate wrappers */
%include "rust_bindings/twitter-text-swig.h"
%include "rust_bindings/cpp/twitter.h"

namespace twitter_text {
    %template(Extractor) Extractor<::std::vector<twitter_text::Entity>, twitter_text::Entity*>;
    %template(ValidatingExtractor) ValidatingExtractor<twitter_text::SwigExtractResult*, twitter_text::SwigMentionResult*>;
}