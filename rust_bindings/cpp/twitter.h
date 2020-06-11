#pragma once
#include "rust/twitter-text/twitter-text.h"

namespace twitter_text {

class HitHighlighter {
 public:
  HitHighlighter(): 
    config(new twitter_text_ffi::HitHighlighter{
      .highlight_tag = ::rust::String("em")
    }) 
  {}   
  
  HitHighlighter(std::string tag_str):
    config(new twitter_text_ffi::HitHighlighter{
      .highlight_tag = ::rust::String(tag_str)
    }) 
  {}

  std::string GetHighlightTag();   

 private:
  std::unique_ptr<twitter_text_ffi::HitHighlighter> config;
};  

} // twitter_text

