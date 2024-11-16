TAG ?= staging

build:
ifeq ($(TAG), staging)
	docker build -t rustfinity-runner:$(TAG) ../../ -f Dockerfile
else
	docker build -t rustfinity-runner:$(TAG) ../../ -f Dockerfile
endif
