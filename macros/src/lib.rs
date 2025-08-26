use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Ident, parse_macro_input};

#[proc_macro_derive(Reactive)]
pub fn make_reactive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let name = input.ident;
    let reactive_name = Ident::new(&format!("{}Reactive", name), name.span());

    let fields = if let Data::Struct(data) = input.data {
        data.fields
    } else {
        panic!("Only structs can be reactive")
    };

    // Build the new fields.
    let mut new_fields = Vec::new();
    let mut new_fn_args = Vec::new();
    let mut from_fn_args = Vec::new();

    for field in fields {
        let field_name = field.ident.unwrap();
        let field_type = field.ty;

        let new_field = quote! {
            pub #field_name: _RwSignal<#field_type>,
        };

        new_fields.push(new_field);

        // Create new function arguments/inits
        let new_fn_arg = quote! {
            #field_name: _RwSignal::new(data.#field_name),
        };

        let from_fn_arg = quote! {
            #field_name: self.#field_name.get_untracked(),
        };

        new_fn_args.push(new_fn_arg);
        from_fn_args.push(from_fn_arg);
    }

    let reactive_docs = format!("Reactive version of the [`{name}`] struct. Automatically created by the `Reactive` macro.");

    // Expand the macro. The output is a struct with new reactive fields and a new function,
    // which initializes the reactive fields based on the original struct's fields.
    let expanded = quote! {
        use leptos::prelude::RwSignal as _RwSignal;  // in case the user imports it themselves
        use leptos::prelude::GetUntracked as _;

        #[doc = #reactive_docs]
        pub struct #reactive_name {
            #(#new_fields)*
        }

        impl #reactive_name {
            /// Creates a new `#reactive_name` struct from a non-reactive `#name`.
            pub fn new(data: #name) -> Self {
                Self {
                    #(#new_fn_args)*
                }
            }

            /// Captures the information of this struct into a `#name` struct, for use
            /// in server functions.
            pub fn capture(&self) -> #name {
                #name {
                    #(#from_fn_args)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
