require "redis"
require "pry"
require "pp"

redis = Redis.new(host: "127.0.0.1", port: 6379, db: 15)
redis.set("test", 23)
pp redis.get("test")

redis.set("test", 24)
pp redis.get("test")

redis.set("test-2", "nice")
pp redis.get("test-2")

redis.set("test-3", 'Woohoo\r\nThis\r\nIs\r\nSuper\r\nWeird')
pp redis.get("test-3")
