require 'rust_bindings/ruby/spec/spec_helper'
require 'rust_bindings/ruby/twittertext'
require 'yaml'

RSpec.describe Twittertext::Extractor do
    it 'has a working constructor' do
        extractor = Twittertext::Extractor.new
        expect(extractor).not_to be nil
    end

    it 'has working accessors' do
        extractor = Twittertext::Extractor.new

        expect(extractor.get_extract_url_without_protocol).to eq true
        extractor.set_extract_url_without_protocol(false)
        expect(extractor.get_extract_url_without_protocol).to eq false
    end

    it 'passes the conformance tests' do
        extractor = Twittertext::Extractor.new
        yaml = YAML.load_file("rust/conformance/tests/extract.yml")

        expect(yaml["tests"]["mentions"].length).to be > 0
        yaml["tests"]["mentions"].each { |test| 
            mentions = extractor.extract_mentioned_screennames(test["text"])
            for index in (0...mentions.length)
                expect(mentions[index]).to eq test['expected'][index]
            end
        }
    end
end
