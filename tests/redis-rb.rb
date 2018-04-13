require "redis"
require "pry"
require "pp"

redis = Redis.new(host: "127.0.0.1", port: 6379, db: 15)
redis.set("test", 23)

# pp redis.get("test")
