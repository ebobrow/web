use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AngleBracketedGenericArguments, Data, DataStruct, DeriveInput, Fields,
    FieldsNamed, GenericArgument, Ident, Path, PathArguments, Type, TypePath,
};

pub fn builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { named, .. }),
        ..
    }) = &input.data
    {
        named
    } else {
        panic!("Must be a struct");
    };
    let methods = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;

        let (param_ty, set) = if let Some(inner_ty) = inner_ty(ty, "Option") {
            if is_string(inner_ty) {
                (
                    quote! { impl ToString },
                    quote! { ::core::option::Option::Some(#name.to_string()) },
                )
            } else {
                (
                    quote! { #inner_ty },
                    quote! { ::core::option::Option::Some(#name) },
                )
            }
        } else {
            if is_string(ty) {
                (quote! { impl ToString }, quote! { #name.to_string() })
            } else {
                (quote! { #ty }, quote! { #name })
            }
        };

        quote! {
            fn #name(mut self, #name: #param_ty) -> Self {
                self.#name = #set;
                self
            }
        }
    });

    TokenStream::from(quote! {
        impl #name {
            #(#methods)*
        }
    })
}

fn inner_ty<'a>(ty: &'a Type, outer: &str) -> Option<&'a Type> {
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    {
        if segments[0].ident == outer {
            if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
                &segments[0].arguments
            {
                if let GenericArgument::Type(t) = &args[0] {
                    return Some(t);
                }
            }
        }
    }
    None
}

fn is_string(ty: &Type) -> bool {
    if let Type::Path(p) = ty {
        if let Some(ident) = p.path.get_ident() {
            if &Ident::new("String", ident.span()) == ident {
                return true;
            }
        }
    }
    false
}
