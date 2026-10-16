require_relative 'spec_helper'

RSpec.describe Twittertext::TwitterTextConfiguration do
    it 'has a working constructor' do
        config = Twittertext::TwitterTextConfiguration.new
        # The default configuration is v3; v4 is opt-in.
        expect(config.get_version).to eq 3
    end

    it 'has working path loading' do
        path = "rust/cpp-bindings/test_data/test_config.json"
        config = Twittertext::TwitterTextConfiguration.configuration_from_path(path)
        expect(config.get_version).to eq 42
        expect(config.get_max_weighted_tweet_length).to eq 400
        expect(config.get_scale).to eq 43
        expect(config.get_default_weight).to eq 213
        expect(config.get_transformed_url_length).to eq 32

        ranges = config.get_ranges
        expect(ranges.length).to eq 1
        wr = ranges[0]
        expect(wr.range.start).to eq 0
        expect(wr.range.end).to eq 4351
        expect(wr.weight).to eq 200
    end

    it 'has working json reading' do
        path = "rust/cpp-bindings/test_data/test_config.json"

        begin
            f = File.open(path)
            json = f.read
            config = Twittertext::TwitterTextConfiguration.configuration_from_json(json)
            expect(config.get_version).to eq 42
            expect(config.get_max_weighted_tweet_length).to eq 400
            expect(config.get_scale).to eq 43
            expect(config.get_default_weight).to eq 213
            expect(config.get_transformed_url_length).to eq 32

            ranges = config.get_ranges
            expect(ranges.length).to eq 1
            wr = ranges[0]
            expect(wr.range.start).to eq 0
            expect(wr.range.end).to eq 4351
            expect(wr.weight).to eq 200
        ensure
            f.close
        end
    end

    it 'has working accessors' do
        config = Twittertext::TwitterTextConfiguration.new

        expect(config.get_version).to eq 3
        config.set_version(199)
        expect(config.get_version).to eq 199

        expect(config.get_max_weighted_tweet_length).to eq 280
        config.set_max_weighted_tweet_length(199)
        expect(config.get_max_weighted_tweet_length).to eq 199

        expect(config.get_scale).to eq 100
        config.set_scale(199)
        expect(config.get_scale).to eq 199

        expect(config.get_default_weight).to eq 200
        config.set_default_weight(199)
        expect(config.get_default_weight).to eq 199

        expect(config.get_transformed_url_length).to eq 23
        config.set_transformed_url_length(199)
        expect(config.get_transformed_url_length).to eq 199

        expect(config.get_emoji_parsing_enabled).to eq true
        config.set_emoji_parsing_enabled(false)
        expect(config.get_emoji_parsing_enabled).to eq false
    end

    it 'has the correct default ranges' do
        # The default configuration is v3; v4 is opt-in.
        config = Twittertext::TwitterTextConfiguration.new
        expect(config.get_version).to eq 3
        ranges = config.get_ranges
        expect(ranges.length).to eq 4

        expect(ranges[0].range.start).to eq 0
        expect(ranges[0].range.end).to eq 4351
        expect(ranges[0].weight).to eq 100

        expect(ranges[1].range.start).to eq 8192
        expect(ranges[1].range.end).to eq 8205
        expect(ranges[1].weight).to eq 100

        expect(ranges[2].range.start).to eq 8208
        expect(ranges[2].range.end).to eq 8223
        expect(ranges[2].weight).to eq 100

        expect(ranges[3].range.start).to eq 8242
        expect(ranges[3].range.end).to eq 8247
        expect(ranges[3].weight).to eq 100
    end

    it 'has a config v4' do
        config = Twittertext::TwitterTextConfiguration.config_v4
        expect(config.get_version).to eq 4
        expect(config.get_emoji_parsing_enabled).to eq true
        ranges = config.get_ranges
        v3_ranges = Twittertext::TwitterTextConfiguration.config_v3.get_ranges
        # v4 is v3 plus the alphabetic scripts above U+10FF.
        expect(ranges.length).to be > v3_ranges.length
        v3_ranges.each do |v3_range|
            expect(ranges.any? { |r| r.range.start == v3_range.range.start &&
                                     r.range.end == v3_range.range.end &&
                                     r.weight == v3_range.weight }).to eq true
        end

        covers = lambda do |first, last|
            ranges.any? { |r| r.range.start <= first && r.range.end >= last && r.weight == 100 }
        end
        expect(covers.call(0x16A0, 0x16FF)).to eq true    # Runic
        expect(covers.call(0x1200, 0x137F)).to eq true    # Ethiopic
        expect(covers.call(0x1780, 0x17FF)).to eq true    # Khmer
        expect(covers.call(0x1E00, 0x1EFF)).to eq true    # Latin Extended Additional
        expect(covers.call(0x1F00, 0x1FFF)).to eq true    # Greek Extended
        expect(covers.call(0xFB50, 0xFDFF)).to eq true    # Arabic Presentation Forms-A
        expect(covers.call(0x1E900, 0x1E95F)).to eq true  # Adlam
        expect(covers.call(0x4E00, 0x9FFF)).to eq false   # CJK Unified Ideographs
        expect(covers.call(0xAC00, 0xD7AF)).to eq false   # Hangul Syllables
        expect(covers.call(0x1D400, 0x1D7FF)).to eq false # Mathematical Alphanumeric Symbols
    end

    it 'has a config v3' do
        config = Twittertext::TwitterTextConfiguration.config_v3
        expect(config.get_version).to eq 3
        expect(config.get_emoji_parsing_enabled).to eq true
        expect(config.get_ranges.length).to eq 4
    end

    it 'has a config v2' do
        config = Twittertext::TwitterTextConfiguration.config_v2
        expect(config.get_version).to eq 2
        expect(config.get_emoji_parsing_enabled).to eq false
        expect(config.get_ranges.length).to eq 4
    end

    it 'has a config v1' do
        config = Twittertext::TwitterTextConfiguration.config_v1
        expect(config.get_version).to eq 1
        expect(config.get_emoji_parsing_enabled).to eq false
        expect(config.get_ranges.length).to eq 0
    end
end
