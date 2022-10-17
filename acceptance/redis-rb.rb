#!/usr/bin/env ruby

require "redis"
require "pry"
require "pp"
require "benchmark"

def incr_tests
  25.times { @redis.incr("incr-test") }

  puts "Incr test final value: #{@redis.get("incr-test")}, Expected: 25"
end

def decr_tests
  3.times { @redis.incr("decr-test") }
  2.times { @redis.decr("decr-test") }

  puts "Decr test final value: #{@redis.get("decr-test")}, Expected: 1"
end

def basic_tests
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
end

def multi_client_tests
  15.times do |n|
    redis = Redis.new(host: "127.0.0.1", port: 7379, db: 15)
    redis.set("test-#{n}", 23)
    redis.close
  end
end

def test_with_long_running_connections
  puts "Testing with long running connections."
  redis = Redis.new(host: "127.0.0.1", port: 7379, db: 15)
  redis.set("test_with_long_running_connections", 500)
  sleep(6);
  pp redis.get("test_with_long_running_connections")
end

@redis = Redis.new(host: "127.0.0.1", port: 7379, db: 15)
puts @redis.set("test", 23)

time = Benchmark.realtime do
  @redis.del("incr-test")
  @redis.del("decr-test")

  puts "Running Redis tests\n\n"
  basic_tests
  incr_tests
  decr_tests
  multi_client_tests
  # test_with_long_running_connections
end
puts "Total time: #{time}"

@redis.close