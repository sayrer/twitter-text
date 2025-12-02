require_relative 'spec_helper'
require 'rust_bindings/ruby/twittertext'
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
