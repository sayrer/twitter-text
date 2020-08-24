require "pp"

puts pp $:
puts "hi"

require "rust_bindings/ruby/twittertext"

puts Twittertext