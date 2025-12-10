// Copyright 2018 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

package com.sayrer.twitter_text;

import com.sayrer.twitter_text.Entity.Type;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import junit.framework.Test;
import junit.framework.TestCase;
import junit.framework.TestSuite;

public class ExtractorTest extends TestCase {

    protected Extractor extractor;

    public static Test suite() {
        final Class<?>[] testClasses = {
            OffsetConversionTest.class,
            ReplyTest.class,
            MentionTest.class,
            HashtagTest.class,
            URLTest.class,
            FederatedMentionTest.class,
            EntitiesTest.class,
        };
        return new TestSuite(testClasses);
    }

    public void setUp() throws Exception {
        extractor = new Extractor();
    }

    public static class OffsetConversionTest extends ExtractorTest {

        public void testConvertIndices() {
            assertOffsetConversionOk("abc", "abc");
            assertOffsetConversionOk("\ud83d\ude02abc", "abc");
            assertOffsetConversionOk("\ud83d\ude02abc\ud83d\ude02", "abc");
            assertOffsetConversionOk("\ud83d\ude02abc\ud838\ude02abc", "abc");
            assertOffsetConversionOk(
                "\ud83d\ude02abc\ud838\ude02abc\ud83d\ude02",
                "abc"
            );
            assertOffsetConversionOk("\ud83d\ude02\ud83d\ude02abc", "abc");
            assertOffsetConversionOk(
                "\ud83d\ude02\ud83d\ude02\ud83d\ude02abc",
                "abc"
            );

            assertOffsetConversionOk(
                "\ud83d\ude02\ud83d\ude02\ud83d\ude02abc\ud83d\ude02",
                "abc"
            );

            // Several surrogate pairs following the entity
            assertOffsetConversionOk(
                "\ud83d\ude02\ud83d\ude02\ud83d\ude02abc\ud83d\ude02\ud83d" +
                    "\ude02\ud83d\ude02",
                "abc"
            );

            // Several surrogate pairs surrounding multiple entities
            assertOffsetConversionOk(
                "\ud83d\ude02\ud83d\ude02\ud83d\ude02\ud83d\ude02abc\ud83d" +
                    "\ude02\ud83d\ude02\ud83d\ude02\ud83d\ude02abc\ud83d" +
                    "\ude02\ud83d\ude02\ud83d\ude02\ud83d\ude02",
                "abc"
            );

            // unpaired low surrogate (at start)
            assertOffsetConversionOk(
                "\ude02\ud83d\ude02\ud83d\ude02\ud83d\ude02abc\ud83d" +
                    "\ude02\ud83d\ude02\ud83d\ude02\ud83d\ude02abc\ud83d" +
                    "\ude02\ud83d\ude02\ud83d\ude02\ud83d\ude02",
                "abc"
            );

            // unpaired low surrogate (at end)
            assertOffsetConversionOk(
                "\ud83d\ude02\ud83d\ude02\ud83d\ude02\ud83d\ude02abc\ud83d" +
                    "\ude02\ud83d\ude02\ud83d\ude02\ud83d\ude02abc\ud83d" +
                    "\ude02\ud83d\ude02\ud83d\ude02\ude02",
                "abc"
            );

            // unpaired low and high surrogates (at end)
            assertOffsetConversionOk(
                "\ud83d\ude02\ud83d\ude02\ud83d\ude02\ud83d\ude02abc\ud83d" +
                    "\ude02\ud83d\ude02\ud83d\ude02\ud83d\ude02abc\ud83d" +
                    "\ude02\ud83d\ude02\ud83d\ud83d\ude02\ude02",
                "abc"
            );

            assertOffsetConversionOk("\ud83dabc\ud83d", "abc");

            assertOffsetConversionOk("\ude02abc\ude02", "abc");

            assertOffsetConversionOk("\ude02\ude02abc\ude02\ude02", "abc");

            assertOffsetConversionOk("abcabc", "abc");

            assertOffsetConversionOk("abc\ud83d\ude02abc", "abc");

            assertOffsetConversionOk("aa", "a");

            assertOffsetConversionOk(
                "\ud83d\ude02a\ud83d\ude02a\ud83d\ude02",
                "a"
            );
        }

        private void assertOffsetConversionOk(String testData, String patStr) {
            // Build an entity at the location of patStr
            final Pattern pat = Pattern.compile(patStr);
            final Matcher matcher = pat.matcher(testData);

            final List<Entity> entities = new ArrayList<>();
            final List<Integer> codePointOffsets = new ArrayList<>();
            final List<Integer> charOffsets = new ArrayList<>();
            while (matcher.find()) {
                final int charOffset = matcher.start();
                charOffsets.add(charOffset);
                codePointOffsets.add(testData.codePointCount(0, charOffset));
                entities.add(new Entity(matcher, Type.HASHTAG, 0, 0));
            }

            extractor.modifyIndicesFromUTF16ToUnicode(testData, entities);

            for (int i = 0; i < entities.size(); i++) {
                assertEquals(
                    codePointOffsets.get(i),
                    entities.get(i).getStart()
                );
            }

            extractor.modifyIndicesFromUnicodeToUTF16(testData, entities);

            for (int i = 0; i < entities.size(); i++) {
                // This assertion could fail if the entity location is in the middle
                // of a surrogate pair, since there is no equivalent code point
                // offset to that location. It would be pathological for an entity to
                // start at that point, so we can just let the test fail in that case.
                assertEquals(charOffsets.get(i), entities.get(i).getStart());
            }
        }
    }

    /**
     * Tests for the extractReplyScreenname method
     */
    public static class ReplyTest extends ExtractorTest {

        public void testReplyAtTheStart() {
            final String extracted = extractor.extractReplyScreenname(
                "@user reply"
            );
            assertEquals(
                "Failed to extract reply at the start",
                "user",
                extracted
            );
        }

        public void testReplyWithLeadingSpace() {
            final String extracted = extractor.extractReplyScreenname(
                " @user reply"
            );
            assertEquals(
                "Failed to extract reply with leading space",
                "user",
                extracted
            );
        }
    }

    /**
     * Tests for the extractMentionedScreennames{WithIndices} methods
     */
    public static class MentionTest extends ExtractorTest {

        public void testMentionAtTheBeginning() {
            final String[] extracted = extractor.extractMentionedScreennames(
                "@user mention"
            );
            assertList(
                "Failed to extract mention at the beginning",
                new String[] { "user" },
                Arrays.asList(extracted)
            );
        }

        public void testMentionWithLeadingSpace() {
            final String[] extracted = extractor.extractMentionedScreennames(
                " @user mention"
            );
            assertList(
                "Failed to extract mention with leading space",
                new String[] { "user" },
                Arrays.asList(extracted)
            );
        }

        public void testMentionInMidText() {
            final String[] extracted = extractor.extractMentionedScreennames(
                "mention @user here"
            );
            assertList(
                "Failed to extract mention in mid text",
                new String[] { "user" },
                Arrays.asList(extracted)
            );
        }

        public void testMultipleMentions() {
            final String[] extracted = extractor.extractMentionedScreennames(
                "mention @user1 here and @user2 here"
            );
            assertList(
                "Failed to extract multiple mentioned users",
                new String[] { "user1", "user2" },
                Arrays.asList(extracted)
            );
        }

        public void testMentionWithIndices() {
            final List<Entity> extracted =
                extractor.extractMentionedScreennamesWithIndices(
                    " @user1 mention @user2 here @user3 "
                );
            assertEquals(extracted.size(), 3);
            assertEquals(extracted.get(0).getStart().intValue(), 1);
            assertEquals(extracted.get(0).getEnd().intValue(), 7);
            assertEquals(extracted.get(1).getStart().intValue(), 16);
            assertEquals(extracted.get(1).getEnd().intValue(), 22);
            assertEquals(extracted.get(2).getStart().intValue(), 28);
            assertEquals(extracted.get(2).getEnd().intValue(), 34);
        }

        public void testMentionWithSupplementaryCharacters() {
            // insert U+10400 before " @mention"
            final String text = String.format(
                "%c @mention %c @mention",
                0x00010400,
                0x00010400
            );

            // count U+10400 as 2 characters (as in UTF-16)
            final List<Entity> extracted =
                extractor.extractMentionedScreennamesWithIndices(text);
            assertEquals(extracted.size(), 2);
            assertEquals(extracted.get(0).value, "mention");
            assertEquals(extracted.get(0).start, 3);
            assertEquals(extracted.get(0).end, 11);
            assertEquals(extracted.get(1).value, "mention");
            assertEquals(extracted.get(1).start, 15);
            assertEquals(extracted.get(1).end, 23);

            // count U+10400 as single character
            extractor.modifyIndicesFromUTF16ToUnicode(text, extracted);
            assertEquals(extracted.size(), 2);
            assertEquals(extracted.get(0).start, 2);
            assertEquals(extracted.get(0).end, 10);
            assertEquals(extracted.get(1).start, 13);
            assertEquals(extracted.get(1).end, 21);

            // count U+10400 as 2 characters (as in UTF-16)
            extractor.modifyIndicesFromUnicodeToUTF16(text, extracted);
            assertEquals(2, extracted.size());
            assertEquals(3, extracted.get(0).start);
            assertEquals(11, extracted.get(0).end);
            assertEquals(15, extracted.get(1).start);
            assertEquals(23, extracted.get(1).end);
        }
    }

    /**
     * Tests for the extractHashtags method
     */
    public static class HashtagTest extends ExtractorTest {

        public void testHashtagAtTheBeginning() {
            final String[] extracted = extractor.extractHashtags(
                "#hashtag mention"
            );
            assertList(
                "Failed to extract hashtag at the beginning",
                new String[] { "hashtag" },
                Arrays.asList(extracted)
            );
        }

        public void testHashtagWithLeadingSpace() {
            final String[] extracted = extractor.extractHashtags(
                " #hashtag mention"
            );
            assertList(
                "Failed to extract hashtag with leading space",
                new String[] { "hashtag" },
                Arrays.asList(extracted)
            );
        }

        public void testHashtagInMidText() {
            final String[] extracted = extractor.extractHashtags(
                "mention #hashtag here"
            );
            assertList(
                "Failed to extract hashtag in mid text",
                new String[] { "hashtag" },
                Arrays.asList(extracted)
            );
        }

        public void testMultipleHashtags() {
            final String[] extracted = extractor.extractHashtags(
                "text #hashtag1 #hashtag2"
            );
            assertList(
                "Failed to extract multiple hashtags",
                new String[] { "hashtag1", "hashtag2" },
                Arrays.asList(extracted)
            );
        }

        public void testHashtagWithIndices() {
            final List<Entity> extracted = extractor.extractHashtagsWithIndices(
                " #user1 mention #user2 here #user3 "
            );
            assertEquals(extracted.size(), 3);
            assertEquals(extracted.get(0).getStart().intValue(), 1);
            assertEquals(extracted.get(0).getEnd().intValue(), 7);
            assertEquals(extracted.get(1).getStart().intValue(), 16);
            assertEquals(extracted.get(1).getEnd().intValue(), 22);
            assertEquals(extracted.get(2).getStart().intValue(), 28);
            assertEquals(extracted.get(2).getEnd().intValue(), 34);
        }

        public void testHashtagWithSupplementaryCharacters() {
            // insert U+10400 before " #hashtag"
            final String text = String.format(
                "%c #hashtag %c #hashtag",
                0x00010400,
                0x00010400
            );

            // count U+10400 as 2 characters (as in UTF-16)
            final List<Entity> extracted = extractor.extractHashtagsWithIndices(
                text
            );
            assertEquals(extracted.size(), 2);
            assertEquals(extracted.get(0).value, "hashtag");
            assertEquals(extracted.get(0).start, 3);
            assertEquals(extracted.get(0).end, 11);
            assertEquals(extracted.get(1).value, "hashtag");
            assertEquals(extracted.get(1).start, 15);
            assertEquals(extracted.get(1).end, 23);

            // count U+10400 as single character
            extractor.modifyIndicesFromUTF16ToUnicode(text, extracted);
            assertEquals(extracted.size(), 2);
            assertEquals(extracted.get(0).start, 2);
            assertEquals(extracted.get(0).end, 10);
            assertEquals(extracted.get(1).start, 13);
            assertEquals(extracted.get(1).end, 21);

            // count U+10400 as 2 characters (as in UTF-16)
            extractor.modifyIndicesFromUnicodeToUTF16(text, extracted);
            assertEquals(extracted.size(), 2);
            assertEquals(extracted.get(0).start, 3);
            assertEquals(extracted.get(0).end, 11);
            assertEquals(extracted.get(1).start, 15);
            assertEquals(extracted.get(1).end, 23);
        }
    }

    /**
     * Tests for the extractURLsWithIndices method
     */
    public static class URLTest extends ExtractorTest {

        public void testUrlWithIndices() {
            final List<Entity> extracted = extractor.extractURLsWithIndices(
                "http://t.co url https://www.twitter.com "
            );
            assertEquals(extracted.get(0).getStart().intValue(), 0);
            assertEquals(extracted.get(0).getEnd().intValue(), 11);
            assertEquals(extracted.get(1).getStart().intValue(), 16);
            assertEquals(extracted.get(1).getEnd().intValue(), 39);
        }

        public void testUrlWithoutProtocol() {
            final String text =
                "www.twitter.com, www.yahoo.co.jp, t.co/blahblah, www.poloshirts.uk.com";
            assertList(
                "Failed to extract URLs without protocol",
                new String[] {
                    "www.twitter.com",
                    "www.yahoo.co.jp",
                    "t.co/blahblah",
                    "www.poloshirts.uk.com",
                },
                extractor.extractURLs(text)
            );

            final List<Entity> extracted = extractor.extractURLsWithIndices(
                text
            );
            assertEquals(extracted.get(0).getStart().intValue(), 0);
            assertEquals(extracted.get(0).getEnd().intValue(), 15);
            assertEquals(extracted.get(1).getStart().intValue(), 17);
            assertEquals(extracted.get(1).getEnd().intValue(), 32);
            assertEquals(extracted.get(2).getStart().intValue(), 34);
            assertEquals(extracted.get(2).getEnd().intValue(), 47);

            extractor.setExtractURLWithoutProtocol(false);
            assertTrue(
                "Should not extract URLs w/o protocol",
                extractor.extractURLs(text).isEmpty()
            );
        }

        public void testURLFollowedByPunctuations() {
            final String text =
                "http://games.aarp.org/games/mahjongg-dimensions.aspx!!!!!!";
            assertList(
                "Failed to extract URLs followed by punctuations",
                new String[] {
                    "http://games.aarp.org/games/mahjongg-dimensions.aspx",
                },
                extractor.extractURLs(text)
            );
        }

        public void testUrlWithPunctuation() {
            final String[] urls = new String[] {
                "http://www.foo.com/foo/path-with-period./",
                "http://www.foo.org.za/foo/bar/688.1",
                "http://www.foo.com/bar-path/some.stm?param1=foo;param2=P1|0||P2|0",
                "http://foo.com/bar/123/foo_&_bar/",
                "http://foo.com/bar(test)bar(test)bar(test)",
                "www.foo.com/foo/path-with-period./",
                "www.foo.org.za/foo/bar/688.1",
                "www.foo.com/bar-path/some.stm?param1=foo;param2=P1|0||P2|0",
                "foo.com/bar/123/foo_&_bar/",
            };

            for (String url : urls) {
                assertEquals(url, extractor.extractURLs(url).get(0));
            }
        }

        public void testUrlnWithSupplementaryCharacters() {
            // insert U+10400 before " http://twitter.com"
            final String text = String.format(
                "%c http://twitter.com %c http://twitter.com",
                0x00010400,
                0x00010400
            );

            // count U+10400 as 2 characters (as in UTF-16)
            final List<Entity> extracted = extractor.extractURLsWithIndices(
                text
            );
            assertEquals(extracted.size(), 2);
            assertEquals(extracted.get(0).value, "http://twitter.com");
            assertEquals(extracted.get(0).start, 3);
            assertEquals(extracted.get(0).end, 21);
            assertEquals(extracted.get(1).value, "http://twitter.com");
            assertEquals(extracted.get(1).start, 25);
            assertEquals(extracted.get(1).end, 43);

            // count U+10400 as single character
            extractor.modifyIndicesFromUTF16ToUnicode(text, extracted);
            assertEquals(extracted.size(), 2);
            assertEquals(extracted.get(0).start, 2);
            assertEquals(extracted.get(0).end, 20);
            assertEquals(extracted.get(1).start, 23);
            assertEquals(extracted.get(1).end, 41);

            // count U+10400 as 2 characters (as in UTF-16)
            extractor.modifyIndicesFromUnicodeToUTF16(text, extracted);
            assertEquals(extracted.size(), 2);
            assertEquals(extracted.get(0).start, 3);
            assertEquals(extracted.get(0).end, 21);
            assertEquals(extracted.get(1).start, 25);
            assertEquals(extracted.get(1).end, 43);
        }
    }

    public void testUrlWithSpecialCCTLDWithoutProtocol() {
        final String text = "MLB.tv vine.co";
        assertList(
            "Failed to extract URLs without protocol",
            new String[] { "MLB.tv", "vine.co" },
            extractor.extractURLs(text)
        );

        final List<Entity> extracted = extractor.extractURLsWithIndices(text);
        assertEquals(extracted.get(0).getStart().intValue(), 0);
        assertEquals(extracted.get(0).getEnd().intValue(), 6);
        assertEquals(extracted.get(1).getStart().intValue(), 7);
        assertEquals(extracted.get(1).getEnd().intValue(), 14);

        extractor.setExtractURLWithoutProtocol(false);
        assertTrue(
            "Should not extract URLs w/o protocol",
            extractor.extractURLs(text).isEmpty()
        );
    }

    /**
     * Helper method for asserting that the List of extracted Strings match the expected values.
     *
     * @param message to display on failure
     * @param expected Array of Strings that were expected to be extracted
     * @param actual List of Strings that were extracted
     */
    protected void assertList(
        String message,
        String[] expected,
        List<String> actual
    ) {
        final List<String> expectedList = Arrays.asList(expected);
        if (expectedList.size() != actual.size()) {
            fail(
                message +
                    "\n\nExpected list and extracted list are differnt sizes:\n" +
                    "  Expected (" +
                    expectedList.size() +
                    "): " +
                    expectedList +
                    "\n" +
                    "  Actual   (" +
                    actual.size() +
                    "): " +
                    actual
            );
        } else {
            for (int i = 0; i < expectedList.size(); i++) {
                assertEquals(expectedList.get(i), actual.get(i));
            }
        }
    }

    /**
     * Tests for the extractFederatedMentions method
     */
    public static class FederatedMentionTest extends ExtractorTest {

        public void testFederatedMentionSimple() {
            final String[] extracted = extractor.extractFederatedMentions(
                "@user@domain.tld"
            );
            assertList(
                "Failed to extract federated mention",
                new String[] { "@user@domain.tld" },
                Arrays.asList(extracted)
            );
        }

        public void testFederatedMentionInText() {
            final String[] extracted = extractor.extractFederatedMentions(
                "hello @user@mastodon.social world"
            );
            assertList(
                "Failed to extract federated mention in text",
                new String[] { "@user@mastodon.social" },
                Arrays.asList(extracted)
            );
        }

        public void testFederatedMentionMultiple() {
            final String[] extracted = extractor.extractFederatedMentions(
                "@user1@domain1.com and @user2@domain2.org"
            );
            assertList(
                "Failed to extract multiple federated mentions",
                new String[] { "@user1@domain1.com", "@user2@domain2.org" },
                Arrays.asList(extracted)
            );
        }

        public void testFederatedMentionMixed() {
            // extract_federated_mentions returns both regular and federated mentions
            final String[] extracted = extractor.extractFederatedMentions(
                "test @sayrer @user1@domain1.com"
            );
            assertList(
                "Failed to extract mixed mentions",
                new String[] { "sayrer", "@user1@domain1.com" },
                Arrays.asList(extracted)
            );
        }

        public void testFederatedMentionWithIndices() {
            final List<Entity> extracted =
                extractor.extractFederatedMentionsWithIndices(
                    "hello @user@mastodon.social world"
                );
            assertEquals(1, extracted.size());
            assertEquals("@user@mastodon.social", extracted.get(0).getValue());
            assertEquals(6, extracted.get(0).getStart().intValue());
            assertEquals(27, extracted.get(0).getEnd().intValue());
            assertEquals(
                Entity.Type.FEDERATEDMENTION,
                extracted.get(0).getType()
            );
        }
    }

    /**
     * Tests for the extractEntitiesWithIndices methods
     */
    public static class EntitiesTest extends ExtractorTest {

        public void testEntitiesExcludesFederatedMentions() {
            final String text =
                "Check https://example.com @user @fed@mastodon.social #tag $CASH";
            final List<Entity> entities = extractor.extractEntitiesWithIndices(
                text
            );

            // Should have 4 entities: URL, mention, hashtag, cashtag (no federated mention)
            assertEquals(4, entities.size());

            // Verify types
            boolean hasUrl = entities
                .stream()
                .anyMatch(e -> e.getType() == Entity.Type.URL);
            boolean hasMention = entities
                .stream()
                .anyMatch(e -> e.getType() == Entity.Type.MENTION);
            boolean hasHashtag = entities
                .stream()
                .anyMatch(e -> e.getType() == Entity.Type.HASHTAG);
            boolean hasCashtag = entities
                .stream()
                .anyMatch(e -> e.getType() == Entity.Type.CASHTAG);
            boolean hasFederated = entities
                .stream()
                .anyMatch(e -> e.getType() == Entity.Type.FEDERATEDMENTION);

            assertTrue("Should have URL", hasUrl);
            assertTrue("Should have MENTION", hasMention);
            assertTrue("Should have HASHTAG", hasHashtag);
            assertTrue("Should have CASHTAG", hasCashtag);
            assertFalse("Should NOT have FEDERATEDMENTION", hasFederated);
        }

        public void testEntitiesIncludesFederatedMentions() {
            final String text =
                "Check https://example.com @user @fed@mastodon.social #tag $CASH";
            final List<Entity> entities =
                extractor.extractEntitiesWithIndicesFederated(text);

            // Should have 5 entities: URL, mention, federated mention, hashtag, cashtag
            assertEquals(5, entities.size());

            // Verify federated mention is present
            boolean hasFederated = entities
                .stream()
                .anyMatch(e -> e.getType() == Entity.Type.FEDERATEDMENTION);
            assertTrue("Should have FEDERATEDMENTION", hasFederated);

            // Verify the federated mention value
            Entity federated = entities
                .stream()
                .filter(e -> e.getType() == Entity.Type.FEDERATEDMENTION)
                .findFirst()
                .orElse(null);
            assertNotNull(federated);
            assertEquals("@fed@mastodon.social", federated.getValue());
        }
    }
}
