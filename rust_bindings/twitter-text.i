#ifdef SWIGJAVA
%module TwitterText
#elif SWIGPYTHON
%module twitter_text
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
    %template(ExtractorStrings) vector<string>;
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

#endif

#ifdef SWIGJAVA

%rename (RustString) String;
%rename("%(lowercamelcase)s", %$not %$isclass) "";

namespace rust {
    class String;
    %typemap(jni) String, String* "jbyteArray"

  %typemap(in) String {
    jsize length = jenv->GetArrayLength($input);
    char* buf = (char *) jenv->GetPrimitiveArrayCritical($input, NULL);
    // TODO: null/error check
    $1 = std::string(buf, length);
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
      return java.nio.charset.StandardCharsets.UTF_8.decode(java.nio.ByteBuffer.wrap($jnicall)).toString();
    }
    %typemap(javain,
    pre= "byte[] temp$javainput = $javainput.getBytes(java.nio.charset.StandardCharsets.UTF_8);"
  ) String 
    "temp$javainput"


  %typemap(in) String* {
    jsize length = jenv->GetArrayLength($input);
    char* buf = (char *) jenv->GetPrimitiveArrayCritical($input, NULL);
    // TODO: null/error check
    $1 = std::string(buf, length);
    jenv->ReleasePrimitiveArrayCritical($input, buf, 0);
  }

    %typemap(out) String* {
      const size_t len = $1.size();
      $result = jenv->NewByteArray(len);
      // TODO: check that this succeeded
      jenv->SetByteArrayRegion($result, 0, len, (const jbyte*)$1.data());
    }

    %typemap(jtype) String* "byte[]"
    %typemap(jstype) String* "String"
    %typemap(javaout) String* {
      return java.nio.charset.StandardCharsets.UTF_8.decode(java.nio.ByteBuffer.wrap($jnicall)).toString();
    }
    %typemap(javain,
    pre= "byte[] temp$javainput = $javainput.getBytes(java.nio.charset.StandardCharsets.UTF_8);"
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
    return java.nio.charset.StandardCharsets.UTF_8.decode(java.nio.ByteBuffer.wrap($jnicall)).toString();
  }
  %typemap(javain,
    pre= "byte[] temp$javainput = $javainput.getBytes(java.nio.charset.StandardCharsets.UTF_8);"
  ) string 
    "temp$javainput"
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

%ignore AutolinkerConfig;
%ignore Configuration;
%ignore ExtractResult;
%ignore ExtractorString;
%ignore MentionResult;

%ignore Box;

/* Parse the header file to generate wrappers */
%include "rust/twitter-text/twitter-text.h"
%include "rust_bindings/cpp/twitter.h"