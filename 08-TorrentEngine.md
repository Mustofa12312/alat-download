# Velocity Download Manager (VDM)

# Torrent Engine Specification

**Document ID:** VDM-TE-001
**Version:** 1.0.0
**Status:** Draft
**Confidentiality:** Internal Use Only

---

# 1. Cover

# 2. Document Information

# 3. Revision History

# 4. Table of Contents

---

# PART I — Overview

5. Introduction

6. Torrent Engine Architecture

7. Design Principles

8. Torrent Lifecycle

---

# PART II — Core Engine

9. Torrent Parser

10. Magnet Link Parser

11. Tracker Manager

12. DHT Engine

13. Peer Manager

14. Piece Manager

15. Download Worker

16. Upload Worker

17. Verification Engine

18. Resume Engine

---

# PART III — BitTorrent Protocol

19. BitTorrent Protocol

20. Peer Wire Protocol

21. Tracker Protocol

22. DHT Protocol

23. Magnet Link Support

24. Metadata Exchange

25. Peer Discovery

26. Encryption Support

---

# PART IV — Performance

27. Peer Optimization

28. Piece Selection Strategy

29. Bandwidth Manager

30. Upload Optimizer

31. Download Optimizer

32. Disk Optimization

33. Memory Optimization

34. Network Optimization

---

# PART V — Torrent Intelligence

35. Smart Peer Selection

36. Intelligent Piece Picker

37. Seeder/Leecher Analyzer

38. Swarm Optimizer

39. Duplicate Torrent Detection

40. Self-Healing Engine

41. Health Monitor

42. Torrent Statistics

---

# PART VI — Integration

43. Download Engine Integration

44. Queue Integration

45. Scheduler Integration

46. Browser Integration

47. Notification Integration

48. Plugin Support (Future)

---

# PART VII — Security

49. Torrent Validation

50. Magnet Validation

51. Hash Verification

52. Peer Validation

53. Secure Connections

54. Integrity Protection

---

# PART VIII — Monitoring

55. Swarm Statistics

56. Performance Metrics

57. Logging

58. Diagnostics

59. Crash Recovery

---

# PART IX — Testing

60. Unit Testing

61. Integration Testing

62. Stress Testing

63. Benchmark

64. Acceptance Criteria

65. Appendix

# PART I — Overview

---

# 5. Introduction

## Overview

Torrent Engine merupakan modul yang menangani seluruh proses download dan upload menggunakan protokol BitTorrent. Engine ini bekerja secara independen namun terintegrasi dengan Download Engine, Queue Manager, Scheduler, Logging System, Notification System, dan Database.

Torrent Engine dirancang untuk memberikan performa tinggi, stabilitas, serta kemampuan pemulihan otomatis pada proses download torrent.

---

## Objectives

- Mendukung file `.torrent` dan Magnet Link.
- Mengoptimalkan kecepatan download dan upload.
- Mengelola Peer dan Swarm secara efisien.
- Memastikan integritas file melalui verifikasi hash.
- Mendukung Resume dan Crash Recovery.
- Menggunakan sumber daya sistem secara efisien.

---

# 6. Torrent Engine Architecture

## Architecture

```text id="krx5aj"
Torrent File / Magnet Link
           │
           ▼
Torrent Parser
           │
           ▼
Metadata Loader
           │
           ▼
Tracker Manager
           │
           ▼
DHT Engine
           │
           ▼
Peer Manager
           │
           ▼
Piece Manager
           │
           ▼
Download / Upload Workers
           │
           ▼
Verification Engine
           │
           ▼
Completed
```

---

## Main Components

- Torrent Parser
- Magnet Parser
- Tracker Manager
- DHT Engine
- Peer Manager
- Piece Manager
- Download Worker
- Upload Worker
- Verification Engine
- Resume Engine

Seluruh modul berkomunikasi melalui Event Bus.

---

# 7. Design Principles

## Core Principles

- Performance First
- Scalability
- Reliability
- Fault Tolerance
- Security
- Resource Efficiency
- Modular Architecture

---

## Engine Principles

- Asynchronous Processing
- Multi Peer Download
- Dynamic Peer Management
- Intelligent Piece Selection
- Automatic Recovery
- Shared Infrastructure dengan Download Engine

---

## Supported Features

- Torrent File
- Magnet Link
- Resume Download
- Multi File Torrent
- Selective Download
- Sequential Download
- Automatic Seeding

---

# 8. Torrent Lifecycle

## Standard Flow

```text id="epxagv"
Open Torrent
      │
      ▼
Parse Metadata
      │
      ▼
Tracker & DHT Discovery
      │
      ▼
Peer Discovery
      │
      ▼
Piece Allocation
      │
      ▼
Download Pieces
      │
      ▼
Hash Verification
      │
      ▼
Complete Download
      │
      ▼
Seeding
```

---

## Torrent States

- Pending
- Loading Metadata
- Discovering Peers
- Connecting
- Downloading
- Verifying
- Seeding
- Paused
- Completed
- Failed

---

## Engine Responsibilities

Torrent Engine bertanggung jawab untuk:

- Membaca file torrent.
- Memproses Magnet Link.
- Menemukan tracker dan peer.
- Mengelola piece.
- Mengelola upload dan download.
- Memverifikasi hash.
- Menyimpan status resume.
- Mengirim statistik secara real-time.

---

# Performance Goals

| Metric            |             Target |
| ----------------- | -----------------: |
| Metadata Load     |           < 200 ms |
| Magnet Parsing    |           < 100 ms |
| Peer Discovery    |              < 2 s |
| Hash Verification | < 500 ms per piece |
| Resume Detection  |           < 100 ms |
| CPU (Idle)        |               < 2% |
| Memory (Idle)     |           < 150 MB |

---

# Torrent Workflow

```text id="ocpfrx"
Torrent
    │
    ▼
Parser
    │
    ▼
Tracker + DHT
    │
    ▼
Peer Manager
    │
    ▼
Piece Manager
    │
    ▼
Workers
    │
    ▼
Verification
    │
    ▼
Completed / Seeding
```

---

# Acceptance Criteria

- Torrent Engine mendukung file `.torrent` dan Magnet Link.
- Metadata berhasil diproses dengan benar.
- Peer ditemukan melalui Tracker maupun DHT.
- Download dan upload berjalan secara paralel.
- Piece diverifikasi sebelum dinyatakan selesai.
- Resume dan Seeding bekerja secara otomatis.
- Engine tetap stabil pada torrent dengan banyak peer dan file berukuran besar.

# PART II — Core Engine

---

# 9. Torrent Parser

## Overview

Torrent Parser membaca dan memvalidasi file `.torrent`.

### Responsibilities

- Bencode Parsing
- Metadata Validation
- File List Extraction
- Piece Information
- Tracker Extraction

### Output

- Torrent Name
- Total Size
- Piece Length
- Piece Hashes
- File Structure
- Tracker List

---

# 10. Magnet Link Parser

## Overview

Magnet Parser memproses Magnet Link menjadi metadata torrent.

### Responsibilities

- URI Parsing
- Info Hash Extraction
- Tracker Extraction
- Display Name Extraction
- Parameter Validation

### Supported Parameters

- xt
- dn
- tr
- ws (Future)
- xl (Future)

---

# 11. Tracker Manager

## Overview

Tracker Manager berkomunikasi dengan tracker BitTorrent.

### Responsibilities

- Tracker Connection
- Peer Request
- Announce
- Tracker Rotation
- Retry Failed Tracker

### Features

- HTTP Tracker
- HTTPS Tracker
- UDP Tracker
- Multi Tracker Support

---

# 12. DHT Engine

## Overview

DHT Engine menemukan peer tanpa bergantung pada tracker.

### Responsibilities

- Bootstrap
- Node Discovery
- Peer Discovery
- Routing Table
- Node Maintenance

### Features

- Automatic Bootstrap
- Node Cache
- Distributed Lookup

---

# 13. Peer Manager

## Overview

Peer Manager mengelola seluruh peer yang terhubung.

### Responsibilities

- Peer Discovery
- Peer Ranking
- Connection Management
- Peer Health Monitoring
- Peer Selection

### Features

- Dynamic Peer Pool
- Fast Peer Detection
- Peer Filtering
- Automatic Reconnection

---

# 14. Piece Manager

## Overview

Piece Manager mengatur seluruh piece torrent.

### Responsibilities

- Piece Allocation
- Piece Scheduling
- Piece Verification
- Piece Recovery
- Piece Completion

### Features

- Dynamic Piece Assignment
- Sequential Mode
- Random Mode
- Rarest First
- End Game Mode

---

# 15. Download Worker

## Overview

Download Worker mengunduh piece dari peer.

### Responsibilities

- Request Piece
- Receive Block
- Verify Block
- Report Progress
- Retry Failed Block

### Features

- Parallel Download
- Worker Scaling
- Adaptive Scheduling

---

# 16. Upload Worker

## Overview

Upload Worker membagikan piece kepada peer lain.

### Responsibilities

- Upload Piece
- Upload Queue
- Bandwidth Control
- Peer Priority
- Upload Statistics

### Features

- Dynamic Upload Slot
- Upload Scheduling
- Fair Sharing

---

# 17. Verification Engine

## Overview

Verification Engine memastikan seluruh piece valid.

### Verification

- SHA-1 Piece Hash
- Piece Integrity
- File Integrity
- Resume Validation

### Result

- Valid
- Invalid
- Corrupted

---

# 18. Resume Engine

## Overview

Resume Engine menyimpan dan memulihkan status torrent.

### Responsibilities

- Save Session
- Load Session
- Restore Pieces
- Restore Peer State
- Restore Queue

### Features

- Automatic Resume
- Crash Recovery
- Session Recovery
- Fast Resume

---

# Core Engine Workflow

```text id="4vjlwm"
Torrent
   │
   ▼
Parser
   │
   ▼
Tracker + DHT
   │
   ▼
Peer Manager
   │
   ▼
Piece Manager
   │
   ▼
Download Worker
   │
   ▼
Verification
   │
   ▼
Resume Database
   │
   ▼
Completed / Seeding
```

---

# Core Engine Rules

- Semua modul bekerja secara asynchronous.
- Peer dikelola secara dinamis.
- Piece diverifikasi sebelum ditulis sebagai selesai.
- Resume disimpan secara berkala.
- Worker dapat ditambah atau dikurangi secara otomatis.
- Seluruh komunikasi antar modul menggunakan Event Bus.

---

# Acceptance Criteria

- File `.torrent` dan Magnet Link berhasil diproses.
- Tracker dan DHT menemukan peer yang tersedia.
- Piece dialokasikan dan diverifikasi dengan benar.
- Download dan upload berjalan secara paralel.
- Resume memulihkan sesi tanpa kehilangan progres.
- Torrent Engine tetap stabil pada torrent besar dengan ribuan piece dan banyak peer.

# PART III — BitTorrent Protocol

---

# 19. BitTorrent Protocol

## Overview

Torrent Engine mengimplementasikan BitTorrent Protocol sebagai standar utama komunikasi antar peer.

### Features

- Torrent Metadata
- Piece Distribution
- Peer Communication
- Resume Support
- Upload & Download

### Supported Versions

- BitTorrent v1
- BitTorrent v2 (Future)
- Hybrid Torrent (Future)

---

# 20. Peer Wire Protocol

## Overview

Peer Wire Protocol digunakan untuk komunikasi langsung antar peer.

### Supported Messages

- Keep Alive
- Choke
- Unchoke
- Interested
- Not Interested
- Have
- Bitfield
- Request
- Piece
- Cancel

### Features

- Persistent Connection
- Message Validation
- Connection Recovery

---

# 21. Tracker Protocol

## Overview

Tracker Protocol digunakan untuk memperoleh daftar peer.

### Supported Trackers

- HTTP
- HTTPS
- UDP

### Operations

- Announce
- Scrape
- Peer List Update
- Tracker Rotation
- Retry Failed Tracker

---

# 22. DHT Protocol

## Overview

DHT memungkinkan pencarian peer tanpa bergantung pada tracker.

### Features

- Bootstrap
- Node Discovery
- Peer Lookup
- Routing Table
- Distributed Hash Table

### Benefits

- Tidak bergantung pada satu tracker.
- Peer tetap dapat ditemukan ketika tracker tidak tersedia.

---

# 23. Magnet Link Support

## Overview

Torrent Engine mendukung Magnet Link tanpa memerlukan file `.torrent`.

### Supported Parameters

- Info Hash
- Display Name
- Tracker List
- Web Seed (Future)
- File Size (Future)

### Features

- Metadata Download
- Automatic Verification
- Resume Support

---

# 24. Metadata Exchange

## Overview

Metadata Exchange memungkinkan pengambilan metadata dari peer.

### Metadata

- Torrent Name
- File List
- Piece Length
- Piece Hashes
- Total Size

### Features

- Automatic Retrieval
- Metadata Validation
- Resume Metadata

---

# 25. Peer Discovery

## Overview

Peer ditemukan melalui berbagai mekanisme agar swarm tetap optimal.

### Sources

- Tracker
- DHT
- Peer Exchange (PEX)
- Local Peer Discovery (LPD)

### Features

- Automatic Discovery
- Peer Refresh
- Peer Filtering
- Duplicate Removal

---

# 26. Encryption Support

## Overview

Torrent Engine mendukung enkripsi komunikasi antar peer untuk meningkatkan kompatibilitas dan privasi.

### Features

- Message Stream Encryption (MSE)
- Protocol Encryption (PE)
- Encrypted Handshake
- Optional Encryption Mode
- Preferred Encryption Mode

### Security

- Data Integrity
- Secure Handshake
- Connection Validation

---

# Protocol Workflow

```text id="v81rnk"
Torrent / Magnet
        │
        ▼
Metadata
        │
        ▼
Tracker + DHT
        │
        ▼
Peer Discovery
        │
        ▼
Peer Wire Protocol
        │
        ▼
Piece Exchange
        │
        ▼
Verification
        │
        ▼
Seeding
```

---

# Protocol Rules

- Seluruh komunikasi mengikuti spesifikasi BitTorrent.
- Metadata harus diverifikasi sebelum digunakan.
- Peer yang tidak valid diputuskan secara otomatis.
- Tracker dan DHT dapat digunakan bersamaan.
- Enkripsi digunakan jika didukung oleh peer.
- Integritas piece diverifikasi sebelum diterima.

---

# Acceptance Criteria

- BitTorrent Protocol berjalan sesuai standar.
- Peer Wire Protocol mampu bertukar piece dengan benar.
- Tracker dan DHT berhasil menemukan peer.
- Magnet Link dapat diproses tanpa file `.torrent`.
- Metadata berhasil diperoleh dan diverifikasi.
- Komunikasi terenkripsi dapat digunakan apabila didukung oleh peer.

# PART IV — Performance

---

# 27. Peer Optimization

## Overview

Peer Optimization memilih dan mengelola peer terbaik untuk memperoleh kecepatan download dan upload maksimal.

### Features

- Peer Ranking
- Peer Scoring
- Fast Peer Selection
- Slow Peer Detection
- Automatic Peer Replacement

### Optimization Factors

- Download Speed
- Upload Speed
- Latency
- Stability
- Availability
- Reliability

---

# 28. Piece Selection Strategy

## Overview

Piece Manager menentukan urutan pengambilan piece secara dinamis.

### Supported Strategies

- Rarest First
- Random First
- Sequential Download
- End Game Mode
- Smart Piece Selection

### Optimization

- Mengurangi risiko bottleneck.
- Mempercepat penyelesaian torrent.
- Menjaga kesehatan swarm.

---

# 29. Bandwidth Manager

## Overview

Bandwidth Manager mengatur penggunaan bandwidth secara otomatis.

### Features

- Download Limit
- Upload Limit
- Dynamic Bandwidth Allocation
- Smart Traffic Distribution
- Automatic Throttling

### Modes

- Unlimited
- Manual
- Adaptive

---

# 30. Upload Optimizer

## Overview

Mengoptimalkan proses upload kepada peer lain.

### Features

- Dynamic Upload Slots
- Upload Queue
- Fair Share
- Upload Prioritization
- Seed Optimization

### Goals

- Menjaga rasio upload.
- Mengurangi bottleneck.
- Mempertahankan performa download.

---

# 31. Download Optimizer

## Overview

Download Optimizer mengatur strategi download berdasarkan kondisi swarm.

### Features

- Adaptive Peer Selection
- Dynamic Piece Assignment
- Parallel Piece Download
- Automatic Retry
- Smart Scheduling

### Goals

- Memaksimalkan throughput.
- Mengurangi waktu tunggu.
- Menstabilkan kecepatan download.

---

# 32. Disk Optimization

## Overview

Mengoptimalkan proses baca dan tulis data torrent.

### Features

- Sequential Write
- Asynchronous Write
- Piece Cache
- File Preallocation
- Sparse File Support

### Storage Support

- HDD
- SATA SSD
- NVMe SSD
- External Storage

---

# 33. Memory Optimization

## Overview

Mengurangi penggunaan RAM tanpa mengurangi performa.

### Features

- Memory Pool
- Buffer Reuse
- Piece Cache Management
- Automatic Cleanup
- Lazy Allocation

### Targets

| State       |   Target |
| ----------- | -------: |
| Idle        | < 150 MB |
| Active      | < 400 MB |
| Heavy Swarm | < 700 MB |

---

# 34. Network Optimization

## Overview

Network Optimization menyesuaikan strategi komunikasi berdasarkan kondisi jaringan.

### Features

- Latency Detection
- Packet Loss Detection
- Adaptive Timeout
- Connection Reuse
- Dynamic Peer Connection

### Goals

- Mengurangi retransmission.
- Menjaga koneksi tetap stabil.
- Memaksimalkan bandwidth yang tersedia.

---

# Performance Workflow

```text id="n6t9xr"
Swarm Analysis
      │
      ▼
Peer Optimization
      │
      ▼
Piece Selection
      │
      ▼
Bandwidth Manager
      │
      ▼
Download / Upload
      │
      ▼
Disk Writer
      │
      ▼
Performance Monitor
      │
      ▼
Dynamic Optimization
```

---

# Performance Rules

- Jumlah peer aktif disesuaikan secara otomatis.
- Piece dipilih berdasarkan strategi yang paling efisien.
- Bandwidth dibagi secara adaptif.
- Operasi disk menggunakan penulisan asynchronous.
- Cache dibersihkan ketika tidak lagi diperlukan.
- Optimasi berlangsung secara real-time tanpa menghentikan proses torrent.

---

# Acceptance Criteria

- Peer terbaik dipilih secara otomatis.
- Strategi piece meningkatkan kecepatan download.
- Bandwidth digunakan secara efisien.
- Penggunaan CPU, RAM, dan Disk tetap berada dalam batas target.
- Torrent Engine tetap stabil pada swarm besar dengan ratusan peer.
- Optimasi berlangsung otomatis tanpa memerlukan konfigurasi manual.

# PART V — Torrent Intelligence

---

# 35. Smart Peer Selection

## Overview

Smart Peer Selection memilih peer terbaik secara real-time berdasarkan performa aktual.

### Selection Factors

- Download Speed
- Upload Speed
- Latency
- Availability
- Reliability
- Connection Stability
- Response Time

### Features

- Dynamic Peer Ranking
- Automatic Peer Replacement
- Fast Peer Detection
- Poor Peer Elimination

---

# 36. Intelligent Piece Picker

## Overview

Piece Picker menentukan urutan pengambilan piece untuk memaksimalkan kecepatan dan menjaga kesehatan swarm.

### Strategies

- Rarest First
- Sequential
- Random First
- End Game Mode
- Adaptive Piece Selection

### Optimization

- Dynamic Piece Priority
- Piece Redistribution
- Parallel Piece Assignment

---

# 37. Seeder / Leecher Analyzer

## Overview

Menganalisis kondisi swarm sebelum dan selama proses download.

### Analysis

- Seeder Count
- Leecher Count
- Seeder Ratio
- Swarm Availability
- Peer Distribution

### Output

- Estimated Speed
- Estimated Completion Time
- Swarm Health Score

---

# 38. Swarm Optimizer

## Overview

Mengoptimalkan seluruh aktivitas swarm secara otomatis.

### Responsibilities

- Peer Balancing
- Connection Optimization
- Upload Optimization
- Download Optimization
- Piece Distribution

### Features

- Adaptive Swarm Strategy
- Dynamic Peer Allocation
- Automatic Recovery

---

# 39. Duplicate Torrent Detection

## Overview

Mendeteksi torrent yang telah atau sedang diunduh.

### Detection Methods

- Info Hash
- Magnet Hash
- File Structure
- Torrent Name
- Download History

### Actions

- Skip
- Resume Existing
- Merge Session
- Rename
- Ask User

---

# 40. Self-Healing Engine

## Overview

Memulihkan proses torrent secara otomatis saat terjadi gangguan.

### Recovery Features

- Peer Recovery
- Tracker Recovery
- DHT Recovery
- Piece Recovery
- Worker Recovery
- Resume Recovery

### Automatic Actions

- Reconnect
- Reallocate Pieces
- Restart Workers
- Restore Session

---

# 41. Health Monitor

## Overview

Memantau kesehatan Torrent Engine dan swarm secara real-time.

### Monitoring

- Active Peers
- Active Seeders
- Active Leechers
- Swarm Health
- Connection Health
- Piece Availability
- Upload Ratio

### Automatic Actions

- Replace Slow Peers
- Increase Connections
- Reduce Connections
- Restart Failed Workers
- Optimize Piece Strategy

---

# 42. Torrent Statistics

## Overview

Menyediakan statistik torrent secara real-time.

### Statistics

- Download Speed
- Upload Speed
- Downloaded Size
- Uploaded Size
- Progress
- ETA
- Active Peers
- Seeder Count
- Leecher Count
- Share Ratio

### History

- Total Torrents
- Completed Torrents
- Average Speed
- Total Uploaded
- Total Downloaded

---

# Torrent Intelligence Workflow

```text id="z0btxj"
Torrent
    │
    ▼
Swarm Analysis
    │
    ▼
Peer Intelligence
    │
    ▼
Piece Intelligence
    │
    ▼
Swarm Optimizer
    │
    ▼
Health Monitor
    │
    ▼
Self-Healing
    │
    ▼
Statistics
```

---

# Intelligence Rules

- Analisis berlangsung secara real-time.
- Peer diberi skor berdasarkan performa aktual.
- Strategi piece dapat berubah selama download.
- Swarm dioptimalkan tanpa intervensi pengguna.
- Pemulihan dilakukan otomatis jika memungkinkan.
- Statistik diperbarui secara langsung.

---

# Acceptance Criteria

- Peer terbaik dipilih secara otomatis.
- Piece didistribusikan secara efisien.
- Kondisi swarm dipantau secara berkelanjutan.
- Torrent duplikat terdeteksi sebelum download dimulai.
- Self-Healing mampu memulihkan sesi torrent yang terganggu.
- Statistik torrent akurat dan diperbarui secara real-time.

# PART VI — Integration

---

# 43. Download Engine Integration

## Overview

Torrent Engine terintegrasi dengan Download Engine untuk berbagi infrastruktur inti tanpa menduplikasi komponen.

### Shared Components

- Queue Manager
- Scheduler
- File Manager
- Logging System
- Notification System
- Database
- Configuration Manager

### Integration Features

- Shared Download List
- Shared Statistics
- Shared Settings
- Shared Event Bus

---

# 44. Queue Integration

## Overview

Torrent menggunakan Queue Manager yang sama dengan download biasa.

### Features

- Unified Queue
- Priority Queue
- Queue Resume
- Queue Recovery
- Batch Queue

### Queue Rules

- Torrent dan HTTP dapat berada dalam antrean yang sama.
- Prioritas dapat diubah kapan saja.
- Queue dipulihkan setelah aplikasi dijalankan kembali.

---

# 45. Scheduler Integration

## Overview

Scheduler mengatur waktu mulai, jeda, dan penghentian torrent.

### Features

- Schedule Download
- Schedule Seeding
- Schedule Pause
- Schedule Resume
- Schedule Stop

### Triggers

- Date & Time
- Daily
- Weekly
- Monthly
- Custom Rule

---

# 46. Browser Integration

## Overview

Browser Extension dapat mengirim Magnet Link atau file `.torrent` langsung ke VDM.

### Supported Input

- Magnet Link
- Torrent File URL
- Local Torrent File

### Features

- One Click Download
- Automatic Metadata Detection
- Browser Context Menu
- Native Messaging

---

# 47. Notification Integration

## Overview

Notification System memberikan informasi mengenai aktivitas torrent.

### Notification Types

- Torrent Added
- Metadata Loaded
- Download Started
- Download Completed
- Seeding Started
- Seeding Completed
- Error
- Warning

### Delivery

- In-App Notification
- System Notification

---

# 48. Plugin Support (Future)

## Overview

Plugin memungkinkan penambahan fitur baru tanpa mengubah inti Torrent Engine.

### Plugin Categories

- Tracker Provider
- DHT Provider
- Piece Strategy
- Peer Strategy
- Notification Provider
- Analytics Provider

### Plugin API

Plugin dapat mengakses:

- Torrent Events
- Peer Events
- Queue Events
- Scheduler Events
- Statistics API
- Logging API

### Security Rules

- Plugin berjalan dalam sandbox.
- Permission berdasarkan kemampuan plugin.
- Plugin dapat diaktifkan atau dinonaktifkan.
- Plugin tidak dapat mengakses modul inti secara langsung.

---

# Integration Workflow

```text id="5aw1ch"
Browser
   │
   ▼
Torrent Engine
   │
   ├───────────────┐
   ▼               ▼
Queue         Scheduler
   │               │
   ├──────┐        │
   ▼      ▼        ▼
Download  Notification
Engine       System
   │
   ▼
Logging & Database
```

---

# Integration Rules

- Semua modul menggunakan Event Bus yang sama.
- Queue menjadi pusat pengelolaan seluruh jenis download.
- Scheduler dapat mengatur download maupun seeding.
- Browser Extension tidak memproses torrent secara langsung.
- Notification bersifat real-time.
- Plugin hanya menggunakan API resmi.

---

# Acceptance Criteria

- Torrent Engine terintegrasi dengan Download Engine tanpa konflik.
- Queue dan Scheduler bekerja untuk seluruh jenis download.
- Browser dapat mengirim Magnet Link dan file `.torrent` secara langsung.
- Notification menampilkan status torrent secara real-time.
- Plugin dapat dikembangkan tanpa memodifikasi inti Torrent Engine.
- Seluruh integrasi mengikuti arsitektur modular VDM.

# PART V — Torrent Intelligence

---

# 35. Smart Peer Selection

## Overview

Smart Peer Selection memilih peer terbaik secara real-time berdasarkan performa aktual.

### Selection Factors

- Download Speed
- Upload Speed
- Latency
- Availability
- Reliability
- Connection Stability
- Response Time

### Features

- Dynamic Peer Ranking
- Automatic Peer Replacement
- Fast Peer Detection
- Poor Peer Elimination

---

# 36. Intelligent Piece Picker

## Overview

Piece Picker menentukan urutan pengambilan piece untuk memaksimalkan kecepatan dan menjaga kesehatan swarm.

### Strategies

- Rarest First
- Sequential
- Random First
- End Game Mode
- Adaptive Piece Selection

### Optimization

- Dynamic Piece Priority
- Piece Redistribution
- Parallel Piece Assignment

---

# 37. Seeder / Leecher Analyzer

## Overview

Menganalisis kondisi swarm sebelum dan selama proses download.

### Analysis

- Seeder Count
- Leecher Count
- Seeder Ratio
- Swarm Availability
- Peer Distribution

### Output

- Estimated Speed
- Estimated Completion Time
- Swarm Health Score

---

# 38. Swarm Optimizer

## Overview

Mengoptimalkan seluruh aktivitas swarm secara otomatis.

### Responsibilities

- Peer Balancing
- Connection Optimization
- Upload Optimization
- Download Optimization
- Piece Distribution

### Features

- Adaptive Swarm Strategy
- Dynamic Peer Allocation
- Automatic Recovery

---

# 39. Duplicate Torrent Detection

## Overview

Mendeteksi torrent yang telah atau sedang diunduh.

### Detection Methods

- Info Hash
- Magnet Hash
- File Structure
- Torrent Name
- Download History

### Actions

- Skip
- Resume Existing
- Merge Session
- Rename
- Ask User

---

# 40. Self-Healing Engine

## Overview

Memulihkan proses torrent secara otomatis saat terjadi gangguan.

### Recovery Features

- Peer Recovery
- Tracker Recovery
- DHT Recovery
- Piece Recovery
- Worker Recovery
- Resume Recovery

### Automatic Actions

- Reconnect
- Reallocate Pieces
- Restart Workers
- Restore Session

---

# 41. Health Monitor

## Overview

Memantau kesehatan Torrent Engine dan swarm secara real-time.

### Monitoring

- Active Peers
- Active Seeders
- Active Leechers
- Swarm Health
- Connection Health
- Piece Availability
- Upload Ratio

### Automatic Actions

- Replace Slow Peers
- Increase Connections
- Reduce Connections
- Restart Failed Workers
- Optimize Piece Strategy

---

# 42. Torrent Statistics

## Overview

Menyediakan statistik torrent secara real-time.

### Statistics

- Download Speed
- Upload Speed
- Downloaded Size
- Uploaded Size
- Progress
- ETA
- Active Peers
- Seeder Count
- Leecher Count
- Share Ratio

### History

- Total Torrents
- Completed Torrents
- Average Speed
- Total Uploaded
- Total Downloaded

---

# Torrent Intelligence Workflow

```text id="z0btxj"
Torrent
    │
    ▼
Swarm Analysis
    │
    ▼
Peer Intelligence
    │
    ▼
Piece Intelligence
    │
    ▼
Swarm Optimizer
    │
    ▼
Health Monitor
    │
    ▼
Self-Healing
    │
    ▼
Statistics
```

---

# Intelligence Rules

- Analisis berlangsung secara real-time.
- Peer diberi skor berdasarkan performa aktual.
- Strategi piece dapat berubah selama download.
- Swarm dioptimalkan tanpa intervensi pengguna.
- Pemulihan dilakukan otomatis jika memungkinkan.
- Statistik diperbarui secara langsung.

---

# Acceptance Criteria

- Peer terbaik dipilih secara otomatis.
- Piece didistribusikan secara efisien.
- Kondisi swarm dipantau secara berkelanjutan.
- Torrent duplikat terdeteksi sebelum download dimulai.
- Self-Healing mampu memulihkan sesi torrent yang terganggu.
- Statistik torrent akurat dan diperbarui secara real-time.

# PART VIII — Monitoring

---

# 55. Swarm Statistics

## Overview

Swarm Statistics menampilkan kondisi swarm secara real-time untuk setiap torrent.

### Statistics

- Active Peers
- Connected Peers
- Seeders
- Leechers
- Peer Availability
- Swarm Health Score
- Share Ratio
- Download Progress

### Usage

- Dashboard
- Torrent Details
- Statistics Page

---

# 56. Performance Metrics

## Overview

Performance Metrics memantau efisiensi Torrent Engine selama proses download dan seeding.

### Monitored Metrics

- Download Speed
- Upload Speed
- Network Throughput
- Active Connections
- Active Peers
- Active Pieces
- CPU Usage
- Memory Usage
- Disk I/O
- Cache Usage

### Performance Targets

| Metric            |           Target |
| ----------------- | ---------------: |
| Engine Response   |          < 50 ms |
| Peer Update       |         < 200 ms |
| Piece Assignment  |          < 20 ms |
| Hash Verification | < 500 ms / Piece |
| Queue Processing  |          < 20 ms |

---

# 57. Logging

## Overview

Logging mencatat seluruh aktivitas Torrent Engine untuk debugging dan analisis.

### Log Categories

- Torrent
- Peer
- Tracker
- DHT
- Download
- Upload
- Performance
- Security
- Error

### Log Levels

- Trace
- Debug
- Info
- Warning
- Error
- Critical

### Features

- Structured Logging
- Log Rotation
- Automatic Cleanup
- Export Logs
- Search Logs

---

# 58. Diagnostics

## Overview

Diagnostics mendeteksi masalah pada Torrent Engine dan lingkungan jaringan.

### Diagnostic Modules

- Tracker Diagnostics
- DHT Diagnostics
- Peer Diagnostics
- Network Diagnostics
- Disk Diagnostics
- Memory Diagnostics

### Automatic Actions

- Detect Slow Tracker
- Detect Weak Swarm
- Detect Slow Peer
- Generate Diagnostic Report
- Recommend Optimization

---

# 59. Crash Recovery

## Overview

Crash Recovery memulihkan seluruh sesi torrent setelah aplikasi atau sistem mengalami kegagalan.

### Recovery Features

- Resume Session
- Restore Queue
- Restore Peer State
- Restore Piece State
- Restart Workers
- Restore Seeding

### Recovery Process

- Load Previous Session
- Validate Metadata
- Verify Pieces
- Restore Queue
- Continue Download / Seeding

---

# Monitoring Workflow

```text id="sh6czu"
Torrent Engine
       │
       ▼
Performance Monitor
       │
       ├── Swarm Statistics
       ├── Logging
       ├── Diagnostics
       └── Health Check
               │
               ▼
Crash Recovery
               │
               ▼
Dashboard & Notifications
```

---

# Monitoring Rules

- Monitoring berjalan secara asynchronous.
- Statistik diperbarui secara real-time.
- Logging tidak mengganggu performa torrent.
- Diagnostics dijalankan saat diperlukan atau sesuai jadwal.
- Crash Recovery berlangsung otomatis saat aplikasi dibuka kembali.
- Data monitoring digunakan untuk analisis lokal dan tidak dikirim keluar tanpa persetujuan pengguna.

---

# Acceptance Criteria

- Statistik swarm ditampilkan secara akurat.
- Performance Metrics diperbarui secara real-time.
- Logging mencatat aktivitas penting secara terstruktur.
- Diagnostics mampu mendeteksi masalah pada tracker, DHT, peer, dan jaringan.
- Crash Recovery berhasil memulihkan sesi torrent yang dapat dipulihkan.
- Monitoring tetap ringan dan tidak menurunkan performa Torrent Engine.

# PART IX — Testing

---

# 60. Unit Testing

## Overview

Unit Testing memastikan setiap modul Torrent Engine bekerja secara independen sesuai spesifikasi.

### Tested Modules

- Torrent Parser
- Magnet Link Parser
- Tracker Manager
- DHT Engine
- Peer Manager
- Piece Manager
- Download Worker
- Upload Worker
- Verification Engine
- Resume Engine

### Objectives

- Memastikan logika setiap modul benar.
- Mengurangi bug sejak tahap pengembangan.
- Mendukung refactoring dengan aman.

### Target

- Code Coverage ≥ 90%

---

# 61. Integration Testing

## Overview

Integration Testing memverifikasi komunikasi antar modul Torrent Engine dan modul lain dalam VDM.

### Integration Scenarios

- Torrent Engine ↔ Download Engine
- Torrent Engine ↔ Queue Manager
- Torrent Engine ↔ Scheduler
- Torrent Engine ↔ Database
- Torrent Engine ↔ Logging System
- Torrent Engine ↔ Notification System
- Torrent Engine ↔ Browser Extension

### Validation

- Event Bus
- Data Consistency
- Resume Process
- Error Recovery
- Session Synchronization

---

# 62. Stress Testing

## Overview

Stress Testing memastikan Torrent Engine tetap stabil pada kondisi ekstrem.

### Test Scenarios

- 1.000 Peer Aktif
- 100 Torrent Bersamaan
- Torrent > 500 GB
- Torrent > 100.000 Piece
- Banyak Tracker
- DHT Intensif
- Seeding Jangka Panjang

### Success Criteria

- Tidak crash.
- Tidak terjadi memory leak.
- Resume tetap berjalan.
- Swarm tetap stabil.

---

# 63. Benchmark

## Benchmark Targets

| Metric            |           Target |
| ----------------- | ---------------: |
| Metadata Parsing  |         < 200 ms |
| Magnet Parsing    |         < 100 ms |
| Peer Discovery    |            < 2 s |
| Piece Assignment  |          < 20 ms |
| Hash Verification | < 500 ms / Piece |
| Queue Processing  |          < 20 ms |

### Resource Targets

| Resource        |   Target |
| --------------- | -------: |
| CPU Idle        |     < 2% |
| CPU Active      |    < 25% |
| RAM Idle        | < 150 MB |
| RAM Heavy Swarm | < 700 MB |

### Benchmark Goals

- Download dan upload stabil.
- Penggunaan resource efisien.
- Swarm tetap sehat pada beban tinggi.

---

# 64. Acceptance Criteria

Torrent Engine dinyatakan siap digunakan apabila:

- Seluruh Unit Test lulus.
- Seluruh Integration Test lulus.
- Seluruh Stress Test lulus.
- Target Benchmark tercapai.
- Hash Verification berhasil pada seluruh piece.
- Resume berjalan tanpa kehilangan progres.
- Seeding tetap stabil setelah download selesai.
- Tidak ditemukan data corruption.
- Tidak terjadi memory leak selama pengujian jangka panjang.
- Engine mampu menangani swarm besar secara stabil.

---

# 65. Appendix

## Test Environment

### Operating System

- Windows 10
- Windows 11

### Network Conditions

- LAN
- Wi-Fi
- Mobile Hotspot
- High Latency
- Packet Loss
- Limited Bandwidth
- High Bandwidth

### Storage Devices

- HDD
- SATA SSD
- NVMe SSD
- External SSD
- External HDD

### Torrent Sources

- Single File Torrent
- Multi File Torrent
- Magnet Link
- Private Tracker
- Public Tracker
- DHT Network

---

# Test Workflow

```text id="1z2l6g"
Unit Test
     │
     ▼
Integration Test
     │
     ▼
Performance Test
     │
     ▼
Stress Test
     │
     ▼
Benchmark
     │
     ▼
Acceptance Test
     │
     ▼
Release
```

---

# Testing Rules

- Pengujian dilakukan secara otomatis melalui CI/CD jika memungkinkan.
- Seluruh hasil benchmark harus dapat direproduksi.
- Pengujian dilakukan pada berbagai kondisi jaringan dan perangkat penyimpanan.
- Setiap perubahan algoritma wajib disertai pengujian regresi.
- Bug kritis harus diselesaikan sebelum rilis.

---

# End of Torrent Engine Specification

Dokumen ini menjadi acuan utama implementasi Torrent Engine VDM. Seluruh perubahan pada arsitektur, protokol, algoritma pemilihan peer, optimasi performa, dan mekanisme keamanan harus mengikuti spesifikasi yang ditetapkan dalam dokumen ini.
