# Velocity Download Manager (VDM)

# API Specification

**Document ID:** VDM-API-001
**Version:** 1.0.0
**Status:** Draft
**Confidentiality:** Internal Use Only

---

# 1. Cover

| Item          | Information                     |
| ------------- | ------------------------------- |
| Project       | Velocity Download Manager (VDM) |
| Document      | API Specification               |
| Document ID   | VDM-API-001                     |
| Version       | 1.0.0                           |
| API Type      | Internal IPC API                |
| Communication | Tauri IPC + Event Bus           |
| Data Format   | JSON                            |
| Author        | Mustofa & ChatGPT               |
| Last Updated  | July 2026                       |

---

# 2. Document Information

## Purpose

Dokumen ini mendefinisikan seluruh komunikasi antar modul VDM, termasuk IPC antara Frontend dan Backend, Event Bus internal, Browser Extension Protocol, serta kontrak data yang digunakan aplikasi.

## Scope

* IPC API
* Internal Services
* Event Bus
* Browser Protocol
* JSON Schema
* Error Response
* Security Rules
* API Versioning

## Target Audience

* Rust Developer
* Frontend Developer
* Extension Developer
* QA Engineer
* Software Architect

## Related Documents

* 01-PRD.md
* 02-SDS.md
* 03-Database.md
* 05-Development-Guide.md

---

# 3. Revision History

| Version | Date      | Author            | Description               |
| ------- | --------- | ----------------- | ------------------------- |
| 0.1.0   | July 2026 | Mustofa & ChatGPT | Initial API Specification |
| 1.0.0   | TBD       | Development Team  | Production Release        |

---

# 4. Table of Contents

## PART I — Overview

5. Introduction
6. API Architecture
7. Design Principles
8. API Versioning

## PART II — IPC API

9. IPC Overview
10. Download API
11. Queue API
12. Scheduler API
13. File API
14. Settings API
15. Statistics API
16. Notification API

## PART III — Internal Communication

17. Event Bus
18. Message Queue
19. Event Types
20. Error Response

## PART IV — Browser Extension

21. Native Messaging
22. Browser Protocol
23. URL Capture API

## PART V — Security

24. Authentication
25. Validation
26. Privacy

## PART VI — Testing

27. API Testing
28. Performance Testing
29. Acceptance Criteria
30. Appendix

# PART I — Overview

---

# 5. Introduction

## Overview

API Specification mendefinisikan seluruh komunikasi internal pada Velocity Download Manager (VDM). Dokumen ini menjadi acuan bagi Frontend (React), Backend (Rust), Browser Extension, dan seluruh modul aplikasi agar berkomunikasi menggunakan kontrak data yang konsisten.

VDM menggunakan **Tauri IPC**, **Event Bus**, dan **Native Messaging** sebagai mekanisme komunikasi utama.

---

# 6. API Architecture

## Architecture

```text
React UI
    │
    ▼
Tauri IPC
    │
    ▼
Application Service
    │
    ▼
Event Bus
    │
    ▼
Rust Modules
    │
    ▼
Download Engine
```

### Components

* Frontend (React)
* Tauri IPC
* Service Layer
* Event Bus
* Rust Backend
* Browser Extension

Seluruh komunikasi menggunakan JSON yang tervalidasi.

---

# 7. Design Principles

## API Principles

* Simple
* Consistent
* Type Safe
* Asynchronous
* Secure
* Versioned
* Modular

### Rules

* Semua request memiliki response.
* Seluruh payload menggunakan JSON.
* Error memiliki format yang konsisten.
* Input divalidasi sebelum diproses.
* Komunikasi tidak boleh memblokir UI.

---

# 8. API Versioning

## Version Format

Menggunakan Semantic Versioning.

Contoh:

* v1
* v1.1
* v2

### Compatibility

* Backward Compatible bila memungkinkan.
* Breaking Change hanya pada Major Version.

### Deprecation Policy

API lama tetap didukung selama masa transisi sebelum dihapus pada rilis mayor berikutnya.

---

# Communication Flow

```text
User Action
      │
      ▼
React UI
      │
      ▼
IPC Request
      │
      ▼
Rust Service
      │
      ▼
Business Logic
      │
      ▼
IPC Response
      │
      ▼
UI Update
```

---

# General Response Format

Seluruh respons menggunakan struktur yang konsisten.

### Success

* success
* data
* message
* timestamp

### Error

* success
* error_code
* message
* details
* timestamp

---

# Naming Convention

### Commands

Menggunakan PascalCase.

Contoh:

* StartDownload
* PauseDownload
* ResumeDownload
* DeleteDownload

### Events

Menggunakan bentuk lampau (Past Tense).

Contoh:

* DownloadStarted
* DownloadPaused
* DownloadCompleted
* QueueUpdated
* SettingsChanged

### JSON Fields

Menggunakan `camelCase`.

Contoh:

* downloadId
* fileName
* totalSize
* downloadSpeed
* createdAt

---

# Acceptance Criteria

* Arsitektur komunikasi terdokumentasi dengan jelas.
* Seluruh API mengikuti aturan penamaan yang konsisten.
* Request dan Response menggunakan format JSON yang seragam.
* API mendukung pengembangan modular dan mudah diperluas.
* Versioning memungkinkan evolusi API tanpa mengganggu kompatibilitas yang diperlukan.

# PART II — IPC API

---

# 9. IPC Overview

## Overview

IPC (Inter-Process Communication) digunakan sebagai media komunikasi antara Frontend (React) dan Backend (Rust) melalui Tauri.

Semua komunikasi bersifat **asynchronous**, **type-safe**, dan menggunakan **JSON**.

---

## Communication Flow

```text
React UI
    │
IPC Request
    │
    ▼
Rust Backend
    │
Business Logic
    │
    ▼
IPC Response
    │
    ▼
React UI
```

---

## IPC Rules

* Request harus tervalidasi.
* Response selalu dikembalikan.
* Error menggunakan format standar.
* Tidak ada proses blocking pada UI.

---

# 10. Download API

## Purpose

Mengelola seluruh operasi download.

### Commands

* StartDownload
* PauseDownload
* ResumeDownload
* CancelDownload
* RestartDownload
* DeleteDownload
* VerifyDownload

### Events

* DownloadStarted
* DownloadProgress
* DownloadPaused
* DownloadCompleted
* DownloadFailed

---

# 11. Queue API

## Purpose

Mengelola antrean download.

### Commands

* AddToQueue
* RemoveFromQueue
* ReorderQueue
* ClearQueue
* StartQueue
* PauseQueue

### Events

* QueueUpdated
* QueueStarted
* QueueCompleted

---

# 12. Scheduler API

## Purpose

Mengelola tugas otomatis.

### Commands

* CreateSchedule
* UpdateSchedule
* DeleteSchedule
* EnableSchedule
* DisableSchedule

### Events

* ScheduleCreated
* ScheduleExecuted
* ScheduleCompleted

---

# 13. File API

## Purpose

Mengelola file hasil download.

### Commands

* OpenFile
* OpenFolder
* RenameFile
* MoveFile
* DeleteFile
* ExportFile

### Events

* FileOpened
* FileMoved
* FileDeleted

---

# 14. Settings API

## Purpose

Mengelola konfigurasi aplikasi.

### Commands

* GetSettings
* SaveSettings
* ResetSettings
* ImportSettings
* ExportSettings

### Events

* SettingsLoaded
* SettingsSaved
* SettingsReset

---

# 15. Statistics API

## Purpose

Mengambil statistik penggunaan aplikasi.

### Commands

* GetStatistics
* ResetStatistics

### Events

* StatisticsUpdated

---

# 16. Notification API

## Purpose

Mengelola notifikasi aplikasi.

### Commands

* GetNotifications
* MarkAsRead
* ClearNotifications

### Events

* NotificationCreated
* NotificationRead
* NotificationsCleared

---

# Standard Request Structure

Setiap request minimal berisi:

* command
* requestId
* payload
* timestamp

---

# Standard Response Structure

### Success

* success
* requestId
* data
* message
* timestamp

### Error

* success
* requestId
* errorCode
* message
* details
* timestamp

---

# IPC Lifecycle

```text
User Action
      │
      ▼
IPC Command
      │
      ▼
Validation
      │
      ▼
Rust Service
      │
      ▼
Business Logic
      │
      ▼
Response
      │
      ▼
Event (jika diperlukan)
```

---

# Acceptance Criteria

* Seluruh komunikasi Frontend–Backend menggunakan IPC.
* Semua command memiliki response yang konsisten.
* Event digunakan untuk perubahan status yang berlangsung setelah command dijalankan.
* Request dan response tervalidasi sebelum diproses.
* UI tetap responsif selama komunikasi IPC berlangsung.
# PART III — Internal Communication

---

# 17. Event Bus

## Overview

Event Bus merupakan sistem komunikasi internal antar modul Backend Rust. Event Bus memungkinkan setiap modul saling bertukar informasi tanpa saling bergantung secara langsung (Loose Coupling).

---

## Event Categories

* Download
* Queue
* Scheduler
* File
* Settings
* Notification
* Statistics
* Logging
* Security
* Recovery
* System

---

## Common Events

* DownloadCreated
* DownloadStarted
* DownloadProgress
* DownloadPaused
* DownloadCompleted
* DownloadFailed
* QueueUpdated
* SchedulerExecuted
* SettingsChanged
* NotificationCreated
* RecoveryCompleted

Event bersifat asynchronous dan dapat memiliki banyak subscriber.

---

# 18. Message Queue

## Overview

Message Queue mengatur pemrosesan pesan internal secara berurutan dan aman.

---

## Responsibilities

* Queue Message
* Dispatch Message
* Prioritize Message
* Retry Failed Message
* Dead Letter Queue (Future)

---

## Priority

* Critical
* High
* Normal
* Low

Pesan diproses berdasarkan prioritas dan waktu masuk.

---

# 19. Event Types

## Command

Digunakan untuk meminta aksi.

Contoh:

* StartDownload
* PauseDownload
* ResumeDownload
* DeleteDownload

---

## Event

Digunakan untuk memberi tahu perubahan status.

Contoh:

* DownloadStarted
* DownloadProgress
* DownloadCompleted
* QueueUpdated

---

## Notification

Digunakan untuk menampilkan informasi kepada pengguna.

Contoh:

* Download Finished
* Download Failed
* Scheduler Started

---

## System Event

Digunakan untuk komunikasi internal.

Contoh:

* StartupCompleted
* ShutdownRequested
* ConfigurationReloaded
* HealthStatusChanged

---

# 20. Error Response

## Overview

Seluruh error menggunakan format yang konsisten.

---

## Error Categories

* Validation Error
* Network Error
* Database Error
* File Error
* Permission Error
* Download Error
* IPC Error
* Internal Error

---

## Error Information

Setiap error minimal berisi:

* Error Code
* Message
* Module
* Severity
* Timestamp

---

## Severity Levels

* Info
* Warning
* Error
* Critical

---

## Error Handling Flow

```text id="djlwm3"
Module
   │
   ▼
Error Generated
   │
   ▼
Logging System
   │
   ▼
Event Bus
   │
   ▼
Notification Service
   │
   ▼
Frontend
```

---

# Internal Communication Flow

```text id="2j2rj4"
Frontend
     │
     ▼
IPC Command
     │
     ▼
Service Layer
     │
     ▼
Event Bus
     │
 ┌───┼─────────────┐
 ▼   ▼             ▼
Queue Download Scheduler
 │      │            │
 └──────┼────────────┘
        ▼
Logging
        │
        ▼
IPC Event
        │
        ▼
Frontend Update
```

---

# Communication Rules

* Modul tidak boleh saling memanggil secara langsung jika tersedia Event Bus.
* Semua event harus memiliki nama yang konsisten.
* Event harus bersifat immutable.
* Error harus diteruskan ke Logging System.
* Komunikasi bersifat asynchronous jika memungkinkan.
* Semua pesan divalidasi sebelum diproses.

---

# Acceptance Criteria

* Event Bus mendistribusikan event ke seluruh subscriber yang sesuai.
* Message Queue memproses pesan berdasarkan prioritas.
* Error diteruskan secara konsisten ke Logging System dan Frontend.
* Komunikasi antar modul tetap modular tanpa ketergantungan langsung.
* Event dan Command memiliki format yang terdokumentasi dan konsisten.
# PART IV — Browser Extension API

---

# 21. Native Messaging

## Overview

Native Messaging digunakan sebagai jalur komunikasi antara Browser Extension dan aplikasi VDM Desktop.

Komunikasi bersifat:

* Asynchronous
* Secure
* JSON Based
* Type Safe

---

## Responsibilities

* Mengirim URL download.
* Mengirim metadata file.
* Mengirim cookie (jika diperlukan).
* Mengirim informasi browser.
* Menerima status download.

---

## Communication Flow

```text id="dr1v9q"
Browser
    │
Extension
    │
Native Messaging
    │
VDM Desktop
    │
Rust Backend
```

---

# 22. Browser Protocol

## Supported Browsers

* Google Chrome
* Microsoft Edge
* Mozilla Firefox
* Brave
* Opera
* Vivaldi

---

## Browser Commands

* CaptureDownload
* CaptureVideo
* CaptureAudio
* OpenVDM
* CheckConnection
* GetVersion

---

## Browser Events

* DownloadCaptured
* VideoDetected
* BrowserConnected
* BrowserDisconnected

---

## Browser Information

Data yang dapat dikirim:

* Browser Name
* Browser Version
* Extension Version
* Platform
* Language

---

# 23. URL Capture API

## Purpose

Mengambil informasi download dari browser sebelum diteruskan ke Download Engine.

---

## Captured Data

* URL
* File Name
* MIME Type
* File Size
* Referer
* User-Agent
* Cookie (jika diperlukan)
* Timestamp

---

## Validation

Sebelum dikirim ke backend:

* URL Validation
* Protocol Validation
* MIME Validation
* Duplicate Detection
* File Name Validation

---

## Download Flow

```text id="mnd3py"
User Click Download
        │
        ▼
Browser Extension
        │
Capture URL
        │
        ▼
Validate Data
        │
        ▼
Native Messaging
        │
        ▼
VDM Desktop
        │
        ▼
Download Intelligence Engine
        │
        ▼
Queue Manager
        │
        ▼
Download Engine
```

---

# Communication Rules

* Semua pesan menggunakan JSON.
* Seluruh data harus tervalidasi.
* Extension tidak menyimpan data sensitif secara permanen.
* Cookie hanya digunakan untuk proses download yang sedang berlangsung.
* Browser harus menerima konfirmasi bahwa permintaan telah diterima.

---

# Error Handling

Jika terjadi kesalahan:

* Native Messaging gagal.
* Browser tidak didukung.
* Extension tidak aktif.
* URL tidak valid.
* VDM tidak berjalan.

Extension harus:

* Menampilkan pesan yang jelas.
* Menawarkan membuka VDM jika diperlukan.
* Mengirim log kesalahan ke aplikasi bila memungkinkan.

---

# Acceptance Criteria

* Extension berhasil menangkap download dari browser yang didukung.
* Native Messaging bekerja stabil.
* URL diteruskan ke VDM dalam waktu <100 ms.
* Validasi dilakukan sebelum data diproses.
* Browser menerima status keberhasilan atau kegagalan permintaan.
* Komunikasi tetap aman dan tidak menyimpan data sensitif tanpa persetujuan pengguna.
# PART V — Security

---

# 24. Authentication

## Overview

Karena VDM adalah aplikasi desktop lokal, API tidak menggunakan sistem login pengguna. Namun, setiap komunikasi internal tetap harus divalidasi untuk memastikan hanya komponen resmi yang dapat berinteraksi.

---

## Trusted Components

* React Frontend
* Tauri IPC
* Rust Backend
* Browser Extension
* Native Messaging

---

## Authentication Rules

* Native Messaging hanya menerima Extension resmi.
* IPC hanya dapat dipanggil oleh Frontend VDM.
* Semua request memiliki `requestId`.
* Setiap command harus tervalidasi sebelum diproses.

---

# 25. Validation

## Overview

Semua data yang masuk wajib divalidasi sebelum diteruskan ke Business Logic.

---

## Validation Rules

### URL

* Format valid
* Protocol didukung
* Tidak kosong

---

### File

* Nama file valid
* Path valid
* Karakter ilegal ditolak

---

### JSON

* Struktur valid
* Field wajib tersedia
* Tipe data sesuai spesifikasi

---

### IPC

* Command dikenal
* Payload sesuai schema
* RequestId unik

---

### Browser Extension

* Extension ID valid
* Browser didukung
* Native Messaging aktif

---

# 26. Privacy

## Privacy Principles

* Local First
* Minimal Data Collection
* User Control
* Secure Storage

---

## Privacy Rules

VDM tidak:

* Mengirim riwayat download ke server.
* Mengirim URL pengguna ke pihak ketiga.
* Mengumpulkan telemetry secara default.
* Menyimpan cookie secara permanen tanpa persetujuan pengguna.

---

## Sensitive Data

Data berikut diperlakukan sebagai sensitif:

* Cookie
* Access Token
* Authorization Header
* Session Data

Data hanya digunakan selama proses download dan dihapus setelah tidak diperlukan.

---

# Secure Communication

Seluruh komunikasi menggunakan:

* Tauri IPC
* Native Messaging
* Event Bus

Setiap pesan harus:

* Tervalidasi.
* Memiliki format JSON yang benar.
* Menggunakan struktur data yang telah ditentukan.

---

# Error Security

Jika terjadi pelanggaran validasi:

* Request ditolak.
* Error dicatat ke Logging System.
* Pengguna menerima pesan yang jelas.
* Tidak ada data sensitif yang ditampilkan.

---

# Security Checklist

* Input tervalidasi.
* IPC tervalidasi.
* Native Messaging tervalidasi.
* JSON Schema tervalidasi.
* Data sensitif terlindungi.
* Logging tidak menyimpan informasi sensitif.
* Privacy Policy diterapkan.

---

# Security Flow

```text id="8bw3oi"
Request
    │
    ▼
Validation
    │
    ▼
Authentication
    │
    ▼
Authorization
    │
    ▼
Business Logic
    │
    ▼
Response
```

---

# Acceptance Criteria

* Seluruh request tervalidasi sebelum diproses.
* Komponen tidak sah tidak dapat berkomunikasi dengan backend.
* Data sensitif tidak disimpan atau ditampilkan tanpa izin.
* Komunikasi IPC dan Native Messaging aman serta terdokumentasi.
* Seluruh pelanggaran keamanan tercatat pada Logging System.
# PART VI — Testing

---

# 27. API Testing

## Overview

API Testing memastikan seluruh komunikasi internal VDM berjalan dengan benar, aman, stabil, dan sesuai spesifikasi.

---

## Test Categories

* IPC Testing
* Command Testing
* Event Testing
* Browser Extension Testing
* Native Messaging Testing
* Validation Testing
* Error Handling Testing
* Security Testing

---

## IPC Testing

Menguji:

* Request berhasil dikirim.
* Response diterima.
* Format JSON valid.
* Request ID sesuai.
* Timeout ditangani dengan benar.

---

## Command Testing

Setiap command harus diuji.

Contoh:

* StartDownload
* PauseDownload
* ResumeDownload
* CancelDownload
* DeleteDownload
* GetSettings
* GetStatistics

Semua command wajib menghasilkan response yang sesuai.

---

## Event Testing

Menguji event:

* DownloadStarted
* DownloadProgress
* DownloadPaused
* DownloadCompleted
* DownloadFailed
* QueueUpdated
* SettingsChanged
* NotificationCreated

Event harus diterima oleh seluruh subscriber yang relevan.

---

## Browser Extension Testing

Pengujian meliputi:

* Capture URL
* Native Messaging
* Browser Connection
* Video Detection
* Error Handling

Browser yang diuji:

* Chrome
* Edge
* Firefox
* Brave
* Opera
* Vivaldi

---

# 28. Performance Testing

## Performance Targets

| Operation         |   Target |
| ----------------- | -------: |
| IPC Request       |  < 20 ms |
| IPC Response      |  < 20 ms |
| Event Dispatch    |  < 10 ms |
| URL Capture       | < 100 ms |
| Native Messaging  | < 100 ms |
| Command Execution | < 100 ms |

---

## Stress Testing

Skenario:

* 100 Download Aktif
* 10.000 Event
* 1.000 IPC Request
* Queue Besar
* Notifikasi Massal

API harus tetap stabil dan responsif.

---

## Reliability Testing

Pengujian:

* Restart aplikasi saat request berjalan.
* Browser Extension terputus.
* Native Messaging gagal.
* Timeout jaringan.
* Event Queue penuh.

Sistem harus melakukan recovery tanpa menyebabkan kerusakan data.

---

# 29. Acceptance Criteria

API dianggap siap digunakan apabila:

* Seluruh IPC Command berhasil dijalankan.
* Response sesuai spesifikasi.
* Event diterima dengan benar.
* Browser Extension dapat berkomunikasi dengan aplikasi.
* Native Messaging stabil.
* Validasi berhasil menolak input yang tidak valid.
* Target performa tercapai.
* Tidak ditemukan kebocoran data sensitif.

---

# 30. Appendix

## Communication Technologies

* Tauri IPC
* Native Messaging
* Event Bus
* JSON

---

## Supported Browsers

* Google Chrome
* Microsoft Edge
* Mozilla Firefox
* Brave
* Opera
* Vivaldi

---

## Related Documents

* 01-PRD.md
* 02-SDS.md
* 03-Database.md
* 05-Development-Guide.md

---

# API Lifecycle

```text id="j3q5nh"
Frontend
    │
IPC Command
    │
Validation
    │
Business Logic
    │
Response
    │
Event
    │
Frontend Update
```

---

# End of API Specification

Dokumen ini menjadi acuan resmi komunikasi antara Frontend, Backend, Browser Extension, dan seluruh modul internal Velocity Download Manager (VDM). Seluruh perubahan API harus terdokumentasi dan mengikuti proses versioning yang telah ditetapkan.
