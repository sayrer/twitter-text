require "rust_bindings/ruby/twittertext"

autolinker = Twittertext::Autolinker.new

puts autolinker.autolink "Just setting up my $TWTR or #TWTR or @twitter? Please visit https://twiter.com."