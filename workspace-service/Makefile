app := workspace-service
image := fnhsproduction.azurecr.io/$(app)
tag := latest

.PHONY: default
default: ## Run locally
	cargo run

.PHONY: test
test: ## Run tests [TEST=test_name (optional)]
	cargo test $$TEST

.PHONY: docker-build
docker-build: ## Build and tag Docker image
	tar cv \
		--exclude event-models/typescript \
		--exclude manifests \
		--exclude target \
		Dockerfile \
		-C .. event-models workspace-service | \
		DOCKER_BUILDKIT=1 docker build - \
			--progress plain \
			--tag $(app) \
			--tag $(image) \
			--tag $(image):$(tag)

.PHONY: docker-test
docker-test: ## Run tests in Docker
	tar cv \
		--exclude event-models/typescript \
		--exclude manifests \
		--exclude target \
		Dockerfile \
		-C .. event-models workspace-service | \
		DOCKER_BUILDKIT=1 docker build - \
			--progress plain \
			--target test \
			--build-arg TEST_DATABASE_URL=$$TEST_DATABASE_URL

.PHONY: docker-run
docker-run: ## Run Docker image
	docker run -it -p 3030:3030 $(image)

.PHONY: docker-push
docker-push: ## Push Docker image
	docker push $(image)
	docker push $(image):$(tag)

.PHONY: watch
watch: ## Run cargo watch in the way that David likes it.
	env DATABASE_URL=$$(kubectl get secret -n workspace-service workspace-service -o jsonpath='{.data.url}' | base64 -D) cargo watch -x help -x check

# You must have a version of sqlx that includes this patch
# cargo install --git https://github.com/launchbadge/sqlx --rev 106a6a83952bfc0e3316d54f99d865e469dcc15a sqlx-cli
.PHONY: migrate
migrate: ## Run migrations on current cluster
	env DATABASE_URL=$$(kubectl get secret -n workspace-service workspace-service -o jsonpath='{.data.url}' | base64 -D) sqlx migrate run

.PHONY: prepare
prepare: ## Prepares new version of sqlx-data.json
	cargo clean --package workspace_service && env DATABASE_URL=$$(kubectl get secret -n workspace-service workspace-service -o jsonpath='{.data.url}' | base64 -D) cargo sqlx prepare -- --bin workspace_service

.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
