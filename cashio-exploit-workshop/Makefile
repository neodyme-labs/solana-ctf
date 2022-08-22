.PHONY: poc

DEP = \
	cashio/target/deploy/bankman.so \
	cashio/target/deploy/brrr.so

poc: $(DEP)
	cargo run -p cashio-poc

cashio/target/deploy/bankman.so: cashio/programs/bankman/*
	cd cashio/programs/bankman; cargo build-bpf

cashio/target/deploy/brrr.so: cashio/programs/brrr/*
	cd cashio/programs/brrr; cargo build-bpf
