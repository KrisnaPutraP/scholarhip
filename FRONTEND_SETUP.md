# Frontend Setup Guide

## Scholarship Matcher Frontend

Frontend ini telah berhasil diintegrasikan dengan HTML polos yang Anda berikan. Berikut adalah fitur-fitur yang sudah diimplementasikan:

### Fitur yang Sudah Diintegrasikan:

1. **UI Modern dengan Gradient Background**
   - Desain responsif dengan grid layout
   - Styling yang konsisten dengan tema biru-ungu
   - Animasi hover pada tombol

2. **Registrasi User**
   - Form lengkap untuk data mahasiswa
   - Integrasi dengan backend ICP
   - Fallback ke mode lokal jika backend tidak tersedia

3. **Tab System**
   - Tab "Register" untuk pendaftaran baru
   - Tab "My Profile" untuk melihat profil pengguna

4. **Scholarship Recommendations**
   - Tampilan list beasiswa dengan match score
   - Kriteria yang cocok dan yang kurang
   - Tombol untuk melihat detail dan bookmark

5. **Status Connection**
   - Indikator status koneksi ke backend ICP
   - Badge warna hijau/merah sesuai status

### Komponen Frontend:

- **App.js**: Main component dengan lit-html
- **index.scss**: Styling lengkap dengan responsive design
- **index.html**: HTML template yang bersih

### Backend Integration:

Frontend ini sudah diintegrasikan dengan backend Rust/Candid yang ada:
- `register_user()` - untuk registrasi pengguna
- `get_my_profile()` - untuk load profil
- `get_my_recommendations()` - untuk mendapatkan rekomendasi
- `bookmark_scholarship()` - untuk bookmark beasiswa

### Data Sample:

Jika backend tidak tersedia, aplikasi akan menggunakan data sample beasiswa:
- LPDP Scholarship 2025
- Chevening Scholarship 2025  
- Fulbright Scholarship 2025

### Cara Menjalankan:

1. Pastikan dfx sudah berjalan:
   ```bash
   dfx start --background
   ```

2. Deploy backend jika belum:
   ```bash
   dfx deploy scholarship_backend
   ```

3. Install dependencies dan jalankan frontend:
   ```bash
   cd src/scholarship_frontend
   npm install
   npm start
   ```

4. Buka browser di `http://localhost:3000`

### Notes:

- Frontend menggunakan lit-html untuk rendering
- Styling menggunakan SCSS dengan compile otomatis
- Responsive design untuk mobile dan desktop
- Error handling untuk offline mode

Frontend siap digunakan dan akan berfungsi baik dengan atau tanpa koneksi backend ICP!
