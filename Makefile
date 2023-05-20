.DEFAULT: help

help:
	@echo "You can run:"
	@echo " - 'make base-builder' to get the base-builder in place"
	@echo " - 'make build' to build it for scaleway"
	@echo " - 'make push' to just push to scaleway"

SCALEWAY_TAG := rg.fr-par.scw.cloud/funcscwlinkdokutestoawqzyvy/linkdoku:beta

base-builder:
	@docker build -t linkdoku:base-builder --target base-builder .

build:
	@docker build -t $(SCALEWAY_TAG) .

push:
	@docker push $(SCALEWAY_TAG)

clean:
	@docker rmi $(SCALEWAY_TAG)
	@docker system prune

reallyclean:
	@docker rmi linkdoku:base-builder
	@docker rmi $(SCALEWAY_TAG)
	@docker system prune

run:
	@reset
	@cd css; make
	@touch components/src/lib.rs
	@touch frontend-core/src/lib.rs
	@cd frontend; trunk build index.html
	@cd backend; env RUST_BACKTRACE=1 RUST_LOG=info cargo run --target=x86_64-unknown-linux-musl -- --config linkdoku-config-dev.yaml

release:
	@reset || true
	@cd css; cargo run
	@touch components/src/lib.rs
	@touch frontend-core/src/lib.rs
	@cd frontend; trunk build --release index.html
	@cd backend; cargo build --release --target=x86_64-unknown-linux-musl
