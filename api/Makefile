# Default build rule for SAM CLI
.PHONY: build build-all build-ProcessLambda build-HealthLambda build-StatusLambda build-TokenInfoLambda

# General build rule for all Lambda functions
build-%:
	cargo build --release --target x86_64-unknown-linux-musl
	mkdir -p $(ARTIFACTS_DIR)
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap $(ARTIFACTS_DIR)/bootstrap

# Build all Lambda functions
build-all: build-ProcessLambda build-HealthLambda build-StatusLambda build-TokenInfoLambda

# Specific build rules for individual Lambdas (if needed)
build-ProcessLambda:
	cargo build --release --target x86_64-unknown-linux-musl
	mkdir -p $(ARTIFACTS_DIR)
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap $(ARTIFACTS_DIR)/bootstrap

build-HealthLambda:
	cargo build --release --target x86_64-unknown-linux-musl
	mkdir -p $(ARTIFACTS_DIR)
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap $(ARTIFACTS_DIR)/bootstrap

build-StatusLambda:
	cargo build --release --target x86_64-unknown-linux-musl
	mkdir -p $(ARTIFACTS_DIR)
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap $(ARTIFACTS_DIR)/bootstrap

build-TokenInfoLambda:
	cargo build --release --target x86_64-unknown-linux-musl
	mkdir -p $(ARTIFACTS_DIR)
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap $(ARTIFACTS_DIR)/bootstrap
