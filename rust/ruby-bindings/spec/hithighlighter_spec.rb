require_relative 'spec_helper'
require 'rust_bindings/ruby/twittertext'
require 'yaml'

def array_to_hits(arr)
    hits = Twittertext::Hits.new
    arr.each { |hit| 
        h = Twittertext::Hit.new
        h.start = hit[0]
        h.end = hit[1]
        hits.push(h)
    }

    hits
end

RSpec.describe Twittertext::HitHighlighter do
    it 'has a working constructor' do
        hit_highlighter = Twittertext::HitHighlighter.new
        expect(hit_highlighter).not_to be nil
    end

    it 'has a working constructor with a string' do
        hit_highlighter = Twittertext::HitHighlighter.new('hmm')
        expect(hit_highlighter).not_to be nil
    end

    it 'passes the conformance tests' do
        hit_highlighter = Twittertext::HitHighlighter.new
        yaml = YAML.load_file("rust/conformance/tests/hit_highlighting.yml")

        expect(yaml["tests"]["plain_text"].length).to be > 0
        yaml["tests"]["plain_text"].each { |test| 
            expect(hit_highlighter.highlight(test["text"], array_to_hits(test["hits"]))).to eq test["expected"]
        }

        expect(yaml["tests"]["with_links"].length).to be > 0
        yaml["tests"]["with_links"].each { |test| 
            expect(hit_highlighter.highlight(test["text"], array_to_hits(test["hits"]))).to eq test["expected"]
        }
    end
end