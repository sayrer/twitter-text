#include "twitter.h"
#include "gtest/gtest.h"

namespace twitter_text {

TEST(ExtractorTest, Ctor) {
  Extractor *extractor = new Extractor();
  ASSERT_NE(extractor, nullptr);
}

}