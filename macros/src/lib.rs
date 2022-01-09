use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, Pat};

mod builder;

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = match parse_macro_input!(item as syn::Item) {
        syn::Item::Fn(item_fn) => item_fn,
        _ => panic!("must be called on function"),
    };
    let ident = match item.sig.inputs.first() {
        Some(FnArg::Typed(t)) => {
            // match &*t.ty {
            //     Type::Path(p) => println!("{:?}", p.path.segments.last().unwrap().ident),
            //     _ => println!("not path"),
            // };
            match &*t.pat {
                Pat::Ident(i) => &i.ident,
                _ => panic!("invalid parameter"),
            }
        }
        _ => panic!("must take one argument"),
    };
    let block = &item.block;

    // Doesn't handle irregular use cases like preexisting return type (is there any need for this?)
    let expanded = quote! {
        fn main() -> ::std::io::Result<()> {
            web::App::new("127.0.0.1:3000", |mut #ident| async move {
                #block
                #ident.listen().await;
            })
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    builder::builder(input)
}
