# E-Finance Backend

Backend E-Finance
*  Programming Language : Scala
*  Database : PostgreSQL
*  Web Framework : http4s
*  Database access framework: Doobie

DB INSTALLATION  

```
gag_finance=# grant all privileges on database gag_finance to postgres;
GRANT
```

Query utk data isian verifikasi :  
```
SELECT 20, jenis, harga, kurs, diskon FROM (
  select 'Internal' as jenis UNION ALL select 'Buyer' UNION ALL select 'Independent'
) as jenis_verifikasi
LEFT JOIN (
  SELECT verifikasi, harga, kurs, diskon FROM detail_transaksi_penjualan
  WHERE id_transaksi=20) as list_detail ON jenis::verifikasi_t=verifikasi
```
