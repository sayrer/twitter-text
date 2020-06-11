#include "twitter.h"

namespace twitter_text {

std::string 
HitHighlighter::GetHighlightTag() {
    return std::string(config->highlight_tag);
}

} // twitter_text