<details>
<summary>Tutorial Modul 6</summary>

# Commit 1 Reflection Notes

handle_connection function berfungsi untuk membaca data dari TCP stream dan mencetaknya agar kita bisa melihat data yang dikirim dari browser. 
    
Pada fungsi `handle_connection`, kita membuat sebuah instance baru dari BufReader yang mengelilingi sebuah mutable reference ke stream. BufReader menambahkan buffering dengan mengelola panggilan ke metode-metode trait `std::io::Read` untuk kita.

Kita membuat sebuah variabel bernama `http_request` untuk mengumpulkan baris-baris dari permintaan yang dikirimkan oleh browser ke server kita. Kita menunjukkan bahwa kita ingin mengumpulkan baris-baris ini dalam sebuah vector dengan menambahkan anotasi tipe `Vec<_>`.

BufReader mengimplementasikan trait `std::io::BufRead`, yang menyediakan metode lines. Metode lines mengembalikan sebuah iterator dari Result<String, std::io::Error> dengan memisahkan aliran data setiap kali melihat sebuah byte newline. Untuk mendapatkan setiap String, kita memetakan dan mem unwrap setiap Result. Result mungkin merupakan sebuah error jika data tidak valid dalam UTF-8 atau jika ada masalah dalam membaca dari stream. Sekali lagi, sebuah program produksi seharusnya menangani error-error ini dengan lebih bijak, tetapi kita memilih untuk menghentikan program dalam kasus error untuk kesederhanaan.

Browser menandakan akhir dari sebuah permintaan HTTP dengan mengirimkan dua karakter newline berturut-turut, jadi untuk mendapatkan satu permintaan dari stream, kita mengambil baris-baris sampai kita mendapatkan sebuah baris yang berupa string kosong. Setelah kita mengumpulkan baris-baris ke dalam vector, kita mencetaknya menggunakan format debug yang cantik agar kita dapat melihat instruksi-instruksi yang dikirimkan oleh web browser ke server kita.

# Commit 2 Reflection Notes

![Commit 2 screen capture](/assets/commit2.png)

# Commit 3 Reflection Notes

Saat ini, blok if dan else memiliki banyak pengulangan: keduanya membaca file dan menuliskan konten file ke stream. Satu-satunya perbedaan adalah baris status dan nama file. Mari buat kode lebih ringkas dengan menarik perbedaan-perbedaan tersebut ke dalam blok if dan else terpisah yang akan menetapkan nilai dari baris status dan nama file ke variabel; kemudian, variabel-variabel tersebut dapat digunakan tanpa syarat dalam kode untuk membaca file dan menulis respons. Berikut adalah hasilnya setelah mengganti blok if dan else yang besar
    <pre>
        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
        };
    </pre>

![Commit 3 screen capture](/assets/commit3.png)

# Commit 4 Reflection Notes

    <pre>
        let (status_line, filename) = match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"), 
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(10)); 
                ("HTTP/1.1 200 OK", "hello.html") 
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        };
    </pre>

Dari hasil menjalankan program, harus menunggu beberapa saat untuk load aplikasi, ini disimulasikan dengan `thread::sleep`, ini mensimulasikan jika aplikasi diakses oleh banyak pengguna, penyebab lainnya adalah karena ini adalah aplikasi single thread, jadi jika thread yang ada berhenti, maka seluruh akses ke aplikasi jadi harus menunggu. Saat yang mengakses aplikasi juga banyak, jika single thread maka akan memerlukan beberapa waktu untuk mengakses aplikasi, karena harus antri terlebih dahulu dengan request lainnya. 
</details>