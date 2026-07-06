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
