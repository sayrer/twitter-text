#include "twitter.h"
#include "gtest/gtest.h"

namespace twitter_text {

TEST(HithighlighterTest, Ctor) {
  HitHighlighter *highlighter = new HitHighlighter();
  ASSERT_NE(highlighter, nullptr);
}

TEST(HithighlighterTest, CtorWithString) {
  HitHighlighter *highlighter = new HitHighlighter("hmm");
  ASSERT_NE(highlighter, nullptr);
}

TEST(HithighlighterTest, GetTag) {
  HitHighlighter *highlighter = new HitHighlighter("hmm");
  ASSERT_EQ(highlighter->GetHighlightTag(), "hmm");
}

TEST(HithighlighterTest, GetDefaultTag) {
  HitHighlighter *highlighter = new HitHighlighter();
  ASSERT_EQ(highlighter->GetHighlightTag(), "em");
}

} // twitter_text