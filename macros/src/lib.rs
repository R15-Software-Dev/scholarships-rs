use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, Ident};

/// Derives the `AsReactive` and `ReactiveCapture` traits for a struct, while also
/// providing a new reactive struct with the same field names, but they are wrapped
/// in the Leptos `RwSignal`.
///
/// Fields that are custom types are not automatically converted into reactive types. They
/// will be wrapped in an `RwSignal` struct, but that is where this process stops. Making
/// other information reactive must be implemented manually.
///
/// For example, when the `Reactive` macro is expanded for the struct `User`, it will look
/// like this:
///
/// ```
/// #[derive(Reactive)]
/// struct User {
///     id: i32,
///     name: String,
///     custom: CustomType
/// }
///
/// // This code will be added:
/// struct UserReactive {
///     id: RwSignal<i32>,
///     name: RwSignal<String>,
///     custom: RwSignal<CustomType>
/// }
///
/// impl AsReactive for User {
///     type ReactiveType = UserReactive;
///     // Other methods - refer to AsReactive documentation.
/// }
///
/// impl ReactiveCapture for UserReactive {
///     type CaptureType = User;
///     // Other methods - refer to ReactiveCapture documentation.
/// }
/// ```
///
/// The base type (`User`) can then be converted into the reactive type:
/// ```
/// let user = User::new();
/// let user_reactive = user.as_reactive();
/// ```
/// And captured back into a non-reactive type:
/// ```
/// let captured_user = user_reactive.capture()
/// ```
///
/// # Important Considerations
///
/// A reminder that custom types, in this case `CustomType`, are just wrapped in an `RwSignal`.
/// They are not automatically converted to their own `Reactive` type in the case that they
/// derive this macro. As a result, custom reactive types *must* be updated manually by utilizing
/// the `RwSignal::update` and `RwSignal::set` methods.
///
/// For example, the `UserReactive` struct from earlier does not convert the `CustomType` into a
/// `CustomTypeReactive`. We have to do that ourselves:
/// ```
/// let user: User = User::new();
/// let user_reactive: UserReactive = user.as_reactive();
/// let custom_reactive: CustomTypeReactive = user_reactive.custom.as_reactive();
/// // And then convert back...
/// user_reactive.custom.set(custom_reactive.capture());
/// ```
/// If a shorthand for the `capture` and `as_reactive` functions are absolutely *required*, then this
/// macro is probably not the way to go - as of now, this functionality should be impmlemented manually.
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
            #field_name: _RwSignal::new(self.#field_name),
        };

        let from_fn_arg = quote! {
            #field_name: self.#field_name.get_untracked(),
        };

        new_fn_args.push(new_fn_arg);
        from_fn_args.push(from_fn_arg);
    }

    let reactive_docs = format!(
        "Reactive version of the [`{name}`] struct. Automatically created by the `Reactive` macro."
    );

    // Expand the macro. The output is a struct with new reactive fields and a new function,
    // which initializes the reactive fields based on the original struct's fields.
    let expanded = quote! {
        use leptos::prelude::RwSignal as _RwSignal;  // in case the user imports it themselves
        use leptos::prelude::GetUntracked as _;
        use traits::{AsReactive, ReactiveCapture};

        #[doc = #reactive_docs]
        pub struct #reactive_name {
            #(#new_fields)*
        }

        impl AsReactive for #name {
            type ReactiveType = #reactive_name;

            fn as_reactive(self) -> Self::ReactiveType {
                #reactive_name {
                    #(#new_fn_args)*
                }
            }
        }

        impl ReactiveCapture for #reactive_name {
            type CaptureType = #name;

            fn capture(&self) -> Self::CaptureType {
                #name {
                    #(#from_fn_args)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
