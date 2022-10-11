require "redis"
require "pry"
require "pp"

def set_load_test
  redis = Redis.new(host: "127.0.0.1", port: 7379, db: 15)

  500_000.times do
    redis.set("test-#{rand(0..9999)}", rand(0..9999))
  end
end

5.times.map do
  Thread.new { set_load_test }
end.map(&:join)

