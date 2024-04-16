#![allow(
  clippy::cognitive_complexity,
  clippy::large_enum_variant,
  clippy::module_inception,
  clippy::needless_doctest_main
)]
#![warn(
  missing_debug_implementations,
//   missing_docs,
  rust_2021_compatibility,
  unreachable_pub
)]
#![deny(unused_must_use)]
#![doc(test(
  no_crate_inject,
  attr(
    deny(warnings, rust_2021_compatibility),
    allow(dead_code, unused_variables)
  )
))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![cfg_attr(loom, allow(dead_code, unreachable_pub))]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// # 对象克隆
/// ```toml
/// [dependencies]
/// serde_json = "1.0"
/// serde = { version = "1.0", features = ["derive"] }
/// e-macros = { version = "0.1", git="https://gitee.com/eternalnight996/e-macros"}
/// ```
/// # Example
///```rust
/// #[derive(serde::Deserialize, Debug, serde::Serialize)]
/// struct A {
///   a: String,
///   b: i32,
///   d: i32,
/// }
/// #[derive(serde::Deserialize, Debug, serde::Serialize, Default, e_utils::Json)]
/// struct B {
///   d: i32,
///   f: String,
/// }
/// fn test() {
///   let mut a: A = A {
///     a: "A".to_string(),
///     b: 1,
///     d: 2,
///   };
///   println!("A {:?}", a);
///   let mut b: B = B::auto_json_cloned(&a);
///   println!("B {:?}", b);
///   a.d = 10;
///   b.f = "test".to_string();
///   b = b.self_json_cloned(&a);
///   println!("B {:?}", b);
/// }
#[proc_macro_derive(Json)]
pub fn json_derive(input: TokenStream) -> TokenStream {
  // Parse the input tokens as a DeriveInput
  let input = parse_macro_input!(input as DeriveInput);

  // Get the name of the struct
  let struct_name = &input.ident;

  // Extract the fields of the struct
  // let _fields = if let syn::Data::Struct(syn::DataStruct {
  //   fields: syn::Fields::Named(fields),
  //   ..
  // }) = &input.data
  // {
  //   fields.named.iter().map(|f| &f.ident).collect::<Vec<_>>()
  // } else {
  //   panic!("This macro only supports structs with named fields");
  // };

  // Create the output tokens
  let expanded = quote! {
      // Implement the trait for the struct
      impl #struct_name {
          /// ```toml
          /// [dependencies]
          /// serde_json = "1.0"
          /// serde = { version = "1.0", features = ["derive"] }
          /// e-macros = { version = "0.1", git="https://gitee.com/eternalnight996/e-macros"}
          /// ```
          /// # Example
          ///```rust
          /// #[derive(serde::Deserialize, Debug, serde::Serialize)]
          /// struct A {
          ///   a: String,
          ///   b: i32,
          ///   d: i32,
          /// }
          /// #[derive(serde::Deserialize, Debug, serde::Serialize, Default, e_utils::Json)]
          /// struct B {
          ///   d: i32,
          ///   f: String,
          /// }
          /// fn test() {
          ///   let mut a: A = A {
          ///     a: "A".to_string(),
          ///     b: 1,
          ///     d: 2,
          ///   };
          ///   println!("A {:?}", a);
          ///   let mut b: B = B::auto_json_cloned(&a);
          ///   println!("B {:?}", b);
          ///   a.d = 10;
          ///   b.f = "test".to_string();
          ///   b = b.self_json_cloned(&a);
          ///   println!("B {:?}", b);
          /// }
          pub fn self_json_cloned<T:serde::ser::Serialize>(&self, target:&T) -> Self{
            Self::own_json_cloned(self,target)
          }
          /// ```toml
          /// [dependencies]
          /// serde_json = "1.0"
          /// serde = { version = "1.0", features = ["derive"] }
          /// e-macros = { version = "0.1", git="https://gitee.com/eternalnight996/e-macros"}
          /// ```
          /// # Example
          ///```rust
          /// #[derive(serde::Deserialize, Debug, serde::Serialize)]
          /// struct A {
          ///   a: String,
          ///   b: i32,
          ///   d: i32,
          /// }
          /// #[derive(serde::Deserialize, Debug, serde::Serialize, Default, e_utils::Json)]
          /// struct B {
          ///   d: i32,
          ///   f: String,
          /// }
          /// fn test() {
          ///   let mut a: A = A {
          ///     a: "A".to_string(),
          ///     b: 1,
          ///     d: 2,
          ///   };
          ///   println!("A {:?}", a);
          ///   let mut b: B = B::auto_json_cloned(&a);
          ///   println!("B {:?}", b);
          ///   a.d = 10;
          ///   b.f = "test".to_string();
          ///   b = b.self_json_cloned(&a);
          ///   println!("B {:?}", b);
          /// }
          pub fn auto_json_cloned<R:serde::de::DeserializeOwned,T:serde::ser::Serialize>(target:&T) -> R {
            Self::own_json_cloned(&Self::default(),target)
          }
          /// 内部
          fn own_json_cloned<R:serde::de::DeserializeOwned,S:serde::ser::Serialize,T:serde::ser::Serialize>(source:&S,target:&T) -> R {
              let mut e_self:serde_json::Map<String, serde_json::Value> =
                serde_json::from_str::<serde_json::Value>(&serde_json::to_string(source).unwrap())
                  .unwrap().as_object()
                  .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Target is not an object"))
                  .unwrap()
                  .clone();
              let e_target:serde_json::Map<String, serde_json::Value> =
                serde_json::from_str::<serde_json::Value>(&serde_json::to_string(target).unwrap())
                  .unwrap().as_object()
                  .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Target is not an object"))
                  .unwrap()
                  .clone();
                for (k, v) in e_target {
                  e_self.insert(k, v);
                }
                serde_json::from_value(serde_json::Value::Object(e_self)).unwrap()
          }
          /// #智能写入Json
          /// # Example
          /// ```rust
          /// #[derive(serde::Deserialize, Debug, serde::Serialize, Default, e_utils::Json)]
          /// struct B {
          ///   d: i32,
          ///   f: String,
          /// }
          /// fn test() {
          ///   let mut b: B = B::default();
          ///   b.f = "test".to_string();
          ///   b.auto_write_json(Path::new("."), "test.json").unwrap();
          ///   let b = B::auto_read_json(Path::new("test.json")).unwrap();
          ///   println!("B {:?}", b);
          /// }
          /// ```
          /// B B { d: 0, f: "test" }
          pub fn auto_write_json<P:AsRef<std::path::Path>,S:AsRef<str>>(&self, fpath: P, fname: S) -> std::io::Result<()> {
            let fpath = fpath.as_ref();
            let fname = fname.as_ref();
            if !fpath.exists() {
              std::fs::create_dir_all(fpath)?;
            }
            std::fs::write(fpath.join(fname), serde_json::to_string_pretty(self)?)?;
            std::io::Result::Ok(())
          }
          /// #智能读取Json
          /// # Example
          /// ```rust
          /// #[derive(serde::Deserialize, Debug, serde::Serialize, Default, e_utils::Json)]
          /// struct B {
          ///   d: i32,
          ///   f: String,
          /// }
          /// fn test() {
          ///   let mut b: B = B::default();
          ///   b.f = "test".to_string();
          ///   b.auto_write_json(Path::new("."), "test.json").unwrap();
          ///   let b = B::auto_read_json(Path::new("test.json")).unwrap();
          ///   println!("B {:?}", b);
          /// }
          /// ```
          /// B B { d: 0, f: "test" }
          pub fn auto_read_json<P:AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
            let path = path.as_ref();
            std::io::Result::Ok(serde_json::from_str::<Self>(&std::fs::read_to_string(
              path
            )?)?)
          }
          /// 序列化
          pub fn to_s(&self) ->  String{
            serde_json::to_string_pretty(self).unwrap_or_default()
          }
      }

  };

  // Return the generated implementation
  TokenStream::from(expanded)
}

/// 对C语言的增加处理方法
/// ```toml
/// [dependencies]
/// serde_json = "1.0"
/// serde = { version = "1.0", features = ["derive"] }
/// e-macros = { version = "0.1", git="https://gitee.com/eternalnight996/e-macros"}
/// ```
/// # Example
/// ```rust
/// #[derive(e_utils::C)]
/// struct B {
///   d: i32,
///   f: String,
/// }
/// fn test() -> Result<()> {
///   // 假设我们有一个T类型的实例
///   let value: B = B {
///     d: 1,
///     f: "test".to_string(),
///   };
///   let ptr = value.to_c_ptr();
///   // 还原*c_void指针为<Box<T>>实例
///   if let Some(restored_boxed_value) = unsafe { B::from_c_ptr(ptr) } {
///     // 成功还原Box<T>实例
///     println!("Restored value: {:?}", *restored_boxed_value);
///   } else {
///     // 还原过程中出现错误
///     println!("Failed to restore value");
///   }
///   Ok(())
/// }
/// ```
#[proc_macro_derive(C)]
pub fn c_derive(input: TokenStream) -> TokenStream {
  // Parse the input tokens as a DeriveInput
  let input = parse_macro_input!(input as DeriveInput);

  // Get the name of the struct
  let struct_name = &input.ident;

  // Extract the fields of the struct
  // let _fields = if let syn::Data::Struct(syn::DataStruct {
  //   fields: syn::Fields::Named(fields),
  //   ..
  // }) = &input.data
  // {
  //   fields.named.iter().map(|f| &f.ident).collect::<Vec<_>>()
  // } else {
  //   panic!("This macro only supports structs with named fields");
  // };

  // Create the output tokens
  let expanded = quote! {
    // Implement the trait for the struct
    impl #struct_name {
      // /// 获取*const c_void指针
      // pub fn to_c_ptr2(self) -> *const std::ffi::c_void {
      //   self as *const Self *const std::ffi::c_void
      // }
      /// 安全地获取*const c_void指针
      pub fn to_c_ptr(self) -> *const std::ffi::c_void {
        Box::into_raw(Box::new(self)) as *const std::ffi::c_void
      }
      /// 安全地还原*const c_void指针为Box<Self>
      /// # Example
      /// ```rust
      /// #[derive(e_utils::C)]
      /// struct B {
      ///   d: i32,
      ///   f: String,
      /// }
      /// fn test() -> Result<()> {
      ///   // 假设我们有一个T类型的实例
      ///   let value: B = B {
      ///     d: 1,
      ///     f: "test".to_string(),
      ///   };
      ///   let ptr = value.to_c_ptr();
      ///   // 还原*c_void指针为<Box<T>>实例
      ///   if let Some(restored_boxed_value) = unsafe { B::from_c_ptr(ptr) } {
      ///     // 成功还原Box<T>实例
      ///     println!("Restored value: {:?}", *restored_boxed_value);
      ///   } else {
      ///     // 还原过程中出现错误
      ///     println!("Failed to restore value");
      ///   }
      ///   Ok(())
      /// }
      /// ```
      pub unsafe fn from_c_ptr(void_ptr: *const std::ffi::c_void) -> Option<Box<Self>> {
        // 使用NonNull来避免空指针
        let non_null_ptr = std::ptr::NonNull::new(void_ptr as *mut Self);
        // 安全地解引用NonNull指针并创建一个新的Box
        non_null_ptr.map(|ptr| Box::from_raw(ptr.as_ptr() as *mut Self))
      }
      /// 直接指针还原类型
      pub unsafe fn from_c_ptr2(void_ptr: *const std::ffi::c_void) -> Self {
        std::ptr::read(void_ptr as *const Self)
      }
      // /// 安全地获取*mut c_void指针
      // pub fn to_c_mut_ptr(self) -> *mut std::ffi::c_void {
      //   Box::into_raw(Box::new(self)) as *mut std::ffi::c_void
      // }
      // 安全地还原*mut c_void指针为Box<Self>
      pub unsafe fn from_c_mut_ptr(void_ptr: *mut c_void) -> Option<Box<Self>> {
        // 使用NonNull来避免空指针
        let non_null_ptr = std::ptr::NonNull::new(void_ptr as *mut Self);
        // 安全地解引用NonNull指针并创建一个新的Box
        non_null_ptr.map(|ptr| Box::from_raw(ptr.as_ptr() as *mut Self))
      }
    }
  };

  // Return the generated implementation
  TokenStream::from(expanded)
}
