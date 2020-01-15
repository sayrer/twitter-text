 %module twittertext
 %{
 /* Includes the header in the wrapper code */
 #include "test.h"
 %}
 
 /* Parse the header file to generate wrappers */
 %include "test.h"