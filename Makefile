TAG := $(shell git rev-parse --short HEAD)

CONTAINER_IMAGE_REPO := ghcr.io/shikharbhardwaj/zero2prod

build-docker:
	docker build -t zero2prod:latest .

push-docker: build-docker
	docker tag zero2prod:latest $(CONTAINER_IMAGE_REPO):$(TAG)
	docker push $(CONTAINER_IMAGE_REPO):$(TAG)
