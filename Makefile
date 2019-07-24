.PHONY: bench report docker

bench:
	cargo +stable bench -p crypto_bench_ring

report:
	http-server ring/target/criterion

docker:
	docker build -f docker/Dockerfile -t crypto_bench .

scone:
	docker build -f docker/Dockerfile.sgx -t crypto_bench_scone .
