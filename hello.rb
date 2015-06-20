x = 5

h1 = Thread.new do
  x += 1;
end

puts x

h1.join

puts x
