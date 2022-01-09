redis:
	mini-redis-server &

channels: redis
	cargo run --example channels
	@pkill mini-redis-server
