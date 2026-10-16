require_relative 'spec_helper'
require 'yaml'

RSpec.describe Twittertext::Validator do
    it 'has a working constructor' do
        validator = Twittertext::Validator.new
        expect(validator).not_to be nil
    end

    it 'has working accessors' do
        validator = Twittertext::Validator.new

        expect(validator.get_max_tweet_length).to eq 280

        expect(validator.get_short_url_length).to eq 23
        validator.set_short_url_length(42)
        expect(validator.get_short_url_length).to eq 42

        expect(validator.get_short_url_length_https).to eq 23
        validator.set_short_url_length_https(43)
        expect(validator.get_short_url_length_https).to eq 43
    end

    it 'passes conformance tests' do
        validator = Twittertext::Validator.new
        yaml = YAML.load_file("rust/conformance/tests/validate.yml")

        expect(yaml["tests"]["tweets"].length).to be > 0
        yaml["tests"]["tweets"].each { |test| 
            expect(validator.is_valid_tweet(test["text"])).to eq(test["expected"])
        }

        expect(yaml["tests"]["usernames"].length).to be > 0
        yaml["tests"]["usernames"].each { |test| 
            expect(validator.is_valid_username(test["text"])).to eq(test["expected"])
        }

        expect(yaml["tests"]["lists"].length).to be > 0
        yaml["tests"]["lists"].each { |test| 
            expect(validator.is_valid_list(test["text"])).to eq(test["expected"])
        }

        expect(yaml["tests"]["hashtags"].length).to be > 0
        yaml["tests"]["hashtags"].each { |test| 
            expect(validator.is_valid_hashtag(test["text"])).to eq(test["expected"])
        }

        expect(yaml["tests"]["urls"].length).to be > 0
        yaml["tests"]["urls"].each { |test| 
            expect(validator.is_valid_url(test["text"])).to eq(test["expected"])
        }

        expect(yaml["tests"]["urls_without_protocol"].length).to be > 0
        yaml["tests"]["urls_without_protocol"].each { |test| 
            expect(validator.is_valid_url_without_protocol(test["text"])).to eq(test["expected"])
        }
    end

    it 'validates weighting' do
        validator = Twittertext::Validator.new
        yaml = YAML.load_file("rust/conformance/tests/validate.yml")

        validate_weighting(yaml["tests"]["WeightedTweetsCounterTest"], Twittertext::TwitterTextConfiguration.config_v2)
        validate_weighting(yaml["tests"]["WeightedTweetsWithDiscountedEmojiCounterTest"], Twittertext::TwitterTextConfiguration.config_v3)
        validate_weighting(yaml["tests"]["UnicodeDirectionalMarkerCounterTest"], Twittertext::TwitterTextConfiguration.config_v3)
    end

    it 'weighs runes like Latin under v4 only' do
        # https://github.com/twitter/twitter-text/issues/430
        runes = "\u16A0\u16A2\u16A6\u16A8\u16B1\u16B2"

        v4 = Twittertext::TwitterTextParser.parse(runes, Twittertext::TwitterTextConfiguration.config_v4, true)
        expect(v4.weighted_length).to eq 6
        expect(v4.is_valid).to eq true

        v3 = Twittertext::TwitterTextParser.parse(runes, Twittertext::TwitterTextConfiguration.config_v3, true)
        expect(v3.weighted_length).to eq 12

        # v4 is opt-in: the default configuration still weighs runes as logograms.
        default = Twittertext::TwitterTextParser.parse(runes, Twittertext::TwitterTextConfiguration.new, true)
        expect(default.weighted_length).to eq 12
    end

    {
        'Latin Extended Additional' => "\u1E00\u1E01\u1E02\u1E03\u1E04\u1E05",
        'Greek Extended' => "\u1F00\u1F01\u1F02\u1F03\u1F04\u1F05",
        'Ethiopic' => "\u1200\u1208\u1210\u1218\u1220\u1228",
        'Cherokee' => "\u13A0\u13A1\u13A2\u13A3\u13A4\u13A5",
        'Khmer' => "\u1780\u1781\u1782\u1783\u1784\u1785",
        'Arabic Presentation Forms-A' => "\uFB50\uFB51\uFB52\uFB53\uFB54\uFB55",
        'Adlam' => "\u{1E900}\u{1E901}\u{1E902}\u{1E903}\u{1E904}\u{1E905}",
    }.each do |name, text|
        it "weighs #{name} like Latin under v4 only" do
            # Six letters from an alphabetic block above U+10FF weigh six units
            # under v4 and twelve under v3 and under the default.
            expect(text.length).to eq 6

            v4 = Twittertext::TwitterTextParser.parse(text, Twittertext::TwitterTextConfiguration.config_v4, true)
            expect(v4.weighted_length).to eq 6
            expect(v4.is_valid).to eq true

            v3 = Twittertext::TwitterTextParser.parse(text, Twittertext::TwitterTextConfiguration.config_v3, true)
            expect(v3.weighted_length).to eq 12

            default = Twittertext::TwitterTextParser.parse(text, Twittertext::TwitterTextConfiguration.new, true)
            expect(default.weighted_length).to eq 12
        end
    end
end

def validate_weighting(tests, config)
    expect(tests.length).to be > 0
    tests.each { |test|
        result = Twittertext::TwitterTextParser.parse(test["text"], config, true)
        expect(test["expected"]["weightedLength"]).to eq result.weighted_length
        expect(test["expected"]["valid"]).to eq result.is_valid
        expect(test["expected"]["permillage"]).to eq result.permillage
        expect(test["expected"]["displayRangeStart"]).to eq result.display_text_range.start
        expect(test["expected"]["displayRangeEnd"]).to eq result.display_text_range.end
        expect(test["expected"]["validRangeStart"]).to eq result.valid_text_range.start
        expect(test["expected"]["validRangeEnd"]).to eq result.valid_text_range.end
    }
end
