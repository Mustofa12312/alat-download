# Velocity Download Manager (VDM)

# Software Design Specification (SDS)

**Document ID:** VDM-SDS-001
**Version:** 1.0.0
**Status:** Draft
**Confidentiality:** Internal Use Only

---

# 1. Cover

| Item            | Information                           |
| --------------- | ------------------------------------- |
| Project         | Velocity Download Manager (VDM)       |
| Document        | Software Design Specification (SDS)   |
| Document ID     | VDM-SDS-001                           |
| Version         | 1.0.0                                 |
| Status          | Draft                                 |
| Architecture    | Clean Architecture + Modular Monolith |
| Frontend        | Tauri + React + TypeScript            |
| Backend         | Rust                                  |
| Database        | SQLite                                |
| Target Platform | Windows 10 & Windows 11               |
| License         | Free                                  |
| Author          | Mustofa & ChatGPT                     |
| Last Updated    | July 2026                             |

---

# 2. Document Information

## Purpose

Dokumen ini menjelaskan desain teknis dan implementasi perangkat lunak Velocity Download Manager (VDM). SDS menjadi acuan utama bagi Software Architect, Backend Developer, Frontend Developer, QA Engineer, dan DevOps selama proses pengembangan.

## Scope

Dokumen ini mencakup:

- Arsitektur sistem
- Struktur proyek
- Desain modul
- Komunikasi antar modul
- Desain database
- API Internal
- Browser Integration
- Download Engine
- Security
- Performance
- Testing
- Deployment

## Intended Audience

- Software Architect
- Rust Developer
- Frontend Developer
- UI/UX Developer
- QA Engineer
- DevOps Engineer
- Project Manager

## Related Documents

- `01-PRD.md` — Product Requirements Document
- `03-Database.md` — Database Design
- `04-API.md` — API Specification
- `05-TestPlan.md` — Testing Plan

---

# 3. Revision History

| Version | Date      | Author            | Description                 |
| ------- | --------- | ----------------- | --------------------------- |
| 0.1.0   | July 2026 | Mustofa & ChatGPT | Initial SDS Structure       |
| 0.5.0   | TBD       | Development Team  | Core Architecture Completed |
| 0.9.0   | TBD       | Development Team  | Feature Complete            |
| 1.0.0   | TBD       | Development Team  | Production Release          |

---

# 4. Table of Contents

## PART I — System Overview

1. Cover
2. Document Information
3. Revision History
4. Table of Contents
5. Introduction
6. Scope
7. Architecture Goals
8. Design Principles
9. Technology Stack
10. High-Level Architecture
11. System Components

## PART II — Frontend

12. Tauri Architecture
13. UI Layer
14. State Management
15. IPC Communication
16. Theme System
17. Window Management

## PART III — Backend

18. Rust Workspace
19. Crate Structure
20. Download Engine
21. Segment Manager
22. Connection Manager
23. Buffer Manager
24. Resume Engine
25. Retry Engine
26. Queue Manager
27. Scheduler
28. File Manager
29. Download Intelligence Engine
30. Intelligent Download Optimizer
31. Self-Healing Engine
32. System Health Monitor

## PART IV — Database

33. SQLite Schema
34. Database Tables
35. Index Strategy
36. Migration
37. Backup & Recovery

## PART V — Browser Extension

38. Extension Architecture
39. Native Messaging
40. Message Protocol
41. Browser Security

## PART VI — External Components

42. yt-dlp Integration
43. FFmpeg Integration
44. Torrent Engine

## PART VII — Internal Communication

45. Internal API
46. IPC API
47. Event Bus
48. Message Queue

## PART VIII — Security

49. Authentication
50. Secure Storage
51. Certificate Validation
52. Privacy & Data Protection

## PART IX — Performance

53. Thread Model
54. Async Runtime
55. Memory Management
56. CPU Optimization
57. Disk Optimization
58. Network Optimization

## PART X — Testing & Deployment

59. Testing Strategy
60. Build System
61. Installer
62. Auto Update
63. Release Pipeline

## PART XI — Appendix

64. Coding Standards
65. Project Folder Structure
66. Dependencies
67. Future Architecture
68. Glossary
# PART I — System Overview

# 5. Introduction

## 5.1 Purpose

Software Design Specification (SDS) mendefinisikan desain teknis Velocity Download Manager (VDM). Dokumen ini menjadi referensi utama bagi seluruh tim pengembang selama proses implementasi.

SDS bertujuan untuk:

* Menjelaskan arsitektur sistem.
* Menentukan struktur proyek.
* Mendefinisikan modul aplikasi.
* Menentukan komunikasi antar komponen.
* Menjadi acuan implementasi dan pengujian.

---

## 5.2 Design Philosophy

VDM dibangun berdasarkan prinsip:

* Performance First
* Clean Architecture
* Modular Monolith
* Event Driven
* Asynchronous Processing
* Low Memory Usage
* High Reliability
* Easy Maintenance

---

## 5.3 Design Objectives

Target utama desain:

* Startup sangat cepat.
* Download Engine independen dari UI.
* Mudah diuji (testable).
* Mudah dikembangkan.
* Aman terhadap crash.
* Modular.
* Cross-platform ready.

---

# 6. Scope

SDS mencakup seluruh desain teknis berikut:

## Frontend

* Tauri
* React
* TypeScript
* UI Components
* State Management
* IPC

---

## Backend

* Rust Workspace
* Download Engine
* Queue Manager
* Scheduler
* File Manager
* Intelligence Layer
* Self-Healing Engine

---

## Database

* SQLite
* Migration
* Backup
* Recovery

---

## Browser Integration

* Browser Extension
* Native Messaging
* URL Capture
* Media Detection

---

## External Components

* yt-dlp
* FFmpeg
* Windows API

---

# 7. Architecture Goals

Arsitektur harus memenuhi tujuan berikut:

## High Performance

Semua proses berat berjalan di Rust.

---

## Scalability

Mudah menambah:

* Browser baru
* Protocol baru
* Downloader baru
* Plugin
* Cloud Provider

---

## Reliability

* Resume Download
* Crash Recovery
* Self-Healing
* Automatic Retry

---

## Maintainability

Kode dipisahkan menjadi modul kecil dengan tanggung jawab yang jelas.

---

## Security

Semua komunikasi tervalidasi.

Tidak ada data sensitif yang disimpan tanpa perlindungan.

---

# 8. Design Principles

## Single Responsibility Principle

Setiap modul hanya memiliki satu tanggung jawab utama.

---

## Separation of Concerns

UI tidak mengetahui implementasi Download Engine.

Download Engine tidak mengetahui implementasi UI.

---

## Dependency Inversion

Seluruh komunikasi menggunakan interface atau service abstraction.

---

## Event Driven

Komunikasi internal menggunakan Event Bus.

Contoh event:

* DownloadStarted
* DownloadCompleted
* DownloadFailed
* QueueUpdated
* SettingsChanged

---

## Async First

Operasi berikut harus asynchronous:

* HTTP Request
* Disk I/O
* Database
* File Verification
* Video Analysis

---

# 9. Technology Stack

## Frontend

| Technology   | Purpose           |
| ------------ | ----------------- |
| Tauri        | Desktop Framework |
| React        | UI                |
| TypeScript   | Frontend Logic    |
| Tailwind CSS | Styling           |

---

## Backend

| Technology      | Purpose       |
| --------------- | ------------- |
| Rust            | Core Engine   |
| Tokio           | Async Runtime |
| Reqwest         | HTTP Client   |
| Serde           | Serialization |
| SQLx / rusqlite | SQLite Access |

---

## Database

SQLite

---

## External Tools

| Component | Purpose          |
| --------- | ---------------- |
| yt-dlp    | Video Download   |
| FFmpeg    | Media Processing |

---

## Development

* Cargo
* Node.js
* pnpm
* Git
* GitHub Actions

---

# 10. High-Level Architecture

```text
                 Browser Extension
                        │
                        ▼
             Native Messaging Service
                        │
                        ▼
               Tauri Desktop Application
                        │
        ┌───────────────┴───────────────┐
        ▼                               ▼
  React UI                      IPC Communication
        │                               │
        └───────────────┬───────────────┘
                        ▼
              Application Service Layer
                        │
        ┌───────────────┼────────────────┐
        ▼               ▼                ▼
 Queue Manager   Scheduler Service   Settings Service
        │               │                │
        └───────────────┼────────────────┘
                        ▼
              Core Intelligence Layer
                        │
   ┌────────────┬─────────────┬──────────────┐
   ▼            ▼             ▼              ▼
 Download     Intelligent   Self-Healing   Health
 Intelligence Optimizer      Engine        Monitor
 Engine
                        │
                        ▼
                Rust Download Engine
                        │
      ┌──────────┬──────────┬──────────┐
      ▼          ▼          ▼          ▼
 Connection   Segment   Resume    Disk Writer
 Manager      Manager   Engine
                        │
                        ▼
                     SQLite
```

---

# 11. System Components

## UI Layer

Tanggung jawab:

* Dashboard
* Downloads
* Video
* Torrent
* Settings
* Notification Center

Tidak melakukan proses download.

---

## IPC Layer

Menghubungkan Tauri dengan Rust.

Fungsi:

* Request
* Response
* Event

---

## Service Layer

Mengelola logika aplikasi.

Meliputi:

* Queue
* Scheduler
* History
* Settings
* Notification
* Statistics

---

## Intelligence Layer

Mengambil keputusan otomatis.

Komponen:

* Download Intelligence Engine
* Intelligent Download Optimizer
* Self-Healing Engine
* System Health Monitor

---

## Download Engine

Komponen inti.

Sub-modul:

* URL Analyzer
* HTTP Client
* Connection Manager
* Segment Manager
* Worker Manager
* Resume Engine
* Retry Engine
* Buffer Manager
* Disk Writer
* Integrity Checker

---

## Storage Layer

Mengelola:

* SQLite
* Configuration
* History
* Queue
* Statistics
* Logs

---

## External Layer

Integrasi dengan:

* Browser Extension
* yt-dlp
* FFmpeg
* Windows API

---

# End of PART I

PART II akan membahas implementasi Frontend menggunakan Tauri dan React, termasuk struktur proyek, state management, IPC, sistem tema, dan pengelolaan jendela aplikasi.
# PART II — Frontend

---

# 12. Tauri Architecture

## Overview

Frontend dibangun menggunakan **Tauri + React + TypeScript**. Tauri bertugas sebagai desktop runtime, React menangani antarmuka pengguna, sedangkan seluruh proses berat dijalankan oleh backend Rust melalui IPC.

### Responsibilities

* Render UI
* User Interaction
* Navigation
* Theme Management
* IPC Communication
* Notification Display

Frontend **tidak menangani proses download**, hanya mengirim perintah dan menerima status dari backend.

---

# 13. Project Folder Structure

```text
src/
 ├── app/
 ├── components/
 ├── layouts/
 ├── pages/
 ├── hooks/
 ├── services/
 ├── stores/
 ├── types/
 ├── utils/
 ├── assets/
 └── styles/

src-tauri/
 ├── src/
 ├── capabilities/
 ├── icons/
 └── tauri.conf.json
```

### Folder Responsibilities

* **components/** → Komponen UI
* **pages/** → Halaman aplikasi
* **stores/** → State Management
* **services/** → IPC & API
* **hooks/** → Custom React Hooks
* **utils/** → Fungsi utilitas
* **src-tauri/** → Backend Rust

---

# 14. React UI Layer

UI dibagi menjadi beberapa halaman utama:

* Dashboard
* Downloads
* Video Downloader
* Torrent
* History
* Statistics
* Scheduler
* Settings
* Notification Center
* About

Setiap halaman bersifat modular dan independen.

---

# 15. State Management

Menggunakan **Zustand** sebagai state management utama.

### Global State

* Theme
* Settings
* Queue
* Download List
* Notifications
* Statistics
* Window State

### Principles

* Single Source of Truth
* Immutable State
* Reactive Updates
* Minimal Re-render

---

# 16. IPC Communication

Frontend berkomunikasi dengan backend melalui **Tauri IPC**.

### Request

Frontend → Rust

Contoh:

* Start Download
* Pause Download
* Resume Download
* Delete Download

### Response

Rust → Frontend

Contoh:

* Progress Update
* Download Completed
* Error
* Notification

Semua komunikasi menggunakan format JSON.

---

# 17. Theme System

Tema yang didukung:

* Light
* Dark
* Auto (mengikuti sistem)

Pengguna juga dapat mengatur:

* Accent Color
* Font Size
* Compact Mode
* Animation On/Off

Perubahan tema diterapkan secara langsung tanpa restart aplikasi.

---

# 18. Window Management

Fitur utama:

* Resize
* Minimize
* Maximize
* Fullscreen
* Remember Window Size
* Remember Window Position
* Multi Monitor Support

Target startup jendela: **< 1 detik**.

---

# 19. Navigation System

Navigasi utama menggunakan **Sidebar**.

Menu:

1. Dashboard
2. Downloads
3. Video Downloader
4. Torrent
5. History
6. Statistics
7. Scheduler
8. Settings
9. About

### Shortcut

* Ctrl + N → New Download
* Ctrl + K → Search
* Ctrl + S → Settings
* Ctrl + H → History

---

# 20. Component Library

Komponen standar:

### Inputs

* TextField
* URLField
* SearchBox
* NumberField

### Controls

* Button
* IconButton
* Toggle
* Checkbox
* Radio
* Dropdown
* Slider

### Display

* Card
* Table
* Progress Bar
* Badge
* Tooltip
* Dialog
* Toast
* Notification

Seluruh komponen wajib mengikuti Design System VDM.

---

# 21. Frontend Event Flow

```text
User Action
      │
      ▼
React Component
      │
      ▼
Zustand Store
      │
      ▼
IPC Service
      │
      ▼
Rust Backend
      │
      ▼
Download Engine
      │
      ▼
Event Response
      │
      ▼
IPC
      │
      ▼
Store Update
      │
      ▼
UI Re-render
```

### Acceptance Criteria

* Frontend tidak menjalankan proses download secara langsung.
* Seluruh komunikasi menggunakan IPC.
* State terpusat dan konsisten.
* UI tetap responsif selama proses download.
* Komponen dapat digunakan ulang (reusable).
* Struktur proyek modular dan mudah dikembangkan.
# PART III — Backend (Rust Core)

Backend VDM dibangun menggunakan **Rust** dengan **Clean Architecture**, **Modular Monolith**, dan **Tokio Async Runtime**. Setiap modul memiliki tanggung jawab tunggal (Single Responsibility Principle) dan berkomunikasi melalui **Service Layer** serta **Event Bus**.

---

# 22. Rust Workspace

Workspace dibagi menjadi beberapa crate agar modular dan mudah dipelihara.

**Crate utama:**

* app
* core
* download
* torrent
* video
* scheduler
* queue
* storage
* network
* intelligence
* logging
* config
* shared

---

# 23. Project Structure

```text
src-tauri/
├── crates/
│   ├── app/
│   ├── core/
│   ├── download/
│   ├── network/
│   ├── queue/
│   ├── scheduler/
│   ├── storage/
│   ├── intelligence/
│   ├── logging/
│   └── shared/
├── src/
└── Cargo.toml
```

Seluruh crate dapat diuji secara independen.

---

# 24. Crate Architecture

Setiap crate terdiri dari:

* models
* services
* repositories
* events
* errors
* traits
* utils
* tests

Dependency hanya mengarah ke bawah (Dependency Rule).

---

# 25. Core Modules

Modul inti:

* Download Engine
* Queue Manager
* Scheduler
* File Manager
* Intelligence Layer
* Logging
* Configuration
* Storage
* Network

Seluruh modul bersifat independen.

---

# 26. Download Engine

Komponen utama pengunduhan.

Fungsi:

* Download Multi-thread
* Resume
* Retry
* Segment Download
* Buffer Management
* File Merge
* Integrity Verification

---

# 27. URL Analyzer

Melakukan analisis URL sebelum download.

Mendeteksi:

* Protocol
* Redirect
* Resume Support
* MIME Type
* File Size
* Server Capability
* HTTP Version

Output berupa metadata download.

---

# 28. Connection Manager

Mengelola seluruh koneksi jaringan.

Fitur:

* Connection Pool
* Keep Alive
* Timeout
* Reconnect
* HTTP/2
* HTTP/3
* Adaptive Connection

---

# 29. Segment Manager

Membagi file menjadi beberapa segment berdasarkan:

* Ukuran file
* Dukungan server
* Kondisi jaringan

Segment dapat bertambah atau berkurang secara dinamis.

---

# 30. Download Worker

Worker bertugas mengunduh setiap segment.

Fungsi:

* Download
* Pause
* Resume
* Retry
* Progress Update
* Error Reporting

Worker berjalan paralel menggunakan Tokio.

---

# 31. Buffer Manager

Mengelola buffer memori.

Fungsi:

* Dynamic Buffer
* Memory Optimization
* Write Queue
* Cache Management

Target utama adalah RAM rendah dan throughput tinggi.

---

# 32. Disk Writer

Menulis data ke media penyimpanan.

Fitur:

* Sequential Write
* Parallel Merge
* Temporary File
* Safe Rename
* Checksum Verification

---

# 33. Resume Engine

Menyimpan status download.

Mampu melanjutkan download setelah:

* Restart aplikasi
* Restart komputer
* Gangguan jaringan

---

# 34. Retry Engine

Mengelola percobaan ulang secara otomatis.

Strategi:

* Exponential Backoff
* Adaptive Retry
* Retry Limit
* Retry Queue

---

# 35. Queue Manager

Mengelola antrean download.

Fitur:

* Priority Queue
* Smart Queue
* Auto Start
* Auto Retry
* Parallel Queue
* Queue Groups

---

# 36. Scheduler

Menjalankan tugas otomatis.

Mendukung:

* Shutdown
* Restart
* Sleep
* Hibernate
* Exit Application

Berdasarkan waktu atau status download.

---

# 37. File Manager

Mengelola file hasil unduhan.

Fitur:

* Rename
* Move
* Delete
* Category
* Duplicate Detection
* Checksum
* Open Folder

---

# 38. Download Intelligence Engine

Menganalisis URL dan server.

Menentukan:

* Strategi download
* Jumlah segment
* Prioritas
* Estimasi kecepatan
* Tingkat risiko

---

# 39. Intelligent Download Optimizer

Mengoptimalkan download secara real-time.

Mengatur:

* Thread
* Buffer
* Segment
* Connection
* Bandwidth

Target:

Kecepatan maksimum dengan penggunaan sumber daya minimum.

---

# 40. Self-Healing Engine

Memantau dan memperbaiki masalah secara otomatis.

Recovery meliputi:

* Restart Worker
* Resume Download
* Restart Connection
* Queue Recovery
* Browser Recovery

---

# 41. System Health Monitor

Memantau kondisi aplikasi.

Monitoring:

* CPU
* RAM
* Disk
* Network
* Queue
* Download Engine
* Browser Extension

Menghasilkan Health Score dan Performance Score.

---

# 42. Logging System

Mencatat aktivitas sistem.

Kategori:

* Download
* Queue
* Scheduler
* Network
* Security
* Error
* Recovery

Mendukung:

* Rotation
* Search
* Export
* Debug Mode

---

# 43. Configuration Manager

Mengelola konfigurasi aplikasi.

Fungsi:

* Load
* Save
* Validate
* Reset
* Import
* Export

Konfigurasi disimpan dalam SQLite atau file JSON sesuai kebutuhan.

---

# 44. Plugin Architecture (Future)

Arsitektur plugin memungkinkan penambahan fitur tanpa mengubah inti aplikasi.

Contoh plugin:

* Browser baru
* Cloud Storage
* Antivirus Scanner
* Metadata Downloader
* AI Assistant
* Download Provider

Plugin menggunakan manifest, versi, permission, dan lifecycle yang terdokumentasi.

---

# Backend Communication Flow

```text
UI (React)
      │
      ▼
Tauri IPC
      │
      ▼
Application Service
      │
      ▼
Download Intelligence Engine
      │
      ▼
Intelligent Download Optimizer
      │
      ▼
Download Engine
      │
      ▼
Connection Manager
      │
      ▼
Download Worker
      │
      ▼
Buffer Manager
      │
      ▼
Disk Writer
      │
      ▼
File Manager
      │
      ▼
History + Logging
```

---

# Acceptance Criteria

* Backend menggunakan arsitektur modular.
* Seluruh modul memiliki tanggung jawab tunggal.
* Komunikasi antar modul melalui Service Layer dan Event Bus.
* Download Engine independen dari UI.
* Modul dapat diuji secara terpisah.
* Sistem mudah dikembangkan tanpa mengubah arsitektur inti.
# PART IV — Database

Database menggunakan **SQLite** karena ringan, cepat, tidak memerlukan server terpisah, dan sangat cocok untuk aplikasi desktop. Seluruh akses database dilakukan melalui Repository Layer.

---

# 45. SQLite Architecture

## Overview

SQLite digunakan untuk menyimpan seluruh data aplikasi secara lokal.

Karakteristik:

* Embedded Database
* ACID Compliant
* WAL Mode
* Transaction Support
* Prepared Statement
* Auto Migration

---

# 46. Database Schema

Database terdiri dari tabel utama berikut:

* settings
* downloads
* download_segments
* queue
* history
* scheduler
* notifications
* statistics
* logs
* recovery
* torrent
* video_metadata

Relasi menggunakan **Foreign Key**.

---

# 47. Tables

## settings

Menyimpan konfigurasi aplikasi.

## downloads

Menyimpan seluruh download aktif.

## download_segments

Status setiap segment download.

## queue

Antrean download.

## history

Riwayat download.

## scheduler

Tugas terjadwal.

## notifications

Riwayat notifikasi.

## statistics

Statistik penggunaan.

## logs

Log aplikasi.

## recovery

Data Resume & Self-Healing.

---

# 48. Relationships

```text id="iw98bg"
downloads
     │
     ├──────── download_segments
     │
     ├──────── history
     │
     ├──────── logs
     │
     └──────── recovery
```

Semua relasi menggunakan referensi ID internal.

---

# 49. Index Strategy

Index dibuat pada kolom yang sering dicari.

Contoh:

* download_id
* status
* created_at
* category
* filename
* url
* queue_priority

Target pencarian:

< 100 ms

---

# 50. Migration

Migration dilakukan otomatis saat aplikasi dijalankan.

Strategi:

* Versioned Migration
* Rollback Support
* Schema Validation
* Backup Before Migration

---

# 51. Backup & Recovery

VDM menyediakan:

* Auto Backup Database
* Manual Backup
* Restore Database
* Export JSON
* Import JSON

Recovery otomatis dilakukan jika database rusak dan backup valid tersedia.

---

# Database Access Rules

* Seluruh query menggunakan Prepared Statement.
* Tidak ada query langsung dari UI.
* Semua akses melalui Repository Layer.
* Transaksi digunakan untuk operasi yang melibatkan beberapa tabel.

---

# Acceptance Criteria

* Database berjalan dalam WAL Mode.
* Seluruh tabel memiliki indeks yang sesuai.
* Migrasi otomatis berhasil tanpa kehilangan data.
* Backup dan restore berfungsi.
* Seluruh akses database aman, konsisten, dan tervalidasi.
# PART V — Browser Extension

Browser Extension berfungsi sebagai penghubung antara browser dan VDM Desktop. Seluruh proses download tetap dijalankan oleh **Rust Download Engine**, sedangkan extension hanya mendeteksi dan mengirim informasi download.

---

# 52. Browser Extension Architecture

## Supported Browsers

* Google Chrome
* Microsoft Edge
* Mozilla Firefox
* Brave
* Opera
* Vivaldi

## Responsibilities

* Capture Download URL
* Detect Video & Audio
* Detect File Metadata
* Context Menu Integration
* Native Messaging Communication

Extension harus memiliki codebase yang mudah dipelihara dan kompatibel dengan browser berbasis Chromium maupun Firefox.

---

# 53. Native Messaging

Native Messaging digunakan sebagai jalur komunikasi aman antara Browser Extension dan aplikasi desktop.

## Communication Flow

```text id="7n5phw"
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

## Message Types

* StartDownload
* PauseDownload
* ResumeDownload
* CancelDownload
* GetStatus
* OpenApplication
* Ping

Semua pesan menggunakan format JSON tervalidasi.

---

# 54. Message Protocol

## Request

Informasi yang dapat dikirim:

* URL
* File Name
* MIME Type
* File Size (jika tersedia)
* Referer
* User-Agent
* Cookie (jika diperlukan)
* Timestamp

## Response

Backend dapat mengirim:

* Accepted
* Progress
* Completed
* Failed
* Error
* Version

Komunikasi bersifat asynchronous.

---

# 55. Browser Security

## Security Rules

* Extension hanya meminta permission yang diperlukan.
* Tidak mengumpulkan riwayat browsing.
* Tidak mengirim data ke server eksternal.
* Cookie hanya digunakan untuk download aktif.
* Native Messaging hanya menerima koneksi dari Extension resmi.
* Seluruh input divalidasi sebelum diproses.

## Privacy

VDM tidak menyimpan:

* Password
* Access Token
* Session Cookie secara permanen

Semua data sesi dibuang setelah proses download selesai, kecuali pengguna memilih untuk menyimpannya.

---

# Browser Event Flow

```text id="4hv8o2"
User Click Download
        │
        ▼
Browser Extension
        │
        ▼
Capture URL
        │
        ▼
Validate Request
        │
        ▼
Native Messaging
        │
        ▼
Rust Backend
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

# Acceptance Criteria

* Extension berhasil menangkap download dari browser yang didukung.
* Native Messaging stabil dan aman.
* URL diteruskan ke VDM dalam waktu <100 ms.
* Video dan media yang didukung dapat terdeteksi.
* Tidak ada data sensitif yang dikirim atau disimpan tanpa persetujuan pengguna.
* Extension tetap berfungsi setelah pembaruan browser selama API kompatibel.
# PART VI — External Components

VDM menggunakan beberapa komponen eksternal untuk memperluas kemampuan aplikasi tanpa membangun semuanya dari awal. Seluruh komponen diintegrasikan melalui lapisan adapter agar mudah diganti atau diperbarui.

---

# 56. External Components Architecture

## Principles

* Loose Coupling
* Adapter Pattern
* Independent Modules
* Replaceable Components
* Version Controlled
* Automatic Validation

Setiap komponen harus dapat diperbarui tanpa memengaruhi modul inti.

---

# 57. yt-dlp Integration

## Purpose

Digunakan sebagai backend Video Downloader.

## Supported Features

* Video Download
* Audio Download
* Playlist
* Subtitle
* Thumbnail
* Metadata
* Batch Download

## Responsibilities

* Analisis URL video
* Mengambil daftar format
* Mengambil subtitle
* Mengirim metadata ke VDM
* Menjalankan proses download sesuai pilihan pengguna

Jika yt-dlp gagal, VDM harus menampilkan pesan yang jelas dan menyimpan log.

---

# 58. FFmpeg Integration

## Purpose

Digunakan untuk pemrosesan media.

## Features

* Merge Audio & Video
* Extract Audio
* Convert Format
* Embed Subtitle
* Generate Thumbnail
* Metadata Processing

## Execution

FFmpeg dijalankan sebagai proses terpisah (child process) dan dipantau hingga selesai.

---

# 59. Torrent Engine

## Purpose

Menangani download berbasis BitTorrent.

## Supported Features

* Torrent File (.torrent)
* Magnet Link
* Resume
* Sequential Download
* File Priority
* Peer Management
* Piece Verification

## Responsibilities

* Peer Discovery
* Tracker Communication
* Piece Download
* Integrity Check
* Seeding (opsional)

---

# 60. Windows API Integration

## Purpose

Mengakses fitur sistem operasi Windows.

## Features

* Shutdown
* Restart
* Sleep
* Hibernate
* System Tray
* Notification
* File Association
* Startup Registration
* Default Download Folder
* Clipboard Access

Semua akses dilakukan melalui wrapper Rust agar mudah diuji dan dipelihara.

---

# 61. External Process Manager

Modul ini bertanggung jawab mengelola seluruh proses eksternal.

## Responsibilities

* Start Process
* Stop Process
* Monitor Process
* Restart Process
* Capture Output
* Capture Error
* Timeout Handling

Komponen yang dikelola:

* yt-dlp
* FFmpeg
* Torrent Engine (jika berjalan sebagai proses terpisah)

---

# 62. Dependency Management

Seluruh dependensi memiliki informasi:

* Name
* Version
* License
* Source
* Checksum

VDM harus melakukan pemeriksaan kompatibilitas saat startup dan sebelum pembaruan.

---

# 63. Update Strategy

Komponen eksternal dapat diperbarui secara terpisah dari aplikasi.

Strategi:

* Version Check
* Integrity Verification
* Rollback jika pembaruan gagal
* Kompatibilitas dengan versi VDM

---

# 64. Error Handling

Jika komponen eksternal gagal:

* Catat ke Logging System.
* Berikan notifikasi kepada pengguna.
* Jalankan Recovery jika memungkinkan.
* Tampilkan solusi yang disarankan.

Contoh:

* yt-dlp tidak ditemukan.
* FFmpeg gagal dijalankan.
* Torrent Engine gagal dimuat.

---

# External Component Flow

```text id="d9c4r1"
User Request
      │
      ▼
Application Service
      │
      ▼
External Adapter
      │
 ┌────┼───────────┬─────────┐
 ▼    ▼           ▼         ▼
yt-dlp FFmpeg Torrent Windows API
      │
      ▼
Result
      │
      ▼
Download Engine / UI
```

---

# Acceptance Criteria

* Seluruh komponen eksternal dapat dimuat dan diverifikasi saat startup.
* Adapter memisahkan logika aplikasi dari implementasi komponen eksternal.
* Kesalahan pada satu komponen tidak menyebabkan aplikasi berhenti.
* Pembaruan komponen dapat dilakukan secara independen.
* Seluruh proses eksternal dipantau, dicatat, dan dapat dihentikan dengan aman.
# PART VII — Internal Communication

Internal Communication mengatur bagaimana seluruh modul VDM saling berkomunikasi tanpa saling bergantung secara langsung. Arsitektur menggunakan **Service Layer**, **IPC**, dan **Event Bus** untuk menjaga modularitas, skalabilitas, dan kemudahan pengujian.

---

# 65. Communication Architecture

## Principles

* Loose Coupling
* Event Driven
* Asynchronous Communication
* Type Safe
* Non-Blocking
* Modular

Semua komunikasi harus melalui interface yang telah ditentukan.

---

# 66. IPC API

## Overview

Frontend (React) berkomunikasi dengan Backend (Rust) menggunakan **Tauri IPC**.

### Request

Frontend → Backend

Contoh:

* StartDownload
* PauseDownload
* ResumeDownload
* DeleteDownload
* OpenFolder
* UpdateSettings

### Response

Backend → Frontend

Contoh:

* Success
* Error
* Progress
* Notification
* Statistics

Seluruh data menggunakan format JSON.

---

# 67. Event Bus

## Purpose

Event Bus menjadi media komunikasi antar modul backend tanpa ketergantungan langsung.

### Event Categories

* Download
* Queue
* Scheduler
* Settings
* Network
* Notification
* Logging
* Recovery
* Security

### Sample Events

* DownloadCreated
* DownloadStarted
* DownloadProgress
* DownloadPaused
* DownloadCompleted
* DownloadFailed
* QueueUpdated
* SettingsChanged
* NetworkChanged
* RecoveryCompleted

Modul hanya berlangganan (subscribe) pada event yang dibutuhkan.

---

# 68. Internal Services

Service Layer menyediakan logika bisnis aplikasi.

## Services

* DownloadService
* QueueService
* SchedulerService
* FileService
* HistoryService
* NotificationService
* StatisticsService
* LoggingService
* ConfigurationService

UI tidak berkomunikasi langsung dengan Download Engine maupun database.

---

# 69. Message Queue

Message Queue mengelola pesan internal yang berjalan secara asynchronous.

## Responsibilities

* Queue Message
* Prioritize Message
* Retry Message
* Dead Letter Queue (Future)
* Event Dispatch

Semua pesan diproses sesuai urutan dan prioritas.

---

# 70. Communication Flow

```text id="mj8kq2"
React UI
     │
     ▼
Tauri IPC
     │
     ▼
Service Layer
     │
     ▼
Event Bus
     │
 ┌───┼──────────────┐
 ▼   ▼              ▼
Queue Download Scheduler
 │      │             │
 └──────┼─────────────┘
        ▼
Logging System
```

---

# 71. Error Propagation

Jika terjadi kesalahan:

1. Modul menghasilkan Error.
2. Logging System mencatat Error.
3. Event Bus mengirim Error Event.
4. Notification Service membuat notifikasi.
5. UI menampilkan pesan kepada pengguna.

Tidak boleh ada error yang hilang tanpa tercatat.

---

# 72. Data Flow

```text id="jv67f9"
User Action
      │
      ▼
React Component
      │
      ▼
IPC Request
      │
      ▼
Service Layer
      │
      ▼
Business Logic
      │
      ▼
Download Engine
      │
      ▼
Event Bus
      │
      ▼
IPC Event
      │
      ▼
UI Update
```

---

# 73. Communication Rules

* Semua komunikasi bersifat asynchronous jika memungkinkan.
* Modul tidak boleh memanggil database secara langsung kecuali melalui Repository.
* UI hanya berkomunikasi melalui IPC.
* Backend menggunakan Event Bus untuk komunikasi internal.
* Semua pesan harus divalidasi sebelum diproses.
* Event tidak boleh mengandung data sensitif yang tidak diperlukan.

---

# 74. Acceptance Criteria

* Komunikasi antar modul tidak memiliki ketergantungan langsung (tight coupling).
* Seluruh IPC terdokumentasi dan tervalidasi.
* Event Bus berhasil mendistribusikan event ke modul yang berlangganan.
* Service Layer menjadi satu-satunya pintu masuk logika bisnis.
* Error dan notifikasi mengalir secara konsisten dari backend ke frontend.
* Sistem tetap responsif saat banyak event diproses secara bersamaan.
# PART VIII — Security

Security memastikan seluruh komponen VDM bekerja secara aman, melindungi data pengguna, menjaga integritas file, serta mencegah penyalahgunaan sistem. Seluruh implementasi mengikuti prinsip **Security by Design** dan **Least Privilege**.

---

# 75. Security Architecture

## Principles

* Security by Design
* Least Privilege
* Defense in Depth
* Zero Trust
* Privacy First
* Secure by Default

Keamanan diterapkan pada setiap lapisan aplikasi:

* UI
* IPC
* Service Layer
* Download Engine
* Database
* Browser Extension
* External Components

---

# 76. Authentication & Authorization

VDM tidak memerlukan akun pengguna.

Namun, seluruh komunikasi internal harus divalidasi.

### Validation

* IPC Validation
* Native Messaging Validation
* JSON Schema Validation
* Input Validation

Setiap request harus diperiksa sebelum diproses.

---

# 77. Secure Storage

Data lokal disimpan secara aman.

### Stored Data

* Settings
* Queue
* History
* Statistics
* Logs

### Sensitive Data

Data berikut tidak boleh disimpan secara permanen tanpa persetujuan pengguna:

* Password
* Token
* Session Cookie
* Authorization Header

---

# 78. Certificate Validation

Untuk koneksi HTTPS:

* Validasi sertifikat TLS.
* Validasi hostname.
* Cek masa berlaku sertifikat.
* Tolak sertifikat yang tidak tepercaya.

Download dibatalkan jika validasi gagal.

---

# 79. Privacy & Data Protection

VDM menerapkan kebijakan privasi lokal.

### Rules

* Tidak mengirim data pengguna ke server.
* Tidak melacak aktivitas browsing.
* Tidak mengumpulkan telemetry secara default.
* Crash Report bersifat opsional (opt-in).

Semua data tetap berada di perangkat pengguna kecuali dipilih sebaliknya.

---

# 80. Input Validation

Semua input diperiksa sebelum diproses.

### Validation

* URL
* File Name
* Path
* JSON
* IPC Message
* Browser Message
* Configuration File

Input yang tidak valid ditolak dan dicatat ke Logging System.

---

# 81. File Security

Setiap file download menggunakan mekanisme berikut:

* Temporary File (`.vdm`)
* Resume Metadata
* Checksum Verification
* Safe Rename
* Duplicate Detection

File hanya dianggap selesai setelah proses verifikasi berhasil.

---

# 82. Secure Communication

Komunikasi internal:

* Tauri IPC
* Event Bus
* Native Messaging

Seluruh pesan harus:

* Tervalidasi.
* Bertipe jelas (type-safe).
* Tidak membawa data sensitif yang tidak diperlukan.

---

# 83. Dependency Security

Seluruh dependensi diperiksa secara berkala.

### Security Checks

* Vulnerability Scan
* License Check
* Version Check
* Integrity Verification

Dependensi yang memiliki kerentanan kritis tidak boleh digunakan pada rilis produksi.

---

# 84. Security Monitoring

System Health Monitor bekerja sama dengan Logging System untuk memantau:

* Failed Requests
* Invalid IPC
* Certificate Errors
* Database Errors
* Recovery Events
* Crash Events

Peristiwa keamanan dicatat untuk analisis lebih lanjut.

---

# 85. Security Testing

Pengujian keamanan meliputi:

* URL Validation Test
* HTTPS Validation Test
* IPC Validation Test
* Native Messaging Test
* File Integrity Test
* Input Validation Test
* Dependency Audit
* Penetration Test dasar untuk komponen internal

---

# 86. Acceptance Criteria

* Seluruh input tervalidasi sebelum diproses.
* Data sensitif tidak disimpan tanpa persetujuan pengguna.
* Komunikasi internal tervalidasi dan aman.
* Sertifikat HTTPS diverifikasi sebelum download dimulai.
* File hasil download diverifikasi sebelum dianggap selesai.
* Ketergantungan eksternal telah melalui pemeriksaan keamanan sebelum rilis.
# PART IX — Performance

Performance merupakan prioritas utama VDM. Seluruh sistem dirancang agar memberikan kecepatan download maksimal, penggunaan sumber daya yang rendah, serta antarmuka yang tetap responsif pada berbagai kondisi perangkat dan jaringan.

---

# 87. Async Runtime

Backend menggunakan **Tokio** sebagai asynchronous runtime.

## Responsibilities

* Async Task
* Network I/O
* File I/O
* Timer
* Background Worker
* Event Processing

### Principles

* Non-Blocking
* Async First
* High Concurrency
* Efficient Scheduling

---

# 88. Thread Model

VDM menggunakan **Hybrid Thread Model**.

### Main Thread

* UI
* IPC

### Worker Threads

* Download Worker
* Queue Worker
* Retry Worker
* Scheduler Worker
* Logging Worker
* Verification Worker

Jumlah worker bersifat dinamis sesuai kemampuan perangkat.

---

# 89. Memory Management

Target penggunaan RAM:

| State      |   Target |
| ---------- | -------: |
| Idle       | < 100 MB |
| Normal     | < 250 MB |
| Heavy Load | < 500 MB |

### Strategy

* Buffer Pool
* Memory Reuse
* Lazy Allocation
* Automatic Cleanup
* Zero-Copy bila memungkinkan

---

# 90. CPU Optimization

Target penggunaan CPU:

| State      | Target |
| ---------- | -----: |
| Idle       |   < 1% |
| Normal     |  < 10% |
| Heavy Load |  < 30% |

### Techniques

* Adaptive Worker Count
* Dynamic Scheduling
* Background Processing
* Intelligent Download Optimizer

---

# 91. Disk Optimization

Strategi penulisan file:

* Sequential Write
* Buffered Write
* Parallel Merge
* Safe Rename
* Temporary File
* Cache Management

Target:

* Mengurangi fragmentasi.
* Meminimalkan operasi tulis kecil.
* Menjaga respons sistem tetap baik.

---

# 92. Network Optimization

Fitur:

* Connection Pool
* Keep Alive
* HTTP/2
* HTTP/3
* Adaptive Segment
* Smart Retry
* Bandwidth Optimization
* Resume Support

Engine menyesuaikan jumlah koneksi berdasarkan kemampuan server dan kondisi jaringan.

---

# 93. Performance Monitoring

System Health Monitor memantau:

* CPU Usage
* RAM Usage
* Disk I/O
* Network Speed
* Active Downloads
* Queue Length
* Thread Count
* Buffer Usage

Data diperbarui secara berkala dan tersedia untuk halaman Statistics.

---

# 94. Performance Benchmark

Target internal:

| Metric           | Target   |
| ---------------- | -------- |
| Startup          | < 1 s    |
| UI Response      | < 100 ms |
| Search           | < 100 ms |
| Queue Update     | < 100 ms |
| Progress Refresh | ≤ 1 s    |
| Resume           | < 2 s    |

Kecepatan download ditargetkan setara atau lebih baik dibanding aplikasi sekelas pada kondisi server dan jaringan yang sama.

---

# 95. Performance Testing

Pengujian wajib meliputi:

* Cold Start
* Warm Start
* Large File Download
* 100 Concurrent Downloads
* Large Queue (10.000 item)
* Long Running Test (24 jam)
* Memory Leak Test
* Network Interruption Test
* SSD & HDD Performance Test

---

# 96. Optimization Strategy

Optimasi dilakukan melalui:

* Intelligent Download Optimizer
* Download Intelligence Engine
* Self-Healing Engine
* Adaptive Thread Management
* Dynamic Buffer
* Lazy Loading
* Virtual List Rendering
* Incremental Statistics
* Connection Pooling

Seluruh optimasi harus tetap menjaga stabilitas aplikasi.

---

# 97. Acceptance Criteria

* UI tetap responsif selama proses download.
* Penggunaan CPU dan RAM berada dalam target.
* Download Engine mampu memanfaatkan bandwidth secara efisien.
* Tidak ditemukan memory leak pada pengujian jangka panjang.
* Startup dan Resume memenuhi target performa.
* Sistem tetap stabil saat menangani antrean dan download dalam jumlah besar.
# PART X — Testing & Deployment

Testing dan Deployment memastikan VDM memiliki kualitas tinggi, stabil, aman, serta mudah didistribusikan kepada pengguna. Seluruh proses mengikuti prinsip **Continuous Integration (CI)** dan **Continuous Delivery (CD)**.

---

# 98. Testing Strategy

## Objectives

* Memastikan seluruh fitur bekerja sesuai spesifikasi.
* Mencegah regresi.
* Menjamin stabilitas aplikasi.
* Mengukur performa.
* Memastikan keamanan.

### Testing Levels

* Unit Test
* Integration Test
* System Test
* Performance Test
* Stress Test
* UI Test
* Security Test
* Regression Test

Target Code Coverage:

**≥ 80%**

---

# 99. Unit Testing

Pengujian dilakukan untuk setiap module Rust dan React.

## Modules

* Download Engine
* Queue Manager
* Scheduler
* File Manager
* Intelligence Engine
* Self-Healing Engine
* Configuration Manager
* Logging System

Framework:

* Rust → `cargo test`
* React → `Vitest`

---

# 100. Integration Testing

Menguji komunikasi antar modul.

## Scenarios

* UI ↔ IPC
* IPC ↔ Backend
* Download Engine ↔ Queue
* Queue ↔ Scheduler
* Browser Extension ↔ Native Messaging
* yt-dlp ↔ Download Engine
* FFmpeg ↔ Video Module
* Database ↔ Repository

---

# 101. Performance & Stress Testing

## Performance

* Startup Time
* UI Response
* Download Speed
* Memory Usage
* CPU Usage

## Stress

* 100 Download Bersamaan
* Queue 10.000 Item
* File >100 GB
* Download 24 Jam Nonstop

Target:

Tidak terjadi crash atau memory leak.

---

# 102. Security Testing

Pengujian keamanan meliputi:

* URL Validation
* HTTPS Validation
* IPC Validation
* Native Messaging Validation
* File Integrity
* Dependency Audit
* Input Validation

Semua kerentanan kritis harus diperbaiki sebelum rilis.

---

# 103. Build System

Build menggunakan:

* Cargo
* Vite
* Tauri CLI

Target Build:

* Windows x64
* Windows ARM64 (Future)
* Linux (Future)

Build harus bersifat reproducible dan otomatis.

---

# 104. Installer

Installer dibuat menggunakan paket yang didukung Tauri.

Fitur:

* Install
* Upgrade
* Uninstall
* Shortcut Desktop
* Start Menu
* File Association (opsional)

Installer harus memeriksa kompatibilitas sistem sebelum proses instalasi.

---

# 105. Auto Update

Fitur pembaruan mendukung:

* Manual Check
* Automatic Check
* Download Update
* Background Update
* Rollback jika pembaruan gagal

Seluruh paket pembaruan harus diverifikasi sebelum dipasang.

---

# 106. CI/CD Pipeline

Pipeline otomatis meliputi:

1. Lint
2. Format Check
3. Unit Test
4. Integration Test
5. Security Audit
6. Build
7. Package
8. Sign (jika digunakan)
9. Release

Platform yang direkomendasikan:

* GitHub Actions

---

# 107. Release Strategy

## Release Types

* Alpha
* Beta
* Release Candidate (RC)
* Stable
* Hotfix

Setiap rilis harus memiliki:

* Changelog
* Version Number
* Build Number
* Release Notes

Mengikuti prinsip **Semantic Versioning**:

`MAJOR.MINOR.PATCH`

---

# 108. Acceptance Criteria

* Seluruh pengujian wajib lulus sebelum rilis.
* Code coverage memenuhi target.
* Installer dapat memasang, memperbarui, dan menghapus aplikasi tanpa masalah.
* Auto Update memverifikasi integritas paket sebelum instalasi.
* Pipeline CI/CD menghasilkan build yang konsisten dan dapat direproduksi.
* Setiap rilis memiliki dokumentasi perubahan yang jelas dan terdokumentasi.
