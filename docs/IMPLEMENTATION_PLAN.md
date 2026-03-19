# Implementation Sequence Plan

> Tracks the build-out of Bharat Basha Prachar Sabha from current state to Phase 1 MVP and beyond.
> Each item is a discrete, shippable unit of work. Check off as completed.

---

## Legend

- `[x]` — Done
- `[ ]` — Not started
- Items are ordered by dependency (top items unblock bottom items)

---

## Stage 0: Foundation (Done)

> Monorepo scaffold, service skeletons, and dev tooling.

- [x] Monorepo structure with Cargo workspace
- [x] Rust service crates (auth, curriculum, assessment, content, live-class, notification, payment, shared)
- [x] Three-layer architecture (API / Domain / DB) in each service
- [x] Diesel migrations for all services
- [x] Smithy IDL models (auth, curriculum, assessment, common)
- [x] React + TypeScript + Tailwind web app scaffold (pages, auth context, API client)
- [x] Python FastAPI AI/ML service stub
- [x] Docker Compose (PostgreSQL, Redis, all services)
- [x] Justfiles (root + per-service)
- [x] CI workflow structure (.github/workflows)
- [x] `flake.nix` for reproducible dev environment

---

## Stage 1: Service Integration & Auth (Next Up)

> Make the auth service fully functional end-to-end: registration, login, JWT, role-based access.

- [ ] **1.1** Implement Smithy-to-Axum wiring for auth-service (route handlers call domain services)
- [ ] **1.2** Complete auth domain logic: registration, login, password hashing (Argon2), JWT issue/refresh
- [ ] **1.3** Role-based access control (Student, Parent, Teacher, Admin) in shared middleware
- [ ] **1.4** Auth integration tests (register → login → access protected route)
- [ ] **1.5** Wire React frontend login/register pages to auth-service API
- [ ] **1.6** Auth context: store JWT, auto-refresh, redirect on expiry
- [ ] **1.7** Protected route wrapper in frontend

---

## Stage 2: Curriculum Engine

> Students can browse languages, view modules, and consume lessons.

- [ ] **2.1** Implement curriculum-service domain: CRUD for languages, curricula, modules, lessons
- [ ] **2.2** Curriculum-service API handlers (list languages, get modules by grade, get lesson content)
- [ ] **2.3** Seed data: 6 Phase 1 languages with placeholder curricula (Classes 5–8)
- [ ] **2.4** Frontend: Languages page fetches from curriculum-service API
- [ ] **2.5** Frontend: Module listing page with progress indicators
- [ ] **2.6** Frontend: Lesson viewer (video player, text content, interactive exercises)
- [ ] **2.7** Curriculum-service unit tests (domain layer, 80%+ coverage)

---

## Stage 3: Student Enrollment & Progress Tracking

> Students enroll in a language and their progress is persisted.

- [ ] **3.1** Enrollment domain: student selects language → enrollment record created
- [ ] **3.2** Progress tracking domain: mark lessons complete, track module progress %
- [ ] **3.3** API endpoints for enrollment and progress
- [ ] **3.4** Frontend: enrollment flow (select language → confirm → redirected to curriculum)
- [ ] **3.5** Frontend: student dashboard with per-language progress bars
- [ ] **3.6** Integration tests for enrollment + progress lifecycle

---

## Stage 4: Basic Assessment System

> Auto-graded quizzes after each module.

- [ ] **4.1** Assessment domain: create quiz, submit answers, auto-grade, store results
- [ ] **4.2** Question types: multiple-choice, fill-in-the-blank, matching, drag-and-drop
- [ ] **4.3** Assessment-service API (get quiz for module, submit answers, get results)
- [ ] **4.4** Frontend: quiz component with timed questions, submit, and results screen
- [ ] **4.5** Grading rubric aligned with CBSE language paper standards
- [ ] **4.6** Seed data: sample quizzes for each seeded module
- [ ] **4.7** Assessment domain unit tests

---

## Stage 5: Proficiency Diagnostic Test

> Initial placement test when a student picks a language.

- [ ] **5.1** Diagnostic test domain: adaptive question selection based on answers
- [ ] **5.2** Map diagnostic score → proficiency level (Beginner / Intermediate / Advanced)
- [ ] **5.3** Generate personalized learning path based on grade + proficiency
- [ ] **5.4** Frontend: diagnostic test flow (before enrollment completes)
- [ ] **5.5** Seed diagnostic questions for 6 Phase 1 languages

---

## Stage 6: Content Management System

> Teachers and content authors can create and manage curriculum content.

- [ ] **6.1** Content-service domain: CRUD for lessons, exercises, media assets
- [ ] **6.2** Media upload (S3 or local storage for dev) with presigned URLs
- [ ] **6.3** Content review/approval workflow (draft → review → published)
- [ ] **6.4** Frontend: CMS admin pages (create/edit lessons, upload video/audio, manage exercises)
- [ ] **6.5** Rich text editor for lesson content (multilingual input support)
- [ ] **6.6** Content versioning

---

## Stage 7: Payment Integration

> Razorpay subscription for Standard and Premium tiers.

- [ ] **7.1** Payment-service domain: create subscription, handle webhooks, track billing
- [ ] **7.2** Razorpay SDK integration (subscriptions, UPI, cards, wallets)
- [ ] **7.3** Tier enforcement: gate features by subscription level in shared middleware
- [ ] **7.4** Frontend: pricing page, checkout flow, subscription management
- [ ] **7.5** Webhook handler for payment success/failure/renewal
- [ ] **7.6** Payment integration tests (sandbox mode)

---

## Stage 8: Notifications

> Email and push notifications for key events.

- [ ] **8.1** Notification-service domain: templates, delivery channels (email, SMS, push)
- [ ] **8.2** RabbitMQ consumer: other services publish events, notification-service delivers
- [ ] **8.3** Email integration (SES / SMTP)
- [ ] **8.4** Notification types: welcome, progress milestones, assessment results, payment receipts
- [ ] **8.5** Parent notification preferences

---

## Stage 9: Gamification & Engagement

> XP, streaks, badges, and leaderboards.

- [ ] **9.1** Gamification domain: XP award rules, streak tracking, badge criteria
- [ ] **9.2** Leaderboard service (Redis sorted sets)
- [ ] **9.3** API endpoints for XP balance, badges, leaderboard
- [ ] **9.4** Frontend: XP display, streak counter, badge showcase, leaderboard page
- [ ] **9.5** Weekly language challenges

---

## Stage 10: CI/CD & Production Deploy

> Ship Phase 1 MVP to production.

- [ ] **10.1** Complete GitHub Actions CI pipeline (fmt, clippy, test, build for all services)
- [ ] **10.2** Frontend CI (lint, typecheck, build)
- [ ] **10.3** Kubernetes manifests (deployments, services, ingress, secrets, configmaps)
- [ ] **10.4** Terraform for AWS infrastructure (RDS, ElastiCache, ECS/EKS, S3, CloudFront)
- [ ] **10.5** Database migration strategy for production (zero-downtime)
- [ ] **10.6** Monitoring: Prometheus metrics export from each service + Grafana dashboards
- [ ] **10.7** Sentry error tracking integration
- [ ] **10.8** Production deploy to AWS India region (ap-south-1)

---

## Stage 11: Live Classes & Teacher Marketplace (Phase 2)

> Video classes, teacher profiles, booking system.

- [ ] **11.1** Live-class-service domain: scheduling, room creation, WebRTC signaling
- [ ] **11.2** Jitsi Meet integration (self-hosted or JaaS)
- [ ] **11.3** Teacher profiles: qualifications, languages, availability slots, ratings
- [ ] **11.4** Booking system: 1-on-1 and group sessions
- [ ] **11.5** Teacher onboarding: credential verification, demo class
- [ ] **11.6** Frontend: teacher directory, booking flow, video classroom UI
- [ ] **11.7** Revenue sharing and teacher payouts

---

## Stage 12: AI-Powered Learning (Phase 2)

> Speech recognition, pronunciation practice, AI tutor.

- [ ] **12.1** Speech recognition endpoint (Wav2Vec2 / IndicASR fine-tuned per language)
- [ ] **12.2** Pronunciation scoring algorithm
- [ ] **12.3** Text-to-speech for lesson audio (Vakyansh TTS)
- [ ] **12.4** AI tutor chatbot (Claude API) for conversational practice
- [ ] **12.5** Translation bridge (IndicTrans2) between instruction language and mother tongue
- [ ] **12.6** Frontend: pronunciation practice UI with recording + feedback
- [ ] **12.7** Frontend: AI tutor chat interface

---

## Stage 13: Credit Issuance & Proctored Exams (Phase 2)

> Formal assessments that yield board-recognized credits.

- [ ] **13.1** Proctored exam infrastructure (secure browser, identity verification)
- [ ] **13.2** Speaking assessment: recorded oral exams + teacher evaluation queue
- [ ] **13.3** Credit issuance domain: generate certificate, verification code, QR code
- [ ] **13.4** Digital certificate generation (PDF with verification)
- [ ] **13.5** School integration API: schools pull credit data
- [ ] **13.6** DigiLocker integration for certificate storage

---

## Stage 14: Parent Dashboard & Mobile App (Phase 2)

- [ ] **14.1** Parent dashboard: link to children, view progress, manage subscriptions
- [ ] **14.2** React Native mobile app (Android first)
- [ ] **14.3** Offline mode: download lessons, sync progress on reconnect
- [ ] **14.4** Expand curriculum to Classes 1–4 and 9–10

---

## Dependency Graph (Stages)

```
Stage 0 (Done)
  └─► Stage 1 (Auth)
        ├─► Stage 2 (Curriculum)
        │     ├─► Stage 3 (Enrollment + Progress)
        │     │     ├─► Stage 4 (Assessments)
        │     │     │     └─► Stage 5 (Diagnostic Test)
        │     │     └─► Stage 9 (Gamification)
        │     └─► Stage 6 (CMS)
        ├─► Stage 7 (Payments)
        └─► Stage 8 (Notifications)

  Stage 10 (CI/CD + Deploy) — can progress in parallel from Stage 1 onward

  Stages 11–14 (Phase 2) — after MVP ships
```

---

## Current Status

**Last updated:** 2026-03-19

| Stage | Status | Notes |
|-------|--------|-------|
| 0. Foundation | **Complete** | All scaffolding in place |
| 1. Auth Integration | **Next** | Service skeleton exists, needs end-to-end wiring |
| 2–9 | Not started | Blocked on Stage 1 |
| 10. CI/CD | Partial | Workflow structure exists, needs implementation |
| 11–14 | Phase 2 | After MVP |
