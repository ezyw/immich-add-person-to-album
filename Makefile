IMAGE := immich-person-to-album

.PHONY: build run

build:
	docker build -t $(IMAGE) .

run:
	docker run --rm \
		--mount type=bind,source="$(CURDIR)/config",target=/app/config \
		$(IMAGE)