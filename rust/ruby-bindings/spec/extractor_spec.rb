require_relative 'spec_helper'
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

        expect(yaml["tests"]["mentions_with_indices"].length).to be > 0
        yaml["tests"]["mentions_with_indices"].each { |test|
            entities = extractor.extract_mentioned_screennames_with_indices(test["text"])
            expect(test["expected"].length).to eq entities.size
            for index in (0...entities.size)
                expect(entities[index].value).to eq test['expected'][index]["screen_name"]
                expect(entities[index].start).to eq test['expected'][index]["indices"][0]
                expect(entities[index].end).to eq test['expected'][index]["indices"][1]
            end
        }

        expect(yaml["tests"]["mentions_or_lists_with_indices"].length).to be > 0
        yaml["tests"]["mentions_or_lists_with_indices"].each { |test|
            entities = extractor.extract_mentions_or_lists_with_indices(test["text"])
            expect(test["expected"].length).to eq entities.size
            for index in (0...entities.size)
                expect(entities[index].value).to eq test['expected'][index]["screen_name"]
                expect(entities[index].list_slug).to eq test['expected'][index]["list_slug"]
                expect(entities[index].start).to eq test['expected'][index]["indices"][0]
                expect(entities[index].end).to eq test['expected'][index]["indices"][1]
            end
        }

        expect(yaml["tests"]["replies"].length).to be > 0
        yaml["tests"]["replies"].each { |test|
            entity = extractor.extract_reply_screenname(test["text"])
            if entity != nil then
                expect(entity.value).to eq test['expected']
            else
                expect(test["expected"]).to be nil
            end
        }

        expect(yaml["tests"]["urls"].length).to be > 0
        yaml["tests"]["urls"].each { |test|
            urls = extractor.extract_urls(test["text"])
            for index in (0...urls.length)
                expect(urls[index]).to eq test['expected'][index]
            end
        }

        expect(yaml["tests"]["urls_with_indices"].length).to be > 0
        yaml["tests"]["urls_with_indices"].each { |test|
            entities = extractor.extract_urls_with_indices(test["text"])
            expect(test["expected"].length).to eq entities.size
            for index in (0...entities.size)
                expect(entities[index].value).to eq test['expected'][index]["url"]
                expect(entities[index].start).to eq test['expected'][index]["indices"][0]
                expect(entities[index].end).to eq test['expected'][index]["indices"][1]
            end
        }

        expect(yaml["tests"]["urls_with_directional_markers"].length).to be > 0
        yaml["tests"]["urls_with_directional_markers"].each { |test|
            entities = extractor.extract_urls_with_indices(test["text"])
            expect(test["expected"].length).to eq entities.size
            for index in (0...entities.size)
                expect(entities[index].value).to eq test['expected'][index]["url"]
                expect(entities[index].start).to eq test['expected'][index]["indices"][0]
                expect(entities[index].end).to eq test['expected'][index]["indices"][1]
            end
        }

        expect(yaml["tests"]["tco_urls_with_params"].length).to be > 0
        yaml["tests"]["tco_urls_with_params"].each { |test|
            entities = extractor.extract_urls_with_indices(test["text"])
            expect(test["expected"].length).to eq entities.size
            for index in (0...entities.size)
                expect(entities[index].value).to eq test['expected'][index]
            end
        }

        expect(yaml["tests"]["hashtags"].length).to be > 0
        yaml["tests"]["hashtags"].each { |test|
            hashtags = extractor.extract_hashtags(test["text"])
            for index in (0...hashtags.length)
                expect(hashtags[index]).to eq test['expected'][index]
            end
        }

        expect(yaml["tests"]["hashtags_from_astral"].length).to be > 0
        yaml["tests"]["hashtags_from_astral"].each { |test|
            hashtags = extractor.extract_hashtags(test["text"])
            for index in (0...hashtags.length)
                expect(hashtags[index]).to eq test['expected'][index]
            end
        }

        expect(yaml["tests"]["hashtags_with_indices"].length).to be > 0
        yaml["tests"]["hashtags_with_indices"].each { |test|
            entities = extractor.extract_hashtags_with_indices(test["text"])
            expect(test["expected"].length).to eq entities.size
            for index in (0...entities.size)
                expect(entities[index].value).to eq test['expected'][index]["hashtag"]
                expect(entities[index].start).to eq test['expected'][index]["indices"][0]
                expect(entities[index].end).to eq test['expected'][index]["indices"][1]
            end
        }

        expect(yaml["tests"]["cashtags"].length).to be > 0
        yaml["tests"]["cashtags"].each { |test|
            hashtags = extractor.extract_cashtags(test["text"])
            for index in (0...hashtags.length)
                expect(hashtags[index]).to eq test['expected'][index]
            end
        }

        expect(yaml["tests"]["cashtags_with_indices"].length).to be > 0
        yaml["tests"]["cashtags_with_indices"].each { |test|
            entities = extractor.extract_cashtags_with_indices(test["text"])
            expect(test["expected"].length).to eq entities.size
            for index in (0...entities.size)
                expect(entities[index].value).to eq test['expected'][index]["cashtag"]
                expect(entities[index].start).to eq test['expected'][index]["indices"][0]
                expect(entities[index].end).to eq test['expected'][index]["indices"][1]
            end
        }
    end

    it 'extracts federated mentions' do
        extractor = Twittertext::Extractor.new

        # Simple federated mention
        mentions = extractor.extract_federated_mentions("Hello @user@mastodon.social!")
        expect(mentions.length).to eq 1
        expect(mentions[0]).to eq "@user@mastodon.social"

        # With indices
        entities = extractor.extract_federated_mentions_with_indices("Hello @user@mastodon.social!")
        expect(entities.length).to eq 1
        expect(entities[0].value).to eq "@user@mastodon.social"
        expect(entities[0].start).to eq 6
        expect(entities[0].end).to eq 27
        expect(entities[0].entity_type).to eq 4  # FEDERATEDMENTION

        # Multiple federated mentions
        mentions = extractor.extract_federated_mentions("@alice@example.com and @bob@other.org")
        expect(mentions.length).to eq 2
        expect(mentions[0]).to eq "@alice@example.com"
        expect(mentions[1]).to eq "@bob@other.org"

        # Mixed with regular mentions (extractFederatedMentions includes both)
        mentions = extractor.extract_federated_mentions("@regular and @federated@domain.com")
        expect(mentions.length).to eq 2
        expect(mentions[0]).to eq "regular"  # Regular mentions don't include @ prefix
        expect(mentions[1]).to eq "@federated@domain.com"
    end

    it 'extracts entities with indices federated' do
        extractor = Twittertext::Extractor.new

        # Test that extract_entities_with_indices_federated includes federated mentions
        entities = extractor.extract_entities_with_indices_federated(
            "Check @user@mastodon.social and https://example.com #hashtag $CASH"
        )
        expect(entities.length).to eq 4

        # Find federated mention
        federated = entities.find { |e| e.entity_type == 4 }
        expect(federated).not_to be nil
        expect(federated.value).to eq "@user@mastodon.social"

        # Regular extract_entities_with_indices should NOT include federated mentions
        regular_entities = extractor.extract_entities_with_indices(
            "Check @user@mastodon.social and https://example.com #hashtag $CASH"
        )
        federated_in_regular = regular_entities.find { |e| e.entity_type == 4 }
        expect(federated_in_regular).to be nil
    end
end

RSpec.describe Twittertext::ValidatingExtractor do
    it 'has a working constructor' do
        extractor = Twittertext::ValidatingExtractor.new(Twittertext::TwitterTextConfiguration.new)
        expect(extractor).not_to be nil
    end

    it 'has working accessors' do
        extractor = Twittertext::ValidatingExtractor.new(Twittertext::TwitterTextConfiguration.new)

        expect(extractor.get_extract_url_without_protocol).to eq true
        extractor.set_extract_url_without_protocol(false)
        expect(extractor.get_extract_url_without_protocol).to eq false


        expect(extractor.get_normalize).to eq true
        extractor.set_normalize(false)
        expect(extractor.get_normalize).to eq false
    end

    it 'passes the conformance tests' do
        extractor = Twittertext::ValidatingExtractor.new(Twittertext::TwitterTextConfiguration.new)
        yaml = YAML.load_file("rust/conformance/tests/extract.yml")

        expect(yaml["tests"]["mentions_with_indices"].length).to be > 0
        yaml["tests"]["mentions_with_indices"].each { |test|
            result = extractor.extract_mentions_or_lists_with_indices(test["text"])
            expect(test["expected"].length).to eq result.entities.size
            for index in (0...result.entities.size)
                expect(result.entities[index].value).to eq test['expected'][index]["screen_name"]
                expect(result.entities[index].start).to eq test['expected'][index]["indices"][0]
                expect(result.entities[index].end).to eq test['expected'][index]["indices"][1]
            end
        }

        expect(yaml["tests"]["mentions_or_lists_with_indices"].length).to be > 0
        yaml["tests"]["mentions_or_lists_with_indices"].each { |test|
            result = extractor.extract_mentions_or_lists_with_indices(test["text"])
            expect(test["expected"].length).to eq result.entities.size
            for index in (0...result.entities.size)
                expect(result.entities[index].value).to eq test['expected'][index]["screen_name"]
                expect(result.entities[index].list_slug).to eq test['expected'][index]["list_slug"]
                expect(result.entities[index].start).to eq test['expected'][index]["indices"][0]
                expect(result.entities[index].end).to eq test['expected'][index]["indices"][1]
            end
        }

        expect(yaml["tests"]["replies"].length).to be > 0
        yaml["tests"]["replies"].each { |test|
            result = extractor.extract_reply_screenname(test["text"])
            if result.entity != nil then
                expect(result.entity.value).to eq test['expected']
            else
                expect(test["expected"]).to be nil
            end
        }

        expect(yaml["tests"]["urls_with_indices"].length).to be > 0
        yaml["tests"]["urls_with_indices"].each { |test|
            result = extractor.extract_urls_with_indices(test["text"])
            expect(test["expected"].length).to eq result.entities.size
            for index in (0...result.entities.size)
                expect(result.entities[index].value).to eq test['expected'][index]["url"]
                expect(result.entities[index].start).to eq test['expected'][index]["indices"][0]
                expect(result.entities[index].end).to eq test['expected'][index]["indices"][1]
            end
        }

        expect(yaml["tests"]["urls_with_directional_markers"].length).to be > 0
        yaml["tests"]["urls_with_directional_markers"].each { |test|
            result = extractor.extract_urls_with_indices(test["text"])
            expect(test["expected"].length).to eq result.entities.size
            for index in (0...result.entities.size)
                expect(result.entities[index].value).to eq test['expected'][index]["url"]
                expect(result.entities[index].start).to eq test['expected'][index]["indices"][0]
                expect(result.entities[index].end).to eq test['expected'][index]["indices"][1]
            end
        }

        expect(yaml["tests"]["hashtags_with_indices"].length).to be > 0
        yaml["tests"]["hashtags_with_indices"].each { |test|
            result = extractor.extract_hashtags_with_indices(test["text"])
            expect(test["expected"].length).to eq result.entities.size
            for index in (0...result.entities.size)
                expect(result.entities[index].value).to eq test['expected'][index]["hashtag"]
                expect(result.entities[index].start).to eq test['expected'][index]["indices"][0]
                expect(result.entities[index].end).to eq test['expected'][index]["indices"][1]
            end
        }

        expect(yaml["tests"]["cashtags_with_indices"].length).to be > 0
        yaml["tests"]["cashtags_with_indices"].each { |test|
            result = extractor.extract_cashtags_with_indices(test["text"])
            expect(test["expected"].length).to eq result.entities.size
            for index in (0...result.entities.size)
                expect(result.entities[index].value).to eq test['expected'][index]["cashtag"]
                expect(result.entities[index].start).to eq test['expected'][index]["indices"][0]
                expect(result.entities[index].end).to eq test['expected'][index]["indices"][1]
            end
        }
    end

    it 'extracts federated mentions' do
        extractor = Twittertext::ValidatingExtractor.new(Twittertext::TwitterTextConfiguration.new)

        # Simple federated mention
        mentions = extractor.extract_federated_mentions("Hello @user@mastodon.social!")
        expect(mentions.length).to eq 1
        expect(mentions[0]).to eq "@user@mastodon.social"

        # With indices
        result = extractor.extract_federated_mentions_with_indices("Hello @user@mastodon.social!")
        expect(result.entities.length).to eq 1
        expect(result.entities[0].value).to eq "@user@mastodon.social"
        expect(result.entities[0].entity_type).to eq 4  # FEDERATEDMENTION
    end

    it 'extracts entities with indices federated' do
        extractor = Twittertext::ValidatingExtractor.new(Twittertext::TwitterTextConfiguration.new)

        # Test that extract_entities_with_indices_federated includes federated mentions
        result = extractor.extract_entities_with_indices_federated(
            "Check @user@mastodon.social and https://example.com #hashtag"
        )

        # Find federated mention
        federated = result.entities.find { |e| e.entity_type == 4 }
        expect(federated).not_to be nil
        expect(federated.value).to eq "@user@mastodon.social"
    end
end
