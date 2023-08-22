-----
**INFO**

| NAME | CENTIMETER | METER |
|:---|:---:|:---:|
| diameter_roller                    | 14.8  cm | 0.148 m |
| diameter_gear_belt_encoder         | 10    cm | 0.1   m |
| diameter_gear_belt_roller_beban    | 5.5   cm | 0.55  m |
| jarak_antar_titik_tengah_gear_belt | 14.4  cm | 0.144 m |

-----

### $T = (F * r) / g$

dapat diterapkan pada informasi yang telah diberikan sebelumnya untuk
menghitung torsi pada sistem dynotest.


Dalam kasus ini, F adalah gaya yang dihasilkan oleh drum pemberat (18.5 kg) yang terhubung dengan
gear belt kedua dan roller kedua. Kita dapat menggunakan rumus F = m * a untuk menghitung gaya yang
dihasilkan oleh drum pemberat, di mana m adalah massa drum pemberat (18.5 kg) dan a adalah
percepatan drum pemberat.

Diketahui bahwa diameter drum roller adalah 14.8 cm, maka jari-jari roller dapat dihitung sebagai r
= 14.8 cm / 2 = 0.074 m. Jarak antara titik tengah gear belt adalah 17.4 cm = 0.174 m.

Kita juga dapat mengasumsikan bahwa gaya yang dihasilkan oleh drum pemberat seimbang dengan gaya
yang dihasilkan oleh sepeda motor saat diuji pada dynotest. Oleh karena itu, gaya F yang dihasilkan
oleh drum pemberat dapat dianggap sebagai gaya yang diterapkan pada sepeda motor.

Dalam hal ini, g dapat dianggap sama dengan 9.81 m/s^2, nilai percepatan gravitasi di permukaan
bumi.

Maka, torsi T yang dihasilkan oleh sepeda motor saat diuji pada dynotest dapat dihitung menggunakan
rumus:

### $T = (F * r) / g$

dengan nilai F dihitung terlebih dahulu menggunakan rumus F = m * a.

Untuk menghitung gaya (F), kita bisa menggunakan persamaan sebagai berikut:

### $F = m * a$

Di mana:
    m = massa beban (kg)
    a = percepatan linear dari beban (m/s^2)

Percepatan linear dari beban dapat dihitung sebagai:

### $a = r * α$

Di mana:
    r = radius dari drum roller (m)
    α = percepatan sudut dari drum roller (rad/s^2)

Percepatan sudut dari drum roller (α) dapat dihitung sebagai:

### $α = (2π * n) / t$

Di mana:
    n = kecepatan putaran drum roller (putaran/detik)
    t = waktu yang dibutuhkan untuk satu putaran drum roller (detik)

Dengan menggabungkan persamaan di atas, kita dapat menghitung gaya (F) yang diterapkan pada beban pada sistem dynotest.


------------------------------------------------------------------------------------

Untuk menghitung torsi dari sepeda motor yang diuji pada dynotest dengan sistem yang Anda sebutkan,
Anda perlu mengetahui beberapa parameter tambahan seperti kecepatan putaran (RPM) dari sepeda motor
dan rasio pengurangan antara tuas encoder dan tuas drum pemberat.

Langkah-langkah yang dapat dilakukan untuk menghitung torsi dari sepeda motor adalah sebagai
berikut:

Hitung jarak lintasan belt antara tuas encoder dan tuas drum pemberat dengan rumus:

### $S = π × D$

dimana S adalah jarak lintasan belt (m), dan D adalah diameter drum pemberat (m).

Karena Anda tidak memberikan nilai diameter drum pemberat, Anda perlu mengukurnya terlebih dahulu
atau mencari tahu informasi ini dari spesifikasi dynotest yang digunakan.

Hitung rasio pengurangan antara tuas encoder dan tuas drum pemberat dengan rumus:

### $GR = L / S$

dimana GR adalah rasio pengurangan, L adalah jarak antara tuas encoder dan tuas drum pemberat (m),
dan S adalah jarak lintasan belt yang telah dihitung pada langkah 1.

Hitung kecepatan putaran (RPM) sepeda motor dengan menggunakan alat pengukur RPM atau sensor
kecepatan.

Konversi RPM sepeda motor ke kecepatan sudut (ω) dalam satuan rad/s dengan rumus:

### $ω = RPM × 2π / 60$

Hitung frekuensi sinyal encoder dengan rumus:

### $f = N × ω / 360$

dimana f adalah frekuensi sinyal encoder (Hz), N adalah jumlah pulsa encoder dalam satu putaran (360
pulse), dan ω adalah kecepatan sudut sepeda motor dalam satuan rad/s yang telah dihitung pada
langkah 4.

Hitung momen inersia drum pemberat dengan rumus:

### $J = (1/2) × M × R^2$

dimana J adalah momen inersia drum pemberat (kg.m^2), M adalah massa drum pemberat (kg), dan R
adalah jari-jari drum pemberat (m).

Karena Anda tidak memberikan nilai jari-jari drum pemberat, Anda perlu mengukurnya terlebih dahulu
atau mencari tahu informasi ini dari spesifikasi dynotest yang digunakan.

Hitung torsi yang dihasilkan oleh sepeda motor dengan rumus:

### $T = (2π × f) / (GR × K) × J$

dimana T adalah torsi yang dihasilkan oleh sepeda motor (N.m), f adalah frekuensi sinyal encoder
yang telah dihitung pada langkah 5, GR adalah rasio pengurangan yang telah dihitung pada langkah 2,
J adalah momen inersia drum pemberat yang telah dihitung pada langkah 6, dan K adalah konstanta
kalibrasi untuk mengubah satuan pulsa encoder menjadi satuan torsi (N.m/pulse).

Konstanta kalibrasi (K) dapat dihitung dengan cara melakukan kalibrasi pada dynotest menggunakan
beban terukur (misalnya dengan timbangan) pada berbagai kecepatan putaran seped

kita dapat menghitung rasio pengurangan antara gear belt drum pemberat dan gear belt pada encoder
sebagai berikut:

Rasio pengurangan (GR) = radius gear belt pada encoder / radius gear belt drum pemberat

### $GR = 10 cm / 5.5 cm = 1.818$

Selanjutnya, kita dapat menggunakan persamaan yang sama seperti di atas untuk menghitung torsi yang
dihasilkan oleh sepeda motor. Namun, kita perlu menghitung nilai konstanta kalibrasi (K) terlebih
dahulu.

Untuk menghitung konstanta kalibrasi (K), kita dapat melakukan kalibrasi pada dynotest dengan
menggunakan beban terukur pada berbagai kecepatan putaran sepeda motor. Dalam contoh ini, kita akan
asumsikan bahwa kita telah melakukan kalibrasi dan menentukan bahwa K = 0,05 N.m/pulse.

Dengan menggunakan nilai GR dan K yang telah dihitung dan diasumsikan, langkah-langkah yang dapat
dilakukan untuk menghitung torsi dari sepeda motor adalah sebagai berikut:

Hitung jarak lintasan belt antara gear belt pada encoder dan gear belt drum pemberat dengan rumus:

#### $S = π × (diameter gear belt pada encoder + diameter gear belt drum pemberat) / 2$
#### $S = π × (10 cm + 5.5 cm) / 2$
#### $S = 17.03 cm atau 0.1703 m$

Hitung kecepatan sudut (ω) dalam satuan rad/s dengan rumus:

### $ω = RPM × 2π / 60$

Asumsikan bahwa sepeda motor berputar dengan kecepatan 5000 RPM. Maka,
#### $ω = 5000 RPM × 2π / 60$
#### $ω = 523.6 rad/s$

Hitung frekuensi sinyal encoder dengan rumus:

### $f = N × ω / 360$

Asumsikan bahwa encoder menghasilkan 360 pulsa per putaran. Maka,
#### $f = 360 × 523.6 / 360$
#### $f = 523.6 Hz$

Hitung momen inersia drum pemberat dengan rumus:

### $J = (1/2) × M × R^2$

Asumsikan bahwa drum pemberat memiliki jari-jari sebesar 0,15 m dan massa 18,5 kg. Maka,
#### $J = (1/2) × 18.5 kg × (0.15 m)^2$
#### $J = 0.20775 kg.m^2$

Hitung torsi yang dihasilkan oleh sepeda motor dengan rumus:

### $T = (2π × f) / (GR × K) × J$

#### $T = (2π × 523.6) / (1.818 × 0.05) × 0.20775$
#### $T = 74.2 N.m$

Jadi, torsi yang dihasilkan oleh sepeda motor adalah sebesar 74,2 N.m. Namun, perlu diingat bahwa
perhitungan ini merupakan hasil yang diasumsikan dengan beberapa asumsi tertentu dan nilai-nilai
yang diberikan mungkin perlu disesuaikan dengan kondisi sebenarnya pada dynotest yang digunakan.
