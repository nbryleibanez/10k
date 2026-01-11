SHELL := /bin/bash

.PHONY: bootstrap dev fmt backend-test frontend-test generate-openapi

bootstrap:
	cd frontend && pnpm install

dev:
	docker-compose up -d localstack
	cd frontend && pnpm dev

fmt:
	cd backend && cargo fmt

backend-test:
	cd backend && cargo test

frontend-test:
	cd frontend && pnpm test

generate-openapi:
	cd backend && cargo run -p tenk-api-spec > ../openapi.json
