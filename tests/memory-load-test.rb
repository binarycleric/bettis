require "redis"
require "pry"
require "pp"

@redis = Redis.new(host: "127.0.0.1", port: 6379, db: 15)

def set_load_test
  500_000.times do
    @redis.set("test-#{rand(0..9999)}", rand(0..9999))
  end
end

5.times.map do
  Thread.new { set_load_test }
end.map(&:join)

