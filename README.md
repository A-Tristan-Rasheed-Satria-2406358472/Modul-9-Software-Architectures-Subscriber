# Subscriber Reflection Notes

## (1) What is amqp?

AMQP adalah singkatan dari `Advanced Message Queuing Protocol`. Ini adalah protokol komunikasi yang dipakai untuk pertukaran message antara aplikasi dan message broker, misalnya RabbitMQ.

Kalau dibayangin secara sederhana, AMQP itu kayak aturan lalu lintas untuk kirim-terima pesan. Jadi publisher bisa mengirim data ke broker dengan format dan cara yang sudah disepakati, lalu subscriber bisa menerima data itu dengan aturan yang sama.

Di program ini, AMQP dipakai supaya aplikasi publisher dan subscriber tidak perlu saling terhubung secara langsung. Publisher cukup kirim event ke broker, lalu subscriber tinggal mendengarkan event yang sesuai.

## (2) What does it mean? guest:guest@localhost:5672, what is the first guest, and what is the second guest, and what is localhost:5672 is for?

String `amqp://guest:guest@localhost:5672` adalah URL koneksi untuk terhubung ke message broker.

Kalau dipecah:

- `amqp://` menunjukkan bahwa protokol yang dipakai adalah AMQP
- `guest` yang pertama adalah username untuk login ke broker
- `guest` yang kedua adalah password dari user tersebut
- `localhost` berarti broker dijalankan di komputer yang sama dengan program
- `5672` adalah port default yang biasa dipakai AMQP untuk menerima koneksi client

Jadi kalau dibaca lengkap, artinya program mencoba login ke broker AMQP yang berjalan di mesin lokal, memakai username `guest`, password `guest`, dan menghubungi service itu lewat port `5672`.

- URL koneksi AMQP mirip format URL biasa, tapi isinya khusus untuk akses broker
- Username dan password dipakai untuk autentikasi
- `localhost` berarti koneksi diarahkan ke mesin sendiri
- Port `5672` adalah jalur komunikasi default untuk service AMQP

# Simulation slow subscriber

Pada percobaan saya, total queue (peak-nya) sempat mencapai sekitar **80**. Nilai ini bisa lebih besar dari contoh (misalnya 20) karena publisher dijalankan berkali-kali dengan cepat, sehingga laju message masuk jauh lebih tinggi daripada laju proses subscriber.

Di sisi lain, subscriber sengaja diperlambat dengan delay 1 detik per message, jadi message diproses satu per satu dengan throughput rendah. Akibatnya terjadi backlog: message baru terus menumpuk di queue sebelum sempat diproses, dan puncak jumlah queue menjadi tinggi (sekitar 80 pada mesin saya).

![# Simulation slow subscriber](public/image/simulate.png)
