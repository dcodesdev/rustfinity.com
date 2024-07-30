TAG ?= staging

build:
ifeq ($(TAG), staging)
	docker build --no-cache -t rustfinity-runner:$(TAG) . -f Dockerfile.staging
else
	docker build --no-cache -t rustfinity-runner:$(TAG) . -f Dockerfile
endif
