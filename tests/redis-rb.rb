require "redis"
require "pry"
require "pp"



def incr_tests
  25.times { @redis.incr("incr-test") }

  puts "Incr test final value: #{@redis.get("incr-test")}, Expected: 25"
end

def decr_tests
  3.times { @redis.incr("decr-test") }
  2.times { @redis.decr("decr-test") }

  puts "Decr test final value: #{@redis.get("decr-test")}, Expected: 1"
end

@redis = Redis.new(host: "127.0.0.1", port: 6379, db: 15)

@redis.del("incr-test")
@redis.del("decr-test")
#
# puts "Running Reddis tests\n\n"
# incr_tests
# decr_tests




@redis.set("test", 23)
puts "test key"
pp @redis.get("test")

@redis.set("test", 24)
puts "modified test key"
pp @redis.get("test")

@redis.set("test-2", "nice")
puts "test-2 key"
pp @redis.get("test-2")

@redis.set("test-3", "Woohoo\r\nThis\r\nIs\r\nSuper\r\nWeird")
pp @redis.get("test-3")