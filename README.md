<details>
<summary>Tutorial Modul 6</summary>
<h3>Commit 1 Reflection Notes</h3>
handle_connection function berfungsi untuk membaca data dari TCP stream dan mencetaknya agar kita bisa melihat data yang dikirim dari browser. 
    
Pada fungsi `handle_connection`, kita membuat sebuah instance baru dari BufReader yang mengelilingi sebuah mutable reference ke stream. BufReader menambahkan buffering dengan mengelola panggilan ke metode-metode trait std::io::Read untuk kita.

Kita membuat sebuah variabel bernama `http_request` untuk mengumpulkan baris-baris dari permintaan yang dikirimkan oleh browser ke server kita. Kita menunjukkan bahwa kita ingin mengumpulkan baris-baris ini dalam sebuah vector dengan menambahkan anotasi tipe Vec<_>.

BufReader mengimplementasikan trait std::io::BufRead, yang menyediakan metode lines. Metode lines mengembalikan sebuah iterator dari Result<String, std::io::Error> dengan memisahkan aliran data setiap kali melihat sebuah byte newline. Untuk mendapatkan setiap String, kita memetakan dan mem unwrap setiap Result. Result mungkin merupakan sebuah error jika data tidak valid dalam UTF-8 atau jika ada masalah dalam membaca dari stream. Sekali lagi, sebuah program produksi seharusnya menangani error-error ini dengan lebih bijak, tetapi kita memilih untuk menghentikan program dalam kasus error untuk kesederhanaan.

Browser menandakan akhir dari sebuah permintaan HTTP dengan mengirimkan dua karakter newline berturut-turut, jadi untuk mendapatkan satu permintaan dari stream, kita mengambil baris-baris sampai kita mendapatkan sebuah baris yang berupa string kosong. Setelah kita mengumpulkan baris-baris ke dalam vector, kita mencetaknya menggunakan format debug yang cantik agar kita dapat melihat instruksi-instruksi yang dikirimkan oleh web browser ke server kita.

<h3>Commit 2 Reflection Notes</h3>

![Commit 2 screen capture](/assets/commit2.png)
</details>