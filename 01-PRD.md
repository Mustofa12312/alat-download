# Velocity Download Manager (VDM)

# Product Requirements Document (PRD)

**Document ID:** VDM-PRD-001
**Version:** 1.0.0 (Draft)
**Status:** Draft
**Confidentiality:** Internal Use Only

---

# Document Information

| Item                 | Value                                             |
| -------------------- | ------------------------------------------------- |
| Product Name         | Velocity Download Manager                         |
| Short Name           | VDM                                               |
| Document             | Product Requirements Document                     |
| Document ID          | VDM-PRD-001                                       |
| Version              | 1.0.0                                             |
| Status               | Draft                                             |
| Author               | Mustofa & ChatGPT                                 |
| Target Platform      | Windows 10, Windows 11                            |
| Development Platform | Linux, Windows                                    |
| Architecture         | Rust + Tauri                                      |
| License              | Free                                              |
| Primary Language     | English (Application), Indonesian (Documentation) |

---

# Revision History

| Version | Date          | Author            | Description          |
| ------- | ------------- | ----------------- | -------------------- |
| 1.0.0   | Initial Draft | Mustofa & ChatGPT | Initial PRD creation |

---

# Table of Contents

1. Cover
2. Document Information
3. Revision History
4. Table of Contents
5. Executive Summary
6. Product Vision
7. Product Mission
8. Product Goals
9. Success Metrics
10. Problem Statement
11. Target Users
12. User Personas
13. Competitive Analysis
14. Product Positioning
15. Core Features
16. Functional Requirements
17. Non-Functional Requirements
18. Download Engine
19. Browser Integration
20. Video Downloader
21. Torrent Engine
22. Queue Manager
23. Scheduler
24. File Manager
25. Settings
26. Notifications
27. UI/UX Requirements
28. Security
29. Performance Requirements
30. Error Handling
31. Logging
32. Future Roadmap
33. Milestones
34. Acceptance Criteria
35. Risks
36. Glossary
37. Appendix

---

# 5. Executive Summary

## 5.1 Introduction

Velocity Download Manager (VDM) adalah aplikasi download manager generasi berikutnya yang dirancang untuk memberikan performa tinggi, penggunaan sumber daya yang efisien, dan pengalaman pengguna modern. Produk ini dibangun menggunakan Rust sebagai inti mesin download dan Tauri sebagai framework desktop sehingga mampu memberikan kecepatan tinggi dengan konsumsi memori yang rendah.

VDM dikembangkan sebagai alternatif modern terhadap Internet Download Manager (IDM), dengan tujuan menghadirkan fitur yang lebih lengkap, antarmuka yang lebih modern, serta arsitektur yang lebih siap untuk pengembangan jangka panjang.

---

## 5.2 Product Overview

VDM merupakan aplikasi desktop yang mengelola berbagai jenis unduhan dalam satu platform.

Jenis unduhan yang didukung meliputi:

* HTTP
* HTTPS
* FTP
* SFTP
* BitTorrent
* Magnet Link
* Video Streaming
* Cloud File Downloads

Selain itu, aplikasi akan menyediakan integrasi browser, sistem antrean pintar, penjadwalan unduhan, kategori otomatis, serta kemampuan melanjutkan unduhan pada berbagai kondisi gangguan.

---

## 5.3 Product Purpose

Tujuan utama produk adalah menyediakan aplikasi download manager yang:

* Memiliki performa tinggi.
* Stabil untuk penggunaan harian.
* Mudah digunakan oleh pengguna umum.
* Mendukung berbagai jenis sumber unduhan.
* Memanfaatkan teknologi jaringan modern.
* Bebas digunakan tanpa batasan kecepatan.

---

## 5.4 Business Objectives

Walaupun didistribusikan secara gratis, VDM dirancang sebagai proyek perangkat lunak berkualitas tinggi dengan sasaran:

* Menjadi alternatif utama bagi pengguna IDM.
* Menawarkan pengalaman pengguna yang lebih modern.
* Menyediakan platform download yang dapat terus dikembangkan.
* Mempermudah pengelolaan seluruh aktivitas unduhan dalam satu aplikasi.

---

## 5.5 Product Scope

Versi pertama VDM akan mencakup kemampuan berikut:

### Download Manager

* Pause Download
* Resume Download
* Stop Download
* Retry Download
* Queue Management
* Download History
* Download Categories

### Browser Integration

* Chrome
* Microsoft Edge
* Firefox
* Brave
* Opera
* Vivaldi

### Video Downloads

* YouTube
* Vimeo
* Playlist
* Subtitle
* Audio Only

### File Management

* Rename
* Move
* Delete
* Open Folder
* Open File

### Scheduler

* Shutdown
* Restart
* Sleep
* Hibernate

### Network Features

* Multi Connection Download
* Adaptive Segmentation
* Speed Limiter
* Download Priority

---

## 5.6 Out of Scope (Version 1.0)

Fitur berikut tidak menjadi target pada rilis awal:

* Cloud synchronization.
* Multi-device synchronization.
* User account system.
* Plugin marketplace.
* Mobile application.
* macOS native version.
* Linux native package.
* Distributed downloading antar perangkat.

Fitur-fitur tersebut akan dipertimbangkan pada versi berikutnya berdasarkan kebutuhan pengguna.

---

## 5.7 Product Principles

Seluruh keputusan desain dan pengembangan VDM akan mengikuti prinsip berikut:

1. Performance First
   Setiap fitur harus dirancang dengan prioritas pada kecepatan, efisiensi CPU, penggunaan memori, dan stabilitas.

2. User Simplicity
   Antarmuka harus sederhana dan mudah dipahami tanpa mengurangi kemampuan aplikasi.

3. Modern Architecture
   Sistem menggunakan arsitektur modular agar mudah dipelihara dan dikembangkan.

4. Reliability
   Proses unduh harus tetap dapat dipulihkan ketika terjadi gangguan jaringan, crash aplikasi, atau restart sistem.

5. Security by Default
   Validasi sertifikat, pemeriksaan integritas file, dan pengelolaan data lokal dilakukan dengan pendekatan yang aman.

---

## 5.8 Success Definition

Rilis pertama dianggap berhasil apabila memenuhi indikator berikut:

* Startup aplikasi kurang dari 1 detik pada perangkat yang memenuhi spesifikasi minimum.
* Penggunaan RAM saat idle di bawah 100 MB.
* Dukungan penuh untuk resume download pada server yang mendukung.
* Integrasi browser berjalan stabil.
* Mendukung seluruh protokol yang telah ditentukan.
* Mampu menangani antrean unduhan dalam jumlah besar tanpa penurunan performa yang signifikan.
* Menyediakan pengalaman pengguna yang modern, konsisten, dan mudah digunakan.

---

**End of Part 1**

Bagian berikutnya akan melanjutkan dokumen yang sama mulai dari:

* Product Vision
* Product Mission
* Product Goals
* Success Metrics
* Problem Statement
* Target Users
* User Personas
* Competitive Analysis
* Product Positioning
# 6. Product Vision

## 6.1 Vision Statement

Velocity Download Manager (VDM) bertujuan menjadi download manager desktop tercepat, paling stabil, dan paling modern yang tersedia secara gratis. Produk ini dirancang untuk menggantikan kebutuhan pengguna terhadap aplikasi download manager konvensional dengan menghadirkan teknologi jaringan terbaru, antarmuka modern, serta pengalaman pengguna yang sederhana namun sangat bertenaga.

VDM tidak hanya berfungsi sebagai aplikasi untuk mengunduh file, tetapi sebagai platform terpadu yang mampu mengelola seluruh aktivitas pengunduhan dari berbagai sumber dalam satu aplikasi.

---

## 6.2 Long-Term Vision

Dalam jangka panjang, VDM akan berkembang menjadi platform unduhan universal yang mendukung berbagai jenis protokol, layanan cloud, media streaming, dan otomatisasi pengelolaan file.

Visi jangka panjang meliputi:

* Menjadi download manager utama bagi pengguna Windows.
* Menjadi alternatif gratis dengan kualitas profesional.
* Menjadi platform yang mudah dikembangkan melalui arsitektur modular.
* Mendukung teknologi jaringan terbaru tanpa mengubah arsitektur inti.
* Memberikan pengalaman pengguna yang cepat, ringan, dan modern.

---

## 6.3 Design Philosophy

Seluruh pengembangan VDM mengikuti filosofi berikut:

### Performance First

Setiap fitur harus dirancang agar memberikan dampak seminimal mungkin terhadap penggunaan CPU, RAM, dan penyimpanan.

### Simplicity

Antarmuka harus mudah dipahami bahkan oleh pengguna baru tanpa mengurangi kemampuan aplikasi.

### Reliability

Unduhan tidak boleh gagal hanya karena gangguan jaringan sementara, restart sistem, atau crash aplikasi.

### Scalability

Arsitektur harus memungkinkan penambahan fitur baru tanpa mengubah modul inti.

### Consistency

Seluruh tampilan, interaksi, ikon, dan navigasi harus memiliki perilaku yang konsisten.

---

# 7. Product Mission

## 7.1 Mission Statement

Menyediakan download manager desktop gratis dengan performa tinggi yang mampu menangani seluruh kebutuhan pengunduhan modern melalui arsitektur yang ringan, aman, cepat, dan mudah digunakan.

---

## 7.2 Core Mission

VDM memiliki lima misi utama.

### Misi 1

Menghadirkan kecepatan unduh terbaik melalui optimisasi jaringan dan pemanfaatan kemampuan server secara maksimal.

### Misi 2

Menyediakan pengalaman pengguna modern dengan antarmuka minimalis yang nyaman digunakan setiap hari.

### Misi 3

Mendukung sebanyak mungkin jenis sumber unduhan tanpa memerlukan aplikasi tambahan.

### Misi 4

Menjaga stabilitas proses unduh dalam berbagai kondisi jaringan.

### Misi 5

Membangun fondasi perangkat lunak yang mudah dipelihara dan dikembangkan untuk jangka panjang.

---

# 8. Product Goals

## 8.1 Primary Goals

### Goal 1

Memberikan kecepatan download optimal dengan memanfaatkan segmentasi adaptif dan kemampuan server.

---

### Goal 2

Mengurangi penggunaan sumber daya sistem dibandingkan aplikasi download manager tradisional.

---

### Goal 3

Mengintegrasikan seluruh aktivitas download ke dalam satu aplikasi.

---

### Goal 4

Menyediakan antarmuka modern bergaya macOS dengan pengalaman pengguna yang sederhana.

---

### Goal 5

Memberikan pengalaman instalasi, konfigurasi, dan penggunaan yang mudah.

---

## 8.2 Secondary Goals

* Mendukung pengembangan fitur baru melalui arsitektur modular.
* Mempermudah debugging dan pemeliharaan.
* Mendukung pembaruan aplikasi yang aman.
* Memastikan kompatibilitas dengan Windows 10 dan Windows 11.

---

# 9. Success Metrics

Keberhasilan produk akan diukur menggunakan indikator berikut.

| Metric                      | Target           |
| --------------------------- | ---------------- |
| Startup Time                | < 1 Second       |
| Idle RAM Usage              | < 100 MB         |
| Idle CPU Usage              | < 1%             |
| Download Resume Success     | > 99%            |
| Browser Integration Success | > 99%            |
| Download Failure Rate       | < 1%             |
| Crash Rate                  | < 0.1%           |
| Maximum Queue Size          | 10,000 Downloads |
| Large File Support          | > 100 GB         |
| Average UI Response         | < 100 ms         |

---

## User Experience Metrics

* Semua aksi utama dapat dilakukan maksimal dalam tiga langkah.
* Menambahkan URL baru tidak lebih dari lima detik.
* Pengguna baru dapat mulai mengunduh tanpa membaca dokumentasi.

---

## Technical Metrics

* Mendukung HTTP/2.
* Mendukung HTTP/3.
* Mendukung QUIC jika tersedia.
* Mendukung Resume Download.
* Mendukung Multi-thread Download.
* Mendukung Adaptive Thread Allocation.

---

# 10. Problem Statement

## 10.1 Current Problems

Banyak aplikasi download manager yang tersedia saat ini masih memiliki beberapa keterbatasan.

### User Interface

* Tampilan sudah ketinggalan zaman.
* Navigasi kurang intuitif.
* Pengaturan terlalu rumit.

---

### Performance

* Penggunaan RAM relatif tinggi.
* Penggunaan CPU meningkat saat banyak unduhan berjalan.
* Kurang optimal pada jaringan modern.

---

### Browser Integration

* Integrasi browser terkadang tidak stabil.
* Gagal mendeteksi beberapa jenis media.
* Tidak semua browser didukung dengan kualitas yang sama.

---

### Video Downloads

* Dukungan platform terbatas.
* Dukungan playlist belum optimal.
* Pengelolaan subtitle kurang fleksibel.

---

### Download Recovery

* Resume tidak selalu berhasil.
* Recovery setelah crash kurang andal.
* Penanganan gangguan jaringan belum optimal.

---

## 10.2 Opportunity

Dengan menggunakan Rust sebagai inti aplikasi dan Tauri sebagai desktop framework, VDM memiliki peluang untuk:

* Mengurangi penggunaan memori.
* Mempercepat proses download.
* Memanfaatkan teknologi jaringan terbaru.
* Menyediakan pengalaman pengguna yang lebih baik.
* Mengembangkan fitur baru dengan lebih cepat.

---

# 11. Target Users

## Primary Users

Pengguna umum yang rutin mengunduh file dari internet.

Contoh aktivitas:

* Mengunduh software.
* Mengunduh video.
* Mengunduh dokumen.
* Mengunduh file pekerjaan.
* Mengunduh ISO sistem operasi.

---

## Secondary Users

### Developer

Mengunduh SDK, library, image container, dan file proyek.

### Content Creator

Mengunduh video, audio, subtitle, dan aset multimedia.

### Student

Mengunduh materi kuliah, e-book, jurnal, serta perangkat lunak pembelajaran.

### Gamer

Mengunduh installer game, patch, mod, dan pembaruan.

### Professional

Mengelola unduhan proyek, arsip, dan dokumen kerja berukuran besar.

---

# 12. User Personas

## Persona 1

**Nama:** Andi

**Usia:** 22 Tahun

**Pekerjaan:** Mahasiswa

### Goals

* Download cepat.
* Resume jika internet putus.
* Organisasi file otomatis.

### Pain Points

* Browser sering gagal melanjutkan unduhan.
* Sulit mengatur file hasil unduhan.

---

## Persona 2

**Nama:** Budi

**Usia:** 29 Tahun

**Pekerjaan:** Software Engineer

### Goals

* Download ISO besar.
* Download GitHub Release.
* Download SDK.

### Pain Points

* Browser tidak memiliki queue manager.
* Sulit mengatur prioritas download.

---

## Persona 3

**Nama:** Citra

**Usia:** 31 Tahun

**Pekerjaan:** Content Creator

### Goals

* Download playlist video.
* Download subtitle.
* Download audio.

### Pain Points

* Harus menggunakan beberapa aplikasi berbeda.
* Tidak semua video dapat dideteksi.

---

# 13. Competitive Analysis

## Internet Download Manager (IDM)

### Strengths

* Cepat.
* Stabil.
* Integrasi browser baik.
* Resume download.

### Weaknesses

* Berbayar.
* Antarmuka sudah mulai terasa usang.
* Tidak memiliki dukungan BitTorrent.
* Tidak memiliki mesin video yang setara dengan yt-dlp.
* Tidak mendukung HTTP/3 secara menyeluruh.

---

## Free Download Manager (FDM)

### Strengths

* Gratis.
* Mendukung torrent.
* Antarmuka modern.

### Weaknesses

* Kecepatan tidak selalu konsisten.
* Integrasi browser masih dapat ditingkatkan.
* Fitur streaming terbatas.

---

## Xtreme Download Manager (XDM)

### Strengths

* Gratis.
* Cross-platform.

### Weaknesses

* Pengembangan tidak seaktif beberapa kompetitor.
* Pengalaman pengguna belum sehalus aplikasi modern.

---

## Position Comparison

| Feature               | VDM | IDM     | FDM     | XDM     |
| --------------------- | --- | ------- | ------- | ------- |
| Free                  | ✅   | ❌       | ✅       | ✅       |
| HTTP/3                | ✅   | Partial | Partial | Partial |
| BitTorrent            | ✅   | ❌       | ✅       | ❌       |
| Magnet                | ✅   | ❌       | ✅       | ❌       |
| yt-dlp Integration    | ✅   | ❌       | ❌       | ❌       |
| Modern UI             | ✅   | Partial | ✅       | Partial |
| Adaptive Segmentation | ✅   | Partial | ❌       | ❌       |
| Rust Engine           | ✅   | ❌       | ❌       | ❌       |

---

# 14. Product Positioning

## Position Statement

Velocity Download Manager diposisikan sebagai download manager desktop modern yang menggabungkan performa tinggi, efisiensi sumber daya, dan dukungan terhadap berbagai jenis sumber unduhan dalam satu aplikasi.

Produk ini ditujukan bagi pengguna yang membutuhkan solusi unduhan profesional tanpa biaya lisensi.

---

## Unique Selling Points

* Download Engine berbasis Rust.
* Antarmuka ringan menggunakan Tauri.
* Dukungan HTTP/2, HTTP/3, dan QUIC.
* Integrasi BitTorrent dan Magnet Link.
* Mesin video berbasis yt-dlp.
* Kategori file otomatis.
* Resume yang andal.
* Browser Extension modern.
* Gratis tanpa batasan kecepatan.
* Arsitektur modular yang siap dikembangkan.
# 15. Core Features

## 15.1 Overview

Core Features merupakan kumpulan fitur utama yang menjadi fondasi Velocity Download Manager (VDM). Seluruh fitur dalam bagian ini termasuk **Minimum Viable Product (MVP)** dan harus tersedia pada rilis versi 1.0, kecuali fitur yang secara eksplisit ditandai untuk versi berikutnya.

---

# 15.2 Download Management

### Description

Mengelola seluruh aktivitas unduhan dari berbagai protokol dan sumber dalam satu antarmuka.

### Features

* Start Download
* Pause Download
* Resume Download
* Cancel Download
* Restart Download
* Retry Failed Download
* Delete Download
* Open Download Folder
* Open Downloaded File
* Copy Download URL
* Rename Before Download
* Rename After Download
* Drag and Drop URL
* Clipboard URL Detection

### Requirements

* Mendukung banyak unduhan secara bersamaan.
* Status unduhan diperbarui secara real-time.
* Perubahan status harus muncul tanpa perlu memuat ulang antarmuka.

---

# 15.3 Smart Download Engine

### Description

Mesin download yang secara otomatis menentukan strategi terbaik berdasarkan kondisi jaringan dan kemampuan server.

### Capabilities

* Adaptive Segmentation
* Dynamic Thread Allocation
* Smart Retry
* Automatic Resume
* Connection Pooling
* Server Capability Detection
* Automatic Speed Optimization

### Benefits

* Tidak memerlukan konfigurasi manual.
* Memaksimalkan kecepatan jika server mendukung.
* Mengurangi kegagalan unduhan.

---

# 15.4 Browser Integration

### Supported Browsers

* Chrome
* Microsoft Edge
* Firefox
* Brave
* Opera
* Vivaldi

### Features

* Automatic Download Capture
* Video Detection
* Audio Detection
* Download Context Menu
* Right Click Download
* Send to VDM
* Automatic File Name Detection

---

# 15.5 Video Downloader

### Supported Sources

* YouTube
* Vimeo
* HLS (M3U8)
* MPEG-DASH (MPD)

### Features

* Download Video
* Download Audio
* Download Playlist
* Download Subtitle
* Select Resolution
* Select Codec
* Select Audio Track
* Merge Audio & Video
* Batch Download

---

# 15.6 Torrent Support

### Supported

* Torrent File
* Magnet Link

### Features

* Sequential Download
* File Priority
* Bandwidth Limit
* Seed Control
* Peer Information
* Resume Torrent

---

# 15.7 Queue Manager

### Features

* Priority Queue
* Sequential Queue
* Parallel Queue
* Retry Queue
* Pause Queue
* Resume Queue
* Stop Queue
* Remove Queue

### Queue Priority

* Highest
* High
* Normal
* Low
* Lowest

---

# 15.8 Scheduler

### Trigger

* Time Based
* Event Based

### Actions

* Shutdown
* Restart
* Sleep
* Hibernate
* Exit Application

---

# 15.9 File Management

### Features

* Auto Rename
* Auto Categorize
* Auto Move
* Delete
* Move
* Copy
* Open Folder
* Open File
* Show File Information

---

# 15.10 Download Categories

Default Categories

* Video
* Anime
* Movie
* Software
* ISO
* Music
* PDF
* Images
* ZIP
* Documents
* Others

---

# 15.11 History

### Features

* Download History
* Search History
* Filter History
* Delete History
* Restore History

---

# 15.12 Statistics

Display

* Total Downloads
* Total Data Downloaded
* Average Speed
* Highest Speed
* Total Time Saved
* Download Count per Category

---

# 15.13 Notifications

Notification Types

* Download Started
* Download Completed
* Download Failed
* Download Paused
* Queue Completed
* Scheduler Executed

---

# 15.14 Settings

Configuration

* Theme
* Language
* Download Folder
* Network
* Scheduler
* Browser Extension
* History
* Notifications

---

# 15.15 Search

Searchable Items

* Download Name
* URL
* Category
* Date
* Status

---

# 15.16 Smart Resume

Resume setelah:

* Restart Windows
* Internet Putus
* Crash Aplikasi
* Listrik Mati
* Sleep
* Hibernate

---

# 15.17 File Verification

Supported

* SHA256
* SHA512
* MD5

Automatic verification dapat diaktifkan setelah proses unduh selesai.

---

# 15.18 Clipboard Monitor

Memantau URL yang disalin pengguna.

Jika URL valid, aplikasi dapat menawarkan untuk langsung menambahkannya ke antrean unduhan.

---

# 15.19 Drag & Drop

Mendukung:

* URL
* Torrent File
* Magnet Link

---

# 15.20 Performance Dashboard

Menampilkan informasi real-time:

* Download Speed
* Upload Speed
* CPU Usage
* RAM Usage
* Active Threads
* Active Downloads
* Queue Status

---

# 16. Functional Requirements

## 16.1 Download Management

### FR-001

Aplikasi harus dapat membuat download baru dari URL yang valid.

Priority

Critical

---

### FR-002

Aplikasi harus dapat menghentikan download kapan saja.

Priority

Critical

---

### FR-003

Aplikasi harus dapat melanjutkan download jika server mendukung Resume.

Priority

Critical

---

### FR-004

Aplikasi harus mendukung beberapa download secara bersamaan.

Priority

Critical

---

### FR-005

Aplikasi harus mendukung antrean download.

Priority

Critical

---

### FR-006

Aplikasi harus menyimpan status download secara otomatis.

Priority

Critical

---

### FR-007

Aplikasi harus memulihkan download setelah restart sistem.

Priority

High

---

### FR-008

Aplikasi harus mengelompokkan file berdasarkan kategori.

Priority

High

---

### FR-009

Aplikasi harus memungkinkan pengguna mengubah folder tujuan sebelum download dimulai.

Priority

High

---

### FR-010

Aplikasi harus mendukung penggantian nama file sebelum proses download.

Priority

Medium

---

## 16.2 Browser Integration

### FR-011

Aplikasi harus dapat menangkap download dari browser yang didukung.

---

### FR-012

Browser Extension harus dapat mengirim URL langsung ke aplikasi desktop.

---

### FR-013

Browser Extension harus dapat mendeteksi media streaming yang didukung.

---

## 16.3 Video Downloader

### FR-014

Pengguna dapat memilih resolusi video sebelum download.

---

### FR-015

Pengguna dapat mengunduh subtitle jika tersedia.

---

### FR-016

Pengguna dapat memilih mode audio-only.

---

## 16.4 Torrent

### FR-017

Aplikasi harus mendukung pembukaan file .torrent.

---

### FR-018

Aplikasi harus mendukung Magnet Link.

---

### FR-019

Pengguna dapat memilih file tertentu dalam torrent.

---

## 16.5 Scheduler

### FR-020

Aplikasi harus menjalankan aksi otomatis setelah semua download selesai.

---

### FR-021

Pengguna dapat membatalkan scheduler kapan saja.

---

## 16.6 Notifications

### FR-022

Aplikasi harus mengirim notifikasi ketika download selesai.

---

### FR-023

Aplikasi harus mengirim notifikasi ketika download gagal.

---

### FR-024

Pengguna dapat menonaktifkan notifikasi tertentu.

---

## 16.7 File Verification

### FR-025

Aplikasi harus dapat menghitung checksum setelah download selesai.

---

### FR-026

Checksum dapat dibandingkan dengan nilai yang diberikan pengguna.

---

## 16.8 Settings

### FR-027

Semua pengaturan harus tersimpan secara otomatis.

---

### FR-028

Pengguna dapat mengembalikan pengaturan ke nilai bawaan.

---

# 17. Non-Functional Requirements

## 17.1 Performance

* Startup aplikasi maksimal 1 detik.
* Respons UI kurang dari 100 ms untuk aksi umum.
* Penggunaan RAM saat idle di bawah 100 MB.
* Penggunaan CPU saat idle di bawah 1%.
* Mendukung unduhan file lebih dari 100 GB.
* Mendukung antrean hingga 10.000 item.

---

## 17.2 Reliability

* Tidak kehilangan status unduhan setelah aplikasi ditutup secara normal.
* Mampu memulihkan unduhan setelah crash.
* Mampu menangani gangguan jaringan tanpa kehilangan progres.

Target Availability:

99.9%

---

## 17.3 Scalability

Arsitektur harus memungkinkan penambahan:

* Plugin
* Protocol baru
* Browser baru
* Scheduler baru
* Downloader baru

tanpa memodifikasi inti aplikasi.

---

## 17.4 Maintainability

* Struktur kode modular.
* Dokumentasi lengkap.
* Pengujian otomatis.
* Dependency seminimal mungkin.

---

## 17.5 Security

* HTTPS Certificate Validation.
* Secure Local Storage.
* Hash Verification.
* Sandboxed Frontend (Tauri).
* Tidak menyimpan kredensial dalam bentuk teks biasa.

---

## 17.6 Compatibility

Sistem Operasi:

* Windows 10 (64-bit)
* Windows 11 (64-bit)

Browser:

* Chrome
* Edge
* Firefox
* Brave
* Opera
* Vivaldi

---

## 17.7 Accessibility

* Navigasi penuh menggunakan keyboard.
* Dukungan High Contrast Mode.
* Dukungan Screen Reader dasar.
* Pengaturan ukuran teks.

---

## 17.8 Localization

Bahasa awal yang didukung:

* English
* Bahasa Indonesia

Arsitektur harus mendukung penambahan bahasa lain tanpa perubahan kode inti.

---

## 17.9 Logging

Semua aktivitas penting harus dicatat, termasuk:

* Download dimulai.
* Download selesai.
* Download gagal.
* Resume.
* Error jaringan.
* Error sistem.
* Aktivitas browser extension.

---

## 17.10 Extensibility

Semua modul harus menggunakan antarmuka (interface) yang jelas sehingga fitur baru dapat ditambahkan tanpa memengaruhi modul lain.

Contoh modul yang dirancang agar mudah diperluas:

* Download Engine
* Video Engine
* Torrent Engine
* Browser Integration
* Scheduler
* Notification Service
* File Verification
* Statistics
# 18. Download Engine

## 18.1 Overview

Download Engine merupakan komponen inti (core) dari Velocity Download Manager (VDM). Seluruh proses pengunduhan dijalankan oleh engine ini, mulai dari analisis URL hingga verifikasi file setelah selesai diunduh.

Download Engine dikembangkan menggunakan **Rust** untuk memperoleh performa tinggi, keamanan memori, dan kemampuan multithreading yang optimal.

---

# 18.2 Objectives

Download Engine harus mampu:

* Mengunduh file dengan kecepatan optimal.
* Menggunakan sumber daya sistem secara efisien.
* Memulihkan unduhan setelah gangguan.
* Mendukung berbagai protokol.
* Mendukung file berukuran sangat besar.
* Memberikan informasi progres secara real-time.

---

# 18.3 Supported Protocols

Protocol yang wajib didukung:

* HTTP
* HTTPS
* HTTP/2
* HTTP/3
* FTP
* SFTP
* BitTorrent
* Magnet Link

Engine harus dirancang agar protokol baru dapat ditambahkan tanpa mengubah modul inti.

---

# 18.4 Download Lifecycle

Seluruh proses download mengikuti tahapan berikut:

```text
User Input URL
        │
        ▼
URL Validation
        │
        ▼
Link Analyzer
        │
        ▼
Server Capability Detection
        │
        ▼
Queue Manager
        │
        ▼
Connection Manager
        │
        ▼
Segment Manager
        │
        ▼
Download Workers
        │
        ▼
Disk Writer
        │
        ▼
Integrity Verification
        │
        ▼
Completed
```

---

# 18.5 URL Validation

Engine harus melakukan validasi sebelum download dimulai.

Pemeriksaan meliputi:

* URL valid
* Protocol dikenali
* Host dapat diakses
* Redirect
* Content Length
* MIME Type
* Resume Support

Jika validasi gagal maka download tidak dimulai.

---

# 18.6 Link Analyzer

Link Analyzer bertugas mengambil metadata file.

Informasi yang dikumpulkan:

* File Name
* File Size
* MIME Type
* Server
* Last Modified
* Accept-Ranges
* HTTP Version
* Redirect Count

---

# 18.7 Connection Manager

Connection Manager mengelola seluruh koneksi jaringan.

Responsibilities:

* Membuka koneksi
* Menutup koneksi
* Reconnect
* Keep Alive
* Connection Pool
* Timeout
* Proxy Support (Future)

---

# 18.8 Adaptive Segment Manager

Segment Manager menentukan bagaimana file dibagi menjadi beberapa bagian.

Contoh strategi:

| Ukuran File |    Segment Awal |
| ----------- | --------------: |
| < 10 MB     |               1 |
| 10–100 MB   |               4 |
| 100 MB–1 GB |               8 |
| 1–10 GB     |              16 |
| > 10 GB     | 32–64 (adaptif) |

Jumlah segment dapat bertambah atau berkurang selama proses unduh berdasarkan kondisi server dan jaringan.

---

# 18.9 Download Workers

Setiap segment dijalankan oleh worker tersendiri.

Worker bertugas:

* Download segment
* Retry jika gagal
* Melaporkan progress
* Mengukur kecepatan
* Mengirim data ke Disk Writer

---

# 18.10 Smart Retry

Retry dilakukan secara otomatis ketika:

* Timeout
* Connection Reset
* Temporary Server Error
* Network Lost

Strategi:

* Exponential Backoff
* Maximum Retry Count
* Adaptive Retry Delay

---

# 18.11 Resume Engine

Resume Engine bertanggung jawab menyimpan status download.

Data yang disimpan:

* URL
* Progress
* Segment
* Download Folder
* Temporary File
* Checksum (jika tersedia)

Resume harus berfungsi setelah:

* Restart aplikasi
* Restart Windows
* Sleep
* Hibernate
* Gangguan internet

---

# 18.12 Disk Writer

Disk Writer bertanggung jawab terhadap penulisan file.

Persyaratan:

* Sequential Write
* Parallel Segment Merge
* Minimal Disk Fragmentation
* Flush Data Safely

Temporary file harus digunakan sampai download selesai.

---

# 18.13 Buffer Management

Engine menggunakan buffer dinamis.

Target:

* RAM tetap rendah
* Disk Write stabil
* Tidak terjadi memory leak
* Buffer menyesuaikan kecepatan jaringan

---

# 18.14 Speed Optimizer

Engine secara otomatis mengoptimalkan:

* Jumlah thread
* Jumlah segment
* Buffer
* Retry
* Connection

Tidak diperlukan konfigurasi manual oleh pengguna.

---

# 18.15 Integrity Verification

Setelah download selesai.

Engine dapat memverifikasi:

* SHA256
* SHA512
* MD5

Jika checksum tidak sesuai maka status menjadi **Verification Failed**.

---

# 18.16 Download State Machine

Status download:

* Waiting
* Analyzing
* Connecting
* Downloading
* Paused
* Retrying
* Verifying
* Completed
* Failed
* Cancelled

Status hanya boleh berubah melalui transisi yang valid.

---

# 18.17 Performance Requirements

Target:

| Metric           | Target   |
| ---------------- | -------- |
| Startup Engine   | < 200 ms |
| Progress Update  | ≤ 100 ms |
| Idle CPU         | < 1%     |
| Idle RAM         | < 50 MB  |
| Download Threads | Adaptive |
| Max Queue        | 10,000   |

---

# 18.18 Functional Requirements

DE-001

Engine harus mendukung Resume Download.

DE-002

Engine harus mendukung Multi-thread Download.

DE-003

Engine harus mendukung Adaptive Segmentation.

DE-004

Engine harus menyimpan progress secara berkala.

DE-005

Engine harus melakukan retry otomatis.

DE-006

Engine harus memverifikasi integritas file bila checksum tersedia.

DE-007

Engine harus tetap stabil ketika menangani file berukuran lebih dari 100 GB.

---

# 18.19 Acceptance Criteria

Download Engine dianggap selesai apabila:

* Resume berhasil.
* Retry bekerja.
* Progress akurat.
* Tidak terjadi file corrupt akibat segment merging.
* Tidak terjadi kehilangan data setelah restart.

---

# 19. Browser Integration

## 19.1 Overview

Browser Integration memungkinkan VDM berinteraksi dengan browser untuk menangkap unduhan, mendeteksi media, dan mengirim informasi ke aplikasi desktop secara otomatis.

---

# 19.2 Supported Browsers

Browser yang didukung:

* Google Chrome
* Microsoft Edge
* Mozilla Firefox
* Brave
* Opera
* Vivaldi

Arsitektur harus memungkinkan penambahan browser baru tanpa mengubah Download Engine.

---

# 19.3 Objectives

Browser Integration bertujuan untuk:

* Menangkap download secara otomatis.
* Mengirim URL ke aplikasi desktop.
* Mendeteksi media streaming yang didukung.
* Memberikan pengalaman pengguna yang mulus.

---

# 19.4 Architecture

```text
Browser
    │
Browser Extension
    │
Native Messaging Host
    │
VDM Desktop
    │
Download Engine
```

Extension hanya bertugas mengumpulkan informasi dan mengirimkannya ke aplikasi desktop. Seluruh proses download dilakukan oleh Download Engine.

---

# 19.5 Download Capture

Extension harus mampu menangkap:

* Klik tombol download.
* Link langsung ke file.
* Redirect menuju file.
* File hasil generate server.

Pengguna dapat memilih:

* Download menggunakan browser.
* Download menggunakan VDM.

---

# 19.6 Media Detection

Media yang harus dideteksi:

* Video MP4
* WebM
* Audio MP3
* AAC
* HLS (M3U8)
* MPEG-DASH (MPD)

Deteksi dilakukan tanpa mengganggu proses pemutaran.

---

# 19.7 Browser Context Menu

Tambahkan menu:

* Download with VDM
* Download All Links
* Download Selected Links
* Copy to VDM Queue

---

# 19.8 Native Messaging

Browser berkomunikasi menggunakan Native Messaging.

Informasi yang dikirim:

* URL
* Referer
* Cookie (bila diperlukan)
* User Agent
* File Name
* MIME Type

Komunikasi dilakukan melalui format JSON.

---

# 19.9 Cookie & Session Handling

Untuk situs yang memerlukan autentikasi, extension dapat meneruskan informasi sesi yang diperlukan agar Download Engine dapat melakukan permintaan yang sah.

Data sesi hanya digunakan selama proses download dan tidak disimpan secara permanen kecuali pengguna mengizinkannya.

---

# 19.10 Security Requirements

Extension tidak boleh:

* Mengirim data ke server pihak ketiga.
* Mengumpulkan riwayat browsing.
* Mengakses halaman yang tidak diperlukan.
* Menyimpan cookie tanpa persetujuan pengguna.

Semua komunikasi hanya dilakukan antara browser dan aplikasi VDM.

---

# 19.11 Functional Requirements

BI-001

Extension harus dapat mendeteksi file download.

BI-002

Extension harus dapat mengirim URL ke aplikasi desktop.

BI-003

Extension harus dapat mendeteksi media streaming yang didukung.

BI-004

Extension harus tetap berfungsi setelah browser diperbarui, selama API ekstensi masih kompatibel.

BI-005

Pengguna dapat menonaktifkan integrasi browser tertentu tanpa memengaruhi browser lainnya.

---

# 19.12 Performance Requirements

Target:

| Metric                   | Target   |
| ------------------------ | -------- |
| URL Transfer             | < 100 ms |
| Popup Response           | < 200 ms |
| Native Messaging Startup | < 300 ms |
| Extension Memory         | < 30 MB  |

---

# 19.13 Acceptance Criteria

Browser Integration dianggap selesai apabila:

* Semua browser yang didukung dapat mengirim URL ke VDM.
* Download dapat dimulai tanpa menyalin URL secara manual.
* Media streaming yang didukung dapat terdeteksi.
* Native Messaging berjalan stabil.
* Tidak ada kebocoran data pengguna selama komunikasi.
# 20. Video Downloader

## 20.1 Overview

Video Downloader merupakan modul yang bertanggung jawab untuk mendeteksi, menganalisis, dan mengunduh media dari berbagai platform yang didukung. Modul ini harus mampu menangani video, audio, subtitle, playlist, serta media streaming modern.

Engine utama menggunakan **yt-dlp** sebagai backend dengan integrasi penuh ke Download Engine VDM.

---

# 20.2 Objectives

Video Downloader harus mampu:

- Mengunduh video berkualitas tinggi.
- Mengunduh audio tanpa video.
- Mengunduh subtitle.
- Mengunduh playlist.
- Mengunduh banyak video sekaligus (Batch Download).
- Menggabungkan audio dan video secara otomatis jika diperlukan.

---

# 20.3 Supported Platforms

Versi 1.0 wajib mendukung:

- YouTube
- Vimeo

Arsitektur harus memungkinkan penambahan platform lain di masa depan tanpa mengubah inti modul.

---

# 20.4 Supported Media

- Video
- Audio
- Live Stream (jika didukung platform)
- Playlist
- Channel (Future)
- Subtitle
- Thumbnail

---

# 20.5 Video Quality

Pengguna dapat memilih:

- Best Available
- 144p
- 240p
- 360p
- 480p
- 720p
- 1080p
- 1440p
- 2160p (4K)
- 4320p (8K jika tersedia)

---

# 20.6 Audio Quality

Pilihan:

- Best
- 320 kbps
- 256 kbps
- 192 kbps
- 128 kbps

Format:

- MP3
- AAC
- M4A
- OPUS

---

# 20.7 Subtitle

Support:

- Embedded Subtitle
- Separate Subtitle

Format:

- SRT
- VTT
- ASS

Pilihan:

- Satu bahasa
- Banyak bahasa
- Semua subtitle

---

# 20.8 Playlist Download

Pengguna dapat:

- Download seluruh playlist
- Download sebagian playlist
- Memilih video tertentu
- Melanjutkan playlist yang terhenti

---

# 20.9 Batch Download

Batch Download harus mendukung:

- Banyak URL
- Banyak Playlist
- Banyak Video

Setiap item menjadi satu tugas (task) dalam Queue Manager.

---

# 20.10 FFmpeg Integration

FFmpeg digunakan untuk:

- Merge Audio & Video
- Convert Format
- Extract Audio
- Thumbnail Processing
- Subtitle Embedding

---

# 20.11 Metadata

Metadata yang dapat disimpan:

- Title
- Author
- Duration
- Upload Date
- Resolution
- Codec
- Thumbnail

---

# 20.12 Functional Requirements

VD-001

Pengguna dapat memilih kualitas video.

VD-002

Pengguna dapat memilih kualitas audio.

VD-003

Pengguna dapat memilih subtitle.

VD-004

Pengguna dapat mengunduh playlist.

VD-005

Pengguna dapat melakukan batch download.

VD-006

Pengguna dapat mengubah nama file sebelum download dimulai.

---

# 20.13 Acceptance Criteria

Video Downloader dianggap selesai apabila:

- Video berhasil diunduh.
- Audio berhasil dipisahkan.
- Subtitle dapat diunduh.
- Playlist berjalan dengan benar.
- Batch Download berjalan stabil.
- FFmpeg dapat menggabungkan media bila diperlukan.

---

# 21. Torrent Engine

## 21.1 Overview

Torrent Engine bertanggung jawab menangani seluruh proses pengunduhan berbasis protokol BitTorrent dan Magnet Link.

Modul ini harus terintegrasi langsung dengan Queue Manager serta File Manager.

---

# 21.2 Objectives

Torrent Engine harus:

- Mendukung file .torrent
- Mendukung Magnet Link
- Mendukung Resume
- Mendukung Sequential Download
- Mendukung File Priority
- Mendukung Peer Discovery

---

# 21.3 Features

- Add Torrent
- Add Magnet
- Pause
- Resume
- Stop
- Remove
- Recheck
- Force Reannounce
- Sequential Download

---

# 21.4 Peer Management

Informasi peer:

- IP
- Country (Future)
- Speed
- Upload
- Download
- Connected Time

---

# 21.5 Piece Management

Engine harus:

- Memverifikasi setiap piece
- Menghindari piece corrupt
- Melanjutkan piece yang belum selesai
- Menulis piece secara aman ke disk

---

# 21.6 File Selection

Pengguna dapat:

- Memilih file tertentu
- Mengubah prioritas file
- Menonaktifkan file tertentu

---

# 21.7 Bandwidth

Pengguna dapat mengatur:

- Download Limit
- Upload Limit
- Unlimited

---

# 21.8 Functional Requirements

TE-001

Membuka file .torrent.

TE-002

Membuka Magnet Link.

TE-003

Resume Torrent.

TE-004

Sequential Download.

TE-005

File Priority.

TE-006

Bandwidth Limit.

---

# 21.9 Acceptance Criteria

Torrent Engine dianggap selesai apabila:

- Torrent dapat diunduh.
- Resume berhasil.
- Magnet berjalan.
- Piece Verification berhasil.
- Tidak terjadi korupsi data.

---

# 22. Queue Manager

## 22.1 Overview

Queue Manager mengatur seluruh pekerjaan download agar berjalan sesuai prioritas dan aturan yang telah ditentukan.

---

# 22.2 Objectives

Queue Manager harus:

- Mengatur antrean.
- Mengatur prioritas.
- Menghindari konflik.
- Mengoptimalkan penggunaan bandwidth.

---

# 22.3 Queue Types

- Highest
- High
- Normal
- Low
- Lowest

---

# 22.4 Queue Modes

- Sequential
- Parallel
- Smart Queue

---

# 22.5 Queue Rules

Pengguna dapat menentukan:

- Maksimum download bersamaan.
- Maksimum video bersamaan.
- Maksimum torrent bersamaan.
- Maksimum retry.

---

# 22.6 Smart Queue

Queue Manager secara otomatis:

- Menunda download jika bandwidth penuh.
- Memulai download berikutnya ketika slot tersedia.
- Mengubah prioritas sesuai aturan pengguna.

---

# 22.7 Queue States

- Waiting
- Ready
- Running
- Paused
- Retrying
- Completed
- Failed
- Cancelled

---

# 22.8 Functional Requirements

QM-001

Tambah Queue.

QM-002

Hapus Queue.

QM-003

Ubah Prioritas.

QM-004

Pause Queue.

QM-005

Resume Queue.

QM-006

Auto Retry.

QM-007

Auto Start Next Download.

---

# 22.9 Acceptance Criteria

Queue Manager dianggap selesai apabila:

- Prioritas bekerja.
- Tidak ada deadlock.
- Download berikutnya dimulai otomatis.
- Queue tetap konsisten setelah restart.

---

# 23. Scheduler

## 23.1 Overview

Scheduler memungkinkan pengguna menjalankan aksi otomatis berdasarkan waktu atau kondisi tertentu.

---

# 23.2 Trigger

- Semua download selesai.
- Waktu tertentu.
- Queue selesai.
- Torrent selesai.

---

# 23.3 Actions

- Shutdown
- Restart
- Sleep
- Hibernate
- Exit Application

---

# 23.4 Delay

Pengguna dapat menentukan jeda:

- 1 menit
- 5 menit
- 10 menit
- 30 menit
- Custom

---

# 23.5 Confirmation

Sebelum aksi dijalankan:

- Countdown
- Cancel Button
- Notification

---

# 23.6 Functional Requirements

SC-001

Shutdown setelah download selesai.

SC-002

Restart setelah download selesai.

SC-003

Sleep.

SC-004

Hibernate.

SC-005

Batalkan Scheduler.

---

# 23.7 Acceptance Criteria

Scheduler dianggap selesai apabila:

- Semua aksi berjalan sesuai jadwal.
- Countdown muncul.
- Pengguna dapat membatalkan aksi sebelum dieksekusi.

---

# 24. File Manager

## 24.1 Overview

File Manager bertanggung jawab mengelola seluruh file hasil unduhan, termasuk organisasi folder, penggantian nama, penghapusan, dan informasi file.

---

# 24.2 Objectives

File Manager harus:

- Mengorganisasi file.
- Menghindari duplikasi.
- Mempermudah pencarian.
- Menjaga konsistensi data.

---

# 24.3 File Operations

- Rename
- Move
- Copy
- Delete
- Open
- Open Folder
- Show Properties

---

# 24.4 Automatic Categories

Kategori bawaan:

- Video
- Anime
- Movie
- Software
- ISO
- Music
- PDF
- Images
- ZIP
- Documents
- Others

Pengguna dapat menambah kategori sendiri.

---

# 24.5 Download Rules

Contoh aturan:

- Semua `.iso` → Folder ISO
- Semua `.pdf` → Folder PDF
- Semua video → Folder Video
- Semua `.zip` → Folder ZIP

Pengguna dapat membuat aturan tambahan berdasarkan:

- Ekstensi
- MIME Type
- Domain
- Ukuran File

---

# 24.6 Duplicate Detection

Jika file sudah ada:

- Skip
- Replace
- Rename
- Compare Hash

---

# 24.7 Search

Pencarian berdasarkan:

- Nama
- Ekstensi
- Ukuran
- Tanggal
- Kategori

---

# 24.8 File Verification

Opsional setelah download:

- SHA256
- SHA512
- MD5

---

# 24.9 Functional Requirements

FM-001

Rename File.

FM-002

Move File.

FM-003

Delete File.

FM-004

Open Folder.

FM-005

Automatic Categorization.

FM-006

Duplicate Detection.

FM-007

Download Rules.

FM-008

Checksum Verification.

---

# 24.10 Acceptance Criteria

File Manager dianggap selesai apabila:

- File tersimpan di lokasi yang benar.
- Aturan kategori diterapkan secara otomatis.
- Penanganan file duplikat sesuai pilihan pengguna.
- Verifikasi checksum berjalan ketika diaktifkan.
- Operasi file tidak menyebabkan kehilangan data.

# 25. Settings

## 25.1 Overview

Settings merupakan pusat konfigurasi utama Velocity Download Manager (VDM). Modul ini memungkinkan pengguna menyesuaikan perilaku aplikasi, performa download, integrasi browser, tampilan antarmuka, notifikasi, serta pengaturan lanjutan lainnya.

Seluruh pengaturan harus disimpan secara lokal menggunakan database SQLite atau file konfigurasi terenkripsi dan diterapkan tanpa memerlukan restart aplikasi, kecuali untuk pengaturan tertentu yang memang membutuhkan inisialisasi ulang.

---

# 25.2 Objectives

Settings harus memenuhi tujuan berikut:

- Mudah dipahami oleh pengguna umum.
- Mendukung konfigurasi lanjutan bagi pengguna berpengalaman.
- Seluruh perubahan dapat diterapkan secara aman.
- Konfigurasi dapat dipulihkan ke nilai bawaan.
- Konfigurasi dapat diekspor dan diimpor.
- Mendukung pencarian pengaturan (Settings Search).

---

# 25.3 Design Principles

Settings mengikuti prinsip berikut:

- Minimalis
- Mudah ditemukan
- Tidak membingungkan
- Tidak memerlukan banyak klik
- Dikelompokkan berdasarkan kategori
- Mendukung pencarian instan

---

# 25.4 Settings Categories

VDM menyediakan kategori berikut:

1. General
2. Downloads
3. Network
4. Browser Integration
5. Video Downloader
6. Torrent
7. File Management
8. Scheduler
9. Notifications
10. Appearance
11. Advanced
12. Privacy
13. About

---

# 25.5 General Settings

## Description

Mengatur perilaku umum aplikasi.

### Options

| Setting                   | Default |
| ------------------------- | ------- |
| Language                  | English |
| Launch at Windows Startup | Off     |
| Start Minimized           | Off     |
| Minimize to System Tray   | On      |
| Close to System Tray      | On      |
| Auto Check Update         | On      |
| Portable Mode             | Off     |

---

# 25.6 Download Settings

## Description

Mengatur perilaku download secara umum.

### Options

| Setting                      | Default   |
| ---------------------------- | --------- |
| Default Download Folder      | Downloads |
| Ask Before Download          | On        |
| Resume Incomplete Downloads  | On        |
| Auto Retry                   | On        |
| Retry Count                  | 5         |
| Retry Delay                  | 5 Seconds |
| Automatically Start Queue    | On        |
| Maximum Concurrent Downloads | 5         |

---

# 25.7 Smart Download Settings

Pengguna dapat memilih mode optimisasi.

### Modes

- Automatic (Recommended)
- Maximum Speed
- Balanced
- Low Resource

Automatic akan menjadi mode bawaan.

---

# 25.8 Network Settings

### Options

| Setting            | Default    |
| ------------------ | ---------- |
| Speed Limit        | Unlimited  |
| Download Threads   | Automatic  |
| Connection Timeout | 30 Seconds |
| DNS Retry          | Automatic  |
| HTTP Version       | Automatic  |
| Use HTTP/3         | Auto       |
| Use Keep Alive     | On         |

---

# 25.9 Browser Integration Settings

### Browser List

- Chrome
- Edge
- Firefox
- Brave
- Opera
- Vivaldi

Untuk setiap browser tersedia:

- Enable Integration
- Detect Video
- Detect Audio
- Intercept Downloads
- Enable Context Menu

---

# 25.10 Video Downloader Settings

### Default Video Quality

Pilihan:

- Best
- 8K
- 4K
- 1080p
- 720p
- 480p

Default:

Best Available

---

### Audio Quality

Pilihan:

- Best
- 320 kbps
- 256 kbps
- 192 kbps
- 128 kbps

---

### Subtitle

Pilihan:

- Download Subtitle
- Preferred Language
- Download All Subtitle

---

### FFmpeg

Pilihan:

- Auto Detect
- Custom Location

---

# 25.11 Torrent Settings

### Options

| Setting             | Default   |
| ------------------- | --------- |
| Enable Torrent      | On        |
| Download Limit      | Unlimited |
| Upload Limit        | Unlimited |
| Maximum Connections | Automatic |
| Enable DHT          | On        |
| Enable PEX          | On        |
| Enable LSD          | On        |
| Sequential Download | Off       |

---

# 25.12 File Management Settings

### Automatic Categories

Enable Automatic Category

Default:

Enabled

---

### Duplicate Files

Pilihan:

- Rename
- Replace
- Skip
- Ask Every Time

Default:

Ask Every Time

---

### Download Rules

Pengguna dapat membuat aturan berdasarkan:

- Domain
- Extension
- MIME Type
- File Size
- File Name

---

# 25.13 Scheduler Settings

### Options

- Enable Scheduler
- Countdown Before Action
- Confirmation Dialog
- Sound Before Shutdown

Default:

Enabled

---

# 25.14 Notification Settings

Pengguna dapat mengaktifkan atau menonaktifkan notifikasi berikut:

- Download Started
- Download Completed
- Download Failed
- Queue Finished
- Scheduler Running
- Verification Failed

Jenis notifikasi:

- Desktop
- Sound
- Tray Notification

---

# 25.15 Appearance Settings

## Theme

Pilihan:

- Light
- Dark
- Auto

---

## Accent Color

Pilihan:

- System
- Blue
- Green
- Purple
- Orange
- Red

---

## Window

Pilihan:

- Rounded Corner
- Blur Effect
- Transparency
- Compact Mode

---

## Font Size

Pilihan:

- Small
- Medium
- Large

---

# 25.16 Advanced Settings

Menu ini ditujukan untuk pengguna tingkat lanjut.

### Download Engine

- Manual Thread Count
- Segment Size
- Buffer Size
- Retry Algorithm

---

### Network

- TCP Optimization
- QUIC
- HTTP Keep Alive
- Socket Buffer

---

### Disk

- Temporary Folder
- Cache Size
- Flush Interval

---

### Debug

- Enable Debug Log
- Enable Verbose Log
- Export Diagnostic Report

---

# 25.17 Privacy Settings

### Options

- Anonymous Usage Statistics
- Crash Report
- Remember Download History
- Remember Recent URLs
- Clear History on Exit

Secara default, VDM tidak mengirim data penggunaan ke server mana pun.

---

# 25.18 About

Informasi yang ditampilkan:

- Product Name
- Version
- Build Number
- Rust Version
- Tauri Version
- License
- Website
- GitHub Repository
- Third-party Licenses

---

# 25.19 Import & Export

Pengguna dapat:

- Export Settings
- Import Settings
- Reset Selected Category
- Reset All Settings

Format:

JSON

---

# 25.20 Search

Kotak pencarian harus dapat menemukan pengaturan berdasarkan:

- Nama
- Deskripsi
- Kata Kunci

Hasil pencarian muncul secara langsung saat pengguna mengetik.

---

# 25.21 Default Configuration

Saat pertama kali dijalankan:

- Theme mengikuti sistem operasi.
- Bahasa mengikuti bahasa sistem jika tersedia.
- Download Folder menggunakan folder Downloads pengguna.
- Browser Integration dinonaktifkan sampai extension terpasang.
- Download Engine menggunakan mode Automatic.

---

# 25.22 Functional Requirements

### ST-001

Pengguna dapat mengubah semua pengaturan tanpa perlu restart aplikasi kecuali pengaturan yang memang memerlukan inisialisasi ulang.

---

### ST-002

Semua perubahan harus disimpan otomatis.

---

### ST-003

Pengguna dapat mengembalikan nilai bawaan.

---

### ST-004

Pengguna dapat mengekspor pengaturan.

---

### ST-005

Pengguna dapat mengimpor pengaturan.

---

### ST-006

Settings Search harus menemukan pengaturan dalam waktu kurang dari 100 ms.

---

### ST-007

Perubahan tema diterapkan secara langsung.

---

### ST-008

Pengaturan Download Engine tidak boleh menyebabkan aplikasi crash meskipun pengguna memasukkan nilai yang tidak valid. Sistem harus melakukan validasi dan menggunakan nilai aman.

---

# 25.23 Non-Functional Requirements

| Requirement        | Target   |
| ------------------ | -------- |
| Settings Open Time | < 300 ms |
| Settings Save Time | < 100 ms |
| Search Response    | < 100 ms |
| Theme Change       | < 300 ms |
| Import Settings    | < 2 s    |
| Export Settings    | < 2 s    |

---

# 25.24 Security Requirements

- Seluruh konfigurasi disimpan secara lokal.
- Tidak ada data pribadi yang dikirim ke server tanpa persetujuan pengguna.
- File konfigurasi harus divalidasi sebelum diimpor.
- Nilai konfigurasi yang tidak valid harus ditolak dengan pesan kesalahan yang jelas.

---

# 25.25 Acceptance Criteria

Modul Settings dianggap selesai apabila:

- Semua pengaturan dapat diubah melalui antarmuka pengguna.
- Perubahan tersimpan secara otomatis.
- Pengguna dapat mengimpor dan mengekspor konfigurasi tanpa kehilangan data.
- Pencarian pengaturan bekerja secara akurat.
- Pengaturan kembali ke nilai bawaan berfungsi dengan benar.
- Seluruh pengaturan diterapkan secara konsisten tanpa menyebabkan perilaku aplikasi yang tidak diharapkan.

# 26. Notifications

## 26.1 Overview

Notification System merupakan modul yang bertanggung jawab untuk menyampaikan informasi penting kepada pengguna mengenai status aplikasi, proses download, perubahan sistem, serta aktivitas lainnya yang memerlukan perhatian pengguna.

Sistem notifikasi harus dirancang agar:

- Informatif.
- Tidak mengganggu aktivitas pengguna.
- Mudah dipahami.
- Dapat dikonfigurasi.
- Memiliki prioritas yang jelas.

Seluruh notifikasi harus menggunakan bahasa yang sederhana dan konsisten.

---

# 26.2 Objectives

Notification System bertujuan untuk:

- Memberikan informasi status download secara real-time.
- Memberitahukan apabila terjadi kesalahan.
- Memberikan konfirmasi ketika proses selesai.
- Membantu pengguna mengambil tindakan jika diperlukan.
- Mengurangi kebutuhan membuka aplikasi untuk memeriksa status download.

---

# 26.3 Design Principles

Seluruh notifikasi harus mengikuti prinsip berikut.

### Clear

Isi notifikasi harus singkat, jelas, dan mudah dipahami.

---

### Actionable

Jika memungkinkan, notifikasi harus menyediakan tindakan langsung (Action Button).

Contoh:

- Open File
- Open Folder
- Retry
- Resume
- Cancel

---

### Non-Intrusive

Notifikasi tidak boleh mengganggu aktivitas pengguna secara berlebihan.

---

### Context Aware

Notifikasi hanya muncul ketika memang diperlukan.

---

### Consistent

Seluruh notifikasi menggunakan format visual dan bahasa yang seragam.

---

# 26.4 Notification Channels

VDM menyediakan beberapa kanal notifikasi.

## Desktop Notification

Menggunakan sistem notifikasi bawaan Windows.

Digunakan untuk:

- Download selesai
- Download gagal
- Scheduler berjalan
- Error penting

---

## In-App Notification

Ditampilkan di dalam aplikasi.

Contoh:

- URL tidak valid
- Folder tidak ditemukan
- Pengaturan berhasil disimpan

---

## System Tray Notification

Muncul di area System Tray.

Digunakan untuk:

- Download aktif
- Queue selesai
- Status aplikasi

---

## Sound Notification

Menggunakan suara bawaan atau suara kustom.

Dapat diaktifkan atau dinonaktifkan oleh pengguna.

---

# 26.5 Notification Categories

## Download

- Download Started
- Download Paused
- Download Resumed
- Download Completed
- Download Failed
- Download Cancelled

---

## Queue

- Queue Started
- Queue Paused
- Queue Completed
- Queue Failed

---

## Scheduler

- Shutdown Countdown
- Restart Countdown
- Sleep Countdown
- Hibernate Countdown
- Scheduler Cancelled

---

## Browser Integration

- Browser Connected
- Browser Disconnected
- Extension Installed
- Extension Outdated

---

## Video Downloader

- Playlist Detected
- Subtitle Available
- Merge Completed
- FFmpeg Missing

---

## Torrent

- Torrent Added
- Metadata Downloaded
- Download Started
- Download Completed
- Seeding Started
- Piece Verification Failed

---

## File Verification

- Checksum Verified
- Checksum Failed

---

## System

- Update Available
- Update Installed
- Low Disk Space
- Temporary Folder Full
- Configuration Imported
- Configuration Exported

---

# 26.6 Notification Priority

Setiap notifikasi memiliki tingkat prioritas.

## Critical

Memerlukan perhatian segera.

Contoh:

- Disk penuh
- File corrupt
- Database rusak
- Download Engine gagal

Warna:

Merah

---

## High

Mempengaruhi proses download.

Contoh:

- Download gagal
- Koneksi terputus
- Checksum gagal

Warna:

Oranye

---

## Normal

Informasi umum.

Contoh:

- Download selesai
- Queue selesai
- Scheduler selesai

Warna:

Biru

---

## Low

Informasi tambahan.

Contoh:

- Pengaturan disimpan
- Tema berubah
- Statistik diperbarui

Warna:

Abu-abu

---

# 26.7 Notification Actions

Notifikasi dapat memiliki tombol aksi.

Contoh:

Download Completed

Actions:

- Open File
- Open Folder
- Copy Path

---

Download Failed

Actions:

- Retry
- Resume
- View Log

---

Low Disk Space

Actions:

- Open Download Folder
- Change Folder
- Ignore

---

# 26.8 Notification Lifetime

Default:

| Priority | Duration        |
| -------- | --------------- |
| Critical | Until Dismissed |
| High     | 10 Seconds      |
| Normal   | 5 Seconds       |
| Low      | 3 Seconds       |

Pengguna dapat mengubah durasi pada Settings.

---

# 26.9 Notification Center

VDM menyediakan Notification Center.

Fitur:

- Menampilkan seluruh notifikasi.
- Search Notification.
- Filter Notification.
- Delete Notification.
- Mark as Read.
- Export Notification History.

---

# 26.10 Sound Settings

Pengguna dapat memilih:

- Default Sound
- Custom Sound
- Silent Mode

Volume dapat diatur secara terpisah dari volume sistem.

---

# 26.11 Quiet Mode

Quiet Mode digunakan agar notifikasi tidak mengganggu.

Ketika aktif:

- Tidak ada popup.
- Tidak ada suara.
- Semua notifikasi tetap tersimpan di Notification Center.

---

# 26.12 Do Not Disturb

Mode ini mengikuti pengaturan sistem operasi.

Jika Windows sedang dalam Focus Assist atau Do Not Disturb:

- Popup tidak ditampilkan.
- Suara dimatikan.
- Notification Center tetap diperbarui.

---

# 26.13 Progress Notification

Saat download berlangsung.

Notifikasi dapat menampilkan:

- Nama File
- Progress
- Download Speed
- Estimated Time Remaining
- Current Size
- Total Size

Progress diperbarui secara real-time.

---

# 26.14 Download Complete Notification

Informasi:

- Nama File
- Ukuran
- Waktu Download
- Kecepatan Rata-rata
- Folder Tujuan

Actions:

- Open File
- Open Folder
- Copy Path

---

# 26.15 Download Failed Notification

Informasi:

- Nama File
- Penyebab Gagal
- Error Code
- Retry Count

Actions:

- Retry
- Resume
- View Details

---

# 26.16 Notification History

Riwayat minimal:

1000 notifikasi terakhir.

Pengguna dapat menentukan batas maksimum.

Pilihan:

- 100
- 500
- 1000
- Unlimited

---

# 26.17 Notification Settings

Pengguna dapat mengaktifkan atau menonaktifkan:

- Desktop Notification
- Sound
- Tray Notification
- Download Notification
- Queue Notification
- Scheduler Notification
- Update Notification
- Torrent Notification
- Browser Notification

---

# 26.18 Functional Requirements

### NT-001

Aplikasi harus menampilkan notifikasi ketika download dimulai.

---

### NT-002

Aplikasi harus menampilkan notifikasi ketika download selesai.

---

### NT-003

Aplikasi harus menampilkan notifikasi ketika download gagal.

---

### NT-004

Pengguna dapat menonaktifkan kategori notifikasi tertentu.

---

### NT-005

Pengguna dapat mengubah suara notifikasi.

---

### NT-006

Pengguna dapat membuka Notification Center.

---

### NT-007

Notification Center harus menyimpan riwayat notifikasi.

---

### NT-008

Notifikasi dengan prioritas Critical harus tetap terlihat hingga ditutup pengguna atau masalah terselesaikan.

---

### NT-009

Action Button harus menjalankan fungsi yang sesuai tanpa membuka dialog tambahan yang tidak diperlukan.

---

# 26.19 Non-Functional Requirements

| Requirement               | Target    |
| ------------------------- | --------- |
| Notification Delay        | < 100 ms  |
| Popup Display Time        | < 200 ms  |
| Progress Refresh          | ≤ 1 detik |
| Notification History Load | < 300 ms  |
| Notification Search       | < 100 ms  |

---

# 26.20 Security Requirements

- Notifikasi tidak boleh menampilkan informasi sensitif seperti token autentikasi, cookie, kata sandi, atau header HTTP.
- Jalur file yang ditampilkan harus berasal dari data lokal yang valid.
- Action Button hanya boleh menjalankan aksi yang telah diizinkan oleh pengguna.
- Notification Center tidak boleh menyimpan data pribadi di luar kebutuhan operasional aplikasi.

---

# 26.21 Acceptance Criteria

Notification System dianggap selesai apabila:

- Seluruh jenis notifikasi tampil sesuai kategori dan prioritas.
- Pengguna dapat mengatur kanal, suara, dan kategori notifikasi.
- Notification Center menyimpan dan menampilkan riwayat secara konsisten.
- Action Button berfungsi sesuai konteks.
- Quiet Mode dan Do Not Disturb bekerja sesuai pengaturan sistem operasi.
- Seluruh notifikasi muncul tanpa mengganggu performa aplikasi maupun proses download.

# 27. UI/UX Requirements

# Part 1 — Design Philosophy → Window Layout

---

# 27.1 Overview

User Interface (UI) dan User Experience (UX) merupakan salah satu faktor utama yang membedakan Velocity Download Manager (VDM) dari download manager lainnya.

UI harus memberikan kesan modern, bersih, ringan, dan profesional, sedangkan UX harus membuat seluruh aktivitas download dapat dilakukan dengan cepat tanpa memerlukan banyak konfigurasi.

Seluruh desain harus mengikuti prinsip:

- Simple
- Fast
- Predictable
- Consistent
- Responsive

---

# 27.2 Design Philosophy

VDM mengadopsi filosofi desain berikut.

## Minimalism

Setiap elemen yang tampil harus memiliki fungsi yang jelas.

Tidak ada elemen dekoratif yang mengurangi fokus pengguna.

---

## Performance First

Animasi tidak boleh mengurangi performa aplikasi.

Semua transisi harus ringan.

---

## Content First

Fokus utama pengguna adalah download.

Daftar download harus menjadi pusat perhatian.

---

## One Click Rule

Aktivitas yang sering dilakukan harus dapat diselesaikan dalam satu klik.

Contoh:

- Pause
- Resume
- Retry
- Open Folder

---

## Reduce Cognitive Load

Informasi hanya ditampilkan ketika dibutuhkan.

Advanced Settings dipisahkan dari pengaturan umum.

---

# 27.3 Visual Language

Inspirasi desain:

- macOS
- Windows 11
- Linear
- Raycast
- Arc Browser

Karakteristik:

- Clean
- Rounded
- Soft Shadow
- Glass Effect (opsional)
- Smooth Animation
- Spacious Layout

---

# 27.4 Design Principles

Semua halaman harus memenuhi prinsip berikut.

### Consistency

Komponen yang sama harus memiliki perilaku yang sama.

---

### Clarity

Tidak boleh ada ikon tanpa label pada area penting.

---

### Accessibility

Semua teks harus mudah dibaca.

---

### Efficiency

Pengguna harus dapat mengakses fungsi utama tanpa membuka banyak menu.

---

### Feedback

Setiap aksi harus memberikan umpan balik.

Misalnya:

- Progress
- Loading
- Success
- Error

---

# 27.5 Design System

VDM menggunakan Design System yang terdiri dari:

- Color System
- Typography
- Icons
- Components
- Spacing
- Grid
- Motion
- Shadows

Seluruh halaman harus menggunakan sistem ini.

---

# 27.6 Color System

## Theme

- Light
- Dark
- Auto

---

## Primary Color

Digunakan untuk:

- Button
- Progress
- Active State

---

## Secondary Color

Digunakan untuk:

- Highlight
- Hover
- Selection

---

## Semantic Colors

Success

- Download selesai
- Verification berhasil

Warning

- Retry
- Queue

Error

- Download gagal
- Checksum gagal

Info

- Progress
- Update

---

## Background

Background dibagi menjadi:

Primary Background

Secondary Background

Sidebar Background

Card Background

Dialog Background

---

# 27.7 Typography

Font utama:

Menggunakan font sistem agar tampilan terasa native.

Hierarchy:

Display

Heading

Sub Heading

Body

Caption

Label

Monospace (untuk URL, checksum, log)

Semua ukuran font harus mengikuti skala yang konsisten.

---

# 27.8 Iconography

Ikon harus:

- Sederhana
- Mudah dikenali
- Konsisten

Kategori ikon:

Download

Pause

Resume

Retry

Delete

Folder

Video

Audio

Torrent

Browser

Settings

Notification

History

Statistics

Search

Help

About

---

# 27.9 Spacing System

Menggunakan spacing konsisten.

Base Unit:

8 px

Contoh:

8

16

24

32

40

48

64

Tidak diperbolehkan menggunakan spacing acak.

---

# 27.10 Grid System

Desktop menggunakan grid fleksibel.

Sidebar:

Fixed Width

Main Content:

Flexible

Dialog:

Centered

Responsive Grid digunakan untuk:

Cards

Statistics

Settings

---

# 27.11 Elevation & Shadow

Shadow hanya digunakan pada:

Dialog

Dropdown

Tooltip

Floating Panel

Tidak menggunakan shadow berlebihan.

---

# 27.12 Motion & Animation

Animasi digunakan untuk:

Opening

Closing

Loading

Notification

Progress

Hover

Duration:

100–250 ms

Animation harus dapat dimatikan melalui Settings.

---

# 27.13 Window Layout

## Main Window

Window dibagi menjadi:

Top Toolbar

Sidebar

Main Content

Bottom Status Bar

---

### Toolbar

Berisi:

Logo

Search

Add URL

Paste URL

Settings

Notification

User Menu (Future)

---

### Sidebar

Menu utama:

Dashboard

Downloads

Video

Torrent

History

Statistics

Scheduler

Settings

About

Sidebar dapat di-collapse.

---

### Main Content

Area utama.

Menampilkan:

Download List

Queue

Video

Torrent

History

Settings

---

### Status Bar

Menampilkan:

Download Speed

Upload Speed

Active Downloads

Queue

CPU

RAM

Connection

---

# 27.14 Window Behavior

Window harus mendukung:

Resize

Maximize

Minimize

Snap Layout (Windows)

Remember Window Position

Remember Window Size

Multi Monitor

---

# 27.15 Dialog Design

Semua dialog menggunakan format yang sama.

Header

Content

Action

Cancel

Dialog tidak boleh melebihi 70% ukuran layar.

---

# 27.16 Loading States

Jenis Loading:

Skeleton

Spinner

Progress Bar

Determinate Progress

Indeterminate Progress

Loading harus muncul dalam waktu kurang dari 200 ms bila proses tidak dapat diselesaikan secara instan.

---

# Part 2 — Navigation → Settings UI

---

# 27.17 Navigation Structure

Navigasi utama harus sederhana dan konsisten.

Urutan menu:

1. Dashboard
2. Downloads
3. Video Downloader
4. Torrent
5. History
6. Statistics
7. Scheduler
8. Settings
9. About

Menu dapat diakses melalui:

- Sidebar
- Keyboard Shortcut
- Search Command

---

# 27.18 Dashboard UI

Dashboard menjadi halaman utama setelah aplikasi dibuka.

Menampilkan:

- Active Downloads
- Recent Downloads
- Queue Status
- Download Speed
- Storage Usage
- Quick Actions

Quick Actions:

- Add URL
- Paste URL
- Open Torrent
- Resume All
- Pause All

---

# 27.19 Downloads UI

Halaman Downloads menampilkan daftar seluruh unduhan.

Kolom default:

- Status
- File Name
- Category
- Size
- Progress
- Speed
- ETA
- Date Added
- Destination

Aksi cepat:

- Pause
- Resume
- Retry
- Cancel
- Open Folder
- Copy URL

Kolom dapat diurutkan dan difilter.

---

# 27.20 Video Downloader UI

Halaman ini digunakan untuk mengelola unduhan video.

Komponen:

- URL Input
- Video Preview
- Thumbnail
- Title
- Duration
- Quality Selector
- Audio Selector
- Subtitle Selector
- Download Button

Playlist ditampilkan dalam tabel dengan opsi pilih semua atau sebagian.

---

# 27.21 Torrent UI

Halaman Torrent menampilkan:

- Torrent Name
- Progress
- Download Speed
- Upload Speed
- Seed
- Peer
- Ratio
- Remaining Time

Panel tambahan:

- File List
- Peer List
- Tracker List
- Piece Map
- Log

---

# 27.22 History UI

History menggunakan tampilan tabel.

Fitur:

- Search
- Filter
- Sort
- Export
- Delete
- Restore

Filter:

- Date
- Category
- Status
- Source

---

# 27.23 Statistics UI

Menampilkan statistik dalam bentuk kartu dan grafik.

Informasi:

- Total Download
- Total Size
- Average Speed
- Highest Speed
- Download by Category
- Daily Activity
- Monthly Activity

Grafik harus dapat diperbesar dan difilter berdasarkan rentang waktu.

---

# 27.24 Scheduler UI

Menampilkan:

- Scheduler List
- Trigger
- Action
- Countdown
- Status

Pengguna dapat:

- Tambah Scheduler
- Edit
- Pause
- Hapus
- Jalankan Sekarang

---

# 27.25 Settings UI

Settings menggunakan navigasi dua panel.

Panel kiri:

- Daftar kategori.

Panel kanan:

- Detail pengaturan.

Komponen yang digunakan:

- Switch
- Checkbox
- Radio Button
- Dropdown
- Slider
- Number Input
- Text Field
- Folder Picker
- File Picker

Setiap perubahan harus memberikan umpan balik visual secara langsung.

Bagian Advanced dipisahkan agar pengguna umum tidak terganggu oleh pengaturan teknis.

Settings Search selalu tersedia di bagian atas dan dapat menemukan pengaturan berdasarkan nama maupun deskripsi.

# 27. UI/UX Requirements

# Part 3 — Notification Center UI → Acceptance Criteria

---

# 27.26 Notification Center UI

## Overview

Notification Center berfungsi sebagai pusat seluruh aktivitas dan pemberitahuan aplikasi.

Pengguna dapat melihat seluruh riwayat notifikasi tanpa kehilangan informasi penting.

---

## Layout

Notification Center terdiri dari:

- Header
- Search Bar
- Filter
- Notification List
- Detail Panel

---

## Notification Card

Setiap notifikasi menampilkan:

- Icon
- Title
- Description
- Timestamp
- Priority
- Status
- Action Button

Contoh:

```
Download Completed

Ubuntu-26.04.iso

Completed in 3m 22s

Open File
Open Folder
```

---

## Filter

Filter berdasarkan:

- All
- Download
- Queue
- Scheduler
- Browser
- Torrent
- Verification
- Error
- Update

---

## Search

Pencarian berdasarkan:

- Nama file
- URL
- Kategori
- Jenis notifikasi

---

## Bulk Actions

Pengguna dapat:

- Mark All as Read
- Delete Selected
- Clear All
- Export History

---

# 27.27 Search UI

## Universal Search

Search tersedia di seluruh aplikasi.

Shortcut:

```
Ctrl + K
```

atau

```
Ctrl + P
```

Search mampu mencari:

- Download
- History
- Settings
- Torrent
- Scheduler
- URL
- Folder
- Notification

---

## Search Result

Hasil pencarian dikelompokkan berdasarkan kategori.

Contoh:

Downloads

History

Settings

Notification

---

# 27.28 Responsive Behavior

Walaupun aplikasi desktop, UI tetap harus responsif.

Minimum Window:

1024 × 720

Recommended:

1440 × 900

Optimal:

1920 × 1080

---

## Resize

Sidebar

- Collapse otomatis

Table

- Kolom menyesuaikan

Toolbar

- Ikon disusun ulang

Dialog

- Tidak keluar dari layar

---

# 27.29 Accessibility

Aplikasi harus memenuhi prinsip aksesibilitas.

## Keyboard Navigation

Seluruh fungsi utama dapat digunakan tanpa mouse.

Tab

Shift + Tab

Arrow Key

Enter

Esc

---

## Screen Reader

Komponen wajib memiliki:

- Accessible Name
- Accessible Description
- Role

---

## Contrast

Light Mode

Minimum 4.5 : 1

Dark Mode

Minimum 4.5 : 1

---

## Font Scaling

Ukuran teks:

- Small
- Medium
- Large
- Extra Large

Perubahan diterapkan secara langsung.

---

# 27.30 Keyboard Shortcuts

Shortcut bawaan:

| Shortcut         | Action           |
| ---------------- | ---------------- |
| Ctrl + N         | New Download     |
| Ctrl + V         | Paste URL        |
| Ctrl + K         | Universal Search |
| Ctrl + H         | History          |
| Ctrl + T         | Torrent          |
| Ctrl + S         | Settings         |
| Ctrl + P         | Pause All        |
| Ctrl + R         | Resume All       |
| Ctrl + Shift + D | Dashboard        |
| F5               | Refresh          |
| Delete           | Remove Download  |
| Enter            | Open Download    |
| Ctrl + O         | Open Folder      |
| Esc              | Close Dialog     |

Seluruh shortcut dapat diubah oleh pengguna pada versi mendatang.

---

# 27.31 Component Library

Komponen standar yang digunakan di seluruh aplikasi.

## Buttons

- Primary
- Secondary
- Outline
- Ghost
- Icon Button
- Danger Button

---

## Inputs

- Text Field
- Password Field
- URL Field
- Number Field

---

## Selection

- Checkbox
- Radio
- Toggle
- Dropdown
- Combo Box

---

## Feedback

- Toast
- Snackbar
- Progress Bar
- Circular Progress
- Badge

---

## Navigation

- Sidebar
- Breadcrumb
- Tabs
- Toolbar
- Context Menu

---

## Containers

- Card
- Dialog
- Drawer
- Panel
- Table

Semua komponen harus mengikuti Design System yang sama.

---

# 27.32 Design Tokens

Design Token digunakan agar tampilan konsisten.

Token utama:

Spacing

Typography

Border Radius

Shadow

Animation Duration

Opacity

Elevation

Color

Tidak diperbolehkan menggunakan nilai hardcoded yang tidak berasal dari Design Token.

---

# 27.33 UX Rules

Aturan pengalaman pengguna.

### Rule 1

Tidak boleh ada aksi penting tanpa konfirmasi.

Contoh:

- Delete Download
- Delete History
- Reset Settings

---

### Rule 2

Setiap aksi harus memberikan feedback.

Misalnya:

Loading

Success

Error

Retry

---

### Rule 3

Tidak boleh ada proses yang membuat UI membeku.

Semua pekerjaan berat dijalankan di backend Rust.

---

### Rule 4

Progress harus selalu terlihat.

---

### Rule 5

Status download harus mudah dikenali melalui ikon, warna, dan teks.

---

# 27.34 Empty States

Halaman tanpa data harus tetap informatif.

Contoh:

Downloads

"Tidak ada download."

Tombol:

Add Download

---

History

"Belum ada riwayat."

---

Torrent

"Belum ada torrent."

---

Notification

"Tidak ada notifikasi."

---

# 27.35 Loading States

Loading menggunakan:

Skeleton

Progress

Spinner

Loading tidak boleh berkedip (flicker).

Jika proses selesai kurang dari 200 ms, loading tidak perlu ditampilkan.

---

# 27.36 Error States

Contoh pesan:

Network Error

Server Unavailable

Permission Denied

Disk Full

Verification Failed

FFmpeg Not Found

Setiap error harus memiliki:

- Penjelasan singkat.
- Penyebab yang mungkin.
- Langkah yang dapat dilakukan pengguna.
- Tombol Retry bila memungkinkan.

---

# 27.37 Empty Screen Illustration

Halaman kosong menggunakan ilustrasi sederhana bergaya minimalis.

Tujuan:

- Mengurangi kesan aplikasi rusak.
- Memberikan petunjuk kepada pengguna.

---

# 27.38 User Feedback

Aplikasi harus memberikan umpan balik visual untuk:

- Hover
- Focus
- Click
- Success
- Warning
- Error
- Disabled

Animasi maksimal 250 ms.

---

# 27.39 Acceptance Criteria

Modul UI/UX dianggap selesai apabila:

- Seluruh halaman mengikuti Design System yang sama.
- Navigasi konsisten di seluruh aplikasi.
- Tidak ada komponen yang memiliki perilaku berbeda untuk fungsi yang sama.
- Aplikasi dapat digunakan sepenuhnya dengan keyboard untuk fungsi utama.
- Seluruh status aplikasi memiliki representasi visual yang jelas.
- Responsive Layout bekerja pada ukuran jendela minimum yang didukung.
- Empty State, Loading State, dan Error State tersedia untuk seluruh halaman utama.
- Notification Center, Search, Dashboard, Downloads, Torrent, Video Downloader, History, Scheduler, dan Settings memiliki desain yang seragam.
- UI tetap responsif saat menjalankan puluhan unduhan secara bersamaan tanpa penurunan pengalaman pengguna yang signifikan.

# 28. Security

## 28.1 Overview

Security merupakan salah satu komponen utama dalam Velocity Download Manager (VDM). Seluruh sistem harus dirancang menggunakan prinsip **Security by Design**, sehingga keamanan menjadi bagian dari arsitektur aplikasi sejak awal, bukan ditambahkan setelah aplikasi selesai dikembangkan.

Karena VDM menangani komunikasi jaringan, akses sistem file, browser integration, torrent, serta proses download dari internet, seluruh komponen wajib mengikuti standar keamanan yang konsisten.

---

# 28.2 Security Objectives

VDM memiliki tujuan keamanan berikut.

- Melindungi data pengguna.
- Melindungi konfigurasi aplikasi.
- Mencegah eksekusi file yang tidak sah.
- Memastikan komunikasi jaringan aman.
- Mengurangi risiko file rusak.
- Mengurangi risiko manipulasi download.
- Mencegah penyalahgunaan Browser Extension.
- Melindungi aplikasi dari input berbahaya.

---

# 28.3 Security Principles

Pengembangan mengikuti prinsip:

## Least Privilege

Setiap modul hanya memiliki hak akses yang benar-benar diperlukan.

---

## Secure by Default

Semua fitur aktif dengan konfigurasi yang aman secara bawaan.

---

## Fail Secure

Jika terjadi kesalahan, aplikasi harus memilih kondisi yang paling aman.

Contoh:

Jika verifikasi sertifikat HTTPS gagal, download dibatalkan dan pengguna diberi penjelasan.

---

## Defense in Depth

Keamanan diterapkan pada beberapa lapisan:

- UI
- Browser Extension
- Native Messaging
- Rust Backend
- Download Engine
- File System
- Database

---

## Privacy First

Data pengguna tidak dikirim ke server mana pun tanpa persetujuan eksplisit.

---

# 28.4 Threat Model

Ancaman utama yang harus dipertimbangkan:

### Network Attack

- Man in the Middle (MITM)
- DNS Spoofing
- Redirect berbahaya
- TLS Downgrade

---

### File Attack

- File Corruption
- File Replacement
- File Injection

---

### Browser Attack

- Extension Abuse
- Fake Download Link
- Malicious Redirect

---

### Local Attack

- Database Manipulation
- Configuration Manipulation
- Permission Escalation

---

### User Attack

- URL Palsu
- Social Engineering
- File Berbahaya

---

# 28.5 URL Validation

Sebelum download dimulai.

Engine wajib melakukan validasi:

- URL Format
- Protocol
- Domain
- Redirect
- HTTPS Certificate
- MIME Type
- File Size
- Resume Capability

URL tidak valid harus ditolak.

---

# 28.6 HTTPS Security

Untuk HTTPS.

Engine harus:

- Memverifikasi sertifikat.
- Memverifikasi hostname.
- Memeriksa masa berlaku sertifikat.
- Mengikuti standar TLS modern.

Tidak diperbolehkan menonaktifkan verifikasi sertifikat melalui pengaturan umum.

---

# 28.7 Certificate Validation

Jika sertifikat:

- Tidak valid
- Kadaluarsa
- Tidak dipercaya
- Tidak cocok dengan domain

Maka:

Download dihentikan.

Pengguna diberikan informasi mengenai penyebabnya.

---

# 28.8 Redirect Protection

Engine harus membatasi jumlah redirect.

Default:

Maximum Redirect

10

Redirect Loop harus dideteksi.

---

# 28.9 Browser Extension Security

Browser Extension hanya boleh:

- Membaca URL download yang diperlukan.
- Mengirim data ke aplikasi lokal.
- Tidak boleh mengirim data ke server eksternal.

Permission harus seminimal mungkin.

---

# 28.10 Native Messaging Security

Native Messaging hanya menerima komunikasi dari Browser Extension resmi.

Validasi dilakukan terhadap:

- Browser ID
- Extension ID
- Message Format
- Version Compatibility

Semua pesan menggunakan format JSON yang tervalidasi.

---

# 28.11 Cookie & Session Security

Jika Browser Extension meneruskan cookie atau sesi:

- Hanya digunakan untuk download aktif.
- Tidak disimpan permanen secara bawaan.
- Dihapus setelah proses selesai kecuali pengguna memilih untuk menyimpannya.

---

# 28.12 Download Verification

Setelah download selesai.

VDM dapat melakukan:

- SHA256
- SHA512
- MD5

Jika checksum tersedia dari sumber resmi.

Status menjadi:

Verified

atau

Verification Failed

---

# 28.13 File Integrity

Temporary File tidak boleh dianggap selesai sebelum:

- Seluruh segment selesai.
- Merge berhasil.
- Verification selesai.

Baru setelah itu file dipindahkan menjadi file akhir.

---

# 28.14 Safe File Writing

File ditulis menggunakan:

Temporary Extension

Contoh:

```text
Ubuntu.iso.vdm
```

Setelah selesai:

```text
Ubuntu.iso
```

Hal ini mencegah file setengah jadi digunakan secara tidak sengaja.

---

# 28.15 Database Security

SQLite harus:

- Menggunakan prepared statement.
- Menghindari SQL Injection.
- Melakukan validasi input.
- Menyimpan hanya data yang diperlukan.

---

# 28.16 Configuration Security

File konfigurasi harus:

- Divalidasi saat dibaca.
- Memiliki versi skema.
- Menggunakan nilai bawaan jika konfigurasi rusak.
- Tidak menyimpan informasi sensitif dalam bentuk teks biasa.

---

# 28.17 Local Storage Security

Data lokal yang disimpan:

- Settings
- History
- Queue
- Scheduler
- Statistics

Tidak boleh menyimpan:

- Password
- Access Token
- Cookie permanen (kecuali diizinkan pengguna)

---

# 28.18 Logging Security

Log tidak boleh berisi:

- Password
- Cookie
- Authorization Header
- Token
- Private Key

URL yang mengandung token harus disamarkan sebelum ditulis ke log.

---

# 28.19 Update Security

Jika VDM mendukung pembaruan otomatis.

Updater harus:

- Menggunakan HTTPS.
- Memverifikasi tanda tangan paket.
- Memverifikasi checksum.
- Menolak paket yang tidak valid.

---

# 28.20 Plugin Security (Future)

Jika Plugin System ditambahkan.

Plugin harus:

- Memiliki manifest.
- Memiliki versi.
- Memiliki izin (permissions).
- Dapat dinonaktifkan.
- Berjalan dalam batas kemampuan yang ditentukan.

---

# 28.21 Memory Safety

Rust digunakan sebagai bahasa utama karena:

- Tidak memiliki null pointer dereference seperti pada banyak bahasa lain.
- Mencegah data race pada kode aman (safe Rust).
- Mengurangi memory leak.
- Mengurangi buffer overflow.

Seluruh kode harus memprioritaskan safe Rust. Penggunaan `unsafe` hanya diperbolehkan jika benar-benar diperlukan, didokumentasikan, dan melalui proses code review.

---

# 28.22 Secure File Deletion

Jika pengguna menghapus file melalui VDM.

Pilihan:

- Remove from History
- Delete from Disk
- Secure Delete (Future)

---

# 28.23 Privacy

VDM tidak mengumpulkan:

- Riwayat browsing.
- Riwayat download untuk dikirim ke server.
- Data pribadi pengguna.

Crash Report dan Anonymous Usage Statistics bersifat opsional (opt-in).

---

# 28.24 Security Audit

Sebelum setiap rilis.

Dilakukan:

- Dependency Audit.
- Static Analysis.
- Rust Clippy.
- Cargo Audit.
- License Check.
- Manual Code Review.

---

# 28.25 Functional Requirements

### SEC-001

Seluruh URL harus divalidasi sebelum download dimulai.

---

### SEC-002

HTTPS Certificate wajib diverifikasi.

---

### SEC-003

Native Messaging hanya menerima komunikasi dari Browser Extension resmi yang tervalidasi.

---

### SEC-004

Temporary file harus digunakan hingga download dan verifikasi selesai.

---

### SEC-005

Checksum dapat diverifikasi apabila nilai referensi tersedia.

---

### SEC-006

Pengguna dapat menghapus riwayat download tanpa menghapus file di disk.

---

### SEC-007

Pengguna dapat menghapus file dari disk melalui VDM dengan konfirmasi.

---

### SEC-008

Log tidak boleh menyimpan data sensitif.

---

### SEC-009

File konfigurasi yang rusak harus dipulihkan menggunakan nilai aman tanpa menyebabkan aplikasi gagal dijalankan.

---

### SEC-010

Semua input dari pengguna, Browser Extension, dan sumber eksternal harus divalidasi sebelum diproses.

---

# 28.26 Non-Functional Requirements

| Requirement            | Target                                      |
| ---------------------- | ------------------------------------------- |
| URL Validation         | < 100 ms                                    |
| Certificate Validation | < 300 ms                                    |
| Checksum Calculation   | Bergantung ukuran file, tanpa membekukan UI |
| Configuration Load     | < 100 ms                                    |
| Database Validation    | < 100 ms                                    |
| Security Audit         | Dilakukan sebelum setiap rilis              |

---

# 28.27 Security Checklist

Setiap rilis harus memastikan:

- HTTPS Validation aktif.
- Certificate Validation aktif.
- Redirect Protection aktif.
- Prepared Statement digunakan untuk akses database.
- Tidak ada informasi sensitif di log.
- Browser Extension menggunakan permission minimum.
- Native Messaging tervalidasi.
- Temporary file dibersihkan jika download dibatalkan.
- Dependensi telah diperiksa terhadap kerentanan yang diketahui.
- Pengujian keamanan dasar selesai tanpa temuan kritis.

---

# 28.28 Acceptance Criteria

Modul Security dianggap selesai apabila:

- Seluruh komunikasi jaringan menggunakan mekanisme validasi yang sesuai.
- URL, konfigurasi, dan input pengguna tervalidasi sebelum diproses.
- Browser Extension dan Native Messaging hanya dapat berkomunikasi melalui jalur yang diizinkan.
- File sementara tidak dapat dianggap sebagai hasil download yang selesai.
- Tidak ada data sensitif yang tersimpan di log maupun konfigurasi tanpa persetujuan pengguna.
- Pengguna mendapatkan informasi yang jelas ketika proses download dibatalkan karena alasan keamanan.
- Audit keamanan internal tidak menemukan kerentanan dengan tingkat risiko tinggi yang belum ditangani sebelum rilis.

# 29. Performance Requirements

## 29.1 Overview

Performance merupakan salah satu nilai utama Velocity Download Manager (VDM). Seluruh arsitektur aplikasi dirancang untuk memberikan kecepatan unduh yang tinggi, penggunaan sumber daya yang rendah, serta pengalaman pengguna yang responsif.

VDM harus mampu menangani ribuan tugas download tanpa mengorbankan stabilitas maupun kenyamanan penggunaan.

Performance bukan hanya diukur dari kecepatan download, tetapi juga mencakup waktu startup, penggunaan CPU, penggunaan memori, respons antarmuka, efisiensi disk, dan kemampuan menangani banyak pekerjaan secara bersamaan.

---

# 29.2 Performance Objectives

VDM harus memenuhi tujuan berikut:

- Startup sangat cepat.
- Penggunaan RAM rendah.
- Penggunaan CPU rendah.
- UI selalu responsif.
- Download berjalan maksimal.
- Penulisan file efisien.
- Mendukung download berukuran sangat besar.
- Tidak mengalami penurunan performa ketika Queue bertambah.

---

# 29.3 Performance Principles

## Performance First

Seluruh fitur harus mempertimbangkan performa sebelum estetika.

---

## Non-Blocking UI

Tidak ada proses berat yang dijalankan di UI Thread.

Seluruh pekerjaan dilakukan oleh Rust Backend.

---

## Asynchronous Architecture

Semua operasi I/O menggunakan asynchronous programming.

Contoh:

- HTTP Request
- Disk Write
- Database
- File Verification

---

## Lazy Loading

Data hanya dimuat ketika diperlukan.

Contoh:

- History
- Statistics
- Notification
- Torrent Peer

---

## Resource Efficiency

Engine harus menggunakan CPU, RAM, dan Disk seminimal mungkin.

---

# 29.4 Startup Performance

Target Startup:

| Metric     | Target     |
| ---------- | ---------- |
| Cold Start | < 1 Second |
| Warm Start | < 500 ms   |
| UI Ready   | < 800 ms   |

Splash Screen tidak boleh digunakan hanya untuk memperlambat startup.

---

# 29.5 Memory Requirements

Target RAM:

| State               | Target   |
| ------------------- | -------- |
| Idle                | < 100 MB |
| 5 Active Downloads  | < 200 MB |
| 20 Active Downloads | < 400 MB |
| Large Queue         | < 600 MB |

Memory Leak tidak diperbolehkan.

---

# 29.6 CPU Requirements

Target CPU:

| State        | Target |
| ------------ | ------ |
| Idle         | < 1%   |
| 5 Downloads  | < 10%  |
| 20 Downloads | < 25%  |

CPU Usage harus bersifat adaptif.

---

# 29.7 Disk Performance

Disk Writer harus:

- Sequential Write.
- Buffered Write.
- Parallel Segment Merge.
- Low Fragmentation.

Tidak boleh melakukan write kecil secara berlebihan.

---

# 29.8 Network Performance

Engine harus mampu:

- Memanfaatkan bandwidth maksimal.
- Menyesuaikan jumlah koneksi.
- Mengurangi reconnect yang tidak perlu.
- Menggunakan Keep Alive.
- Mendukung HTTP/2 Multiplexing.
- Mendukung HTTP/3 apabila tersedia.

---

# 29.9 Download Performance

Engine harus:

- Adaptive Segmentation.
- Dynamic Thread Allocation.
- Automatic Retry.
- Automatic Resume.
- Connection Pooling.
- Smart Buffering.

Target:

Kecepatan download harus setara atau lebih baik dibanding aplikasi sekelas ketika server, jaringan, dan kondisi pengujian sama.

---

# 29.10 UI Performance

Target UI:

| Action        | Target   |
| ------------- | -------- |
| Open Window   | < 300 ms |
| Open Settings | < 300 ms |
| Open History  | < 300 ms |
| Search        | < 100 ms |
| Scroll        | 60 FPS   |
| Animation     | 60 FPS   |

Tidak boleh terjadi UI Freeze.

---

# 29.11 Queue Performance

Queue Manager harus mampu menangani:

- 10.000 Download
- 100 Queue Group
- Unlimited History (dengan paging)

Operasi Queue:

Tambah

Hapus

Sort

Filter

Search

harus tetap responsif.

---

# 29.12 Database Performance

SQLite harus:

- Menggunakan Index.
- Menggunakan Prepared Statement.
- Menggunakan Transaction.
- Menghindari Query Berulang.

Target:

| Operation | Target   |
| --------- | -------- |
| Insert    | < 20 ms  |
| Update    | < 20 ms  |
| Delete    | < 20 ms  |
| Search    | < 100 ms |

---

# 29.13 Browser Integration Performance

Target:

Browser Extension

Memory

< 30 MB

CPU

< 1%

URL Transfer

< 100 ms

Native Messaging

< 200 ms

---

# 29.14 Video Downloader Performance

Target:

Analisis Video

< 2 s

Playlist

< 5 s (tergantung ukuran)

Subtitle Detection

< 2 s

Merge Audio Video

Menggunakan FFmpeg tanpa membekukan UI.

---

# 29.15 Torrent Performance

Target:

Metadata

< 5 s

Piece Verification

Berjalan di background

Resume

< 2 s

Peer Update

Realtime

---

# 29.16 Search Performance

Search harus mampu mencari:

Downloads

History

Settings

Notification

Torrent

Scheduler

Target:

Hasil pertama muncul dalam waktu kurang dari 100 ms.

---

# 29.17 Statistics Performance

Statistics harus dihitung secara bertahap (incremental).

Tidak boleh menghitung ulang seluruh data setiap kali halaman dibuka.

---

# 29.18 Scalability

Aplikasi harus tetap stabil ketika:

- Download >100 GB
- Queue >10.000 item
- History >100.000 entri
- Notification >50.000 entri

Menggunakan:

- Pagination
- Lazy Loading
- Virtual Scrolling

---

# 29.19 Benchmark Targets

Performa dibandingkan dengan aplikasi sejenis.

Target internal:

- Startup lebih cepat.
- Penggunaan RAM lebih rendah.
- Penggunaan CPU lebih rendah.
- UI lebih responsif.
- Kecepatan download setara atau lebih baik apabila kondisi jaringan dan server identik.

---

# 29.20 Performance Monitoring

Aplikasi menyediakan halaman Performance Monitor.

Menampilkan:

- CPU Usage
- RAM Usage
- Disk Write
- Disk Read
- Network Speed
- Active Thread
- Queue Status
- Buffer Usage

Update setiap 1 detik.

---

# 29.21 Functional Requirements

### PF-001

Seluruh operasi jaringan berjalan secara asynchronous.

---

### PF-002

UI tidak boleh terblokir oleh proses download.

---

### PF-003

Memory harus dibebaskan setelah download selesai.

---

### PF-004

Queue harus tetap responsif meskipun berisi ribuan item.

---

### PF-005

Progress download diperbarui secara real-time tanpa menyebabkan penurunan FPS.

---

### PF-006

Engine harus menyesuaikan jumlah segment secara dinamis sesuai kondisi server dan jaringan.

---

### PF-007

Operasi database tidak boleh menyebabkan jeda yang terlihat pada antarmuka.

---

### PF-008

Aplikasi harus mampu melanjutkan download setelah restart tanpa perlu memuat ulang seluruh metadata.

---

# 29.22 Non-Functional Requirements

| Requirement        | Target   |
| ------------------ | -------- |
| Startup Time       | < 1 s    |
| UI Response        | < 100 ms |
| Search             | < 100 ms |
| Queue Update       | < 100 ms |
| Database Query     | < 20 ms  |
| Idle RAM           | < 100 MB |
| Idle CPU           | < 1%     |
| Scroll Performance | 60 FPS   |
| Animation          | 60 FPS   |
| Crash Rate         | < 0.1%   |

---

# 29.23 Performance Testing

Sebelum setiap rilis dilakukan pengujian:

### Startup Test

Mengukur waktu startup pada perangkat spesifikasi minimum.

---

### Stress Test

- 100 download aktif.
- 10.000 queue.
- 100 GB file.

---

### Endurance Test

Download berjalan terus menerus selama 24 jam.

---

### Memory Leak Test

Memastikan penggunaan RAM stabil setelah penggunaan jangka panjang.

---

### Network Test

Pengujian pada:

- Wi-Fi
- Ethernet
- Hotspot
- Jaringan lambat
- Jaringan tidak stabil

---

### Disk Test

Pengujian pada:

- SSD SATA
- NVMe SSD
- HDD

---

# 29.24 Performance Optimization Strategy

Strategi optimasi meliputi:

- Adaptive Segmentation.
- Smart Retry.
- Connection Pooling.
- Lazy Loading.
- Virtual List Rendering.
- Incremental Statistics.
- Buffered Disk Writing.
- Asynchronous Database Access.
- Background File Verification.

Seluruh optimasi harus dapat dinonaktifkan untuk keperluan debugging.

---

# 29.25 Acceptance Criteria

Modul Performance dianggap selesai apabila:

- Startup memenuhi target waktu.
- Penggunaan CPU dan RAM berada dalam batas yang ditentukan.
- UI tetap responsif saat menjalankan banyak download.
- Queue tetap cepat meskipun berisi ribuan item.
- Download Engine mampu memanfaatkan bandwidth secara optimal.
- Tidak ditemukan memory leak pada pengujian endurance.
- Pengujian performa pada perangkat spesifikasi minimum dan rekomendasi berhasil memenuhi target yang telah ditentukan.

# 30. Error Handling

## 30.1 Overview

Error Handling merupakan mekanisme yang memastikan setiap kesalahan yang terjadi di dalam Velocity Download Manager (VDM) dapat dideteksi, diklasifikasikan, ditangani, dicatat, dan disampaikan kepada pengguna dengan cara yang jelas tanpa menyebabkan aplikasi berhenti bekerja.

Kesalahan harus diperlakukan sebagai kondisi yang dapat dipulihkan (recoverable) apabila memungkinkan. Aplikasi tidak boleh berhenti secara tiba-tiba hanya karena satu proses download mengalami kegagalan.

---

# 30.2 Objectives

Error Handling memiliki tujuan berikut:

- Menjaga stabilitas aplikasi.
- Mencegah kehilangan data download.
- Memulihkan proses secara otomatis bila memungkinkan.
- Memberikan informasi yang mudah dipahami.
- Membantu proses debugging.
- Mengurangi kebutuhan intervensi pengguna.

---

# 30.3 Error Handling Principles

## Fail Gracefully

Jika terjadi kesalahan, hanya proses yang bermasalah yang dihentikan. Modul lain tetap berjalan normal.

---

## Automatic Recovery

Sistem harus mencoba memulihkan kondisi secara otomatis sebelum meminta tindakan pengguna.

---

## Clear Communication

Pesan kesalahan harus menjelaskan:

- Apa yang terjadi.
- Mengapa hal itu terjadi (jika diketahui).
- Apa yang dapat dilakukan pengguna.

---

## No Silent Failure

Tidak boleh ada kesalahan yang diabaikan tanpa pencatatan atau pemberitahuan yang sesuai.

---

## Actionable Errors

Setiap error yang memungkinkan harus menyediakan tindakan seperti:

- Retry
- Resume
- Open Settings
- View Details
- Open Log

---

# 30.4 Error Categories

## Network Errors

Contoh:

- Connection Timeout
- Connection Refused
- DNS Failure
- TLS Handshake Failed
- Host Unreachable
- HTTP 404
- HTTP 500
- HTTP 503

---

## Download Errors

Contoh:

- Resume Not Supported
- Segment Failed
- Checksum Failed
- File Corrupted
- Download Cancelled
- Invalid URL

---

## File System Errors

Contoh:

- Disk Full
- Folder Not Found
- Permission Denied
- File Already Exists
- Read Only Folder

---

## Browser Integration Errors

Contoh:

- Extension Not Installed
- Native Messaging Failed
- Browser Not Supported
- Invalid Browser Message

---

## Video Downloader Errors

Contoh:

- Unsupported Video
- FFmpeg Missing
- yt-dlp Error
- Subtitle Not Available
- Playlist Unavailable

---

## Torrent Errors

Contoh:

- Invalid Torrent File
- Magnet Invalid
- Tracker Failed
- Peer Not Found
- Metadata Timeout

---

## Database Errors

Contoh:

- SQLite Locked
- Database Corrupted
- Migration Failed

---

## Internal Errors

Contoh:

- Unexpected Exception
- Panic
- Invalid State
- Configuration Error

---

# 30.5 Error Severity

| Level    | Description                                 |
| -------- | ------------------------------------------- |
| Info     | Informasi umum                              |
| Warning  | Tidak menghentikan proses                   |
| Error    | Proses gagal tetapi aplikasi tetap berjalan |
| Critical | Membutuhkan perhatian segera                |

---

# 30.6 Error Codes

Seluruh error harus memiliki kode unik.

Contoh:

| Code    | Description                   |
| ------- | ----------------------------- |
| NET-001 | Connection Timeout            |
| NET-002 | DNS Failure                   |
| NET-003 | Certificate Validation Failed |
| DLD-001 | Resume Not Supported          |
| DLD-002 | Checksum Failed               |
| FS-001  | Disk Full                     |
| FS-002  | Permission Denied             |
| BRW-001 | Extension Missing             |
| VID-001 | FFmpeg Not Found              |
| TOR-001 | Invalid Torrent               |
| DB-001  | Database Locked               |
| SYS-001 | Internal Error                |

---

# 30.7 Error Dialog

Error dialog harus terdiri dari:

- Icon
- Error Title
- Description
- Error Code
- Suggested Solution
- Technical Details (expandable)
- Action Buttons

Contoh:

Actions:

- Retry
- Resume
- Open Folder
- View Log
- Close

---

# 30.8 Automatic Recovery

Sistem harus mencoba pemulihan otomatis untuk:

- Network Timeout
- Temporary Server Error
- Lost Internet Connection
- Browser Reconnection
- Native Messaging Restart

Recovery dilakukan menggunakan strategi retry adaptif.

---

# 30.9 Retry Strategy

Default:

- Maximum Retry: 5
- Initial Delay: 5 detik
- Exponential Backoff
- Retry dapat dihentikan oleh pengguna

Retry hanya dilakukan pada error yang bersifat sementara.

---

# 30.10 Download Recovery

Jika proses download terganggu:

- Simpan progress.
- Simpan metadata.
- Simpan informasi segment.
- Pulihkan secara otomatis saat aplikasi dibuka kembali.

---

# 30.11 User Messages

Pesan error harus menggunakan bahasa yang mudah dipahami.

Contoh:

❌ Tidak baik:

"Socket Error 10054"

✅ Lebih baik:

"Koneksi ke server terputus. VDM akan mencoba menyambung kembali secara otomatis."

Technical Details tersedia melalui tombol **Show Details**.

---

# 30.12 Recovery Suggestions

Untuk setiap kategori error, aplikasi harus memberikan saran.

Contoh:

Disk Full

Saran:

- Kosongkan ruang penyimpanan.
- Pilih folder download lain.

---

Connection Timeout

Saran:

- Periksa koneksi internet.
- Coba lagi beberapa saat.

---

Checksum Failed

Saran:

- Ulangi proses download.
- Bandingkan checksum dengan sumber resmi.

---

# 30.13 Error History

Semua error disimpan pada Error History.

Informasi:

- Timestamp
- Module
- Error Code
- Description
- Severity
- Recovery Status

Pengguna dapat:

- Search
- Filter
- Export
- Clear History

---

# 30.14 Crash Recovery

Jika aplikasi mengalami crash:

Saat dibuka kembali:

- Restore Queue
- Restore Downloads
- Restore Window Layout
- Restore Scheduler
- Restore Statistics

Pengguna diberi pilihan untuk melanjutkan proses sebelumnya.

---

# 30.15 Safe Mode

Jika crash terjadi berulang kali:

VDM menawarkan Safe Mode.

Safe Mode akan:

- Menonaktifkan Browser Integration.
- Menonaktifkan Plugin (Future).
- Menjalankan konfigurasi minimum.
- Membuka halaman Diagnostics.

---

# 30.16 Diagnostics

Menu Diagnostics menampilkan:

- System Information
- Rust Version
- Tauri Version
- Windows Version
- RAM
- CPU
- Disk Space
- Active Configuration
- Recent Errors

Pengguna dapat mengekspor laporan diagnostik.

---

# 30.17 Functional Requirements

### EH-001

Semua error harus memiliki Error Code.

---

### EH-002

Error harus dicatat ke dalam log.

---

### EH-003

Pengguna harus menerima pesan yang mudah dipahami.

---

### EH-004

Technical Details harus tersedia tetapi disembunyikan secara default.

---

### EH-005

Recovery otomatis harus dicoba untuk error yang dapat dipulihkan.

---

### EH-006

Crash tidak boleh menyebabkan hilangnya progress download yang telah tersimpan.

---

### EH-007

Pengguna dapat melihat riwayat error.

---

### EH-008

Safe Mode harus tersedia ketika aplikasi gagal berjalan secara normal beberapa kali berturut-turut.

---

# 30.18 Non-Functional Requirements

| Requirement                   | Target   |
| ----------------------------- | -------- |
| Error Detection               | < 100 ms |
| Error Dialog Display          | < 300 ms |
| Retry Start                   | < 1 s    |
| Crash Recovery Initialization | < 2 s    |
| Error History Search          | < 100 ms |

---

# 30.19 Testing Requirements

Pengujian wajib meliputi:

- Internet diputus saat download.
- Server mengembalikan HTTP 404.
- Server mengembalikan HTTP 500.
- Disk penuh.
- Folder tujuan dihapus saat download berlangsung.
- Database dikunci.
- Browser Extension tidak tersedia.
- FFmpeg tidak ditemukan.
- Torrent rusak.
- Aplikasi ditutup paksa saat download aktif.
- Restart Windows saat download berlangsung.

Setiap skenario harus menghasilkan perilaku yang sesuai tanpa menyebabkan kerusakan data.

---

# 30.20 Acceptance Criteria

Modul Error Handling dianggap selesai apabila:

- Seluruh error memiliki kode yang unik.
- Pesan error mudah dipahami oleh pengguna umum.
- Error dapat dipulihkan secara otomatis bila memungkinkan.
- Tidak ada crash yang menyebabkan kehilangan data download.
- Riwayat error tersedia untuk analisis.
- Safe Mode dapat dijalankan ketika aplikasi gagal memulai secara normal.
- Error dialog konsisten di seluruh modul aplikasi.

# 30.21 Self-Healing Engine

## 30.21.1 Overview

Self-Healing Engine merupakan sistem otomatis yang memonitor seluruh komponen Velocity Download Manager (VDM), mendeteksi anomali, menganalisis penyebab masalah, dan melakukan tindakan pemulihan tanpa memerlukan intervensi pengguna.

Tujuan utama modul ini adalah meminimalkan kegagalan download, meningkatkan stabilitas aplikasi, dan mengurangi kebutuhan pengguna untuk melakukan konfigurasi atau troubleshooting secara manual.

---

# 30.21.2 Objectives

Self-Healing Engine harus mampu:

- Mendeteksi masalah secara otomatis.
- Menganalisis penyebab kegagalan.
- Menentukan tindakan pemulihan terbaik.
- Menjalankan recovery tanpa mengganggu pengguna.
- Memastikan proses download tetap berjalan jika memungkinkan.
- Mengurangi jumlah download yang gagal.

---

# 30.21.3 Architecture

```text
Download Engine
        │
        ▼
Health Monitor
        │
        ▼
Problem Detector
        │
        ▼
Recovery Engine
        │
        ▼
Action Executor
        │
        ▼
Verification
        │
        ▼
Continue Download
```

Self-Healing Engine bekerja sebagai layanan internal yang berjalan selama aplikasi aktif.

---

# 30.21.4 Health Monitor

Health Monitor melakukan pemantauan berkala terhadap:

### Network

- Internet Connection
- Latency
- Packet Loss
- DNS Resolution
- HTTP Status
- HTTP Version
- QUIC Availability

---

### Download Engine

- Active Threads
- Download Speed
- Retry Count
- Segment Status
- Queue Length
- Worker Status

---

### Storage

- Free Space
- Write Speed
- Read Speed
- Temporary Folder
- Disk Errors

---

### Memory

- RAM Usage
- Memory Pressure
- Buffer Usage

---

### CPU

- CPU Usage
- Thread Count
- Worker Load

---

### Browser Extension

- Connection Status
- Native Messaging
- Extension Version

---

# 30.21.5 Problem Detection

Self-Healing Engine harus mampu mendeteksi:

- Download berhenti tanpa alasan.
- Kecepatan turun drastis.
- Segment macet.
- Thread berhenti merespons.
- Queue tidak berjalan.
- Browser Extension terputus.
- Native Messaging gagal.
- Disk hampir penuh.
- Temporary Folder tidak dapat diakses.
- Database terkunci.
- File sementara rusak.

---

# 30.21.6 Recovery Actions

Jika terjadi masalah, sistem dapat melakukan tindakan berikut.

## Network Recovery

- Reconnect.
- DNS Lookup ulang.
- Ganti HTTP Version.
- Kurangi jumlah koneksi.
- Tambah jumlah koneksi bila memungkinkan.
- Restart Connection Pool.

---

## Download Recovery

- Restart Segment.
- Merge ulang segment.
- Verifikasi ulang metadata.
- Resume otomatis.
- Restart Worker tertentu tanpa mengganggu worker lain.

---

## Queue Recovery

- Susun ulang antrean.
- Lewati item yang rusak.
- Jalankan item berikutnya.
- Sinkronkan ulang status queue.

---

## Browser Recovery

- Restart Native Messaging.
- Hubungkan kembali Browser Extension.
- Validasi ulang komunikasi.
- Muat ulang konfigurasi browser.

---

## Storage Recovery

- Bersihkan cache sementara.
- Pindahkan temporary file jika diperlukan.
- Hentikan download jika ruang penyimpanan tidak mencukupi.
- Sarankan lokasi penyimpanan alternatif kepada pengguna.

---

## Database Recovery

- Buka kembali koneksi database.
- Ulangi transaksi yang gagal.
- Jalankan pemeriksaan integritas database.
- Pulihkan data dari checkpoint bila tersedia.

---

# 30.21.7 Adaptive Recovery

Recovery dilakukan bertahap.

Level 1

Retry sederhana.

↓

Level 2

Restart worker.

↓

Level 3

Bangun ulang koneksi.

↓

Level 4

Restart Download Engine.

↓

Level 5

Minta tindakan pengguna.

Pendekatan bertahap ini mencegah tindakan yang terlalu agresif ketika masalah sebenarnya sederhana.

---

# 30.21.8 Recovery Policy

Recovery harus mengikuti aturan berikut.

### Maximum Retry

Default:

5

---

### Recovery Interval

Exponential Backoff

---

### Cooldown

Recovery yang sama tidak boleh dijalankan terus-menerus tanpa jeda.

---

### Abort Policy

Recovery dihentikan jika:

- Pengguna membatalkan download.
- Server secara eksplisit menolak koneksi.
- File tidak lagi tersedia.

---

# 30.21.9 Learning Strategy

Self-Healing Engine menyimpan statistik lokal mengenai:

- Error yang sering terjadi.
- Recovery yang berhasil.
- Recovery yang gagal.
- Waktu rata-rata pemulihan.

Data ini digunakan untuk mengoptimalkan strategi pemulihan pada perangkat yang sama.

Seluruh data tetap berada di perangkat pengguna dan tidak dikirim ke server.

---

# 30.21.10 User Interaction

Secara bawaan:

Recovery berjalan otomatis.

Pengguna hanya diberi notifikasi jika:

- Recovery gagal.
- Dibutuhkan keputusan pengguna.
- Risiko kehilangan data.

---

# 30.21.11 Recovery Report

Setelah recovery selesai.

VDM dapat menampilkan:

Recovery Completed

Problem:

Network Timeout

Action:

Restart Connection Pool

Result:

Success

Duration:

3 Seconds

---

# 30.21.12 Functional Requirements

### SH-001

Health Monitor harus berjalan selama aplikasi aktif.

---

### SH-002

Problem Detection harus mampu mengenali kegagalan umum tanpa konfigurasi tambahan.

---

### SH-003

Recovery Engine harus mencoba tindakan otomatis sebelum menampilkan dialog error kepada pengguna.

---

### SH-004

Recovery tidak boleh menghentikan download lain yang masih berjalan normal.

---

### SH-005

Recovery harus dicatat ke dalam sistem logging.

---

### SH-006

Pengguna dapat menonaktifkan Self-Healing Engine melalui Settings.

---

### SH-007

Pengguna dapat melihat riwayat recovery yang telah dilakukan.

---

# 30.21.13 Non-Functional Requirements

| Requirement           | Target                                    |
| --------------------- | ----------------------------------------- |
| Problem Detection     | < 1 s                                     |
| Recovery Start        | < 2 s                                     |
| Recovery Success Rate | > 95% (untuk kasus yang dapat dipulihkan) |
| Health Monitor CPU    | < 0.5%                                    |
| Health Monitor RAM    | < 20 MB                                   |

---

# 30.21.14 Acceptance Criteria

Self-Healing Engine dianggap selesai apabila:

- Mendeteksi masalah secara otomatis.
- Menjalankan recovery tanpa mengganggu download lain.
- Berhasil memulihkan sebagian besar gangguan jaringan sementara.
- Menyimpan riwayat recovery untuk keperluan diagnosis.
- Tidak menyebabkan peningkatan penggunaan CPU atau RAM yang signifikan.
- Memberikan notifikasi yang jelas ketika recovery berhasil maupun gagal.

# 31. Logging

## Overview

Logging digunakan untuk mencatat seluruh aktivitas penting aplikasi guna membantu debugging, monitoring, audit, dan analisis performa.

### Log Categories

- Application
- Download Engine
- Browser Integration
- Video Downloader
- Torrent
- Queue
- Scheduler
- Network
- Security
- Error
- Self-Healing

### Log Levels

- Trace
- Debug
- Info
- Warning
- Error
- Critical

### Features

- Real-time logging
- Search & Filter
- Export Log
- Automatic Log Rotation
- Configurable Log Level

### Requirements

- Tidak menyimpan password, token, atau cookie.
- Log disimpan dalam format terstruktur.
- Mendukung Debug Mode dan Production Mode.

---

# 32. Intelligent Download Optimizer (IDO)

## Overview

IDO merupakan mesin optimasi otomatis yang mengatur strategi download berdasarkan kondisi server, jaringan, dan perangkat secara real-time.

### Responsibilities

- Adaptive Thread Management
- Dynamic Segment Allocation
- Smart Buffer Management
- Connection Optimization
- Automatic Retry Strategy
- Bandwidth Optimization

### Inputs

- CPU Usage
- RAM Usage
- Network Speed
- Server Capability
- File Size
- Queue Status

### Outputs

- Optimal Thread Count
- Optimal Segment Count
- Buffer Size
- Retry Strategy
- Connection Strategy

### Goals

- Kecepatan maksimal.
- RAM rendah.
- CPU rendah.
- Stabil pada jaringan tidak stabil.
- Tanpa konfigurasi manual.

---

# 33. Download Intelligence Engine (DIE)

## Overview

DIE menganalisis URL sebelum download dimulai untuk menentukan strategi download terbaik.

### Analysis

- URL Validation
- Redirect Detection
- Resume Support
- HTTP Version
- File Size
- MIME Type
- Server Capability
- Download Risk
- Estimated Speed

### Decision Engine

Menentukan secara otomatis:

- Jumlah Thread
- Jumlah Segment
- Buffer Size
- Download Category
- Destination Folder
- Retry Policy

### Benefits

- Download lebih cepat.
- Risiko gagal lebih rendah.
- Konfigurasi otomatis.
- Pengalaman pengguna lebih sederhana.

---

# 34. System Health Monitor (SHM)

## Overview

SHM memonitor kesehatan seluruh komponen aplikasi secara real-time dan menjadi dasar bagi Self-Healing Engine serta Intelligent Download Optimizer.

### Monitoring

#### System

- CPU
- RAM
- Disk
- Temperature (Future)

#### Network

- Download Speed
- Upload Speed
- Latency
- Packet Loss
- DNS Status

#### Download Engine

- Active Downloads
- Queue
- Thread Count
- Buffer Usage
- Retry Count

#### Browser

- Extension Status
- Native Messaging
- Browser Connection

#### Database

- SQLite Status
- Query Performance
- Storage Size

### Dashboard

Menampilkan:

- Health Score
- Performance Score
- Network Quality
- Active Warnings
- Recovery Status

### Health Status

- Excellent
- Good
- Warning
- Critical

### Automatic Actions

Jika masalah terdeteksi, SHM dapat:

- Memanggil Self-Healing Engine.
- Mengoptimalkan Download Engine.
- Mengurangi Thread.
- Membersihkan Cache.
- Mengulang koneksi.
- Memberikan rekomendasi kepada pengguna.

### Goals

- Menjaga aplikasi tetap stabil.
- Mencegah penurunan performa.
- Mengurangi kegagalan download.
- Memberikan diagnosis secara real-time.

---

# Integration Architecture

```text
Browser Extension
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
System Health Monitor
        │
        ▼
Self-Healing Engine
        │
        ▼
Queue Manager
        │
        ▼
File Manager
```

# Acceptance Criteria

- Logging mencatat seluruh aktivitas penting tanpa menyimpan data sensitif.
- IDO mampu mengoptimalkan proses download secara otomatis.
- DIE mampu menganalisis URL dan menentukan strategi terbaik sebelum download dimulai.
- SHM memonitor seluruh komponen aplikasi secara real-time dan bekerja sama dengan Self-Healing Engine untuk menjaga stabilitas sistem.

# 35. System Architecture

## 35.1 Overview

System Architecture mendefinisikan struktur keseluruhan Velocity Download Manager (VDM), hubungan antar modul, alur data, serta tanggung jawab masing-masing komponen. Arsitektur menggunakan pendekatan **Modular Architecture** agar mudah dikembangkan, diuji, dan dipelihara.

---

# 35.2 Architecture Principles

- Modular
- Scalable
- High Performance
- Secure by Design
- Low Resource Usage
- Event-Driven
- Multi-threaded
- Extensible

---

# 35.3 High-Level Architecture

```text
                    Browser Extension
                           │
                           ▼
                  Native Messaging Host
                           │
                           ▼
+-----------------------------------------------------------+
|                    Tauri Desktop UI                        |
|-----------------------------------------------------------|
| Dashboard | Downloads | Video | Torrent | Settings | Log |
+-----------------------------------------------------------+
                           │
                           ▼
+-----------------------------------------------------------+
|                 Application Service Layer                  |
|-----------------------------------------------------------|
| Settings | Notification | Scheduler | Queue | History     |
+-----------------------------------------------------------+
                           │
                           ▼
+-----------------------------------------------------------+
|                Core Intelligence Layer                     |
|-----------------------------------------------------------|
| Download Intelligence Engine (DIE)                        |
| Intelligent Download Optimizer (IDO)                      |
| Self-Healing Engine (SHE)                                 |
| System Health Monitor (SHM)                               |
+-----------------------------------------------------------+
                           │
                           ▼
+-----------------------------------------------------------+
|                   Rust Download Engine                     |
|-----------------------------------------------------------|
| HTTP | HTTPS | HTTP/2 | HTTP/3 | FTP | SFTP | Torrent     |
| Segment Manager | Connection Manager | Resume Engine       |
| Retry Engine | Buffer Manager | File Writer               |
+-----------------------------------------------------------+
                           │
                           ▼
+-----------------------------------------------------------+
|                  External Components                       |
|-----------------------------------------------------------|
| yt-dlp | FFmpeg | SQLite | Windows API                    |
+-----------------------------------------------------------+
```

---

# 35.4 Module Responsibilities

## Desktop UI (Tauri)

Bertanggung jawab terhadap:

- User Interface
- User Interaction
- Theme
- Dashboard
- Dialog
- Settings
- Notification Center

Tidak melakukan proses download secara langsung.

---

## Browser Extension

Bertanggung jawab:

- Capture Download
- Detect Media
- Native Messaging
- Send URL
- Context Menu

---

## Application Service Layer

Mengelola:

- Queue
- Scheduler
- History
- Notification
- Settings
- Statistics

---

## Core Intelligence Layer

Mengambil keputusan otomatis.

Komponen:

### Download Intelligence Engine

Analisis URL dan server.

---

### Intelligent Download Optimizer

Optimasi thread, segment, dan koneksi.

---

### Self-Healing Engine

Pemulihan otomatis ketika terjadi gangguan.

---

### System Health Monitor

Monitoring performa sistem dan aplikasi.

---

## Rust Download Engine

Merupakan inti aplikasi.

Komponen:

- URL Analyzer
- Connection Manager
- Segment Manager
- Download Worker
- Resume Engine
- Retry Engine
- Buffer Manager
- Disk Writer
- Integrity Checker

---

# 35.5 Data Flow

```text
Paste URL
      │
      ▼
Browser Extension / Manual Input
      │
      ▼
Download Intelligence Engine
      │
      ▼
Queue Manager
      │
      ▼
Download Optimizer
      │
      ▼
Rust Download Engine
      │
      ▼
Disk Writer
      │
      ▼
Checksum Verification
      │
      ▼
History
      │
      ▼
Notification
```

---

# 35.6 Internal Communication

Komunikasi antar modul menggunakan Event Bus.

Contoh Event:

- DownloadCreated
- DownloadStarted
- DownloadPaused
- DownloadCompleted
- DownloadFailed
- QueueUpdated
- SchedulerExecuted
- SettingsChanged
- NetworkChanged

Setiap modul hanya merespons event yang relevan.

---

# 35.7 Database Layer

SQLite digunakan untuk menyimpan:

- Settings
- Download Queue
- Download History
- Scheduler
- Statistics
- Notification History
- Error History
- Recovery History

---

# 35.8 External Dependencies

| Component   | Purpose            |
| ----------- | ------------------ |
| Rust        | Core Engine        |
| Tauri       | Desktop Framework  |
| SQLite      | Database           |
| yt-dlp      | Video Download     |
| FFmpeg      | Media Processing   |
| Windows API | System Integration |

---

# 35.9 Module Dependency Rules

- UI tidak boleh mengakses database secara langsung.
- UI tidak boleh mengakses Download Engine secara langsung.
- Semua komunikasi melalui Service Layer.
- Download Engine tidak mengetahui implementasi UI.
- Core Intelligence Layer tidak bergantung pada UI.
- Browser Extension hanya berkomunikasi melalui Native Messaging.

---

# 35.10 Scalability

Arsitektur harus mendukung penambahan:

- Browser baru
- Protocol baru
- Video Provider baru
- Plugin
- Cloud Download
- AI Features
- Mobile Companion
- Remote Management

Tanpa mengubah modul inti.

---

# 35.11 Future Architecture

Roadmap pengembangan:

Version 1

- Download Engine
- Browser Extension
- Video Downloader
- Torrent
- Queue
- Scheduler

Version 2

- Plugin SDK
- Remote API
- CLI
- Advanced Analytics

Version 3

- AI Optimization
- Distributed Download
- Cloud Sync
- Team Workspace

---

# 35.12 Quality Attributes

Arsitektur harus memenuhi:

- Availability
- Reliability
- Performance
- Security
- Maintainability
- Testability
- Scalability
- Extensibility
- Portability

---

# 35.13 Acceptance Criteria

System Architecture dianggap selesai apabila:

- Setiap modul memiliki tanggung jawab yang jelas.
- Tidak terjadi ketergantungan melingkar (circular dependency).
- Seluruh komunikasi antar modul terdokumentasi.
- Download Engine dapat dikembangkan tanpa mengubah UI.
- UI dapat diperbarui tanpa mengubah Download Engine.
- Penambahan fitur baru dapat dilakukan melalui modul yang sesuai tanpa memengaruhi arsitektur inti.
