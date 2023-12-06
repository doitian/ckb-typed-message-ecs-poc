SCHEMAS_OUT := $(patsubst %.mol,crates/ckb-ecs-schemas/src/%.rs,$(wildcard schemas/*.mol))

test: unit-test contracts-test

unit-test: ${SCHEMAS_OUT}
	cargo test

contracts-test: build
	capsule test

build: ${SCHEMAS_OUT}
	capsule build

setup:
	capsule --version || cargo install ckb-capsule --locked
	moleculec --version || cargo install moleculec@0.7.5 --locked

crates/ckb-ecs-schemas/src/schemas/%.rs: schemas/%.mol
	moleculec --language rust --schema-file $< > $@
	cargo fmt

.PHONY: test unit-test contracts-test build setup
