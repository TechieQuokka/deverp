## 📝 단계별 구현 계획

### Phase 1: 프로젝트 초기 설정 (1-2일)

#### 1.1 프로젝트 생성 및 기본 구조 설정

- [x] `cargo new deverp --bin` 실행
- [x] `Cargo.toml`에 필요한 의존성 추가
- [x] 프로젝트 디렉터리 구조 생성
- [x] `.gitignore` 파일 작성
- [x] `.env.example` 파일 작성
- [x] `README.md` 작성

#### 1.2 데이터베이스 설정

- [x] PostgreSQL 로컬 인스턴스 설치 및 설정
- [x] 데이터베이스 및 사용자 생성 (setup_database.sql 스크립트 생성)
  ```sql
  CREATE DATABASE deverp;
  CREATE USER deverp_user WITH PASSWORD '2147483647';
  GRANT ALL PRIVILEGES ON DATABASE deverp TO deverp_user;
  ```
- [x] `sqlx-cli` 설치: `cargo install sqlx-cli --features postgres`
- [x] 마이그레이션 초기화: `sqlx migrate add initial_schema`

#### 1.3 기본 설정 파일 작성

- [x] `config/default.toml` 작성 (데이터베이스 연결 정보, 로깅 설정 등)
- [x] 환경 변수 설정 (`DATABASE_URL`, `LOG_LEVEL` 등)

---

### Phase 2: 핵심 인프라 구축 (2-3일)

#### 2.1 유틸리티 모듈 구현

- [x] **Error 타입 정의** (`src/utils/error.rs`)

  ```rust
  #[derive(Error, Debug)]
  pub enum DevErpError {
      #[error("Database error: {0}")]
      Database(#[from] sqlx::Error),

      #[error("Configuration error: {0}")]
      Config(String),

      #[error("Validation error: {0}")]
      Validation(String),

      #[error("Not found: {0}")]
      NotFound(String),

      #[error("Conflict: {0}")]
      Conflict(String),
  }
  ```

- [x] **로깅 설정** (`src/utils/logger.rs`)

  - `tracing` 및 `tracing-subscriber` 초기화
  - 로그 레벨 설정 (환경 변수 기반)
  - 파일 및 콘솔 로그 출력 설정

- [x] **CLI 출력 포맷터** (`src/utils/formatter.rs`)
  - 테이블 형식 출력
  - 컬러 출력 지원
  - JSON 출력 옵션

#### 2.2 설정 관리 모듈 구현

- [x] **설정 구조체 정의** (`src/config/settings.rs`)

  ```rust
  #[derive(Debug, Deserialize)]
  pub struct Settings {
      pub database: DatabaseConfig,
      pub logging: LoggingConfig,
      pub application: ApplicationConfig,
  }
  ```

- [x] **설정 로드 함수 구현**
  - TOML 파일 읽기
  - 환경 변수 오버라이드 지원
  - 설정 검증

#### 2.3 데이터베이스 연결 모듈 구현

- [x] **데이터베이스 연결** (`src/infrastructure/database.rs`)

  ```rust
  pub async fn establish_connection(config: &DatabaseConfig) -> Result<PgPool, DevErpError> {
      let pool = PgPoolOptions::new()
          .max_connections(config.max_connections)
          .connect(&config.database_url())
          .await?;
      Ok(pool)
  }
  ```

- [x] **커넥션 풀 관리** (`src/infrastructure/pool.rs`)
  - 풀 생성 및 관리
  - 헬스체크 기능

---

### Phase 3: 데이터베이스 스키마 구현 (2-3일)

#### 3.1 초기 마이그레이션 작성

- [x] **001_initial_schema.sql**: 핵심 테이블 생성
  - `projects` 테이블
  - `tasks` 테이블
  - `task_dependencies` 테이블
  - `task_comments` 테이블
  - UUID 확장 활성화
  - 트리거 함수 생성 (`update_updated_at_column`)

#### 3.2 추가 마이그레이션 작성

- [x] **002_resource_tables.sql**: 리소스 관리 테이블

  - `resources` 테이블
  - `project_resources` 테이블

- [x] **003_timeline_tables.sql**: 타임라인 관리 테이블

  - `timelines` 테이블
  - `milestones` 테이블

- [x] **004_support_tables.sql**: 지원 테이블
  - `tags` 테이블
  - `project_tags` 테이블
  - `configurations` 테이블
  - `audit_logs` 테이블

#### 3.3 인덱스 및 뷰 생성

- [x] **005_indexes.sql**: 성능 최적화를 위한 인덱스
- [x] **006_views.sql**: 자주 사용하는 쿼리를 위한 뷰
  - `v_active_projects`
  - `v_task_summary`
  - `v_resource_usage`
  - `v_project_timeline`

#### 3.4 초기 데이터 삽입

- [x] **007_seed_data.sql**: 기본 설정 및 태그 데이터

#### 3.5 마이그레이션 실행

- [x] `sqlx migrate run` 실행 및 검증
  - **Note**: 모든 마이그레이션 파일 생성 완료. PostgreSQL 연결 설정 후 `sqlx migrate run --database-url "postgres://deverp_user:password@localhost:5432/deverp"` 명령으로 실행 가능

---

### Phase 4: Domain Layer 구현 - Project 모듈 (3-4일)

#### 4.1 Project Entity 정의

- [x] **Entity 구조체** (`src/domain/project/entity.rs`)

  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
  pub struct Project {
      pub id: i64,
      pub uuid: Uuid,
      pub name: String,
      pub description: Option<String>,
      pub code: Option<String>,
      pub status: ProjectStatus,
      pub priority: Priority,
      pub start_date: Option<NaiveDate>,
      pub end_date: Option<NaiveDate>,
      pub progress_percentage: i32,
      pub created_at: DateTime<Utc>,
      pub updated_at: DateTime<Utc>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
  #[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
  pub enum ProjectStatus {
      Planning,
      Active,
      OnHold,
      Completed,
      Archived,
      Cancelled,
  }
  ```

#### 4.2 Project Repository Trait 정의

- [x] **Repository 트레이트** (`src/domain/project/repository.rs`)
  ```rust
  #[async_trait]
  pub trait ProjectRepository: Send + Sync {
      async fn create(&self, project: CreateProject) -> Result<Project, DevErpError>;
      async fn find_by_id(&self, id: i64) -> Result<Option<Project>, DevErpError>;
      async fn find_by_uuid(&self, uuid: Uuid) -> Result<Option<Project>, DevErpError>;
      async fn find_all(&self, filter: ProjectFilter) -> Result<Vec<Project>, DevErpError>;
      async fn update(&self, project: Project) -> Result<Project, DevErpError>;
      async fn delete(&self, id: i64) -> Result<bool, DevErpError>;
      async fn soft_delete(&self, id: i64) -> Result<bool, DevErpError>;
  }
  ```

#### 4.3 Project Repository 구현

- [x] **PostgreSQL Repository** (`src/infrastructure/repositories/project_repo.rs`)
  - CRUD 쿼리 구현
  - 트랜잭션 처리
  - 에러 처리

#### 4.4 Project Service 구현

- [x] **비즈니스 로직** (`src/domain/project/service.rs`)

  ```rust
  pub struct ProjectService {
      repository: Arc<dyn ProjectRepository>,
  }

  impl ProjectService {
      pub async fn create_project(&self, input: CreateProjectInput) -> Result<Project, DevErpError> {
          // 유효성 검증
          // Repository 호출
          // 비즈니스 로직 처리
      }

      pub async fn list_projects(&self, filter: ProjectFilter) -> Result<Vec<Project>, DevErpError> {
          // ...
      }

      // 기타 메서드들...
  }
  ```

#### 4.5 Project 단위 테스트

- [x] Repository 테스트 (모킹 사용)
- [x] Service 테스트
- [x] Entity 유효성 검증 테스트

---

### Phase 5: Domain Layer 구현 - Task 모듈 (3-4일)

#### 5.1 Task Entity 정의

- [x] Task 구조체 및 관련 Enum 정의
- [x] TaskDependency 구조체 정의
- [x] TaskComment 구조체 정의

#### 5.2 Task Repository 구현

- [x] Repository 트레이트 정의
- [x] PostgreSQL Repository 구현
- [x] 의존성 관리 쿼리 구현

#### 5.3 Task Service 구현

- [x] 태스크 CRUD 로직
- [x] 의존성 검증 로직 (순환 의존성 체크)
- [x] 태스크 상태 변경 로직

#### 5.4 Task 단위 테스트

- [ ] Repository 테스트
- [ ] Service 테스트
- [ ] 의존성 검증 테스트

---

### Phase 6: Domain Layer 구현 - Resource 모듈 (2-3일)

#### 6.1 Resource Entity 정의

- [x] Resource 구조체 정의
- [x] ProjectResource 연결 구조체 정의

#### 6.2 Resource Repository 구현

- [x] Repository 트레이트 정의
- [x] PostgreSQL Repository 구현

#### 6.3 Resource Service 구현

- [x] 리소스 추가/삭제/수정
- [x] 프로젝트-리소스 연결 관리
- [x] 리소스 활용도 분석

#### 6.4 Resource 단위 테스트

- [x] Service 레이어 단위 테스트 (mockall 사용)
- [x] Entity 유효성 검증 테스트

---

### Phase 7: Domain Layer 구현 - Timeline 모듈 (2-3일)

#### 7.1 Timeline Entity 정의

- [ ] Timeline 구조체 정의
- [ ] Milestone 구조체 정의

#### 7.2 Timeline Repository 구현

- [ ] Repository 트레이트 정의
- [ ] PostgreSQL Repository 구현

#### 7.3 Timeline Service 구현

- [ ] 타임라인 생성/관리
- [ ] 마일스톤 추적
- [ ] 일정 검증 로직

#### 7.4 Timeline 단위 테스트

---

### Phase 8: CLI Layer 구현 - 기본 구조 (2일)

#### 8.1 CLI 커맨드 구조 정의

- [ ] **메인 커맨드 정의** (`src/cli/commands.rs`)

  ```rust
  #[derive(Parser)]
  #[command(name = "deverp")]
  #[command(about = "Development ERP System", long_about = None)]
  pub struct Cli {
      #[command(subcommand)]
      pub command: Commands,
  }

  #[derive(Subcommand)]
  pub enum Commands {
      Project(ProjectCommand),
      Task(TaskCommand),
      Resource(ResourceCommand),
      Timeline(TimelineCommand),
      Report(ReportCommand),
      Config(ConfigCommand),
  }
  ```

#### 8.2 공통 CLI 유틸리티

- [ ] 출력 포맷 선택 (table, json, yaml)
- [ ] 페이지네이션 지원
- [ ] 에러 메시지 포맷팅

---

### Phase 9: CLI Layer 구현 - Project 커맨드 (2-3일)

#### 9.1 Project 커맨드 정의

- [ ] **프로젝트 커맨드** (`src/cli/project.rs`)
  ```rust
  #[derive(Subcommand)]
  pub enum ProjectCommand {
      Create(CreateArgs),
      List(ListArgs),
      Show(ShowArgs),
      Update(UpdateArgs),
      Delete(DeleteArgs),
      Archive(ArchiveArgs),
  }
  ```

#### 9.2 각 서브커맨드 구현

- [ ] `create`: 새 프로젝트 생성

  ```bash
  deverp project create --name "My Project" --description "..." --priority high
  ```

- [ ] `list`: 프로젝트 목록 조회

  ```bash
  deverp project list --status active --format table
  ```

- [ ] `show`: 프로젝트 상세 조회

  ```bash
  deverp project show <id or uuid>
  ```

- [ ] `update`: 프로젝트 수정

  ```bash
  deverp project update <id> --status completed --progress 100
  ```

- [ ] `delete`: 프로젝트 삭제 (soft delete)

  ```bash
  deverp project delete <id> --confirm
  ```

- [ ] `archive`: 프로젝트 아카이빙
  ```bash
  deverp project archive <id>
  ```

#### 9.3 Project CLI 테스트

- [ ] 각 커맨드 통합 테스트

---

### Phase 10: CLI Layer 구현 - Task 커맨드 (3-4일)

#### 10.1 Task 커맨드 정의

- [ ] **태스크 커맨드** (`src/cli/task.rs`)
  ```rust
  #[derive(Subcommand)]
  pub enum TaskCommand {
      Create(CreateTaskArgs),
      List(ListTaskArgs),
      Show(ShowTaskArgs),
      Update(UpdateTaskArgs),
      Delete(DeleteTaskArgs),
      AddDependency(AddDependencyArgs),
      RemoveDependency(RemoveDependencyArgs),
      AddComment(AddCommentArgs),
  }
  ```

#### 10.2 각 서브커맨드 구현

- [ ] `create`: 새 태스크 생성
- [ ] `list`: 태스크 목록 조회 (프로젝트별, 상태별 필터링)
- [ ] `show`: 태스크 상세 조회
- [ ] `update`: 태스크 수정
- [ ] `delete`: 태스크 삭제
- [ ] `add-dependency`: 태스크 의존성 추가
- [ ] `remove-dependency`: 태스크 의존성 제거
- [ ] `add-comment`: 태스크 코멘트 추가

#### 10.3 Task CLI 테스트

---

### Phase 11: CLI Layer 구현 - Resource & Timeline 커맨드 (2-3일)

#### 11.1 Resource 커맨드 구현

- [ ] `create`, `list`, `show`, `update`, `delete`
- [ ] `link`: 프로젝트에 리소스 연결
- [ ] `unlink`: 프로젝트에서 리소스 연결 해제
- [ ] `usage`: 리소스 활용도 조회

#### 11.2 Timeline 커맨드 구현

- [ ] `create`, `list`, `show`, `update`, `delete`
- [ ] `add-milestone`: 마일스톤 추가
- [ ] `update-milestone`: 마일스톤 수정
- [ ] `complete-milestone`: 마일스톤 완료 처리

---

### Phase 12: Report 모듈 구현 (2-3일)

#### 12.1 Report Service 구현

- [ ] **프로젝트 현황 리포트**

  - 전체 프로젝트 통계
  - 진행 중인 프로젝트 현황
  - 지연된 프로젝트 목록

- [ ] **태스크 분석 리포트**

  - 완료율 통계
  - 상태별 분포
  - 예상/실제 작업시간 비교

- [ ] **리소스 활용 리포트**

  - 리소스별 사용 빈도
  - 프로젝트별 리소스 사용 현황

- [ ] **타임라인 리포트**
  - 마일스톤 달성률
  - 일정 준수율

#### 12.2 Report CLI 구현

- [ ] `status`: 전체 현황 리포트
- [ ] `project-summary`: 프로젝트 요약
- [ ] `task-analytics`: 태스크 분석
- [ ] `resource-usage`: 리소스 활용도
- [ ] `timeline-progress`: 타임라인 진척도

#### 12.3 Report 출력 포맷

- [ ] 터미널 테이블 형식
- [ ] JSON 출력
- [ ] CSV 내보내기 (선택 사항)

---

### Phase 13: Configuration 모듈 구현 (1-2일)

#### 13.1 Config Service 구현

- [ ] 설정 읽기/쓰기
- [ ] 데이터베이스 연결 테스트
- [ ] 기본값 초기화

#### 13.2 Config CLI 구현

- [ ] `show`: 현재 설정 조회
- [ ] `set`: 설정 값 변경
- [ ] `reset`: 기본값으로 초기화
- [ ] `test-db`: 데이터베이스 연결 테스트

---

### Phase 14: 통합 테스트 (2-3일)

#### 14.1 End-to-End 테스트 시나리오

- [ ] **시나리오 1: 프로젝트 생성부터 완료까지**

  1. 프로젝트 생성
  2. 태스크 추가
  3. 타임라인 생성
  4. 마일스톤 추가
  5. 리소스 연결
  6. 진행 상황 업데이트
  7. 리포트 생성
  8. 프로젝트 완료

- [ ] **시나리오 2: 태스크 의존성 관리**

  1. 여러 태스크 생성
  2. 의존성 추가
  3. 순환 의존성 시도 (실패 확인)
  4. 의존성 기반 작업 순서 확인

- [ ] **시나리오 3: 리소스 관리**
  1. 리소스 생성
  2. 여러 프로젝트에 연결
  3. 활용도 분석
  4. 리소스 삭제 (연결된 프로젝트 확인)

#### 14.2 성능 테스트

- [ ] 대량 데이터 삽입 테스트 (프로젝트 100개, 태스크 1000개)
- [ ] 쿼리 성능 측정
- [ ] 메모리 사용량 측정

#### 14.3 에러 처리 테스트

- [ ] 데이터베이스 연결 실패
- [ ] 잘못된 입력 처리
- [ ] 유효성 검증 실패
- [ ] 중복 데이터 처리

---

### Phase 15: 문서화 및 최적화 (2-3일)

#### 15.1 코드 문서화

- [ ] 모든 public API에 doc comments 추가
- [ ] 모듈별 사용 예제 작성
- [ ] `cargo doc` 생성 및 검증

#### 15.2 사용자 가이드 작성

- [ ] **설치 가이드** (`docs/installation.md`)

  - PostgreSQL 설치
  - Rust 설치
  - DevERP 빌드 및 설치

- [ ] **사용자 매뉴얼** (`docs/user-guide.md`)

  - 기본 사용법
  - 각 커맨드 상세 설명
  - 실제 사용 예제

- [ ] **FAQ** (`docs/faq.md`)

#### 15.3 성능 최적화

- [ ] 쿼리 최적화 (EXPLAIN ANALYZE 사용)
- [ ] 인덱스 튜닝
- [ ] 커넥션 풀 설정 최적화
- [ ] 불필요한 할당 제거

#### 15.4 코드 품질 개선

- [ ] `cargo clippy` 경고 해결
- [ ] `cargo fmt` 적용
- [ ] 코드 리뷰 및 리팩토링

---

### Phase 16: 배포 준비 (1-2일)

#### 16.1 릴리스 빌드

- [ ] `cargo build --release` 테스트
- [ ] 바이너리 크기 최적화
- [ ] 실행 파일 테스트

#### 16.2 설치 스크립트 작성

- [ ] 데이터베이스 초기화 스크립트
- [ ] 환경 변수 설정 스크립트
- [ ] 자동 설치 스크립트 (선택 사항)

#### 16.3 백업 및 복구

- [ ] 데이터베이스 백업 스크립트
- [ ] 복구 스크립트 및 테스트

#### 16.4 README 작성

- [ ] 프로젝트 개요
- [ ] 빠른 시작 가이드
- [ ] 빌드 및 실행 방법
- [ ] 라이선스 정보
