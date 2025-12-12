# frozen_string_literal: true

ENV['HOME'] ||= '.'

# In Bazel runfiles, the .so file is in rust/ruby-bindings directory
# We need to find it via the runfiles directory, not via the source path
runfiles_dir = ENV['RUNFILES_DIR'] || File.dirname(File.dirname(__FILE__))
twittertext_so = File.join(runfiles_dir, '_main', 'rust', 'ruby-bindings', 'twittertext.so')
require twittertext_so

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
