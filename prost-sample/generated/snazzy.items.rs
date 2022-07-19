#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shirt {
    #[prost(string, required, tag="1")]
    pub color: ::prost::alloc::string::String,
    #[prost(enumeration="shirt::Size", required, tag="2")]
    pub size: i32,
    #[prost(int32, optional, tag="3")]
    pub message: ::core::option::Option<i32>,
}
/// Nested message and enum types in `Shirt`.
pub mod shirt {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Size {
        Small = 0,
        Medium = 1,
        Large = 2,
    }
}
