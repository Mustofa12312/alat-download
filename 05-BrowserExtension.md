# Velocity Download Manager (VDM)

# Browser Extension Specification

**Document ID:** VDM-BEX-001
**Version:** 1.0.0
**Status:** Draft
**Confidentiality:** Internal Use Only

---

# 1. Cover

| Item               | Information                                  |
| ------------------ | -------------------------------------------- |
| Project            | Velocity Download Manager (VDM)              |
| Document           | Browser Extension Specification              |
| Document ID        | VDM-BEX-001                                  |
| Version            | 1.0.0                                        |
| Supported Browsers | Chrome, Edge, Firefox, Brave, Opera, Vivaldi |
| Communication      | Native Messaging                             |
| Data Format        | JSON                                         |
| Author             | Mustofa & ChatGPT                            |
| Last Updated       | July 2026                                    |

---

# 2. Document Information

## Purpose

Dokumen ini mendefinisikan arsitektur, implementasi, komunikasi, keamanan, dan pengujian Browser Extension yang digunakan oleh Velocity Download Manager (VDM).

## Objectives

* Menangkap download dari browser.
* Mengirim URL ke aplikasi desktop.
* Mendeteksi video dan media.
* Mendukung Native Messaging.
* Menjamin komunikasi yang aman dan cepat.

## Scope

* Browser Extension Architecture
* Download Capture
* Video Detection
* Native Messaging
* Communication Protocol
* Security
* Performance
* Testing

## Target Audience

* Browser Extension Developer
* Rust Developer
* Frontend Developer
* QA Engineer
* Software Architect

## Related Documents

* 01-PRD.md
* 02-SDS.md
* 03-Database.md
* 04-API.md

---

# 3. Revision History

| Version | Date      | Author            | Description                             |
| ------- | --------- | ----------------- | --------------------------------------- |
| 0.1.0   | July 2026 | Mustofa & ChatGPT | Initial Browser Extension Specification |
| 1.0.0   | TBD       | Development Team  | Production Release                      |

---

# 4. Table of Contents

## PART I — Overview

5. Introduction
6. Browser Architecture
7. Design Principles
8. Supported Browsers

---

## PART II — Extension Core

9. Extension Structure
10. Background Service Worker
11. Content Scripts
12. Download Capture Engine
13. Video Detection Engine
14. Context Menu
15. Popup Interface

---

## PART III — Communication

16. Native Messaging
17. IPC Protocol
18. JSON Message Schema
19. Event Flow
20. Error Handling

---

## PART IV — Browser Integration

21. Chrome Integration
22. Edge Integration
23. Firefox Integration
24. Brave Integration
25. Opera Integration
26. Vivaldi Integration

---

## PART V — Security

27. Permissions
28. Privacy Policy
29. Validation Rules
30. Secure Communication

---

## PART VI — Performance

31. Startup Performance
32. Memory Usage
33. CPU Optimization
34. Performance Targets

---

## PART VII — Testing

35. Unit Testing
36. Integration Testing
37. Browser Compatibility Testing
38. Performance Testing
39. Acceptance Criteria
40. Appendix
# PART I — Overview

---

# 5. Introduction

## Overview

Browser Extension merupakan komponen yang menghubungkan browser dengan Velocity Download Manager (VDM). Extension bertugas mendeteksi download, video, audio, dan media lainnya, kemudian meneruskan informasi tersebut ke aplikasi desktop melalui **Native Messaging**.

Extension tidak melakukan proses download secara langsung. Seluruh proses download dijalankan oleh **Rust Download Engine**.

---

# 6. Browser Architecture

## Architecture

```text
User
 │
 ▼
Browser
 │
 ▼
Browser Extension
 │
 ▼
Native Messaging
 │
 ▼
VDM Desktop
 │
 ▼
Rust Backend
 │
 ▼
Download Engine
```

---

## Main Components

* Background Service Worker
* Content Script
* Popup UI
* Context Menu
* Native Messaging Client
* Download Detector
* Video Detector

Masing-masing modul memiliki tanggung jawab yang terpisah.

---

# 7. Design Principles

## Core Principles

* Lightweight
* Fast
* Secure
* Modular
* Event Driven
* Privacy First
* Cross Browser

---

## Responsibilities

Browser Extension hanya bertugas untuk:

* Mendeteksi download.
* Mengambil metadata file.
* Mengirim URL ke aplikasi.
* Menampilkan status singkat.

Browser Extension **tidak bertugas**:

* Menyimpan file.
* Mengelola Queue.
* Mengatur Scheduler.
* Menjalankan Download Engine.

---

# 8. Supported Browsers

## Chromium Based

* Google Chrome
* Microsoft Edge
* Brave
* Opera
* Vivaldi

---

## Gecko Based

* Mozilla Firefox

---

## Minimum Browser Version

Extension harus mendukung versi browser yang masih mendapatkan pembaruan keamanan dari pengembang browser masing-masing.

---

## Supported Features

* Download Capture
* Video Detection
* Audio Detection
* Context Menu
* Native Messaging
* Multiple Downloads
* Resume Support
* File Information Detection

---

## Compatibility Rules

* Menggunakan standar **WebExtension API**.
* Menggunakan satu basis kode untuk browser Chromium dan Firefox sejauh memungkinkan.
* Perbedaan implementasi browser diisolasi dalam lapisan adapter.

---

# Extension Lifecycle

```text
Browser Started
        │
        ▼
Extension Loaded
        │
        ▼
Native Messaging Connected
        │
        ▼
Waiting for Download
        │
        ▼
Capture Request
        │
        ▼
Send to VDM
        │
        ▼
Receive Status
```

---

# Acceptance Criteria

* Extension berhasil dimuat pada seluruh browser yang didukung.
* Native Messaging dapat terhubung dengan aplikasi VDM.
* Download dapat dideteksi secara otomatis.
* Arsitektur modular dan mudah dikembangkan.
* Browser Extension tetap ringan dan tidak mengganggu performa browser.
# PART II — Extension Core

---

# 9. Extension Structure

## Overview

Browser Extension dibangun menggunakan arsitektur modular agar mudah dikembangkan dan dipelihara.

### Structure

```text
extension/
├── manifest.json
├── background/
├── content/
├── popup/
├── options/
├── context-menu/
├── messaging/
├── services/
├── utils/
├── assets/
└── shared/
```

Setiap modul memiliki tanggung jawab yang jelas.

---

# 10. Background Service Worker

## Purpose

Service Worker menjadi pusat logika Browser Extension.

### Responsibilities

* Initialize Extension
* Native Messaging
* Download Listener
* Event Dispatcher
* Update Checker
* Error Handler

Service Worker aktif hanya saat diperlukan untuk menghemat memori.

---

# 11. Content Scripts

## Purpose

Content Script berinteraksi langsung dengan halaman web.

### Responsibilities

* Detect Video
* Detect Audio
* Read Metadata
* Detect Dynamic Content
* Capture Media URL

Content Script tidak boleh mengubah halaman web tanpa kebutuhan yang jelas.

---

# 12. Download Capture Engine

## Purpose

Menangkap seluruh permintaan download dari browser.

### Features

* HTTP Download
* HTTPS Download
* FTP Download
* Multiple Download
* Batch Download
* File Metadata Detection

### Captured Data

* URL
* File Name
* File Size
* MIME Type
* Referer
* User-Agent

---

# 13. Video Detection Engine

## Purpose

Mendeteksi media yang dapat diunduh.

### Supported Media

* Video
* Audio
* Live Stream (Future)
* Playlist
* Subtitle
* Thumbnail

### Detection Sources

* HTML5 Video
* Media Requests
* Streaming Manifest
* Dynamic Page Content

---

# 14. Context Menu

## Purpose

Menambahkan menu VDM pada browser.

### Menu Items

* Download with VDM
* Download Video
* Download Audio
* Copy Download URL
* Open VDM

Context Menu muncul sesuai konteks halaman.

---

# 15. Popup Interface

## Purpose

Popup menyediakan akses cepat ke status Browser Extension.

### Information

* Connection Status
* Browser Version
* Extension Version
* VDM Status
* Active Downloads

### Actions

* Open VDM
* Retry Connection
* Open Settings
* View Logs

Popup harus ringan dan terbuka dalam waktu kurang dari 200 ms.

---

# Extension Workflow

```text
User Click Download
        │
        ▼
Download Capture Engine
        │
        ▼
Video Detection (Optional)
        │
        ▼
Background Service Worker
        │
        ▼
Native Messaging
        │
        ▼
VDM Desktop
```

---

# Acceptance Criteria

* Struktur extension modular.
* Service Worker menangani seluruh logika utama.
* Content Script hanya mengakses data yang diperlukan.
* Download Capture berhasil mendeteksi download yang didukung.
* Video Detection mengenali media yang kompatibel.
* Context Menu muncul pada kondisi yang sesuai.
* Popup menampilkan status extension dan VDM secara akurat.
# PART III — Communication

---

# 16. Native Messaging

## Overview

Native Messaging menjadi jalur komunikasi utama antara Browser Extension dan aplikasi VDM Desktop.

Komunikasi harus:

* Asynchronous
* Fast
* Secure
* Reliable
* JSON Based

---

## Responsibilities

* Connect to VDM
* Send Download Request
* Receive Download Status
* Exchange Metadata
* Handle Connection State

---

# 17. IPC Protocol

## Purpose

IPC Protocol mendefinisikan format komunikasi antara Browser Extension dan Backend VDM.

### Commands

* Connect
* Disconnect
* StartDownload
* PauseDownload
* ResumeDownload
* CancelDownload
* GetStatus
* GetVersion
* Ping

---

### Responses

* Connected
* Accepted
* Progress
* Completed
* Failed
* Error
* Pong

---

# 18. JSON Message Schema

## Request

Setiap request minimal berisi:

* requestId
* command
* payload
* timestamp

---

## Response

Setiap response minimal berisi:

* requestId
* success
* data
* message
* timestamp

---

## Payload

Payload dapat berisi:

* URL
* File Name
* File Size
* MIME Type
* Referer
* User-Agent
* Cookie (jika diperlukan)

---

# 19. Event Flow

## Communication Flow

```text id="wq4tfc"
Browser
     │
     ▼
Browser Extension
     │
     ▼
Native Messaging
     │
     ▼
VDM Desktop
     │
     ▼
Rust Backend
     │
     ▼
Download Engine
     │
     ▼
Progress Event
     │
     ▼
Browser Extension
```

---

## Event Types

### Connection

* Connected
* Disconnected

### Download

* DownloadCaptured
* DownloadStarted
* DownloadProgress
* DownloadCompleted
* DownloadFailed

### System

* VersionChanged
* SettingsUpdated
* ExtensionUpdated

---

# 20. Error Handling

## Error Categories

* Connection Error
* Validation Error
* Timeout Error
* IPC Error
* Native Messaging Error
* Unsupported Browser
* Unsupported URL

---

## Recovery Strategy

Jika terjadi kesalahan:

* Retry Connection
* Reconnect Native Messaging
* Resend Request
* Log Error
* Notify User

---

## Timeout

Jika backend tidak merespons dalam batas waktu:

* Request dibatalkan.
* Error dicatat.
* Pengguna menerima notifikasi.
* Extension mencoba menyambung kembali jika memungkinkan.

---

# Communication Rules

* Seluruh komunikasi menggunakan JSON.
* Semua request memiliki `requestId`.
* Setiap request harus memiliki response.
* Event dikirim secara asynchronous.
* Payload harus tervalidasi sebelum diproses.
* Komunikasi tidak boleh memblokir browser.

---

# Acceptance Criteria

* Native Messaging terhubung dengan VDM secara stabil.
* IPC Protocol berjalan sesuai spesifikasi.
* JSON Message Schema konsisten.
* Event dikirim dan diterima tanpa kehilangan data.
* Error ditangani dengan mekanisme recovery yang sesuai.
* Browser tetap responsif selama komunikasi berlangsung.
# PART IV — Browser Integration

---

# 21. Chrome Integration

## Overview

Integrasi dengan Google Chrome menggunakan **WebExtension API** dan **Manifest V3**.

### Features

* Download Capture
* Video Detection
* Context Menu
* Native Messaging
* Popup Interface

### Requirements

* Chrome versi yang masih didukung.
* Native Messaging Host terpasang.
* Extension aktif.

---

# 22. Edge Integration

## Overview

Menggunakan implementasi yang sama dengan Chromium.

### Features

* Download Capture
* Video Detection
* Native Messaging
* Context Menu

### Compatibility

Sebagian besar kode identik dengan Chrome.

---

# 23. Firefox Integration

## Overview

Menggunakan **WebExtension API** yang kompatibel dengan Firefox.

### Features

* Download Capture
* Video Detection
* Popup
* Native Messaging

### Notes

Beberapa API memiliki implementasi berbeda dibanding Chromium dan ditangani melalui lapisan adapter.

---

# 24. Brave Integration

## Overview

Brave berbasis Chromium sehingga menggunakan implementasi yang sama.

### Supported

* Download Capture
* Video Detection
* Context Menu
* Native Messaging

---

# 25. Opera Integration

## Overview

Opera menggunakan basis Chromium.

### Supported

* Download Capture
* Media Detection
* Native Messaging
* Popup Interface

---

# 26. Vivaldi Integration

## Overview

Vivaldi menggunakan basis Chromium.

### Supported

* Download Capture
* Video Detection
* Context Menu
* Native Messaging

---

# Browser Compatibility Matrix

| Feature          | Chrome | Edge | Firefox | Brave | Opera | Vivaldi |
| ---------------- | :----: | :--: | :-----: | :---: | :---: | :-----: |
| Download Capture |    ✓   |   ✓  |    ✓    |   ✓   |   ✓   |    ✓    |
| Video Detection  |    ✓   |   ✓  |    ✓    |   ✓   |   ✓   |    ✓    |
| Native Messaging |    ✓   |   ✓  |    ✓    |   ✓   |   ✓   |    ✓    |
| Context Menu     |    ✓   |   ✓  |    ✓    |   ✓   |   ✓   |    ✓    |
| Popup UI         |    ✓   |   ✓  |    ✓    |   ✓   |   ✓   |    ✓    |

---

# Browser Detection Flow

```text id="o5n1ru"
Browser Started
        │
        ▼
Extension Loaded
        │
        ▼
Browser Detection
        │
        ▼
Compatibility Check
        │
        ▼
Native Messaging
        │
        ▼
Ready
```

---

# Integration Rules

* Menggunakan satu codebase untuk seluruh browser sejauh memungkinkan.
* Perbedaan implementasi ditangani melalui Browser Adapter.
* Extension harus tetap kompatibel dengan pembaruan browser yang didukung.
* Native Messaging harus diverifikasi sebelum digunakan.
* Semua browser menggunakan format komunikasi yang sama.

---

# Acceptance Criteria

* Extension berhasil berjalan pada seluruh browser yang didukung.
* Download Capture berfungsi secara konsisten.
* Native Messaging aktif pada setiap browser.
* Browser Adapter menangani perbedaan implementasi tanpa memengaruhi fitur inti.
* Pengalaman pengguna tetap konsisten di seluruh browser.
# PART V — Security

---

# 27. Permissions

## Overview

Browser Extension hanya meminta izin (permissions) yang benar-benar diperlukan agar tetap aman dan menghormati privasi pengguna.

### Required Permissions

* Downloads
* Context Menus
* Storage
* Native Messaging
* Active Tab

### Optional Permissions

* Notifications
* Clipboard (Future)

### Rules

* Mengikuti prinsip **Least Privilege**.
* Tidak meminta izin yang tidak digunakan.
* Permission tambahan hanya diminta saat benar-benar diperlukan.

---

# 28. Privacy Policy

## Privacy Principles

* Privacy First
* Local First
* Minimal Data Collection
* User Control

### Data Handling

VDM tidak:

* Mengirim riwayat browsing ke server.
* Mengumpulkan telemetry secara default.
* Melacak aktivitas pengguna.
* Menjual atau membagikan data pengguna.

### Stored Data

Hanya data yang diperlukan untuk menjalankan fitur extension, seperti:

* Konfigurasi extension.
* Preferensi pengguna.
* Status koneksi.

---

# 29. Validation Rules

## Overview

Seluruh data yang diterima extension harus divalidasi sebelum diteruskan ke aplikasi.

### Validation

* URL Validation
* Protocol Validation
* File Name Validation
* MIME Type Validation
* JSON Schema Validation
* Native Messaging Validation

### Rules

* Tolak URL yang tidak valid.
* Tolak payload yang tidak sesuai skema.
* Validasi ukuran dan tipe data.
* Cegah data duplikat yang tidak diperlukan.

---

# 30. Secure Communication

## Communication Security

Komunikasi antara Browser Extension dan VDM menggunakan:

* Native Messaging
* JSON Message Schema
* Request Validation
* Response Validation

### Security Rules

* Semua request memiliki `requestId`.
* Semua response tervalidasi.
* Hanya aplikasi VDM resmi yang dapat menerima komunikasi.
* Tidak ada data sensitif yang ditampilkan di log.

### Sensitive Data

Jika diperlukan untuk proses download (misalnya pada situs yang memerlukan autentikasi), data berikut hanya digunakan selama sesi aktif dan tidak disimpan secara permanen tanpa persetujuan pengguna:

* Cookie
* Access Token
* Authorization Header
* Session Data

---

# Security Workflow

```text id="pw8vka"
Browser
    │
    ▼
Permission Check
    │
    ▼
Input Validation
    │
    ▼
Native Messaging
    │
    ▼
VDM Verification
    │
    ▼
Download Engine
```

---

# Security Checklist

* Permission sesuai kebutuhan.
* URL tervalidasi.
* JSON tervalidasi.
* Native Messaging aman.
* Data sensitif tidak disimpan permanen.
* Logging tidak menyimpan informasi rahasia.
* Privacy Policy diterapkan.

---

# Acceptance Criteria

* Extension hanya menggunakan permission yang diperlukan.
* Semua data tervalidasi sebelum diproses.
* Native Messaging aman dan hanya terhubung ke aplikasi resmi.
* Data sensitif diproses sesuai kebijakan privasi.
* Tidak ada akses yang tidak sah ke Browser Extension maupun VDM Desktop.
# PART VI — Performance

---

# 31. Startup Performance

## Overview

Browser Extension harus aktif dengan cepat tanpa memperlambat proses startup browser.

### Performance Targets

| Metric                      |   Target |
| --------------------------- | -------: |
| Extension Load Time         | < 200 ms |
| Service Worker Startup      | < 100 ms |
| Native Messaging Connection | < 300 ms |
| Popup Open Time             | < 200 ms |

### Strategy

* Lazy Loading
* Service Worker
* Background Initialization
* Deferred Loading

---

# 32. Memory Usage

## Overview

Extension harus menggunakan memori seminimal mungkin.

### Memory Targets

| State           |   Target |
| --------------- | -------: |
| Idle            |  < 20 MB |
| Active          |  < 50 MB |
| Heavy Detection | < 100 MB |

### Optimization

* Automatic Garbage Collection
* Cache Cleanup
* Lazy Object Creation
* Shared Resources
* Release Unused Memory

---

# 33. CPU Optimization

## CPU Targets

| State           | Target |
| --------------- | -----: |
| Idle            |   < 1% |
| Active          |   < 5% |
| Heavy Detection |  < 15% |

### Optimization

* Event-Driven Processing
* Debouncing
* Throttling
* Efficient DOM Scanning
* Background Processing

Content Script hanya aktif ketika diperlukan.

---

# 34. Performance Targets

## Functional Targets

| Feature            |   Target |
| ------------------ | -------: |
| Download Detection | < 100 ms |
| Video Detection    | < 200 ms |
| URL Validation     |  < 20 ms |
| Native Messaging   | < 100 ms |
| Context Menu       | < 100 ms |
| Popup Refresh      | < 100 ms |

---

## Performance Monitoring

Extension memantau:

* Startup Time
* Memory Usage
* CPU Usage
* Communication Latency
* Detection Time
* Error Rate

Data digunakan untuk debugging dan optimasi, bukan untuk pelacakan pengguna.

---

## Performance Rules

* Tidak menghambat proses browsing.
* Tidak memindai halaman secara terus-menerus.
* Tidak menjalankan tugas berat di Main Thread.
* Menggunakan cache secara efisien.
* Menutup koneksi yang tidak lagi digunakan.

---

## Performance Flow

```text id="1puzpo"
Browser Start
      │
      ▼
Extension Load
      │
      ▼
Service Worker
      │
      ▼
Idle Mode
      │
      ▼
User Action
      │
      ▼
Detection
      │
      ▼
Native Messaging
      │
      ▼
Return to Idle
```

---

# Acceptance Criteria

* Extension dimuat sesuai target waktu startup.
* Penggunaan RAM dan CPU berada dalam batas yang ditentukan.
* Download dan video terdeteksi dengan cepat.
* Native Messaging memiliki latensi rendah.
* Browser tetap responsif selama extension aktif.
* Tidak ditemukan memory leak pada penggunaan jangka panjang.
# PART VII — Testing

---

# 35. Unit Testing

## Overview

Unit Testing memastikan setiap modul Browser Extension bekerja secara mandiri sesuai spesifikasi.

### Modules

* Download Capture Engine
* Video Detection Engine
* Native Messaging
* Context Menu
* Popup UI
* Content Script
* Service Worker
* Settings Manager
* Validation Module

### Objectives

* Memastikan logika setiap modul benar.
* Mengurangi bug saat pengembangan.
* Mempermudah proses refactoring.

---

# 36. Integration Testing

## Overview

Integration Testing memastikan komunikasi antar komponen berjalan dengan baik.

### Test Scenarios

* Extension ↔ Native Messaging
* Native Messaging ↔ VDM Desktop
* Content Script ↔ Service Worker
* Popup ↔ Background Service
* Browser ↔ Download Capture
* Browser ↔ Video Detection

### Validation

* JSON Schema
* IPC Message
* Error Handling
* Connection Recovery

---

# 37. Browser Compatibility Testing

## Supported Browsers

* Google Chrome
* Microsoft Edge
* Mozilla Firefox
* Brave
* Opera
* Vivaldi

### Compatibility Tests

* Installation
* Startup
* Download Capture
* Video Detection
* Native Messaging
* Popup UI
* Context Menu
* Extension Update

Seluruh fitur utama harus berjalan konsisten di semua browser yang didukung.

---

# 38. Performance Testing

## Performance Tests

* Startup Time
* Memory Usage
* CPU Usage
* URL Detection Speed
* Video Detection Speed
* Native Messaging Latency

### Stress Tests

* Banyak tab aktif.
* Banyak download bersamaan.
* Halaman dengan media dalam jumlah besar.
* Penggunaan browser dalam waktu lama.

Target utama adalah menjaga browser tetap responsif tanpa penurunan performa yang berarti.

---

# 39. Acceptance Criteria

Browser Extension dinyatakan siap digunakan apabila:

* Seluruh Unit Test lulus.
* Seluruh Integration Test lulus.
* Seluruh Browser Compatibility Test lulus.
* Native Messaging stabil.
* Download Capture berjalan dengan benar.
* Video Detection bekerja sesuai spesifikasi.
* Target performa RAM, CPU, dan startup tercapai.
* Tidak ditemukan memory leak atau crash selama pengujian.

---

# 40. Appendix

## Supported Standards

* WebExtension API
* Manifest V3
* Native Messaging
* JSON Schema

---

## Development Stack

* TypeScript
* WebExtension API
* Native Messaging
* Vite
* ESLint

---

## Related Documents

* 01-PRD.md
* 02-SDS.md
* 03-Database.md
* 04-API.md

---

# Browser Extension Lifecycle

```text id="mjlwmg"
Install
   │
Load
   │
Initialize
   │
Idle
   │
User Action
   │
Capture
   │
Send to VDM
   │
Receive Status
   │
Idle
```

---

# End of Browser Extension Specification

Dokumen ini menjadi referensi utama untuk pengembangan Browser Extension VDM, mencakup arsitektur, komunikasi, keamanan, performa, dan pengujian sehingga implementasi dapat dilakukan secara konsisten pada seluruh browser yang didukung.
