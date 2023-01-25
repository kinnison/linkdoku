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
