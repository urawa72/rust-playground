redis:
	mini-redis-server &

channels: redis
	cargo run --example channels
	@pkill mini-redis-server

echo-client:
	nc localhost 6142

echo-server:
	cargo run --example echo-server

echo-server-copy:
	cargo run --example echo-server-copy

streams: redis
	cargo run --example streams
	@pkill mini-redis-server
