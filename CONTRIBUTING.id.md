# Panduan Kontribusi Rune (Bahasa Indonesia)

Panduan ini ditujukan bagi pengembang yang ingin berkontribusi pada repository Rune, baik untuk pembaruan fitur, perbaikan bug, maupun dokumentasi.

---

## 1. Persiapan Lingkungan

Rune adalah framework UI berbasis Rust dan GPU (`wgpu`). Untuk membangun dan menguji kode:

1. **Rust Toolchain**:
   - Disarankan menginstal via [rustup.rs](https://rustup.rs).
   - **Tanpa Visual Studio di Windows**: Anda dapat memilih target GNU saat instalasi (`x86_64-pc-windows-gnu`) agar tidak memerlukan instalasi Visual Studio Build Tools yang berat.
2. **Kontribusi Dokumentasi**:
   - Jika kontribusi berupa dokumentasi, perbaikan teks, atau terjemahan, instalasi compiler Rust bersifat opsional.

---

## 2. Alur Kerja Git & Branch

Sesuai aturan di `AGENTS.md`:
* **Branch Utama**: Semua pekerjaan berbasis pada branch **`dev`** (bukan `main` atau `master`).
* **Fast-Forward Only**: History branch `dev` tidak pernah di-force push / di-rewrite.
* **Branch Fitur Baru**: Buat branch baru dari `dev`:
  ```bash
  git checkout dev
  git pull origin dev
  git checkout -b nama-fitur-atau-perbaikan
  ```

---

## 3. Pengecekan & Pengujian

Sebelum membuat commit atau membuka Pull Request:
```bash
# Pengecekan syntax dan tipe
cargo check --workspace --all-targets

# Menjalankan unit test
cargo test -p rune -p rune-kit --features rune-kit/markdown
```

---

## 4. Format Commit & Changelog

* **Changelog Unreleased**: Jika perubahan mempengaruhi output aplikasi atau API, tambahkan file baru di `changelog/unreleased/<YYYY-MM-DD>-<slug>.md` dengan satu bullet point penjelasan.
* **Privasi**: Jangan menyertakan path lokal pribadi (misal: `/Users/...`, `C:\Users\...`) atau token rahasia di dalam commit.

---

## 5. Otomatisasi Upload ke GitHub

Untuk meng-upload commit secara otomatis ke GitHub setiap kali `git commit` dijalankan, gunakan Git Hook `.git/hooks/post-commit`:
```sh
#!/bin/sh
branch=$(git rev-parse --abbrev-ref HEAD)
echo "--> Mengupload commit ke GitHub (branch: $branch)..."
git push -u origin "$branch"
```

