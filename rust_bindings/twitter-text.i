 %module twittertext
 %{
 /* Includes the header in the wrapper code */
 #include "rust_bindings/cpp/twitter.h"
 %}
 
 /* Parse the header file to generate wrappers */
 %include "rust_bindings/cpp/twitter.h"