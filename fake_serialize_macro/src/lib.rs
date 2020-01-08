extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;


#[cfg(feature = "serde1")]
#[proc_macro_derive(FakeSerialize)]
/// Implement fake serialize for given type
pub fn fake_serialize_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_fake_serialize_macro(&ast)
}

#[cfg(feature = "serde1")]
fn impl_fake_serialize_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl serde::Serialize for #name {
            fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer
                {
                    Err(serde::ser::Error::custom(format!("serialization is disabled")))
                }
            }

        #[cfg(feature="serde1")]
        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>
                {
                    Err(serde::de::Error::custom(format!("deserialization is disabled")))
                }
            }

    };
    gen.into()
}

#[cfg(not(feature = "serde1"))]
#[proc_macro_derive(FakeSerialize)]
/// If feature `serde1` is disabled, this macros does nothing
pub fn fake_serialize_derive(_input: TokenStream) -> TokenStream {
    let gen = quote! {};
    gen.into()
}
