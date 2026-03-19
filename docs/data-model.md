# Data Model

This document describes the core entities across all services. Each service owns its database; cross-service references use UUIDs.

## Service: auth-service (bbps_auth)

### users
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK, DEFAULT uuid_generate_v4() |
| name | VARCHAR(255) | NOT NULL |
| email | VARCHAR(255) | NOT NULL, UNIQUE |
| password_hash | VARCHAR(255) | NOT NULL |
| phone | VARCHAR(20) | NOT NULL |
| role | VARCHAR(50) | NOT NULL (Student/Parent/Teacher/SchoolAdmin/PlatformAdmin) |
| created_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() |
| updated_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() |

### student_profiles
| Column | Type | Constraints |
|--------|------|-------------|
| user_id | UUID | PK, FK → users(id) ON DELETE CASCADE |
| grade | SMALLINT | NOT NULL, CHECK (1-12) |
| mother_tongue | VARCHAR(100) | NOT NULL |
| state_of_residence | VARCHAR(100) | NOT NULL |
| board | VARCHAR(50) | NULLABLE |
| school_id | UUID | NULLABLE |
| parent_email | VARCHAR(255) | NULLABLE |

### teacher_profiles
| Column | Type | Constraints |
|--------|------|-------------|
| user_id | UUID | PK, FK → users(id) ON DELETE CASCADE |
| languages | VARCHAR(100)[] | NOT NULL |
| qualifications | TEXT | NOT NULL |
| bio | TEXT | NULLABLE |
| verified | BOOLEAN | NOT NULL, DEFAULT FALSE |
| rating | DOUBLE PRECISION | NULLABLE |

---

## Service: curriculum-service (bbps_curriculum)

### languages
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| name | VARCHAR(100) | NOT NULL, UNIQUE |
| script | VARCHAR(100) | NOT NULL |
| iso_code | VARCHAR(10) | NOT NULL, UNIQUE |
| family | VARCHAR(100) | NOT NULL |

### curricula
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| language_id | UUID | FK → languages(id), NOT NULL |
| grade | SMALLINT | NOT NULL, CHECK (1-12) |
| title | VARCHAR(255) | NOT NULL |
| description | TEXT | NULLABLE |

**Unique constraint:** (language_id, grade)

### modules
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| curriculum_id | UUID | FK → curricula(id), NOT NULL |
| title | VARCHAR(255) | NOT NULL |
| description | TEXT | NULLABLE |
| skill_type | VARCHAR(20) | NOT NULL (reading/writing/listening/speaking) |
| order_index | INTEGER | NOT NULL |
| estimated_minutes | INTEGER | NOT NULL |

### lessons
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| module_id | UUID | FK → modules(id), NOT NULL |
| title | VARCHAR(255) | NOT NULL |
| description | TEXT | NULLABLE |
| lesson_type | VARCHAR(20) | NOT NULL (video/text/interactive/exercise/cultural) |
| content | JSONB | NOT NULL |
| media_urls | TEXT[] | NULLABLE |
| duration_minutes | INTEGER | NOT NULL |
| order_index | INTEGER | NOT NULL |

### enrollments
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| student_id | UUID | NOT NULL (references auth-service user) |
| language_id | UUID | FK → languages(id), NOT NULL |
| grade | SMALLINT | NOT NULL |
| proficiency | VARCHAR(20) | NOT NULL (beginner/intermediate/advanced) |
| enrolled_at | TIMESTAMPTZ | NOT NULL |

**Unique constraint:** (student_id, language_id)

### progress
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| student_id | UUID | NOT NULL |
| lesson_id | UUID | FK → lessons(id), NOT NULL |
| completed_at | TIMESTAMPTZ | NOT NULL |
| score | DOUBLE PRECISION | NULLABLE |
| time_spent_seconds | INTEGER | NOT NULL |

---

## Service: assessment-service (bbps_assessment)

### assessments
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| student_id | UUID | NOT NULL |
| module_id | UUID | NOT NULL (references curriculum-service) |
| assessment_type | VARCHAR(20) | NOT NULL (module_quiz/semester_exam/oral_exam/diagnostic) |
| status | VARCHAR(20) | NOT NULL (not_started/in_progress/submitted/graded) |
| time_limit_minutes | INTEGER | NOT NULL |
| started_at | TIMESTAMPTZ | NULLABLE |
| submitted_at | TIMESTAMPTZ | NULLABLE |

### questions
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| assessment_id | UUID | FK → assessments(id), NOT NULL |
| question_text | TEXT | NOT NULL |
| question_type | VARCHAR(20) | NOT NULL |
| options | JSONB | NULLABLE |
| media_url | TEXT | NULLABLE |
| max_score | INTEGER | NOT NULL |
| correct_answer | TEXT | NOT NULL |
| order_index | INTEGER | NOT NULL |

### student_answers
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| assessment_id | UUID | FK → assessments(id), NOT NULL |
| question_id | UUID | FK → questions(id), NOT NULL |
| answer | TEXT | NOT NULL |
| score | DOUBLE PRECISION | NULLABLE |
| is_correct | BOOLEAN | NULLABLE |

### credits
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| student_id | UUID | NOT NULL |
| language_id | UUID | NOT NULL |
| grade | SMALLINT | NOT NULL |
| semester | VARCHAR(20) | NOT NULL |
| score | DOUBLE PRECISION | NOT NULL |
| grade_letter | VARCHAR(5) | NOT NULL |
| credit_points | DOUBLE PRECISION | NOT NULL |
| status | VARCHAR(20) | NOT NULL (pending/issued/verified/revoked) |
| certificate_url | TEXT | NOT NULL |
| verification_code | VARCHAR(50) | NOT NULL, UNIQUE |
| issued_at | TIMESTAMPTZ | NOT NULL |

---

## Service: content-service (bbps_content)

### content_items
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| language_id | UUID | NOT NULL |
| module_id | UUID | NULLABLE |
| lesson_id | UUID | NULLABLE |
| content_type | VARCHAR(20) | NOT NULL |
| title | VARCHAR(255) | NOT NULL |
| body | JSONB | NOT NULL |
| media_urls | TEXT[] | NULLABLE |
| status | VARCHAR(20) | NOT NULL (draft/review/published/archived) |
| created_by | UUID | NOT NULL |
| created_at | TIMESTAMPTZ | NOT NULL |
| updated_at | TIMESTAMPTZ | NOT NULL |

### media_assets
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| filename | VARCHAR(255) | NOT NULL |
| content_type | VARCHAR(100) | NOT NULL |
| url | TEXT | NOT NULL |
| size_bytes | BIGINT | NOT NULL |
| uploaded_by | UUID | NOT NULL |
| uploaded_at | TIMESTAMPTZ | NOT NULL |

### content_reviews
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| content_item_id | UUID | FK → content_items(id), NOT NULL |
| reviewer_id | UUID | NOT NULL |
| status | VARCHAR(20) | NOT NULL (approved/rejected/needs_changes) |
| comments | TEXT | NULLABLE |
| reviewed_at | TIMESTAMPTZ | NOT NULL |

---

## Service: live-class-service (bbps_live_class)

### live_classes
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| teacher_id | UUID | NOT NULL |
| language_id | UUID | NOT NULL |
| title | VARCHAR(255) | NOT NULL |
| description | TEXT | NULLABLE |
| scheduled_at | TIMESTAMPTZ | NOT NULL |
| duration_minutes | INTEGER | NOT NULL |
| max_students | INTEGER | NOT NULL |
| status | VARCHAR(20) | NOT NULL (scheduled/live/completed/cancelled) |
| meeting_url | TEXT | NULLABLE |

### class_enrollments
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| class_id | UUID | FK → live_classes(id), NOT NULL |
| student_id | UUID | NOT NULL |
| enrolled_at | TIMESTAMPTZ | NOT NULL |
| attended | BOOLEAN | NOT NULL, DEFAULT FALSE |

### teacher_availability
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| teacher_id | UUID | NOT NULL |
| day_of_week | SMALLINT | NOT NULL, CHECK (0-6) |
| start_time | TIME | NOT NULL |
| end_time | TIME | NOT NULL |

---

## Service: notification-service (bbps_notification)

### notifications
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| user_id | UUID | NOT NULL |
| notification_type | VARCHAR(50) | NOT NULL |
| channel | VARCHAR(20) | NOT NULL (email/sms/push) |
| title | VARCHAR(255) | NOT NULL |
| body | TEXT | NOT NULL |
| status | VARCHAR(20) | NOT NULL (pending/sent/delivered/failed) |
| sent_at | TIMESTAMPTZ | NULLABLE |
| read_at | TIMESTAMPTZ | NULLABLE |

### notification_preferences
| Column | Type | Constraints |
|--------|------|-------------|
| user_id | UUID | PK |
| email_enabled | BOOLEAN | NOT NULL, DEFAULT TRUE |
| sms_enabled | BOOLEAN | NOT NULL, DEFAULT TRUE |
| push_enabled | BOOLEAN | NOT NULL, DEFAULT TRUE |
| digest_frequency | VARCHAR(20) | NOT NULL, DEFAULT 'daily' |

---

## Service: payment-service (bbps_payment)

### plans
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| name | VARCHAR(100) | NOT NULL |
| price_paise | BIGINT | NOT NULL |
| currency | VARCHAR(3) | NOT NULL, DEFAULT 'INR' |
| interval | VARCHAR(20) | NOT NULL (monthly/yearly) |
| features | JSONB | NOT NULL |
| active | BOOLEAN | NOT NULL, DEFAULT TRUE |

### subscriptions
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| user_id | UUID | NOT NULL |
| plan_id | UUID | FK → plans(id), NOT NULL |
| status | VARCHAR(20) | NOT NULL (active/cancelled/expired/past_due) |
| razorpay_subscription_id | VARCHAR(100) | NULLABLE |
| current_period_start | TIMESTAMPTZ | NOT NULL |
| current_period_end | TIMESTAMPTZ | NOT NULL |
| created_at | TIMESTAMPTZ | NOT NULL |

### payments
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| subscription_id | UUID | FK → subscriptions(id), NOT NULL |
| user_id | UUID | NOT NULL |
| amount_paise | BIGINT | NOT NULL |
| currency | VARCHAR(3) | NOT NULL, DEFAULT 'INR' |
| status | VARCHAR(20) | NOT NULL (pending/captured/failed/refunded) |
| razorpay_payment_id | VARCHAR(100) | NULLABLE |
| razorpay_order_id | VARCHAR(100) | NULLABLE |
| paid_at | TIMESTAMPTZ | NULLABLE |

---

## Cross-Service References

Services reference entities in other services by UUID. These are **not** foreign keys at the database level — they are resolved via API calls at runtime.

| Source Service | Field | References |
|---------------|-------|-----------|
| curriculum-service | enrollments.student_id | auth-service → users.id |
| curriculum-service | progress.student_id | auth-service → users.id |
| assessment-service | assessments.student_id | auth-service → users.id |
| assessment-service | assessments.module_id | curriculum-service → modules.id |
| assessment-service | credits.student_id | auth-service → users.id |
| assessment-service | credits.language_id | curriculum-service → languages.id |
| content-service | content_items.language_id | curriculum-service → languages.id |
| content-service | content_items.created_by | auth-service → users.id |
| live-class-service | live_classes.teacher_id | auth-service → users.id |
| live-class-service | live_classes.language_id | curriculum-service → languages.id |
| live-class-service | class_enrollments.student_id | auth-service → users.id |
| notification-service | notifications.user_id | auth-service → users.id |
| payment-service | subscriptions.user_id | auth-service → users.id |
