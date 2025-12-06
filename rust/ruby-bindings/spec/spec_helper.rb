# frozen_string_literal: true

ENV['HOME'] ||= '.'

require 'rspec'

RSpec.configure do |config|
    config.expect_with :rspec do |expectations|
        expectations.include_chain_clauses_in_custom_matcher_descriptions = true
    end

    config.mock_with :rspec do |mocks|
        mocks.verify_partial_doubles = true
    end

    config.shared_context_metadata_behavior = :apply_to_host_groups

    config.warnings = true
    config.filter_run_when_matching :focus
    config.order = :random
    Kernel.srand config.seed
end
