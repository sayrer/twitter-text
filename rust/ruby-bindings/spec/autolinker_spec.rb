require_relative 'spec_helper'
require 'rust_bindings/ruby/twittertext'
require 'yaml'

RSpec.describe Twittertext::Autolinker do
    it 'has working accessors' do
        autolinker = Twittertext::Autolinker.new

        expect(autolinker.get_no_follow).to eq(false)
        autolinker.set_no_follow(true)
        expect(autolinker.get_no_follow).to eq(true)

        expect(autolinker.get_url_class).to eq("")
        autolinker.set_url_class("foo")
        expect(autolinker.get_url_class).to eq("foo")

        expect(autolinker.get_symbol_tag).to eq("")
        autolinker.set_symbol_tag("bar")
        expect(autolinker.get_symbol_tag).to eq("bar")

        expect(autolinker.get_list_class).to eq("tweet-url list-slug")
        autolinker.set_list_class("tweet-url list-slug foo")
        expect(autolinker.get_list_class).to eq("tweet-url list-slug foo")

        expect(autolinker.get_username_class).to eq("tweet-url username")
        autolinker.set_username_class("tweet-url username bar")
        expect(autolinker.get_username_class).to eq("tweet-url username bar")

        expect(autolinker.get_hashtag_class).to eq("tweet-url hashtag")
        autolinker.set_hashtag_class("tweet-url hashtag baz")
        expect(autolinker.get_hashtag_class).to eq("tweet-url hashtag baz")

        expect(autolinker.get_cashtag_class).to eq("tweet-url cashtag")
        autolinker.set_cashtag_class("tweet-url cashtag qux")
        expect(autolinker.get_cashtag_class).to eq("tweet-url cashtag qux")

        expect(autolinker.get_username_url_base).to eq("https://twitter.com/")
        autolinker.set_username_url_base("https://example.com/")
        expect(autolinker.get_username_url_base).to eq("https://example.com/")

        expect(autolinker.get_hashtag_url_base).to eq("https://twitter.com/search?q=%23")
        autolinker.set_hashtag_url_base("https://example.com/search?q=%23")
        expect(autolinker.get_hashtag_url_base).to eq("https://example.com/search?q=%23")

        expect(autolinker.get_cashtag_url_base).to eq("https://twitter.com/search?q=%24")
        autolinker.set_cashtag_url_base("https://example.com/search?q=%24")
        expect(autolinker.get_cashtag_url_base).to eq("https://example.com/search?q=%24")

        expect(autolinker.get_invisible_tag_attrs).to eq("style='position:absolute;left:-9999px;'")
        autolinker.set_invisible_tag_attrs("")
        expect(autolinker.get_invisible_tag_attrs).to eq("")

        expect(autolinker.get_username_include_symbol).to eq(false)
        autolinker.set_username_include_symbol(true)
        expect(autolinker.get_username_include_symbol).to eq(true)
    end

    it 'can roundtrip unicode' do
        autolinker = Twittertext::Autolinker.new

        expect(autolinker.get_url_class).to eq("")
        autolinker.set_url_class("foo 👳🏿‍♀️")
        expect(autolinker.get_url_class).to eq("foo 👳🏿‍♀️")
    end

    it 'passes conformance tests' do
        autolinker = Twittertext::Autolinker.new
        yaml = YAML.load_file("rust/conformance/tests/autolink.yml")

        expect(yaml["tests"]["usernames"].length).to be > 0
        yaml["tests"]["usernames"].each { |test| 
            expect(autolinker.autolink_usernames_and_lists(test["text"])).to eq(test["expected"])
        }

        expect(yaml["tests"]["lists"].length).to be > 0
        yaml["tests"]["lists"].each { |test| 
            expect(autolinker.autolink_usernames_and_lists(test["text"])).to eq(test["expected"])
        }

        expect(yaml["tests"]["hashtags"].length).to be > 0
        yaml["tests"]["hashtags"].each { |test| 
            expect(autolinker.autolink_hashtags(test["text"])).to eq(test["expected"])
        }

        expect(yaml["tests"]["urls"].length).to be > 0
        yaml["tests"]["urls"].each { |test| 
            expect(autolinker.autolink_urls(test["text"])).to eq(test["expected"])
        }

        expect(yaml["tests"]["cashtags"].length).to be > 0
        yaml["tests"]["cashtags"].each { |test| 
            expect(autolinker.autolink_cashtags(test["text"])).to eq(test["expected"])
        }

        expect(yaml["tests"]["all"].length).to be > 0
        yaml["tests"]["all"].each { |test| 
            expect(autolinker.autolink(test["text"])).to eq(test["expected"])
        }
    end
end
