# frozen_string_literal: true

# Sets HOME here because:
# If otherwise, it causes a runtime failure with the following steps.
# 1. RSpec::Core::ConfigurationOptions.global_options_file raises an exception
#    because $HOME is not set in the sandbox environment of Bazel
# 2. the rescue clause calls RSpec::Support.#warning
# 3. #warning calls #warn_with
# 4. #warn_with tries to lookup the first caller which is not a part of RSpec.
#    But all the call stack entires are about RSpec at this time because
#    it is invoked by rpsec/autorun. So #warn_with raises an exception
# 5. The process fails with an unhandled exception.

ENV['HOME'] ||= '.'

require 'rspec'
require 'rspec/its'

puts "WASSSUP"
for arg in ARGV
   puts arg
end
puts "END"

RSpec.configure do |config|
	puts config.files_to_run

	puts "RUNNING"
  config.expect_with :rspec do |expectations|
    expectations.include_chain_clauses_in_custom_matcher_descriptions = true
  end
	puts "RUNNING 2"

  config.mock_with :rspec do |mocks|
    mocks.verify_partial_doubles = true
  end
	puts "RUNNING 3"

 # config.default_path = 'rust_bindings/foo/spec'

  config.shared_context_metadata_behavior = :apply_to_host_groups

  config.warnings = true
  config.filter_run_when_matching :focus
  # config.disable_monkey_patching!
  config.order = :random
  Kernel.srand config.seed
	puts "RUNNING END"
	puts config.files_to_run
end
