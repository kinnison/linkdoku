.DEFAULT: help

Q=@

help:
	$(Q)echo "You can run:"
	$(Q)echo " - 'make base-builder' to get the base-builder in place"
	$(Q)echo " - 'make build' to build it for scaleway"
	$(Q)echo " - 'make push' to just push to scaleway"

SCALEWAY_TAG := rg.fr-par.scw.cloud/funcscwlinkdokutestoawqzyvy/linkdoku:beta

base-builder:
	$(Q)docker build -t linkdoku:base-builder --target base-builder .

build:
	$(Q)docker build -t $(SCALEWAY_TAG) .

push:
	$(Q)docker push $(SCALEWAY_TAG)

clean:
	$(Q)docker rmi $(SCALEWAY_TAG)
	$(Q)docker system prune

reallyclean:
	$(Q)docker rmi linkdoku:base-builder
	$(Q)docker rmi $(SCALEWAY_TAG)
	$(Q)docker system prune

run:
	$(Q)reset
	$(Q)cd css; cargo run
	$(Q)touch components/src/lib.rs
	$(Q)touch frontend-core/src/lib.rs
	$(Q)cd frontend; trunk build index.html
	$(Q)cd backend; env RUST_BACKTRACE=1 RUST_LOG=info cargo run --target=x86_64-unknown-linux-musl -- --config linkdoku-config-dev.yaml

release:
	$(Q)reset || true
	$(Q)cd css; cargo run
	$(Q)touch components/src/lib.rs
	$(Q)touch frontend-core/src/lib.rs
	$(Q)cd frontend; trunk -v build --release index.html
	$(Q)cd backend; cargo build --release --target=x86_64-unknown-linux-musl

install: release
	$(Q)mkdir -p $(DESTDIR)/bin
	$(Q)cp target/x86_64-unknown-linux-musl/release/backend $(DESTDIR)/bin/linkdoku

