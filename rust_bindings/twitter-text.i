%module twitter_text
%{
/* Includes the header in the wrapper code */
#include "rust_bindings/cpp/twitter.h"
%}

%include <std_string.i>

%ignore TwitterTextConfiguration;
%ignore Extractor;
%ignore ExtractResult;
%ignore ValidatingExtractor;
%ignore HitHighlighter;
%ignore TwitterTextParser;
%ignore Validator;

/* Parse the header file to generate wrappers */
%include "rust_bindings/cpp/twitter.h"