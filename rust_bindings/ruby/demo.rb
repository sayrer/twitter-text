require "rust_bindings/ruby/twittertext"

autolinker = Twittertext::Autolinker.new

puts autolinker.autolink "What's up $TWTR or #TWTR or @twitter?"