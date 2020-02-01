 %module twittertext
 %{
 /* Includes the header in the wrapper code */
 #include "twitter-text.h"
 %}
 
 /* Parse the header file to generate wrappers */
 %include "twitter-text.h"