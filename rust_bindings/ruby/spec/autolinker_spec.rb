require 'rust_bindings/ruby/spec/spec_helper'
require 'rust_bindings/ruby/twittertext'

RSpec.describe Twittertext::Autolinker do
    context 'test accessors' do
        its(:get_no_follow) { should eq false }
    end
end

