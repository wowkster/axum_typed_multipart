//! Designed to seamlessly integrate with [Axum](https://github.com/tokio-rs/axum),
//! this crate simplifies the process of handling `multipart/form-data` requests
//! in your web application by allowing you to parse the request body into a
//! type-safe struct.
//!
//! ## Usage
//!
//! ### Installation
//!
//! ```bash
//! cargo add axum_typed_multipart
//! ```
//!
//! ### Getting started
//!
//! To get started you will need to define a struct with the desired fields and
//! implement the [TryFromMultipart](crate::TryFromMultipart) trait. In the vast
//! majority of cases you will want to use the derive macro to generate the
//! implementation automatically.
//!
//! To be able to derive the [TryFromMultipart](crate::TryFromMultipart) trait
//! every field in the struct must implement the [TryFromField](crate::TryFromField)
//! trait. The trait is implemented by default for all primitive types,
//! [String], and [Bytes](axum::body::Bytes), in case you want to access the
//! raw data.
//!
//! If the request body is malformed or it does not contain the required data
//! the request will be aborted with an error.
//!
//! ```rust
//! use axum::http::StatusCode;
//! use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
//!
//! #[derive(TryFromMultipart)]
//! struct RequestData {
//!     first_name: String,
//!     last_name: String,
//! }
//!
//! async fn handler(
//!     TypedMultipart(RequestData { first_name, last_name }): TypedMultipart<RequestData>,
//! ) -> StatusCode {
//!     println!("full name = '{}' '{}'", first_name, last_name);
//!     StatusCode::OK
//! }
//! ```
//!
//! ### Optional fields
//!
//! If a field is declared as an [Option] the value will default to
//! [None] when the field is missing from the request body.
//!
//! ```rust
//! use axum::http::StatusCode;
//! use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
//!
//! #[derive(TryFromMultipart)]
//! struct RequestData {
//!     first_name: Option<String>,
//! }
//! ```
//!
//! ### Renaming fields
//!
//! If you would like to assign a custom name for the source field you can use
//! the `field_name` parameter of the `form_data` attribute.
//!
//! ```rust
//! use axum::http::StatusCode;
//! use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
//!
//! #[derive(TryFromMultipart)]
//! struct RequestData {
//!     #[form_data(field_name = "first_name")]
//!     name: Option<String>,
//! }
//! ```
//!
//! ### Default values
//!
//! If the `default` parameter in the `form_data` attribute is present the value
//! will be populated using the type's [Default] implementation when the field
//! is not supplied in the request.
//!
//! ```rust
//! use axum::http::StatusCode;
//! use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
//!
//! #[derive(TryFromMultipart)]
//! struct RequestData {
//!     #[form_data(default)]
//!     name: String, // defaults to ""
//! }
//! ```
//!
//! ### Field metadata
//!
//! If you need access to the field metadata (e.g. the request headers) you can
//! use the [FieldData](crate::FieldData) struct to wrap your field.
//!
//! ```rust
//! use axum::body::Bytes;
//! use axum::http::StatusCode;
//! use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
//!
//! #[derive(TryFromMultipart)]
//! struct RequestData {
//!     image: FieldData<Bytes>,
//! }
//!
//! async fn handler(
//!     TypedMultipart(RequestData { image }): TypedMultipart<RequestData>,
//! ) -> StatusCode {
//!     println!(
//!         "file name = '{}', content type = '{}', size = '{}'",
//!         image.metadata.file_name.unwrap_or(String::new()),
//!         image.metadata.content_type.unwrap_or(String::from("text/plain")),
//!         image.contents.len()
//!     );
//!
//!     StatusCode::OK
//! }
//! ```
//!
//! ### Large uploads
//!
//! For large file uploads you can save the contents of the file to the file
//! system using the [TempFile](crate::TempFile) helper. This will efficiently
//! stream the field data directly to the file system, without needing to fit
//! all the data in memory. Once the upload is complete, you can then save the
//! contents to a location of your choice using the
//! [persist](crate::TempFile::persist) method.
//!
//! ```rust
//! use axum::http::StatusCode;
//! use axum_typed_multipart::{
//!     FieldData, TempFile, TryFromMultipart, TypedMultipart, TypedMultipartError,
//! };
//! use std::path::Path;
//!
//! #[derive(TryFromMultipart)]
//! struct RequestData {
//!     image: FieldData<TempFile>,
//! }
//!
//! async fn handler(
//!     TypedMultipart(RequestData { image }): TypedMultipart<RequestData>,
//! ) -> StatusCode {
//!     let file_name = image.metadata.file_name.unwrap_or(String::from("data.bin"));
//!     let path = Path::new("/tmp").join(file_name);
//!
//!     match image.contents.persist(path, false).await {
//!         Ok(_) => StatusCode::OK,
//!         Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
//!     }
//! }
//! ```
//!
//! ### Lists
//!
//! If the incoming request will include multiple fields that share the same
//! name (AKA lists) the field can be declared as a [Vec], allowing for all
//! occurrences of the field to be stored.
//! ```rust
//! use axum::http::StatusCode;
//! use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
//!
//! #[derive(TryFromMultipart)]
//! struct RequestData {
//!     names: Vec<String>,
//! }
//!
//! async fn handler(
//!     TypedMultipart(RequestData { names }): TypedMultipart<RequestData>,
//! ) -> StatusCode {
//!     println!("first name = '{}'", names[0]);
//!     StatusCode::OK
//! }
//! ```

mod field_data;
mod field_metadata;
mod temp_file;
mod try_from_field;
mod try_from_multipart;
mod typed_multipart;
mod typed_multipart_error;

pub use crate::field_data::FieldData;
pub use crate::field_metadata::FieldMetadata;
pub use crate::temp_file::TempFile;
pub use crate::try_from_field::TryFromField;
pub use crate::try_from_multipart::TryFromMultipart;
pub use crate::typed_multipart::TypedMultipart;
pub use crate::typed_multipart_error::TypedMultipartError;
pub use axum_typed_multipart_macros::TryFromMultipart;
