# 10k App – Implementation Plan

## Architecture Overview

- **Client:** SvelteKit SPA with SSR for fast initial paint and SEO (public marketing pages).
- **API Layer:** Amazon API Gateway (HTTP APIs) routing to Rust-based AWS Lambda functions.
- **Business Logic:** Clean architecture modules (domain, application, interface adapters) compiled into Lambda binaries via Cargo + cargo-lambda.
- **Data:** Amazon DynamoDB (single-table) storing user profiles, goals, sessions, milestones, circles, achievements.
- **Identity:** Amazon Cognito (Hosted UI + JS SDK) for user pools, OAuth/OIDC support, MFA, passwordless options.
- **Observability:** CloudWatch Logs & Metrics, AWS X-Ray traces, structured logging, Sentry (frontend) + OpenTelemetry exporters.
- **CI/CD:** GitHub Actions building/test/lint pipelines, Terraform for IaC deployments across dev/staging/prod.

## Key Technology Choices

| Area          | Tech                                                                                                           | Notes                                 |
| ------------- | -------------------------------------------------------------------------------------------------------------- | ------------------------------------- |
| Frontend      | SvelteKit (latest), TypeScript, Vite bundler                                                                   | File-based routing, SSR/CSR hybrid    |
| Styling       | Tailwind CSS + Radix UI + CSS variables for themes                                                             | Speeds consistent design system       |
| State/data    | Svelte stores, TanStack Query for API caching, Zod for schema validation                                       | Keeps UI-state deterministic          |
| Forms         | Felte (Svelte form lib) or SvelteKit form actions with Zod validation                                          | Accessible + server/client validation |
| Testing       | Vitest (unit), Playwright (e2e), Storybook (visual regression via Chromatic)                                   | Automate in CI                        |
| Backend       | Rust 1.78+, cargo-lambda, aws-sdk-rust, serde, sqlx-like builder for Dynamo (dynamoose?) -> `aws-sdk-dynamodb` | Strict typing + perf                  |
| Messaging     | Amazon EventBridge for domain events; Amazon SES/SNS for notifications                                         | Enables decoupled workflows           |
| IaC           | Terraform w/ workspaces or Terragrunt                                                                          | Modules per environment               |
| Observability | AWS X-Ray, CloudWatch, Sentry, OpenTelemetry collector (Lambda extension)                                      | Unified traces                        |

## Clean Architecture Layers

1. **Domain** – Entities (`User`, `Goal`, `Session`, `Milestone`, `Circle`, `Achievement`) + value objects (HourCount, TimeWindow).
2. **Use Cases (Application layer)** – Interactors encapsulating business rules (e.g., `LogPracticeSession`, `StartTimer`, `JoinCircle`, `CalculateLeaderboard`).
3. **Interface Adapters** – Repositories, presenters, controllers translating between domain types and transport (HTTP/JSON, DynamoDB).
4. **Framework/Drivers** – Lambda handlers, API Gateway mappers, AWS SDK calls, Cognito integration, DynamoDB client, Svelte components.

### Backend Project Structure

```
backend/
  Cargo.toml (workspace)
  crates/
    domain/
    usecases/
    adapters/
    infra/           # Dynamo repositories, Cognito client, event emitters
    handlers/        # Lambda binaries per bounded context
```

- Each Lambda handler imports use cases + adapters; dependency inversion ensures domain does not import AWS SDK.
- Shared DTOs generated via `serde` for JSON serialization.

### Frontend Project Structure

```
frontend/
  src/
    lib/
      api/          # API clients (generated OpenAPI)
      stores/
      components/
      features/
        goals/
        sessions/
        leaderboard/
    routes/
      +page.svelte / +page.server.ts for SvelteKit loaders/actions
```

- Domain-style separation: each feature folder houses UI, stores, queries, forms.
- Global types generated from backend schema via `openapi-typescript`.

## Frontend Implementation Plan

1. **Scaffolding**
   - `npm create svelte@latest frontend` with TypeScript, ESLint, Prettier, Vitest, Playwright.
   - Install Tailwind, Autoprefixer, PostCSS, Radix via `@radix-ui/colors`.
   - Configure `src/lib/config.ts` for environment variables (API base URL, Cognito info).
2. **Authentication Flow**
   - Use `amazon-cognito-identity-js` or `@aws-amplify/auth` minimal package.
   - Implement session store (refresh tokens handled by Cognito hosted UI redirect).
   - Protect authenticated routes with SvelteKit hooks and route guards.
3. **API Layer**
   - Maintain OpenAPI spec generated from backend match -> use `openapi-typescript` for typed client.
   - Use TanStack Query (Svelte Query) for caching session lists, scoreboard, etc.
4. **UI/UX Features**
   - Build atomic components: timer controls, progress rings, leaderboard table, milestone cards.
   - Use Tailwind + CSS variables for themes (light/dark) and support custom brand colors for circles.
   - Integrate Chart.js or `echarts` for cumulative hours graphs.
5. **State & Business Logic**
   - Keep derived data (streaks, pace) computed via Svelte stores selectors to avoid duplicating backend logic.
   - Use `zod` to validate forms before hitting API.
6. **Testing & Quality**
   - Component tests via Vitest + Testing Library Svelte.
   - Playwright flows: onboarding, session logging, leaderboard view.
   - Visual snapshots via Storybook + Chromatic to guard design regressions.
7. **Observability**
   - Integrate Sentry browser SDK for error tracking + performance monitoring.
   - Use Web Vitals/Segment for analytics; send events to backend (EventBridge) if needed.

## Backend Implementation Plan

1. **Rust Workspace Setup**
   - `cargo new --lib domain`, `usecases`, `adapters`.
   - Domain crate defines entities, aggregate roots, invariants (e.g., sessions cannot exceed 24h).
   - Usecases crate exposes pure functions returning `Result<DomainEvent, DomainError>`.
   - Adapters crate implements repository traits for DynamoDB using `aws-sdk-dynamodb`.
2. **Lambda Handlers**
   - Use `cargo-lambda new log-session` etc. Each handler:
     - Parse API Gateway event -> DTO.
     - Invoke use case via trait objects injected using `lambda-extension` pattern.
     - Map domain errors to HTTP responses.
   - Handlers grouped:
     - Auth callback / user bootstrap (Cognito trigger).
     - Goals CRUD.
     - Session logging & timer state (start/pause/stop).
     - Leaderboard queries (may use DynamoDB aggregated queries or DAX/Elasticache).
     - Circles & invites.
3. **Data Modeling (DynamoDB Single Table)**
   - Primary key: `PK` (entity type + id), `SK` (sort by relation/time).
   - Example items:
     - `PK=USER#<id>`, `SK=PROFILE` -> user record.
     - `PK=USER#<id>`, `SK=GOAL#<goalId>`.
     - `PK=GOAL#<goalId>`, `SK=SESSION#<timestamp>`.
     - `PK=CIRCLE#<circleId>`, `SK=MEMBER#<userId>`.
   - GSI1 for leaderboard queries (e.g., partition by `GOAL#<goalId>` and sort by cumulative hours).
   - Use DynamoDB Streams -> Lambda for eventual consistency updates (e.g., update leaderboard totals, trigger notifications).
4. **Security & Auth**
   - Cognito authorizer attached to API Gateway (JWT validation).
   - Use IAM roles per Lambda with least privilege; access only needed table indexes.
   - Encrypt data at rest (DynamoDB SSE) and enforce HTTPS everywhere.
5. **Observability**
   - Structured logging with `tracing` crate + `tracing-subscriber`.
   - `aws-lambda-rust-runtime` + `lambda-extension` for OpenTelemetry; export to X-Ray.
   - Custom metrics: hours logged/day, active timers, API latency -> CloudWatch dashboards.
6. **Testing Strategy**
   - Unit tests in domain/usecases crate (pure logic).
   - Integration tests using `cargo test --features integration` with `localstack` for DynamoDB + Cognito mocks.
   - Contract tests generated from OpenAPI spec ensuring request/response parity.

## Infrastructure & DevOps

1. **Terraform Structure (aligned with repo)**

   ```
   terraform/
     modules/
       backend/
         apigateway/
         lambda/
         dynamodb/
         cognito/
       frontend/
         s3/
         cloudfront/
     env/
       prod/
         locals.tf
         main.tf
         terraform.tf
   ```

   - `modules/backend` composes the service modules (Lambda, API Gateway, DynamoDB, Cognito) and exposes cohesive outputs (API endpoint, user-pool IDs, table names). Each nested module owns its IAM, logging, and scaling configuration to stay reusable.
   - `modules/frontend` wraps the S3 + CloudFront submodules; hook outputs (bucket name, distribution ID) into CI for zero-downtime deploys.
   - Add matching `env/dev` and `env/staging` folders mirroring `env/prod`, each setting remote state (S3 + DynamoDB lock) inside `terraform.tf` and referencing shared locals.
   - Keep shared naming/tagging conventions inside `locals.tf`; any cross-stack IDs (e.g., Cognito client IDs) should be re-exposed via `outputs.tf`.
   - Extend this structure with additional modules (EventBridge, observability sinks, Parameter Store) as siblings under `modules/backend` to keep the layout predictable.

2. **CI/CD Pipelines (GitHub Actions)**
   - **Frontend workflow**
     - Install PNPM/NPM, run lint (`pnpm lint`), unit tests, e2e (headless).
     - Build SvelteKit -> deploy to S3 + CloudFront (or Vercel? prefer AWS). Use `s3 sync` + `cloudfront create-invalidation`.
   - **Backend workflow**
     - `cargo fmt --check`, `cargo clippy -- -D warnings`.
     - Run unit tests + integration tests via LocalStack.
     - Build Lambda artifacts via `cargo lambda build --release --arm64`.
     - Package & upload to S3, trigger Terraform apply or use AWS SAM/Serverless Application Model for deployment.
   - **Terraform workflow**
     - `terraform fmt -check`, `terraform validate`, plan -> manual approval -> apply.
3. **Secrets & Config**
   - Use AWS SSM Parameter Store / Secrets Manager for API keys, third-party credentials, and share ARNs/IDs via Terraform outputs for GitHub Actions.
   - Frontend environment variables injected via `.env` (never commit) and GitHub Actions secrets; CI jobs read Terraform outputs for distribution/bucket IDs before kicking off deploys.

## Observability & Monitoring

- **Tracing:** Enable X-Ray for API Gateway + Lambda; propagate trace IDs from frontend via custom headers.
- **Logging:** JSON logs with correlation IDs (userId, goalId, requestId) to CloudWatch; ship to OpenSearch via Kinesis Firehose for ad-hoc queries.
- **Metrics:** Custom metrics (hours logged, active timers) + standard Lambda metrics (invocations, errors, cold starts).
- **Alerting:** CloudWatch Alarms -> SNS -> PagerDuty/Slack for high error rates, throttles, Dynamo capacity issues.
- **Frontend Monitoring:** Sentry for errors + performance; capture Core Web Vitals; send to Slack channel.
- **Analytics:** Mixpanel/Amplitude or AWS Pinpoint for behavior analytics (goal creation funnel, retention).
