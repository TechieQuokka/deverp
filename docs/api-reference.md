# DevERP API Reference

> **DevERP CLI 명령어 완전 가이드**
> 버전: 1.0.0
> 최종 업데이트: 2025-10-23

## 📚 목차

- [개요](#개요)
- [공통 옵션](#공통-옵션)
- [프로젝트 관리 (Project)](#프로젝트-관리-project)
- [작업 관리 (Task)](#작업-관리-task)
- [리소스 관리 (Resource)](#리소스-관리-resource)
- [타임라인 관리 (Timeline)](#타임라인-관리-timeline)
- [리포트 생성 (Report)](#리포트-생성-report)
- [시스템 설정 (Config)](#시스템-설정-config)
- [데이터 타입 참조](#데이터-타입-참조)
- [오류 처리](#오류-처리)
- [사용 예제](#사용-예제)

---

## 개요

DevERP는 개발 프로젝트 관리를 위한 CLI 기반 ERP 시스템입니다. 모든 명령어는 다음 형식을 따릅니다:

```bash
deverp [전역옵션] <명령어> <하위명령어> [옵션] [인자]
```

### 기본 사용법

```bash
# 도움말 보기
deverp --help
deverp project --help

# 버전 확인
deverp --version

# JSON 출력 형식
deverp --format json project list

# 상세 로그 출력
deverp --verbose project create --name "My Project"
```

---

## 공통 옵션

모든 명령어에서 사용 가능한 전역 옵션입니다.

| 옵션 | 단축 | 타입 | 기본값 | 설명 |
|------|------|------|--------|------|
| `--format` | `-f` | enum | `table` | 출력 형식 (`table`, `json`, `plain`) |
| `--verbose` | `-v` | flag | `false` | 상세 로그 출력 |
| `--help` | `-h` | flag | - | 도움말 표시 |
| `--version` | `-V` | flag | - | 버전 정보 표시 |

### 페이징 옵션

리스트 조회 명령어에서 사용 가능한 페이징 옵션입니다.

| 옵션 | 타입 | 기본값 | 설명 |
|------|------|--------|------|
| `--page` | u32 | `1` | 페이지 번호 (1부터 시작) |
| `--per-page` | u32 | `50` | 페이지당 항목 수 |

---

## 프로젝트 관리 (Project)

프로젝트는 DevERP의 핵심 엔티티로, 모든 작업과 리소스를 그룹화합니다.

### 프로젝트 생성

새로운 프로젝트를 생성합니다.

```bash
deverp project create [옵션]
```

#### 필수 옵션

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--name` | `-n` | String | 프로젝트 이름 (최대 255자) |

#### 선택 옵션

| 옵션 | 단축 | 타입 | 기본값 | 설명 |
|------|------|------|--------|------|
| `--description` | `-d` | String | - | 프로젝트 설명 |
| `--code` | `-c` | String | - | 프로젝트 코드 (고유값, 최대 50자) |
| `--status` | `-s` | Enum | `planning` | 프로젝트 상태 ([상태 목록](#projectstatus)) |
| `--priority` | `-p` | Enum | `medium` | 우선순위 ([우선순위 목록](#priority)) |
| `--start-date` | | Date | - | 시작 날짜 (YYYY-MM-DD) |
| `--end-date` | | Date | - | 종료 날짜 (YYYY-MM-DD) |
| `--repository-url` | | String | - | Git 저장소 URL |
| `--repository-branch` | | String | `main` | Git 브랜치 |
| `--tags` | | String | - | 태그 (쉼표로 구분) |

#### 예제

```bash
# 기본 프로젝트 생성
deverp project create \
  --name "DevERP v2.0" \
  --description "ERP 시스템 차세대 버전"

# 완전한 프로젝트 생성
deverp project create \
  --name "Mobile App" \
  --code "MOBILE-001" \
  --status active \
  --priority high \
  --start-date 2025-01-01 \
  --end-date 2025-06-30 \
  --repository-url "https://github.com/myorg/mobile-app" \
  --repository-branch "develop" \
  --tags "mobile,ios,android"
```

#### 출력 예시

```
✓ Project created successfully!

ID:          1
UUID:        550e8400-e29b-41d4-a716-446655440000
Name:        DevERP v2.0
Description: ERP 시스템 차세대 버전
Status:      planning
Priority:    medium
```

---

### 프로젝트 목록 조회

필터링과 페이징을 지원하는 프로젝트 목록을 조회합니다.

```bash
deverp project list [옵션]
```

#### 필터 옵션

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--status` | `-s` | Enum | 상태별 필터링 |
| `--priority` | `-p` | Enum | 우선순위별 필터링 |
| `--search` | `-q` | String | 이름/설명 검색 (부분 일치) |
| `--tags` | | String | 태그별 필터링 (쉼표로 구분) |
| `--page` | | u32 | 페이지 번호 (기본: 1) |
| `--per-page` | | u32 | 페이지당 항목 수 (기본: 50) |

#### 예제

```bash
# 모든 프로젝트 조회
deverp project list

# Active 상태의 High 우선순위 프로젝트
deverp project list --status active --priority high

# 이름에 "mobile" 포함된 프로젝트 검색
deverp project list --search mobile

# 태그로 필터링
deverp project list --tags "mobile,backend"

# 페이징
deverp project list --page 2 --per-page 20

# JSON 형식으로 출력
deverp --format json project list --status active
```

#### 출력 예시

```
Projects (3 found)

  • Mobile App - active
    ID: 1 | UUID: 550e8400-e29b-41d4-a716-446655440000
    모바일 애플리케이션 개발 프로젝트
    Priority: high | Progress: 45%

  • Backend API - active
    ID: 2 | UUID: 660e9511-f39c-52e5-b827-557766551111
    RESTful API 서버 구축
    Priority: critical | Progress: 80%

ℹ Showing 1-3 of 3 items (Page 1)
```

---

### 프로젝트 상세 조회

특정 프로젝트의 상세 정보를 조회합니다.

```bash
deverp project show <식별자>
```

#### 인자

| 인자 | 타입 | 설명 |
|------|------|------|
| `<식별자>` | String | 프로젝트 ID (숫자) 또는 UUID |

#### 예제

```bash
# ID로 조회
deverp project show 1

# UUID로 조회
deverp project show 550e8400-e29b-41d4-a716-446655440000
```

#### 출력 예시

```
Project: Mobile App

ID:              1
UUID:            550e8400-e29b-41d4-a716-446655440000
Name:            Mobile App
Description:     모바일 애플리케이션 개발 프로젝트
Code:            MOBILE-001
Status:          active
Priority:        high
Progress:        45%
Start Date:      2025-01-01
End Date:        2025-06-30
Repository:      https://github.com/myorg/mobile-app
Branch:          develop
Tags:            mobile, ios, android

Created:         2025-01-15 14:30:00
Updated:         2025-01-20 09:15:00
```

---

### 프로젝트 수정

기존 프로젝트를 수정합니다.

```bash
deverp project update <식별자> [옵션]
```

#### 인자

| 인자 | 타입 | 설명 |
|------|------|------|
| `<식별자>` | String | 프로젝트 ID 또는 UUID |

#### 수정 가능 옵션

모든 옵션은 선택 사항이며, 제공된 옵션만 수정됩니다.

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--name` | `-n` | String | 새 프로젝트 이름 |
| `--description` | `-d` | String | 새 설명 |
| `--code` | `-c` | String | 새 프로젝트 코드 |
| `--status` | `-s` | Enum | 새 상태 |
| `--priority` | `-p` | Enum | 새 우선순위 |
| `--start-date` | | Date | 새 시작 날짜 |
| `--end-date` | | Date | 새 종료 날짜 |
| `--actual-start-date` | | Date | 실제 시작 날짜 |
| `--actual-end-date` | | Date | 실제 종료 날짜 |
| `--progress` | | i32 | 진행률 (0-100) |
| `--repository-url` | | String | 새 저장소 URL |
| `--repository-branch` | | String | 새 브랜치 |
| `--tags` | | String | 새 태그 목록 |

#### 예제

```bash
# 상태 변경
deverp project update 1 --status active

# 진행률 업데이트
deverp project update 1 --progress 75

# 여러 필드 동시 수정
deverp project update 1 \
  --status completed \
  --progress 100 \
  --actual-end-date 2025-06-25

# UUID로 수정
deverp project update 550e8400-e29b-41d4-a716-446655440000 \
  --priority critical
```

#### 출력 예시

```
✓ Project updated successfully!

ID:       1
Name:     Mobile App
Status:   completed
Priority: high
Progress: 100%
```

---

### 프로젝트 삭제

프로젝트를 소프트 삭제합니다 (deleted_at 설정).

```bash
deverp project delete <식별자> [옵션]
```

#### 인자

| 인자 | 타입 | 설명 |
|------|------|------|
| `<식별자>` | String | 프로젝트 ID 또는 UUID |

#### 옵션

| 옵션 | 타입 | 기본값 | 설명 |
|------|------|--------|------|
| `--confirm` | flag | `false` | 확인 프롬프트 생략 |

#### 예제

```bash
# 확인 후 삭제
deverp project delete 1

# 즉시 삭제 (확인 없이)
deverp project delete 1 --confirm
```

#### 출력 예시

```
Are you sure you want to delete project 'Mobile App'? This action cannot be undone.
[y/N]: y

✓ Project 'Mobile App' deleted successfully.
```

---

### 프로젝트 아카이브

프로젝트 상태를 'archived'로 변경합니다.

```bash
deverp project archive <식별자>
```

#### 인자

| 인자 | 타입 | 설명 |
|------|------|------|
| `<식별자>` | String | 프로젝트 ID 또는 UUID |

#### 예제

```bash
deverp project archive 1
```

#### 출력 예시

```
✓ Project 'Mobile App' archived successfully.
Status:  archived
```

---

## 작업 관리 (Task)

작업(Task)은 프로젝트 내의 개별 작업 항목을 나타냅니다.

### 작업 생성

```bash
deverp task create [옵션]
```

#### 필수 옵션

| 옵션 | 타입 | 설명 |
|------|------|------|
| `--project-id` | i64 | 소속 프로젝트 ID |
| `--title` | String | 작업 제목 (최대 500자) |

#### 선택 옵션

| 옵션 | 단축 | 타입 | 기본값 | 설명 |
|------|------|------|--------|------|
| `--description` | `-d` | String | - | 작업 설명 |
| `--parent-task-id` | | i64 | - | 상위 작업 ID (하위 작업인 경우) |
| `--task-number` | | String | - | 작업 번호 (예: TASK-001) |
| `--status` | `-s` | Enum | `todo` | 작업 상태 ([상태 목록](#taskstatus)) |
| `--priority` | `-p` | Enum | `medium` | 우선순위 |
| `--assigned-to` | | String | - | 담당자 |
| `--estimated-hours` | | f64 | - | 예상 소요 시간 |
| `--due-date` | | DateTime | - | 마감일 (YYYY-MM-DD 또는 YYYY-MM-DD HH:MM:SS) |
| `--task-type` | | Enum | - | 작업 유형 ([유형 목록](#tasktype)) |
| `--tags` | | String | - | 태그 (쉼표로 구분) |

#### 예제

```bash
# 기본 작업 생성
deverp task create \
  --project-id 1 \
  --title "사용자 로그인 기능 구현"

# 상세 작업 생성
deverp task create \
  --project-id 1 \
  --title "JWT 인증 구현" \
  --description "Access Token과 Refresh Token 구현" \
  --status in_progress \
  --priority high \
  --assigned-to "developer@example.com" \
  --estimated-hours 8.5 \
  --due-date "2025-01-25 18:00:00" \
  --task-type feature \
  --tags "backend,security,authentication"

# 하위 작업 생성
deverp task create \
  --project-id 1 \
  --parent-task-id 5 \
  --title "로그인 API 테스트 작성" \
  --task-type test
```

---

### 작업 목록 조회

```bash
deverp task list [옵션]
```

#### 필터 옵션

| 옵션 | 타입 | 설명 |
|------|------|------|
| `--project-id` | i64 | 특정 프로젝트의 작업만 조회 |
| `--status` | Enum | 상태별 필터링 |
| `--priority` | Enum | 우선순위별 필터링 |
| `--task-type` | Enum | 유형별 필터링 |
| `--assigned-to` | String | 담당자별 필터링 |
| `--parent-task-id` | i64 | 특정 상위 작업의 하위 작업만 조회 |
| `--page` | u32 | 페이지 번호 |
| `--per-page` | u32 | 페이지당 항목 수 |

#### 예제

```bash
# 프로젝트의 모든 작업
deverp task list --project-id 1

# 진행 중인 작업만
deverp task list --status in_progress

# 내가 담당한 작업
deverp task list --assigned-to "developer@example.com"

# 버그 작업만
deverp task list --task-type bug --priority high

# 특정 작업의 하위 작업
deverp task list --parent-task-id 5
```

---

### 작업 상세 조회

```bash
deverp task show <식별자>
```

#### 인자

| 인자 | 타입 | 설명 |
|------|------|------|
| `<식별자>` | String | 작업 ID 또는 UUID |

---

### 작업 수정

```bash
deverp task update <식별자> [옵션]
```

#### 수정 가능 옵션

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--title` | `-t` | String | 새 제목 |
| `--description` | `-d` | String | 새 설명 |
| `--status` | `-s` | Enum | 새 상태 |
| `--priority` | `-p` | Enum | 새 우선순위 |
| `--assigned-to` | | String | 새 담당자 |
| `--estimated-hours` | | f64 | 새 예상 시간 |
| `--actual-hours` | | f64 | 실제 소요 시간 |
| `--due-date` | | DateTime | 새 마감일 |
| `--task-type` | | Enum | 새 작업 유형 |
| `--tags` | | String | 새 태그 |

#### 예제

```bash
# 작업 상태 변경
deverp task update 10 --status done --actual-hours 7.5

# 담당자 변경
deverp task update 10 --assigned-to "another@example.com"

# 우선순위 상향
deverp task update 10 --priority critical
```

---

### 작업 삭제

```bash
deverp task delete <식별자> [--confirm]
```

---

### 작업 의존성 추가

작업 간의 의존 관계를 설정합니다.

```bash
deverp task add-dependency [옵션]
```

#### 필수 옵션

| 옵션 | 타입 | 설명 |
|------|------|------|
| `--task-id` | i64 | 의존하는 작업 ID |
| `--depends-on-task-id` | i64 | 선행 작업 ID |

#### 선택 옵션

| 옵션 | 타입 | 기본값 | 설명 |
|------|------|--------|------|
| `--dependency-type` | Enum | `finish_to_start` | 의존성 유형 ([유형 목록](#dependencytype)) |

#### 예제

```bash
# 기본 의존성 추가 (Finish-to-Start)
deverp task add-dependency \
  --task-id 15 \
  --depends-on-task-id 10

# Start-to-Start 의존성
deverp task add-dependency \
  --task-id 15 \
  --depends-on-task-id 10 \
  --dependency-type start_to_start
```

#### 의존성 유형 설명

- **finish_to_start**: 선행 작업이 완료되어야 후속 작업 시작 가능 (기본값)
- **start_to_start**: 선행 작업이 시작되어야 후속 작업 시작 가능
- **finish_to_finish**: 선행 작업이 완료되어야 후속 작업 완료 가능
- **start_to_finish**: 선행 작업이 시작되어야 후속 작업 완료 가능

---

### 작업 의존성 제거

```bash
deverp task remove-dependency [옵션]
```

#### 필수 옵션

| 옵션 | 타입 | 설명 |
|------|------|------|
| `--task-id` | i64 | 작업 ID |
| `--depends-on-task-id` | i64 | 제거할 의존성의 선행 작업 ID |

#### 예제

```bash
deverp task remove-dependency \
  --task-id 15 \
  --depends-on-task-id 10
```

---

### 작업 댓글 추가

```bash
deverp task add-comment [옵션]
```

#### 필수 옵션

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--task-id` | | i64 | 작업 ID |
| `--comment` | `-c` | String | 댓글 내용 |

#### 선택 옵션

| 옵션 | 타입 | 설명 |
|------|------|------|
| `--author` | String | 작성자 이름/이메일 |

#### 예제

```bash
# 댓글 추가
deverp task add-comment \
  --task-id 10 \
  --comment "API 테스트 완료, 코드 리뷰 요청합니다." \
  --author "developer@example.com"

# 간단한 댓글
deverp task add-comment \
  --task-id 10 \
  --comment "LGTM"
```

---

## 리소스 관리 (Resource)

리소스는 프로젝트에서 사용하는 라이브러리, API, 도구 등을 관리합니다.

### 리소스 생성

```bash
deverp resource create [옵션]
```

#### 필수 옵션

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--name` | `-n` | String | 리소스 이름 |
| `--resource-type` | `-t` | Enum | 리소스 유형 ([유형 목록](#resourcetype)) |

#### 선택 옵션

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--description` | `-d` | String | 설명 |
| `--version` | `-v` | String | 버전 |
| `--url` | `-u` | String | 리소스 URL |
| `--documentation-url` | | String | 문서 URL |
| `--license` | `-l` | String | 라이선스 |
| `--status` | `-s` | Enum | 상태 (`active`, `deprecated`, `archived`) |
| `--tags` | | String | 태그 |

#### 예제

```bash
# 라이브러리 추가
deverp resource create \
  --name "Tokio" \
  --resource-type library \
  --version "1.35.0" \
  --url "https://crates.io/crates/tokio" \
  --documentation-url "https://docs.rs/tokio" \
  --license "MIT" \
  --status active \
  --tags "async,runtime"

# API 리소스 추가
deverp resource create \
  --name "GitHub API v3" \
  --resource-type api \
  --url "https://api.github.com" \
  --documentation-url "https://docs.github.com/en/rest"

# 도구 추가
deverp resource create \
  --name "Docker" \
  --resource-type tool \
  --version "24.0.0"
```

---

### 리소스 목록 조회

```bash
deverp resource list [옵션]
```

#### 필터 옵션

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--resource-type` | `-t` | Enum | 유형별 필터링 |
| `--status` | `-s` | Enum | 상태별 필터링 |
| `--search` | `-q` | String | 이름 검색 |
| `--tags` | | String | 태그별 필터링 |

#### 예제

```bash
# 모든 라이브러리 조회
deverp resource list --resource-type library

# 활성 상태의 API
deverp resource list --resource-type api --status active

# 이름 검색
deverp resource list --search "tokio"
```

---

### 리소스 상세 조회

```bash
deverp resource show <식별자>
```

---

### 리소스 수정

```bash
deverp resource update <식별자> [옵션]
```

---

### 리소스 삭제

```bash
deverp resource delete <식별자> [--confirm]
```

---

### 리소스를 프로젝트에 연결

프로젝트에서 사용하는 리소스를 등록합니다.

```bash
deverp resource link [옵션]
```

#### 필수 옵션

| 옵션 | 타입 | 설명 |
|------|------|------|
| `--project-id` | i64 | 프로젝트 ID |
| `--resource-id` | i64 | 리소스 ID |

#### 선택 옵션

| 옵션 | 타입 | 설명 |
|------|------|------|
| `--usage-notes` | String | 사용 방법/목적 메모 |
| `--version-used` | String | 프로젝트에서 사용하는 버전 |
| `--is-critical` | flag | 핵심 리소스 여부 |

#### 예제

```bash
# 기본 연결
deverp resource link \
  --project-id 1 \
  --resource-id 5

# 상세 정보와 함께 연결
deverp resource link \
  --project-id 1 \
  --resource-id 5 \
  --usage-notes "비동기 런타임으로 사용" \
  --version-used "1.35.0" \
  --is-critical
```

---

### 리소스 연결 해제

```bash
deverp resource unlink [옵션]
```

#### 필수 옵션

| 옵션 | 타입 | 설명 |
|------|------|------|
| `--project-id` | i64 | 프로젝트 ID |
| `--resource-id` | i64 | 리소스 ID |

#### 예제

```bash
deverp resource unlink --project-id 1 --resource-id 5
```

---

### 리소스 사용 현황

특정 리소스 또는 전체 리소스의 사용 통계를 조회합니다.

```bash
deverp resource usage [리소스ID]
```

#### 예제

```bash
# 전체 리소스 사용 현황
deverp resource usage

# 특정 리소스 사용 현황
deverp resource usage 5
```

---

## 타임라인 관리 (Timeline)

타임라인은 프로젝트의 일정과 마일스톤을 관리합니다.

### 타임라인 생성

```bash
deverp timeline create [옵션]
```

#### 필수 옵션

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--project-id` | | i64 | 프로젝트 ID |
| `--name` | `-n` | String | 타임라인 이름 |
| `--start-date` | | Date | 시작 날짜 (YYYY-MM-DD) |
| `--end-date` | | Date | 종료 날짜 (YYYY-MM-DD) |

#### 선택 옵션

| 옵션 | 단축 | 타입 | 기본값 | 설명 |
|------|------|------|--------|------|
| `--description` | `-d` | String | - | 설명 |
| `--timeline-type` | `-t` | Enum | `project` | 타임라인 유형 ([유형 목록](#timelinetype)) |
| `--status` | `-s` | Enum | `planned` | 상태 |

#### 예제

```bash
# 프로젝트 타임라인 생성
deverp timeline create \
  --project-id 1 \
  --name "Mobile App Development" \
  --start-date 2025-01-01 \
  --end-date 2025-12-31

# 스프린트 타임라인 생성
deverp timeline create \
  --project-id 1 \
  --name "Sprint 1" \
  --timeline-type sprint \
  --start-date 2025-01-01 \
  --end-date 2025-01-14 \
  --status active

# 릴리스 타임라인
deverp timeline create \
  --project-id 1 \
  --name "v1.0.0 Release" \
  --timeline-type release \
  --start-date 2025-06-01 \
  --end-date 2025-06-30
```

---

### 타임라인 목록 조회

```bash
deverp timeline list [옵션]
```

#### 필터 옵션

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--project-id` | | i64 | 프로젝트별 필터링 |
| `--timeline-type` | `-t` | Enum | 유형별 필터링 |
| `--status` | `-s` | Enum | 상태별 필터링 |

#### 예제

```bash
# 프로젝트의 모든 타임라인
deverp timeline list --project-id 1

# 활성 스프린트만
deverp timeline list --timeline-type sprint --status active
```

---

### 타임라인 상세 조회

```bash
deverp timeline show <타임라인ID>
```

---

### 타임라인 수정

```bash
deverp timeline update <타임라인ID> [옵션]
```

#### 수정 가능 옵션

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--name` | `-n` | String | 새 이름 |
| `--description` | `-d` | String | 새 설명 |
| `--timeline-type` | `-t` | Enum | 새 유형 |
| `--start-date` | | Date | 새 시작 날짜 |
| `--end-date` | | Date | 새 종료 날짜 |
| `--status` | `-s` | Enum | 새 상태 |

#### 예제

```bash
# 타임라인 완료 처리
deverp timeline update 1 --status completed

# 날짜 연장
deverp timeline update 1 --end-date 2025-02-15
```

---

### 타임라인 삭제

```bash
deverp timeline delete <타임라인ID> [--confirm]
```

---

### 마일스톤 추가

타임라인에 마일스톤을 추가합니다.

```bash
deverp timeline add-milestone [옵션]
```

#### 필수 옵션

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--timeline-id` | | i64 | 타임라인 ID |
| `--project-id` | | i64 | 프로젝트 ID |
| `--name` | `-n` | String | 마일스톤 이름 |
| `--target-date` | | Date | 목표 날짜 (YYYY-MM-DD) |

#### 선택 옵션

| 옵션 | 단축 | 타입 | 기본값 | 설명 |
|------|------|------|--------|------|
| `--description` | `-d` | String | - | 설명 |
| `--status` | `-s` | Enum | `pending` | 상태 ([상태 목록](#milestonestatus)) |

#### 예제

```bash
# 마일스톤 추가
deverp timeline add-milestone \
  --timeline-id 1 \
  --project-id 1 \
  --name "Beta Release" \
  --target-date 2025-03-31 \
  --description "베타 버전 출시"

# 진행 중인 마일스톤
deverp timeline add-milestone \
  --timeline-id 1 \
  --project-id 1 \
  --name "Feature Freeze" \
  --target-date 2025-03-15 \
  --status in_progress
```

---

### 마일스톤 수정

```bash
deverp timeline update-milestone <마일스톤ID> [옵션]
```

#### 수정 가능 옵션

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--name` | `-n` | String | 새 이름 |
| `--description` | `-d` | String | 새 설명 |
| `--target-date` | | Date | 새 목표 날짜 |
| `--actual-date` | | Date | 실제 달성 날짜 |
| `--status` | `-s` | Enum | 새 상태 |
| `--completion-percentage` | | i32 | 완료율 (0-100) |

#### 예제

```bash
# 완료율 업데이트
deverp timeline update-milestone 5 --completion-percentage 75

# 마일스톤 완료
deverp timeline update-milestone 5 \
  --status completed \
  --actual-date 2025-03-30 \
  --completion-percentage 100
```

---

### 마일스톤 완료

마일스톤을 완료 상태로 변경합니다.

```bash
deverp timeline complete-milestone <마일스톤ID> [옵션]
```

#### 선택 옵션

| 옵션 | 타입 | 기본값 | 설명 |
|------|------|--------|------|
| `--actual-date` | Date | 오늘 | 실제 완료 날짜 |

#### 예제

```bash
# 오늘 날짜로 완료
deverp timeline complete-milestone 5

# 특정 날짜로 완료
deverp timeline complete-milestone 5 --actual-date 2025-03-28
```

---

## 리포트 생성 (Report)

프로젝트, 작업, 리소스 등의 통계 및 분석 리포트를 생성합니다.

### 전체 상태 리포트

시스템 전체의 프로젝트 현황을 요약합니다.

```bash
deverp report status
```

#### 출력 내용

- 총 프로젝트 수
- 상태별 프로젝트 분포 (Active, Completed, OnHold 등)
- 우선순위별 분포
- 평균 진행률
- 지연된 프로젝트 수
- 생성 타임스탬프

#### 예제

```bash
# 테이블 형식으로 출력
deverp report status

# JSON 형식으로 출력
deverp --format json report status
```

---

### 프로젝트 요약 리포트

모든 프로젝트의 요약 정보를 조회합니다.

```bash
deverp report project-summary
```

#### 출력 내용

각 프로젝트별:
- 프로젝트 ID, 이름, 코드
- 상태 및 우선순위
- 진행률
- 총 작업 수 / 완료된 작업 수
- 시작/종료 날짜

---

### 작업 분석 리포트

작업 완료율 및 시간 분석 리포트를 생성합니다.

```bash
deverp report task-analytics
```

#### 출력 내용

- 총 작업 수
- 상태별 작업 분포 (Todo, InProgress, Done 등)
- 우선순위별 분포
- 완료율 (%)
- 평균 예상 시간 / 실제 소요 시간
- 시간 분산 (%) - (실제 - 예상) / 예상 * 100
- 기한 초과 작업 수
- 정시 완료 작업 수

---

### 리소스 사용 리포트

리소스 사용 현황 및 통계를 조회합니다.

```bash
deverp report resource-usage
```

#### 출력 내용

- 총 리소스 수
- 활성/폐기된 리소스 수
- 유형별 리소스 분포
- 가장 많이 사용된 리소스 (Top 10)
  - 리소스 이름, 유형
  - 사용 프로젝트 수
  - 핵심 프로젝트 수
- 미사용 리소스 수

---

### 타임라인 진행 리포트

타임라인 및 마일스톤 진행 상황을 조회합니다.

```bash
deverp report timeline-progress
```

#### 출력 내용

- 총 타임라인 수
- 활성/완료된 타임라인 수
- 총 마일스톤 수
- 완료/누락된 마일스톤 수
- 마일스톤 완료율 (%)
- 정시 완료율 (%)
- 향후 30일 내 마일스톤 수

---

## 시스템 설정 (Config)

시스템 설정을 조회하고 관리합니다.

### 설정 조회

```bash
deverp config show [옵션]
```

#### 선택 옵션

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--key` | `-k` | String | 특정 설정 키 조회 (미지정 시 전체 조회) |

#### 예제

```bash
# 모든 설정 조회
deverp config show

# 특정 설정 조회
deverp config show --key database.max_connections
```

---

### 설정 변경

```bash
deverp config set <키> <값> [옵션]
```

#### 인자

| 인자 | 타입 | 설명 |
|------|------|------|
| `<키>` | String | 설정 키 |
| `<값>` | String | 설정 값 |

#### 선택 옵션

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--description` | `-d` | String | 설정 설명 |

#### 예제

```bash
# 단순 설정 변경
deverp config set database.max_connections 10

# 설명과 함께 설정
deverp config set app.timeout 30 \
  --description "API 요청 타임아웃 (초)"

# Boolean 값
deverp config set features.debug_mode true

# JSON 값
deverp config set features.limits '{"max_projects":100,"max_tasks":1000}'
```

---

### 설정 초기화

모든 설정을 기본값으로 재설정합니다.

```bash
deverp config reset [옵션]
```

#### 필수 옵션

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--confirm` | `-c` | flag | 초기화 확인 (필수) |

#### 예제

```bash
deverp config reset --confirm
```

---

### 데이터베이스 연결 테스트

```bash
deverp config test-db [옵션]
```

#### 선택 옵션

| 옵션 | 단축 | 타입 | 설명 |
|------|------|------|------|
| `--verbose` | `-v` | flag | 상세 데이터베이스 정보 표시 |

#### 예제

```bash
# 기본 연결 테스트
deverp config test-db

# 상세 정보 포함
deverp config test-db --verbose
```

---

## 데이터 타입 참조

### ProjectStatus

프로젝트 상태를 나타냅니다.

| 값 | 설명 |
|----|------|
| `planning` | 기획 중 (기본값) |
| `active` | 진행 중 |
| `on_hold` | 보류됨 |
| `completed` | 완료됨 |
| `archived` | 아카이브됨 |
| `cancelled` | 취소됨 |

---

### TaskStatus

작업 상태를 나타냅니다.

| 값 | 설명 |
|----|------|
| `todo` | 할 일 (기본값) |
| `in_progress` | 진행 중 |
| `blocked` | 차단됨 |
| `review` | 리뷰 중 |
| `testing` | 테스트 중 |
| `done` | 완료 |
| `cancelled` | 취소됨 |

---

### Priority

우선순위를 나타냅니다.

| 값 | 설명 |
|----|------|
| `low` | 낮음 |
| `medium` | 보통 (기본값) |
| `high` | 높음 |
| `critical` | 긴급 |

---

### TaskType

작업 유형을 나타냅니다.

| 값 | 설명 |
|----|------|
| `feature` | 새 기능 |
| `bug` | 버그 수정 |
| `enhancement` | 기능 개선 |
| `refactor` | 리팩토링 |
| `docs` | 문서 작업 |
| `test` | 테스트 작성 |
| `chore` | 기타 작업 |

---

### DependencyType

작업 의존성 유형을 나타냅니다.

| 값 | 설명 |
|----|------|
| `finish_to_start` | 선행 작업 완료 → 후속 작업 시작 (기본값) |
| `start_to_start` | 선행 작업 시작 → 후속 작업 시작 |
| `finish_to_finish` | 선행 작업 완료 → 후속 작업 완료 |
| `start_to_finish` | 선행 작업 시작 → 후속 작업 완료 |

---

### ResourceType

리소스 유형을 나타냅니다.

| 값 | 설명 |
|----|------|
| `library` | 라이브러리 |
| `api` | API |
| `tool` | 도구 |
| `service` | 서비스 |
| `documentation` | 문서 |
| `other` | 기타 |

---

### TimelineType

타임라인 유형을 나타냅니다.

| 값 | 설명 |
|----|------|
| `project` | 프로젝트 타임라인 (기본값) |
| `sprint` | 스프린트 |
| `release` | 릴리스 |
| `phase` | 단계/페이즈 |

---

### MilestoneStatus

마일스톤 상태를 나타냅니다.

| 값 | 설명 |
|----|------|
| `pending` | 대기 중 (기본값) |
| `in_progress` | 진행 중 |
| `completed` | 완료됨 |
| `missed` | 놓침 |
| `cancelled` | 취소됨 |

---

## 오류 처리

DevERP는 명확한 오류 메시지를 제공합니다.

### 일반적인 오류

#### Validation Error

입력 데이터가 유효하지 않을 때 발생합니다.

```
Error: Validation error: Project name cannot be empty
```

**해결 방법**: 오류 메시지에 표시된 제약 조건을 확인하고 올바른 값을 입력하세요.

---

#### Not Found Error

요청한 리소스를 찾을 수 없을 때 발생합니다.

```
Error: Not found: Project with id 999 not found
```

**해결 방법**: 올바른 ID 또는 UUID를 사용하고 있는지 확인하세요.

---

#### Conflict Error

중복된 값이 입력되었을 때 발생합니다.

```
Error: Conflict: Project code 'PROJ-001' already exists
```

**해결 방법**: 고유한 값을 사용하거나 기존 리소스를 수정하세요.

---

#### Database Error

데이터베이스 연결 또는 쿼리 오류가 발생했을 때 표시됩니다.

```
Error: Database error: connection refused
```

**해결 방법**:
1. PostgreSQL 서버가 실행 중인지 확인
2. DATABASE_URL 환경 변수가 올바른지 확인
3. `deverp config test-db`로 연결 테스트

---

## 사용 예제

### 시나리오 1: 새 프로젝트 시작

```bash
# 1. 프로젝트 생성
deverp project create \
  --name "Mobile Banking App" \
  --code "MOBILE-BANK-001" \
  --status planning \
  --priority high \
  --start-date 2025-02-01 \
  --end-date 2025-08-31 \
  --repository-url "https://github.com/company/mobile-bank" \
  --tags "mobile,fintech,ios,android"

# 출력: ID: 1, UUID: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx

# 2. 핵심 리소스 등록
deverp resource create \
  --name "React Native" \
  --resource-type library \
  --version "0.73.0" \
  --url "https://reactnative.dev"

# 출력: ID: 10

# 3. 리소스를 프로젝트에 연결
deverp resource link \
  --project-id 1 \
  --resource-id 10 \
  --is-critical \
  --version-used "0.73.0"

# 4. 타임라인 생성
deverp timeline create \
  --project-id 1 \
  --name "Development Phase" \
  --timeline-type phase \
  --start-date 2025-02-01 \
  --end-date 2025-07-31 \
  --status active

# 출력: ID: 1

# 5. 마일스톤 추가
deverp timeline add-milestone \
  --timeline-id 1 \
  --project-id 1 \
  --name "MVP Release" \
  --target-date 2025-05-01

# 6. 초기 작업 생성
deverp task create \
  --project-id 1 \
  --title "프로젝트 초기 설정" \
  --task-type chore \
  --priority high \
  --status in_progress
```

---

### 시나리오 2: 일일 작업 관리

```bash
# 1. 오늘 할 작업 확인
deverp task list \
  --status todo \
  --assigned-to "me@company.com" \
  --project-id 1

# 2. 작업 시작
deverp task update 15 \
  --status in_progress

# 3. 작업 중 댓글 추가
deverp task add-comment \
  --task-id 15 \
  --comment "API 엔드포인트 구현 중, 인증 로직 검토 필요"

# 4. 작업 완료
deverp task update 15 \
  --status done \
  --actual-hours 4.5

# 5. 다음 작업 확인
deverp task list \
  --status todo \
  --priority high \
  --project-id 1
```

---

### 시나리오 3: 주간 리포트 생성

```bash
# 1. 프로젝트 전체 상황
deverp report status

# 2. 프로젝트별 요약
deverp --format json report project-summary > weekly-summary.json

# 3. 작업 분석
deverp report task-analytics

# 4. 타임라인 진행률
deverp report timeline-progress

# 5. 특정 프로젝트의 작업 현황
deverp task list --project-id 1 --status in_progress
```

---

### 시나리오 4: 스프린트 관리

```bash
# 1. 스프린트 타임라인 생성
deverp timeline create \
  --project-id 1 \
  --name "Sprint 3" \
  --timeline-type sprint \
  --start-date 2025-02-01 \
  --end-date 2025-02-14 \
  --status active

# 출력: ID: 5

# 2. 스프린트 목표 마일스톤
deverp timeline add-milestone \
  --timeline-id 5 \
  --project-id 1 \
  --name "Complete User Authentication" \
  --target-date 2025-02-14

# 3. 스프린트 작업 생성
deverp task create \
  --project-id 1 \
  --title "로그인 UI 구현" \
  --status todo \
  --priority high \
  --task-type feature \
  --estimated-hours 8

deverp task create \
  --project-id 1 \
  --title "JWT 토큰 관리" \
  --status todo \
  --priority high \
  --task-type feature \
  --estimated-hours 6

# 4. 작업 의존성 설정
deverp task add-dependency \
  --task-id 25 \
  --depends-on-task-id 24

# 5. 스프린트 종료 시
deverp timeline update 5 --status completed
deverp timeline complete-milestone 10 --actual-date 2025-02-14
```

---

### 시나리오 5: 리소스 감사

```bash
# 1. 모든 리소스 조회
deverp resource list

# 2. 폐기된 리소스 확인
deverp resource list --status deprecated

# 3. 특정 유형의 활성 리소스
deverp resource list \
  --resource-type library \
  --status active

# 4. 리소스 사용 현황
deverp report resource-usage

# 5. 특정 리소스 상세 정보
deverp resource show 10

# 6. 미사용 리소스 확인 (JSON 파싱 필요)
deverp --format json report resource-usage | \
  jq '.unused_resources'
```

---

## 고급 사용법

### JSON 출력 활용

JSON 형식으로 출력하여 다른 도구와 연동할 수 있습니다.

```bash
# jq와 함께 사용
deverp --format json project list --status active | \
  jq '.[] | {id, name, progress_percentage}'

# 파일로 저장
deverp --format json report task-analytics > task-report.json

# Python 스크립트와 연동
deverp --format json project list | python process_projects.py
```

---

### 배치 작업

쉘 스크립트로 여러 작업을 자동화할 수 있습니다.

```bash
#!/bin/bash

# 여러 프로젝트 일괄 생성
for project in "Project A" "Project B" "Project C"; do
  deverp project create \
    --name "$project" \
    --status planning \
    --priority medium
done

# 모든 완료된 프로젝트 아카이브
deverp --format json project list --status completed | \
  jq -r '.[].id' | \
  while read id; do
    deverp project archive "$id"
  done
```

---

### 환경 변수 설정

데이터베이스 연결 정보를 환경 변수로 관리합니다.

```bash
# .env 파일
DATABASE_URL=postgres://user:password@localhost/deverp
RUST_LOG=deverp=debug

# 사용
source .env
deverp project list
```

---

## 문제 해결

### 데이터베이스 연결 실패

```bash
# 연결 테스트
deverp config test-db

# PostgreSQL 상태 확인 (시스템에 따라 다름)
sudo systemctl status postgresql  # Linux
brew services list | grep postgresql  # macOS
```

---

### 성능 최적화

```bash
# 페이지 크기 조정
deverp project list --per-page 100

# 필요한 필드만 조회 (JSON + jq)
deverp --format json project list | \
  jq '[.[] | {id, name, status}]'
```

---

### 로그 확인

```bash
# 상세 로그 활성화
RUST_LOG=debug deverp project create --name "Test"

# 특정 모듈만 로그
RUST_LOG=deverp::domain::project=trace deverp project list
```

---

## 추가 자료

- [Architecture Guide](./architecture.md) - 시스템 아키텍처 설명
- [Database Schema](./database.md) - 데이터베이스 스키마 상세
- [Implementation Plan](./implementation-plan.md) - 구현 계획 및 로드맵

---

**문서 버전**: 1.0.0
**마지막 업데이트**: 2025-10-23
**작성자**: DevERP Team
