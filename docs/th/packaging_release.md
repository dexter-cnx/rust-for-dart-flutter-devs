# Packaging และ Release

[English](../packaging_release.md)

App กับ reusable package มี release model ต่างกัน ควรกำหนดว่าจะ build Rust จาก source หรือแจก prebuilt native binaries

บันทึก compatibility ของ Flutter/Dart, Rust toolchain, FRB runtime/codegen, Android NDK/minSdk/ABI และ Apple deployment targets

FRB v2 ปัจจุบันใช้ Cargokit เป็น default integration backend และมี Native Assets backend สำหรับ SDK ที่รองรับ ควรตรวจ upstream docs ทุกครั้งเมื่อ upgrade
