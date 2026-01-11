# Repository Guidelines

## Project Structure & Module Organization
- `frontend/` – SvelteKit app (src/, tests/, Tailwind config). Route folders follow SvelteKit conventions (`src/routes/(app)/...`). Mock API endpoints live under `src/routes/api/`.
- `backend/` – Rust workspace (`crates/` for domain, usecases, infra, DTOs, Lambda handlers, OpenAPI generator). Handlers compile to AWS Lambda binaries via cargo-lambda.
- `terraform/` – IaC split into `modules/` (backend, frontend, waf, observability) and `env/{dev,staging,prod}` for environment-specific locals/state.
- `docs/` – Concept, implementation plan, and local dev notes.
- `load/` – k6 scripts for quick load tests. Supporting tooling (Makefile, docker-compose) sits at repo root.

## Build, Test, and Development Commands
- `pnpm --dir frontend dev` – start the SvelteKit dev server.
- `pnpm --dir frontend test` / `pnpm --dir frontend test:ui` – run Vitest unit tests and Playwright E2E suites.
- `cargo test` (run from `backend/`) – execute Rust unit/integration tests.
- `make dev` – boot LocalStack and start the frontend dev server.
- `cd terraform/env/<env> && terraform plan` – preview infra changes per environment.

## Coding Style & Naming Conventions
- Frontend: TypeScript + Svelte with ESLint, Prettier, Tailwind. Prefer PascalCase for components, kebab-case for routes, snake_case for API payloads matching backend models.
- Backend: Rust 2021 edition, `cargo fmt` enforced via Lefthook. Modules follow clean architecture naming (domain/usecases/adapters/infra).
- Infrastructure: Terraform HCL formatted with `terraform fmt`; module names are lowercase with dashes.

## Testing Guidelines
- Frontend unit tests use Vitest + Svelte Testing Library; files end with `.test.ts`. Playwright specs live under `frontend/tests/`.
- Backend relies on `cargo test`, including optional LocalStack-backed integration tests gated by `LOCALSTACK_ENDPOINT`.
- Contract tests: `pnpm --dir frontend test:contract` regenerates OpenAPI TypeScript types; ensure zero diff before merging.

## Commit & Pull Request Guidelines
- Follow conventional, descriptive commits (e.g., `feat(timer): persist state`, `fix(terraform): add WAF`). Keep changes scoped.
- PRs should include: summary, testing evidence (`cargo test`, `pnpm test`, Terraform plan), screenshots for UI tweaks, and linked issue/task IDs.
- Avoid force pushes on shared branches; rebase for cleanup only before review completion.

## Security & Configuration Tips
- Keep secrets out of code; use AWS SSM/Secrets Manager and `.env` files ignored by Git.
- When running locally, use the provided `docker-compose` to emulate AWS services; never point dev resources at production accounts.
