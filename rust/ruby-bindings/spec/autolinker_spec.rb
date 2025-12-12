require_relative 'spec_helper'
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

    it 'can add custom attributes to hashtags' do
        autolinker = Twittertext::Autolinker.new

        # Create a modifier that adds data-custom="test" to hashtags
        modifier = Twittertext::AddAttributeModifier.new(["HASHTAG"], "data-custom", "test")
        autolinker.set_add_attribute_modifier(modifier)

        result = autolinker.autolink("#test")
        expect(result).to include("data-custom=\"test\"")
        expect(result).to include("#test")
    end

    it 'can add custom attributes to multiple entity types' do
        autolinker = Twittertext::Autolinker.new

        # Create a modifier that adds data-track="true" to both hashtags and mentions
        modifier = Twittertext::AddAttributeModifier.new(["HASHTAG", "MENTION"], "data-track", "true")
        autolinker.set_add_attribute_modifier(modifier)

        result = autolinker.autolink("#hashtag @mention")
        expect(result.scan(/data-track="true"/).length).to eq(2)
    end

    it 'can replace class attribute' do
        autolinker = Twittertext::Autolinker.new

        # Create a modifier that replaces the class attribute
        modifier = Twittertext::ReplaceClassModifier.new("custom-link")
        autolinker.set_replace_class_modifier(modifier)

        result = autolinker.autolink_hashtags("#test")
        expect(result).to include("class=\"custom-link\"")
        expect(result).not_to include("tweet-url hashtag")
    end

    it 'can add custom attributes to URLs' do
        autolinker = Twittertext::Autolinker.new

        # Create a modifier that adds target="_blank" to URLs
        modifier = Twittertext::AddAttributeModifier.new(["URL"], "target", "_blank")
        autolinker.set_add_attribute_modifier(modifier)

        result = autolinker.autolink_urls("http://example.com")
        expect(result).to include("target=\"_blank\"")
    end

    it 'can modify link text' do
        autolinker = Twittertext::Autolinker.new

        # Create a modifier that changes link text based on entity type
        modifier = Twittertext::LinkTextModifier.new(lambda do |entity, text|
            if entity[:type] == "HASHTAG"
                "#replaced"
            else
                "pre_#{text}_post"
            end
        end)
        autolinker.set_link_text_modifier(modifier)

        result = autolinker.autolink("#hash @mention")
        expect(result).to match(/<a[^>]+>#replaced<\/a>/)
        expect(result).to match(/<a[^>]+>pre_mention_post<\/a>/)
    end

    it 'can modify link text with symbol tags' do
        autolinker = Twittertext::Autolinker.new

        # Create a modifier that wraps text with pre/post
        modifier = Twittertext::LinkTextModifier.new(lambda do |entity, text|
            "pre_#{text}_post"
        end)
        autolinker.set_link_text_modifier(modifier)
        autolinker.set_symbol_tag("s")
        autolinker.set_text_with_symbol_tag("b")
        autolinker.set_username_include_symbol(true)

        result = autolinker.autolink("#hash @mention")
        expect(result).to match(/<a[^>]+>pre_<s>#<\/s><b>hash<\/b>_post<\/a>/)
        expect(result).to match(/<a[^>]+>pre_<s>@<\/s><b>mention<\/b>_post<\/a>/)
    end
end
