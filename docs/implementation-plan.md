# DevERP 구현 계획서

## 📋 프로젝트 개요

**프로젝트명**: DevERP (Development ERP System)
**목적**: 개인 개발자를 위한 다중 프로젝트 관리 및 유지보수 시스템
**기술 스택**: Rust + PostgreSQL
**형태**: CLI (Command Line Interface)
**작성일**: 2025-10-18

---

## 🎯 구현 목표

1. **체계적인 프로젝트 관리**: 여러 개발 프로젝트를 하나의 시스템에서 통합 관리
2. **효율적인 태스크 추적**: 작업 간 의존성 관리 및 진행 상황 모니터링
3. **리소스 최적화**: 개발 리소스(라이브러리, API, 도구) 추적 및 활용도 분석
4. **타임라인 관리**: 프로젝트 일정 및 마일스톤 추적
5. **리포팅 기능**: 프로젝트 현황 및 진척도 분석 리포트 생성

---

## 📐 아키텍처 개요

### 계층 구조 (Layered Architecture)

```
┌─────────────────────────────────────┐
│     CLI Layer (clap)                │  ← 사용자 인터페이스
├─────────────────────────────────────┤
│     Business Logic Layer            │  ← 비즈니스 로직
│  (Services: Project, Task, etc.)    │
├─────────────────────────────────────┤
│     Data Access Layer               │  ← 데이터 접근 계층
│  (Repository Pattern)               │
├─────────────────────────────────────┤
│     PostgreSQL Database             │  ← 영속성 계층
└─────────────────────────────────────┘
```

### 핵심 설계 원칙

1. **관심사의 분리 (Separation of Concerns)**: 각 계층은 명확한 책임을 가짐
2. **의존성 주입 (Dependency Injection)**: 느슨한 결합으로 테스트 용이성 확보
3. **Repository Pattern**: 데이터베이스 추상화로 유연성 제공
4. **SOLID 원칙**: 유지보수 가능하고 확장 가능한 코드 작성

---

## 🗂️ 프로젝트 구조

```
deverp/
├── Cargo.toml                    # 프로젝트 메타데이터 및 의존성
├── .env.example                  # 환경 변수 예시 파일
├── .gitignore                    # Git 제외 파일 목록
│
├── src/
│   ├── main.rs                   # 애플리케이션 진입점
│   ├── lib.rs                    # 라이브러리 루트
│   │
│   ├── cli/                      # CLI 인터페이스 계층
│   │   ├── mod.rs
│   │   ├── commands.rs           # 메인 커맨드 정의
│   │   ├── project.rs            # 프로젝트 관련 커맨드
│   │   ├── task.rs               # 태스크 관련 커맨드
│   │   ├── resource.rs           # 리소스 관련 커맨드
│   │   ├── timeline.rs           # 타임라인 관련 커맨드
│   │   ├── report.rs             # 리포트 관련 커맨드
│   │   └── config.rs             # 설정 관련 커맨드
│   │
│   ├── domain/                   # 비즈니스 로직 계층
│   │   ├── mod.rs
│   │   ├── project/              # 프로젝트 도메인
│   │   │   ├── mod.rs
│   │   │   ├── entity.rs         # 프로젝트 엔티티
│   │   │   ├── service.rs        # 프로젝트 비즈니스 로직
│   │   │   └── repository.rs     # 프로젝트 Repository 트레이트
│   │   ├── task/                 # 태스크 도메인
│   │   ├── resource/             # 리소스 도메인
│   │   └── timeline/             # 타임라인 도메인
│   │
│   ├── infrastructure/           # 데이터 접근 계층
│   │   ├── mod.rs
│   │   ├── database.rs           # 데이터베이스 연결 관리
│   │   ├── pool.rs               # 커넥션 풀 관리
│   │   └── repositories/         # Repository 구현체
│   │       ├── mod.rs
│   │       ├── project_repo.rs
│   │       ├── task_repo.rs
│   │       ├── resource_repo.rs
│   │       └── timeline_repo.rs
│   │
│   ├── config/                   # 설정 관리
│   │   ├── mod.rs
│   │   └── settings.rs           # 설정 로드 및 검증
│   │
│   └── utils/                    # 유틸리티
│       ├── mod.rs
│       ├── error.rs              # 에러 타입 정의
│       ├── logger.rs             # 로깅 설정
│       └── formatter.rs          # CLI 출력 포맷팅
│
├── migrations/                   # 데이터베이스 마이그레이션
│   ├── 001_initial_schema.sql
│   ├── 002_add_indexes.sql
│   └── ...
│
├── tests/                        # 통합 테스트
│   ├── integration_test.rs
│   └── helpers/
│
├── docs/                         # 문서
│   ├── architecture.md
│   ├── database.md
│   ├── implementation-plan.md    # 이 문서
│   └── user-guide.md
│
└── config/                       # 설정 파일
    ├── default.toml
    └── config.example.toml
```

---

## 🗄️ 데이터베이스 설계 요약

### 핵심 테이블

1. **projects**: 프로젝트 정보 (이름, 상태, 우선순위, 진행률 등)
2. **tasks**: 태스크 정보 (제목, 상태, 할당자, 예상/실제 작업시간 등)
3. **task_dependencies**: 태스크 간 의존성 관계
4. **task_comments**: 태스크 코멘트 및 노트
5. **resources**: 개발 리소스 (라이브러리, API, 도구 등)
6. **project_resources**: 프로젝트-리소스 연결
7. **timelines**: 프로젝트 타임라인
8. **milestones**: 타임라인 마일스톤
9. **tags**: 태그 마스터
10. **project_tags**: 프로젝트-태그 연결
11. **configurations**: 시스템 설정
12. **audit_logs**: 감사 로그

### 주요 특징

- **UUID 사용**: 외부 참조를 위한 고유 식별자
- **Soft Delete**: `deleted_at` 타임스탬프로 논리 삭제
- **Audit Trail**: `created_at`, `updated_at` 자동 관리
- **JSONB 메타데이터**: 유연한 확장 가능성
- **GIN 인덱스**: 배열 및 JSONB 필드 검색 최적화

---

## 🔧 기술 스택 상세

### Core Dependencies

```toml
[dependencies]
# CLI Framework
clap = { version = "4.x", features = ["derive"] }

# Async Runtime
tokio = { version = "1.x", features = ["full"] }

# Database (Type-safe SQL)
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls", "chrono", "uuid"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Error Handling
anyhow = "1.0"
thiserror = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# Date/Time
chrono = { version = "0.4", features = ["serde"] }

# UUID
uuid = { version = "1.0", features = ["v4", "serde"] }

# Terminal Output
colored = "2.0"

# Configuration
config = "0.13"
```

### Development Dependencies

```toml
[dev-dependencies]
tokio-test = "0.4"
mockall = "0.11"
testcontainers = "0.14"
```

---

## 🧪 테스트 전략

### 단위 테스트 (Unit Tests)

- **대상**: 비즈니스 로직, 유틸리티 함수
- **도구**: Rust 내장 테스트 프레임워크, `mockall`
- **커버리지 목표**: 80% 이상

### 통합 테스트 (Integration Tests)

- **대상**: Repository 계층, CLI 커맨드
- **도구**: `testcontainers` (PostgreSQL 테스트 컨테이너)
- **커버리지 목표**: 70% 이상

### End-to-End 테스트

- **대상**: 전체 워크플로우
- **방법**: 실제 데이터베이스에 대한 CLI 명령 실행

---

## 🔒 보안 고려사항

1. **SQL Injection 방지**

   - sqlx의 prepared statement 사용
   - 모든 사용자 입력 검증

2. **데이터베이스 자격 증명 보호**

   - 환경 변수 사용
   - `.env` 파일을 `.gitignore`에 추가

3. **입력 유효성 검증**

   - CLI 레벨에서 기본 검증
   - Service 레벨에서 비즈니스 규칙 검증

4. **에러 메시지 제어**
   - 민감한 정보 노출 방지
   - 사용자 친화적인 메시지 제공

---

## 📊 성능 목표

1. **응답 시간**

   - 단순 조회: < 100ms
   - 복잡한 조회: < 500ms
   - 데이터 수정: < 200ms

2. **동시 처리**

   - 로컬 환경이므로 단일 사용자 기준
   - 커넥션 풀: 5-10개 연결

3. **데이터 규모**
   - 프로젝트: 100개 이상
   - 태스크: 1000개 이상
   - 원활한 성능 유지

---

## 🚀 향후 확장 계획

### 단기 확장 (v1.1 - v1.5)

1. **Export/Import 기능**

   - JSON, CSV 형식 지원
   - 프로젝트 백업 및 복원

2. **템플릿 시스템**

   - 프로젝트 템플릿
   - 태스크 템플릿

3. **알림 기능**
   - 마감일 임박 알림
   - 마일스톤 달성 알림

### 중기 확장 (v2.0+)

1. **Git 통합**

   - 프로젝트-Git 저장소 연동
   - 커밋 히스토리 추적

2. **시각화**

   - 터미널 기반 차트 (간트 차트, 번다운 차트)
   - 진행률 그래프

3. **플러그인 시스템**
   - 외부 도구 연동 (JIRA, GitHub, etc.)

### 장기 확장 (v3.0+)

1. **웹 인터페이스** (선택적)

   - 웹 대시보드
   - REST API 제공

2. **팀 협업 기능**
   - 멀티 유저 지원
   - 권한 관리

---

## 📅 예상 일정

| Phase | 작업 내용                 | 예상 기간 | 누적 기간 |
| ----- | ------------------------- | --------- | --------- |
| 1     | 프로젝트 초기 설정        | 1-2일     | 2일       |
| 2     | 핵심 인프라 구축          | 2-3일     | 5일       |
| 3     | 데이터베이스 스키마 구현  | 2-3일     | 8일       |
| 4     | Domain - Project 모듈     | 3-4일     | 12일      |
| 5     | Domain - Task 모듈        | 3-4일     | 16일      |
| 6     | Domain - Resource 모듈    | 2-3일     | 19일      |
| 7     | Domain - Timeline 모듈    | 2-3일     | 22일      |
| 8     | CLI - 기본 구조           | 2일       | 24일      |
| 9     | CLI - Project 커맨드      | 2-3일     | 27일      |
| 10    | CLI - Task 커맨드         | 3-4일     | 31일      |
| 11    | CLI - Resource & Timeline | 2-3일     | 34일      |
| 12    | Report 모듈               | 2-3일     | 37일      |
| 13    | Configuration 모듈        | 1-2일     | 39일      |
| 14    | 통합 테스트               | 2-3일     | 42일      |
| 15    | 문서화 및 최적화          | 2-3일     | 45일      |
| 16    | 배포 준비                 | 1-2일     | 47일      |

**총 예상 개발 기간**: 약 6-7주 (파트타임 기준)

---

## 🎓 학습 리소스

### Rust

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Book](https://rust-lang.github.io/async-book/)

### sqlx

- [sqlx GitHub](https://github.com/launchbadge/sqlx)
- [sqlx Documentation](https://docs.rs/sqlx/)

### clap

- [clap Documentation](https://docs.rs/clap/)
- [clap Derive Tutorial](https://github.com/clap-rs/clap/tree/master/examples)

### PostgreSQL

- [PostgreSQL Documentation](https://www.postgresql.org/docs/)
- [PostgreSQL Tutorial](https://www.postgresqltutorial.com/)

---

## 📋 체크리스트

### 개발 전 준비

- [ ] Rust 툴체인 설치 (rustc, cargo)
- [ ] PostgreSQL 설치 및 설정
- [ ] 코드 에디터 설정 (VSCode + rust-analyzer 권장)
- [ ] Git 저장소 초기화

### 개발 중 체크사항

- [ ] 코드 작성 후 `cargo clippy` 실행
- [ ] 커밋 전 `cargo fmt` 실행
- [ ] 기능 완성 후 테스트 작성
- [ ] 문서 주석 작성

### 배포 전 체크사항

- [ ] 모든 테스트 통과 (`cargo test`)
- [ ] 릴리스 빌드 성공 (`cargo build --release`)
- [ ] 문서 완성도 확인
- [ ] 라이선스 파일 작성
- [ ] README.md 완성

---

## 🔄 개발 워크플로우

### 일일 워크플로우

1. **작업 시작**

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **개발**

   - 코드 작성
   - 테스트 작성
   - 로컬 테스트 실행

3. **코드 품질 체크**

   ```bash
   cargo fmt
   cargo clippy
   cargo test
   ```

4. **커밋**

   ```bash
   git add .
   git commit -m "feat: Add your feature description"
   ```

5. **병합**
   ```bash
   git checkout main
   git merge feature/your-feature-name
   ```

### 주간 워크플로우

1. Phase별 목표 설정
2. 일일 진행 상황 기록
3. 주말 리뷰 및 다음 주 계획

---

## 🛠️ 개발 명령어 참조

### Cargo 명령어

```bash
# 프로젝트 생성
cargo new deverp --bin

# 의존성 추가
cargo add clap --features derive
cargo add tokio --features full
cargo add sqlx --features postgres,runtime-tokio-native-tls,chrono,uuid

# 빌드
cargo build              # 디버그 빌드
cargo build --release    # 릴리스 빌드

# 실행
cargo run -- <args>      # 디버그 빌드로 실행
cargo run --release -- <args>  # 릴리스 빌드로 실행

# 테스트
cargo test               # 모든 테스트 실행
cargo test --test integration_test  # 특정 테스트 실행
cargo test -- --nocapture  # 출력 표시하며 테스트

# 코드 품질
cargo fmt                # 코드 포맷팅
cargo clippy             # 린트 체크
cargo doc --open         # 문서 생성 및 열기

# 벤치마크
cargo bench              # 벤치마크 실행
```

### sqlx-cli 명령어

```bash
# sqlx-cli 설치
cargo install sqlx-cli --features postgres

# 데이터베이스 생성
sqlx database create

# 마이그레이션 추가
sqlx migrate add <migration_name>

# 마이그레이션 실행
sqlx migrate run

# 마이그레이션 되돌리기
sqlx migrate revert

# 쿼리 메타데이터 준비 (오프라인 빌드용)
cargo sqlx prepare
```

### PostgreSQL 명령어

```bash
# PostgreSQL 접속
psql -U deverp_user -d deverp

# 데이터베이스 생성
createdb -U postgres deverp

# 백업
pg_dump -U deverp_user deverp > backup.sql

# 복원
psql -U deverp_user deverp < backup.sql
```

---

## 📝 결론

이 구현 계획서는 DevERP 시스템을 체계적으로 구축하기 위한 로드맵을 제공합니다. 각 Phase는 독립적으로 완성 가능하며, 점진적으로 기능을 확장해 나갈 수 있도록 설계되었습니다.

**핵심 성공 요인**:

1. **계층별 분리**: 명확한 아키텍처로 유지보수성 확보
2. **테스트 주도**: 각 단계마다 테스트를 통한 품질 보장
3. **문서화**: 지속적인 문서 업데이트로 이해도 향상
4. **점진적 개발**: 작은 단위로 완성하며 피드백 반영

이 계획을 따라 개발하면 약 6-7주 내에 완전히 기능하는 DevERP 시스템을 완성할 수 있습니다.

---

**작성자**: Claude Code
**버전**: 1.0
**최종 수정일**: 2025-10-18
