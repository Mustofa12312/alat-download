# Velocity Download Manager (VDM)

# Database Design Specification

**Document ID:** VDM-DB-001
**Version:** 1.0.0
**Status:** Draft
**Confidentiality:** Internal Use Only

---

# 1. Cover

| Item            | Information                             |
| --------------- | --------------------------------------- |
| Project         | Velocity Download Manager (VDM)         |
| Document        | Database Design Specification           |
| Document ID     | VDM-DB-001                              |
| Version         | 1.0.0                                   |
| Status          | Draft                                   |
| Database        | SQLite                                  |
| Architecture    | Repository Pattern + Clean Architecture |
| Storage Mode    | Local Database                          |
| Target Platform | Windows 10 & Windows 11                 |
| Author          | Mustofa & ChatGPT                       |
| Last Updated    | July 2026                               |

---

# 2. Document Information

## Purpose

Dokumen ini menjelaskan desain database Velocity Download Manager (VDM), termasuk struktur tabel, relasi, indeks, aturan penyimpanan data, strategi migrasi, optimasi performa, serta kebijakan keamanan database.

## Objectives

* Menjadi acuan implementasi database.
* Menjamin konsistensi struktur data.
* Mendukung performa tinggi.
* Mempermudah proses pengembangan dan pemeliharaan.
* Menjadi referensi untuk migrasi dan pengujian database.

## Scope

Dokumen ini mencakup:

* Arsitektur database
* Konfigurasi SQLite
* Desain tabel
* Relasi antar tabel
* Primary Key & Foreign Key
* Index Strategy
* Migration
* Backup & Restore
* Data Validation
* Security
* Performance Optimization
* Database Testing

## Target Audience

* Software Architect
* Backend Rust Developer
* Database Engineer
* QA Engineer
* DevOps Engineer
* Technical Reviewer

## Related Documents

* `01-PRD.md` — Product Requirements Document
* `02-SDS.md` — Software Design Specification
* `04-API.md` — API Specification
* `05-Development-Guide.md` — Development Guide

---

# 3. Revision History

| Version | Date      | Author            | Description                    |
| ------- | --------- | ----------------- | ------------------------------ |
| 0.1.0   | July 2026 | Mustofa & ChatGPT | Initial Database Design        |
| 0.5.0   | TBD       | Development Team  | Core Tables Completed          |
| 0.9.0   | TBD       | Development Team  | Optimization & Migration Added |
| 1.0.0   | TBD       | Development Team  | Production Release             |

---

# 4. Table of Contents

## PART I — Overview

1. Cover
2. Document Information
3. Revision History
4. Table of Contents
5. Introduction
6. Database Objectives
7. Database Architecture
8. Design Principles

## PART II — Database Design

9. SQLite Configuration
10. Naming Convention
11. Data Types
12. Tables Overview
13. Relationships
14. Primary Keys
15. Foreign Keys
16. Index Strategy
17. Constraints
18. Views (Future)

## PART III — Table Specifications

19. Settings Table
20. Downloads Table
21. Download Segments Table
22. Queue Table
23. History Table
24. Scheduler Table
25. Notifications Table
26. Statistics Table
27. Logs Table
28. Recovery Table
29. Torrent Table
30. Video Metadata Table

## PART IV — Database Operations

31. CRUD Operations
32. Transactions
33. Migration Strategy
34. Backup & Restore
35. Data Validation
36. Performance Optimization
37. Maintenance

## PART V — Security

38. Database Security
39. Data Privacy
40. Integrity Verification

## PART VI — Testing

41. Database Testing
42. Performance Testing
43. Acceptance Criteria
44. Appendix
# PART I — Overview

---

# 5. Introduction

## Overview

Velocity Download Manager (VDM) menggunakan **SQLite** sebagai database utama untuk menyimpan seluruh data aplikasi secara lokal. Database dirancang agar ringan, cepat, stabil, dan tidak memerlukan server tambahan.

Dokumen ini menjadi acuan desain database yang digunakan oleh Backend Rust melalui **Repository Pattern**.

---

# 6. Database Objectives

Tujuan utama desain database:

* Penyimpanan data yang konsisten.
* Performa tinggi untuk operasi baca dan tulis.
* Mendukung Resume Download.
* Mendukung Queue dan Scheduler.
* Menyimpan riwayat download.
* Mendukung Logging dan Statistics.
* Mudah dimigrasikan.
* Mudah dicadangkan (Backup) dan dipulihkan (Restore).

---

# 7. Database Architecture

VDM menggunakan arsitektur tiga lapis.

```text id="n2f4w8"
Application
      │
      ▼
Repository Layer
      │
      ▼
SQLite Database
```

### Layer Responsibilities

**Application Layer**

* Business Logic
* Services
* Validation

**Repository Layer**

* CRUD Operations
* Query Execution
* Transactions
* Mapping Data

**SQLite Layer**

* Data Storage
* Index
* Constraints
* Foreign Keys

Frontend tidak memiliki akses langsung ke database.

---

# 8. Design Principles

## Local First

Semua data disimpan secara lokal pada perangkat pengguna.

---

## ACID Compliance

Seluruh transaksi harus memenuhi prinsip:

* Atomicity
* Consistency
* Isolation
* Durability

---

## Performance First

Database dioptimalkan untuk:

* Startup cepat
* Query cepat
* Resume instan
* Penulisan log efisien

---

## Data Integrity

Setiap data harus:

* Tervalidasi
* Konsisten
* Memiliki relasi yang benar
* Menghindari duplikasi yang tidak diperlukan

---

## Scalability

Database harus mampu menangani:

* > 10.000 download
* > 100.000 riwayat download
* > 1.000.000 log
* File berukuran sangat besar tanpa penurunan performa yang signifikan

---

## Security

* Menggunakan Prepared Statement.
* Mendukung Foreign Key.
* Validasi seluruh input.
* Melindungi integritas data.

---

## Maintainability

Struktur tabel, indeks, dan migrasi harus mudah dipahami, dikembangkan, dan dipelihara tanpa memengaruhi data yang sudah ada.

---

# Acceptance Criteria

* Arsitektur database terdokumentasi dengan jelas.
* Repository Layer menjadi satu-satunya akses ke SQLite.
* Database memenuhi prinsip ACID.
* Struktur mendukung performa tinggi dan integritas data.
* Desain siap dikembangkan untuk fitur baru tanpa perubahan besar pada skema inti.
# PART II — Database Design

---

# 9. SQLite Configuration

## Overview

SQLite dikonfigurasi untuk memberikan performa tinggi, stabilitas, dan keamanan data.

### Configuration

* Journal Mode : WAL
* Foreign Keys : Enabled
* Auto Vacuum : Incremental
* Synchronous : NORMAL
* Temp Store : Memory
* Cache Size : Optimized
* Busy Timeout : 5000 ms

Target:

* Startup cepat
* Query cepat
* Resume instan
* Risiko korupsi data rendah

---

# 10. Naming Convention

## Tables

Menggunakan huruf kecil dan snake_case.

Contoh:

* settings
* downloads
* download_segments
* notifications

---

## Columns

Menggunakan snake_case.

Contoh:

* download_id
* created_at
* updated_at
* file_name
* total_size

---

## Primary Key

Format:

```text
id
```

---

## Foreign Key

Format:

```text
download_id
queue_id
scheduler_id
```

---

## Index

Format:

```text
idx_download_status
idx_history_date
idx_logs_level
```

---

# 11. Data Types

| Type     | Usage                |
| -------- | -------------------- |
| INTEGER  | ID, Counter, Status  |
| TEXT     | String, URL, Path    |
| REAL     | Speed, Ratio         |
| BLOB     | Binary Data          |
| BOOLEAN  | True / False (0 / 1) |
| DATETIME | Timestamp            |

Semua tanggal menggunakan format **UTC**.

---

# 12. Tables Overview

Database utama terdiri dari:

| Table             | Purpose              |
| ----------------- | -------------------- |
| settings          | Konfigurasi aplikasi |
| downloads         | Download aktif       |
| download_segments | Segment download     |
| queue             | Antrean download     |
| history           | Riwayat download     |
| scheduler         | Jadwal otomatis      |
| notifications     | Riwayat notifikasi   |
| statistics        | Statistik aplikasi   |
| logs              | Log sistem           |
| recovery          | Data resume          |
| torrent           | Informasi torrent    |
| video_metadata    | Metadata video       |

---

# 13. Relationships

Hubungan utama antar tabel:

```text
downloads
     │
     ├── download_segments
     ├── history
     ├── logs
     ├── recovery
     ├── torrent
     └── video_metadata

queue
     │
     └── downloads

scheduler
     │
     └── downloads
```

Seluruh relasi menggunakan **Foreign Key**.

---

# 14. Primary Keys

Seluruh tabel memiliki Primary Key bertipe INTEGER.

Contoh:

```text
id INTEGER PRIMARY KEY AUTOINCREMENT
```

Keuntungan:

* Cepat
* Ringan
* Mudah diindeks

---

# 15. Foreign Keys

Foreign Key digunakan untuk menjaga integritas data.

Contoh relasi:

* download_segments → downloads
* history → downloads
* recovery → downloads
* torrent → downloads

Seluruh Foreign Key menggunakan aturan referensial yang konsisten.

---

# 16. Index Strategy

Index dibuat pada kolom yang sering digunakan.

### Indexed Columns

* id
* status
* created_at
* updated_at
* download_id
* category
* priority
* url
* file_name

Target:

* Query <100 ms
* Search cepat
* Sorting efisien

---

# 17. Constraints

Setiap tabel menggunakan constraint untuk menjaga kualitas data.

### Constraints

* PRIMARY KEY
* FOREIGN KEY
* NOT NULL
* UNIQUE
* CHECK
* DEFAULT

Contoh:

* Status hanya boleh menggunakan nilai yang telah ditentukan.
* Priority tidak boleh bernilai negatif.
* URL tidak boleh kosong.

---

# 18. Views (Future)

View digunakan untuk mempermudah laporan dan statistik.

Contoh View:

* active_downloads
* completed_downloads
* failed_downloads
* download_statistics
* daily_history
* monthly_statistics

View bersifat read-only dan digunakan untuk analisis data tanpa mengubah tabel utama.

---

# Acceptance Criteria

* SQLite dikonfigurasi sesuai standar performa VDM.
* Seluruh tabel mengikuti naming convention yang konsisten.
* Data type dipilih sesuai kebutuhan dan efisien.
* Relasi antar tabel terdokumentasi.
* Seluruh Primary Key dan Foreign Key tervalidasi.
* Index diterapkan pada kolom yang sering digunakan.
* Constraint menjaga integritas data.
* Struktur database siap dikembangkan untuk fitur baru tanpa perubahan besar.
# PART III — Table Specifications

---

# 19. Settings Table

## Purpose

Menyimpan seluruh konfigurasi aplikasi.

### Main Fields

* id
* key
* value
* category
* created_at
* updated_at

---

# 20. Downloads Table

## Purpose

Menyimpan seluruh download aktif.

### Main Fields

* id
* url
* file_name
* file_path
* total_size
* downloaded_size
* download_speed
* status
* priority
* category
* checksum
* created_at
* updated_at

---

# 21. Download Segments Table

## Purpose

Menyimpan informasi setiap segment download.

### Main Fields

* id
* download_id
* segment_index
* start_byte
* end_byte
* downloaded_bytes
* status

Satu download dapat memiliki banyak segment.

---

# 22. Queue Table

## Purpose

Mengatur antrean download.

### Main Fields

* id
* download_id
* queue_order
* priority
* auto_start
* created_at

---

# 23. History Table

## Purpose

Menyimpan riwayat download yang telah selesai atau gagal.

### Main Fields

* id
* download_id
* status
* completed_at
* total_time
* average_speed
* file_size

---

# 24. Scheduler Table

## Purpose

Menyimpan jadwal otomatis.

### Main Fields

* id
* task_name
* trigger_type
* scheduled_time
* action
* enabled

---

# 25. Notifications Table

## Purpose

Menyimpan riwayat notifikasi.

### Main Fields

* id
* title
* message
* type
* priority
* is_read
* created_at

---

# 26. Statistics Table

## Purpose

Menyimpan statistik aplikasi.

### Main Fields

* id
* total_downloads
* completed_downloads
* failed_downloads
* total_download_size
* total_download_time
* average_speed
* updated_at

---

# 27. Logs Table

## Purpose

Menyimpan log sistem.

### Main Fields

* id
* level
* module
* event
* message
* created_at

---

# 28. Recovery Table

## Purpose

Menyimpan data Resume dan Self-Healing.

### Main Fields

* id
* download_id
* resume_data
* recovery_status
* last_position
* updated_at

---

# 29. Torrent Table

## Purpose

Menyimpan informasi torrent.

### Main Fields

* id
* download_id
* magnet_link
* torrent_name
* hash
* tracker
* peers
* seeds
* progress

---

# 30. Video Metadata Table

## Purpose

Menyimpan metadata video yang diperoleh melalui yt-dlp.

### Main Fields

* id
* download_id
* title
* uploader
* duration
* resolution
* format
* subtitle
* thumbnail
* upload_date

---

# Relationships

```text id="g6axzd"
downloads
     │
     ├── download_segments
     ├── queue
     ├── history
     ├── recovery
     ├── torrent
     ├── video_metadata
     └── logs
```

---

# Index Recommendations

Index dibuat pada kolom:

* download_id
* status
* priority
* queue_order
* created_at
* updated_at
* file_name
* url

---

# Data Integrity Rules

* Semua tabel memiliki Primary Key.
* Foreign Key digunakan pada tabel yang berelasi.
* URL tidak boleh kosong.
* Status menggunakan nilai yang telah ditentukan.
* Timestamp menggunakan UTC.
* Data dihapus atau diperbarui sesuai aturan referensial yang ditetapkan.

---

# Acceptance Criteria

* Seluruh tabel memiliki tujuan yang jelas.
* Struktur mendukung fitur Resume, Queue, Scheduler, Torrent, Video Downloader, Logging, dan Statistics.
* Relasi antar tabel terdokumentasi dan konsisten.
* Kolom utama siap diimplementasikan pada SQLite.
* Struktur dapat dikembangkan tanpa mengubah desain inti database.
# PART IV — Database Operations

---

# 31. CRUD Operations

## Overview

Seluruh operasi database dilakukan melalui **Repository Layer**. Frontend maupun Business Logic tidak diperbolehkan mengakses SQLite secara langsung.

### Operations

* Create
* Read
* Update
* Delete
* Search
* Filter
* Sort
* Pagination

### Rules

* Menggunakan Prepared Statement.
* Seluruh input divalidasi.
* Error dicatat ke Logging System.
* Mendukung operasi asynchronous bila memungkinkan.

---

# 32. Transactions

## Purpose

Transaction digunakan untuk memastikan beberapa operasi database berjalan sebagai satu kesatuan (atomic).

### Digunakan pada

* Membuat Download Baru
* Resume Download
* Mengubah Status Download
* Menghapus Download
* Memperbarui Queue
* Menyimpan Recovery Data

### Rules

* Commit jika berhasil.
* Rollback jika terjadi kesalahan.
* Hindari transaksi yang berjalan terlalu lama.

---

# 33. Migration Strategy

## Overview

Migration digunakan untuk memperbarui struktur database tanpa kehilangan data pengguna.

### Strategy

* Versioned Migration
* Automatic Migration
* Rollback Support
* Schema Validation
* Backup Before Migration

Migration dijalankan saat aplikasi pertama kali dibuka setelah pembaruan.

---

# 34. Backup & Restore

## Backup

Mendukung:

* Automatic Backup
* Manual Backup
* Scheduled Backup
* Export Database

## Restore

Mendukung:

* Restore Database
* Import Backup
* Restore Settings
* Restore History

Backup harus diverifikasi sebelum digunakan.

---

# 35. Data Validation

Seluruh data diperiksa sebelum disimpan.

### Validation

* URL
* File Name
* File Path
* Priority
* Status
* Category
* Timestamp

Data yang tidak valid ditolak dan dicatat ke Logging System.

---

# 36. Performance Optimization

Strategi optimasi:

* WAL Mode
* Prepared Statement
* Index Optimization
* Query Optimization
* Transaction Batching
* Lazy Loading
* Periodic VACUUM
* ANALYZE

Target:

* Query < 100 ms
* Insert < 20 ms
* Update < 20 ms
* Delete < 20 ms

---

# 37. Maintenance

Database menyediakan fitur pemeliharaan.

### Features

* Integrity Check
* VACUUM
* ANALYZE
* Cleanup Logs
* Cleanup History
* Optimize Database
* Rebuild Index (jika diperlukan)

Pemeliharaan dapat dijalankan secara manual maupun otomatis sesuai jadwal.

---

# Database Operation Flow

```text id="p6r8tx"
Application
      │
      ▼
Repository
      │
      ▼
Validation
      │
      ▼
Transaction
      │
      ▼
SQLite
      │
      ▼
Commit / Rollback
      │
      ▼
Logging
```

---

# Acceptance Criteria

* Seluruh operasi database melalui Repository Layer.
* Transaction digunakan pada operasi yang melibatkan beberapa tabel.
* Migration berjalan otomatis tanpa kehilangan data.
* Backup dan Restore berhasil dilakukan.
* Data tervalidasi sebelum disimpan.
* Database tetap optimal melalui proses maintenance berkala.
* Target performa query dan transaksi terpenuhi.
# PART V — Database Security

---

# 38. Database Security

## Overview

Database Security memastikan seluruh data yang disimpan oleh Velocity Download Manager (VDM) tetap aman, konsisten, dan terlindungi dari akses yang tidak sah maupun kerusakan data.

### Security Principles

* Least Privilege
* Secure by Default
* Defense in Depth
* Data Integrity
* Privacy First

---

## Access Rules

* Database hanya dapat diakses melalui **Repository Layer**.
* Frontend tidak memiliki akses langsung ke SQLite.
* Semua query menggunakan **Prepared Statement**.
* Semua input wajib divalidasi sebelum diproses.

---

## Security Features

* Foreign Key Enforcement
* Transaction Protection
* WAL Mode
* Automatic Backup
* Integrity Check
* Error Logging

---

# 39. Data Privacy

## Overview

VDM mengutamakan privasi pengguna dengan menyimpan seluruh data secara lokal.

### Stored Data

* Settings
* Downloads
* Queue
* History
* Statistics
* Logs

### Restricted Data

Data berikut tidak disimpan secara permanen tanpa persetujuan pengguna:

* Password
* Access Token
* Session Cookie
* Authorization Header
* Informasi sensitif lain yang tidak diperlukan

---

## Privacy Rules

* Tidak mengirim database ke server eksternal.
* Tidak mengumpulkan telemetry secara default.
* Export database hanya dilakukan atas permintaan pengguna.
* Backup tetap berada di perangkat pengguna kecuali dipilih sebaliknya.

---

# 40. Integrity Verification

## Overview

Seluruh data penting diverifikasi untuk menjaga konsistensi database.

### Verification

* Foreign Key Validation
* Schema Validation
* Constraint Validation
* Checksum Verification (jika digunakan)
* Database Integrity Check

---

## Automatic Verification

Dilakukan saat:

* Startup aplikasi.
* Setelah proses migration.
* Setelah restore database.
* Setelah recovery database.

---

## Recovery

Jika ditemukan kerusakan:

* Pulihkan dari backup terbaru.
* Perbaiki indeks bila memungkinkan.
* Jalankan Integrity Check ulang.
* Catat hasil ke Logging System.

Jika pemulihan gagal, pengguna diberi notifikasi beserta langkah yang dapat dilakukan.

---

# Security Policies

* Tidak ada query dinamis tanpa parameter.
* Seluruh transaksi menggunakan mekanisme Commit/Rollback.
* Data yang rusak tidak boleh diproses lebih lanjut.
* Seluruh kesalahan keamanan dicatat ke Logging System.
* Database harus tetap dapat dipulihkan setelah kegagalan sistem.

---

# Security Checklist

* Prepared Statement digunakan.
* Foreign Key aktif.
* WAL Mode aktif.
* Backup tersedia.
* Integrity Check berhasil.
* Input tervalidasi.
* Data sensitif tidak disimpan tanpa persetujuan.
* Recovery telah diuji.

---

# Acceptance Criteria

* Database hanya dapat diakses melalui Repository Layer.
* Seluruh query aman dari SQL Injection.
* Data sensitif tidak disimpan secara permanen tanpa persetujuan pengguna.
* Integrity Check berhasil dijalankan tanpa error.
* Database dapat dipulihkan dari backup yang valid.
* Seluruh kebijakan keamanan diterapkan secara konsisten pada semua operasi database.
# PART VI — Testing

---

# 41. Database Testing

## Overview

Database Testing memastikan seluruh struktur, relasi, query, dan operasi database berjalan dengan benar, aman, dan konsisten.

### Test Categories

* Schema Test
* CRUD Test
* Relationship Test
* Constraint Test
* Transaction Test
* Migration Test
* Backup & Restore Test
* Recovery Test
* Security Test

---

## Schema Testing

Memastikan:

* Seluruh tabel berhasil dibuat.
* Seluruh kolom sesuai spesifikasi.
* Primary Key valid.
* Foreign Key aktif.
* Index berhasil dibuat.
* Constraint berfungsi dengan benar.

---

## CRUD Testing

Menguji operasi:

* Create
* Read
* Update
* Delete
* Search
* Filter
* Pagination

Seluruh operasi harus menghasilkan data yang konsisten.

---

## Relationship Testing

Memastikan:

* Foreign Key berjalan.
* Relasi antar tabel valid.
* Tidak ada orphan record.
* Cascade Rule bekerja sesuai desain.

---

## Transaction Testing

Skenario:

* Commit berhasil.
* Rollback berhasil.
* Kegagalan transaksi tidak merusak data.
* Operasi multi-tabel tetap konsisten.

---

## Migration Testing

Memastikan:

* Database lama dapat dimigrasikan.
* Data lama tetap utuh.
* Versi database diperbarui dengan benar.
* Rollback tersedia jika migrasi gagal.

---

## Backup & Recovery Testing

Menguji:

* Manual Backup
* Automatic Backup
* Restore Database
* Restore Settings
* Restore History
* Recovery setelah crash

---

# 42. Performance Testing

## Performance Targets

| Operation     |   Target |
| ------------- | -------: |
| Database Open | < 100 ms |
| Insert        |  < 20 ms |
| Update        |  < 20 ms |
| Delete        |  < 20 ms |
| Search        | < 100 ms |
| Backup        |    < 5 s |
| Restore       |   < 10 s |

---

## Stress Testing

Pengujian dengan:

* 10.000 Download
* 100.000 History
* 1.000.000 Log
* 100 Download Aktif
* Queue Besar
* Database > 1 GB

Database harus tetap stabil dan responsif.

---

## Long Running Test

Pengujian penggunaan selama 24 jam atau lebih untuk memastikan:

* Tidak ada memory leak.
* Tidak ada database corruption.
* Performa tetap stabil.

---

# 43. Acceptance Criteria

Database dianggap siap digunakan apabila:

* Seluruh tabel berhasil dibuat.
* CRUD berjalan tanpa kesalahan.
* Relasi dan Foreign Key tervalidasi.
* Migration berhasil tanpa kehilangan data.
* Backup dan Restore berfungsi.
* Integrity Check berhasil.
* Target performa tercapai.
* Tidak ditemukan data corruption pada pengujian.
* Database tetap stabil pada pengujian beban tinggi.

---

# 44. Appendix

## Database Version

* Database Engine : SQLite
* Versioning : Schema Version
* Migration : Automatic

---

## Default Database Files

```text
vdm.db
vdm.db-wal
vdm.db-shm
```

---

## Maintenance Commands

* Integrity Check
* VACUUM
* ANALYZE
* Backup
* Restore
* Optimize Database

---

## Related Documents

* 01-PRD.md
* 02-SDS.md
* 04-API.md
* 05-Development-Guide.md

---

# End of Database Design Specification

Dokumen ini menjadi referensi utama implementasi database VDM dan menjadi dasar pengembangan Repository Layer, Migration System, Backup & Recovery, serta optimasi performa SQLite.
