// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, Item};

// #[proc_macro_attribute]
// pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let mut out = item.clone();
//     let func = match parse_macro_input!(item as Item) {
//         Item::Fn(f) => f,
//         _ => panic!("expected function"),
//     };
//     let name = func.sig.ident;

//     out.extend(TokenStream::from(quote! {
//         fn aa<T>(f: fn(&web::Request, &mut web::response) -> ()) -> web::Cb
//         where
//             T: Future<Output = ()> + 'static,
//         {
//             Box::new(move |n| Box::pin(f(n)))
//         }
//     }));
//     out
// }
