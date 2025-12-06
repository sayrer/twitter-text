// Copyright 2018 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

package com.sayrer.twitter_text;

import static org.junit.Assert.*;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;

public class ValidatorTest {

    private Validator validator;
    private Validator v3Validator;
    private Configuration v3Config;

    @Before
    public void setUp() {
        validator = Validator.create();
        v3Config = Configuration.createV3();
        v3Validator = Validator.withConfig(v3Config);
    }

    @After
    public void tearDown() {
        if (validator != null) {
            validator.close();
        }
        if (v3Validator != null) {
            v3Validator.close();
        }
        if (v3Config != null) {
            v3Config.close();
        }
    }

    @Test
    public void testBOMCharacter() {
        assertFalse(validator.isValidTweet("test \uFFFE"));
        assertFalse(validator.isValidTweet("test \uFEFF"));
    }

    @Test
    public void testInvalidCharacter() {
        assertFalse(validator.isValidTweet("test \uFFFF"));
        assertFalse(validator.isValidTweet("test \uFEFF"));
    }

    @Test
    public void testDirectionChangeCharacters() {
        assertTrue(validator.isValidTweet("test \u202A test"));
        assertTrue(validator.isValidTweet("test \u202B test"));
        assertTrue(validator.isValidTweet("test \u202C test"));
        assertTrue(validator.isValidTweet("test \u202D test"));
        assertTrue(validator.isValidTweet("test \u202E test"));
        assertTrue(validator.isValidTweet("test \u202F test"));
    }

    @Test
    public void testAccentCharacters() {
        String c = "\u0065\u0301";
        StringBuilder builder = new StringBuilder();
        for (int i = 0; i < 279; i++) {
            builder.append(c);
        }
        assertTrue(v3Validator.isValidTweet(builder.toString()));
        assertTrue(v3Validator.isValidTweet(builder.append(c).toString()));
        assertFalse(v3Validator.isValidTweet(builder.append(c).toString()));
    }

    @Test
    public void testMutiByteCharacters() {
        String c = "\ud83d\ude02";
        StringBuilder builder = new StringBuilder();
        for (int i = 0; i < 139; i++) {
            builder.append(c);
        }
        assertTrue(validator.isValidTweet(builder.toString()));
        assertTrue(validator.isValidTweet(builder.append(c).toString()));
        assertFalse(validator.isValidTweet(builder.append(c).toString()));
    }
}
