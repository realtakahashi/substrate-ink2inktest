# やろうとしていること
- flipperコントラクトをデプロイする。
- call_flipperにflipperのアドレスを渡して、call_flippperからflipperをコールする。

# 現時点での問題　※2020/7/25
- call_flipperで、cargo +nightly contract generate-metadataを実行すると、以下のエラーが発生する。
```
error[E0277]: the trait bound `flipper::flipper::__ink_private::__ink_cross_calling::StorageAsDependency: type_metadata::HasTypeDef` is not satisfied
  --> /home/shin/ink_work/substrate-ink2inktest/call_flipper/lib.rs:12:5
   |
12 | /     /// Defines the storage of your contract.
13 | |     /// Add new fields to the below struct in order
14 | |     /// to add new static storage fields to your contract.
15 | |     #[ink(storage)]
...  |
20 | |         //lflipper: flipper::Flipper,
21 | |     }
   | |_____^ the trait `type_metadata::HasTypeDef` is not implemented for `flipper::flipper::__ink_private::__ink_cross_calling::StorageAsDependency`
   |
   = note: required because of the requirements on the impl of `type_metadata::Metadata` for `flipper::flipper::__ink_private::__ink_cross_calling::StorageAsDependency`
   = note: required because of the requirements on the impl of `type_metadata::HasTypeDef` for `ink_core::storage::Value<flipper::flipper::__ink_private::__ink_cross_calling::StorageAsDependency>`
   = note: required by `type_metadata::Metadata::meta_type`
   = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
```