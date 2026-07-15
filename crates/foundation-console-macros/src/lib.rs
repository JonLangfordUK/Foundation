//! Procedural macros for Foundation debug console commands.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Expr, FnArg, GenericArgument, ItemFn, Lit, LitStr, Meta,
    Pat, PathArguments, Type,
};

/// Derives Foundation console input parsing and metadata for a named-field struct.
#[proc_macro_derive(ConsoleCommandInput)]
pub fn derive_console_command_input(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let type_identifier = derive_input.ident;

    let Data::Struct(struct_data) = derive_input.data else {
        return syn::Error::new_spanned(
            type_identifier,
            "ConsoleCommandInput can only be derived for structs",
        )
        .to_compile_error()
        .into();
    };

    let syn::Fields::Named(named_fields) = struct_data.fields else {
        return syn::Error::new_spanned(
            type_identifier,
            "ConsoleCommandInput requires named struct fields",
        )
        .to_compile_error()
        .into();
    };

    let parameter_descriptors = named_fields.named.iter().map(|field| {
        let field_identifier = field
            .ident
            .as_ref()
            .expect("named field should have identifier");
        let field_name = field_identifier.to_string();
        let field_type = &field.ty;
        quote! {
            foundation_runtime_library::console::ConsoleCommandParameter {
                name: #field_name,
                type_name: stringify!(#field_type),
                required: true,
            }
        }
    });

    let field_parsers = named_fields.named.iter().map(|field| {
        let field_identifier = field
            .ident
            .as_ref()
            .expect("named field should have identifier");
        let field_name = field_identifier.to_string();
        let field_type = &field.ty;
        quote! {
            let #field_identifier = console_command_arguments
                .required(#field_name)?
                .parse::<#field_type>()
                .map_err(|parse_error| {
                    foundation_runtime_library::console::ConsoleCommandError::invalid_parameter(
                        #field_name,
                        stringify!(#field_type),
                        parse_error.to_string(),
                    )
                })?;
        }
    });

    let field_initializers = named_fields.named.iter().map(|field| {
        field
            .ident
            .as_ref()
            .expect("named field should have identifier")
    });

    quote! {
        impl foundation_runtime_library::console::ConsoleCommandInput for #type_identifier {
            fn parameters() -> &'static [foundation_runtime_library::console::ConsoleCommandParameter] {
                &[
                    #(#parameter_descriptors,)*
                ]
            }

            fn parse(
                console_command_arguments: &foundation_runtime_library::console::ConsoleCommandArguments,
            ) -> foundation_runtime_library::console::ConsoleCommandResult<Self> {
                #(#field_parsers)*

                Ok(Self {
                    #(#field_initializers,)*
                })
            }
        }
    }
    .into()
}

/// Registers a function as a Foundation console command.
#[proc_macro_attribute]
pub fn console_command(attribute_input: TokenStream, function_input: TokenStream) -> TokenStream {
    let attribute_tokens = proc_macro2::TokenStream::from(attribute_input);
    let command_function = parse_macro_input!(function_input as ItemFn);

    match expand_console_command(attribute_tokens, command_function) {
        Ok(expanded_tokens) => expanded_tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_console_command(
    attribute_tokens: proc_macro2::TokenStream,
    command_function: ItemFn,
) -> syn::Result<proc_macro2::TokenStream> {
    let command_name = command_name(attribute_tokens, &command_function)?;
    let command_function_identifier = &command_function.sig.ident;
    let wrapper_identifier =
        format_ident!("__foundation_console_run_{command_function_identifier}");
    let executor_identifier =
        format_ident!("__foundation_console_execute_{command_function_identifier}");
    let descriptor_identifier = format_ident!(
        "__FOUNDATION_CONSOLE_COMMAND_{}",
        command_function_identifier.to_string().to_uppercase()
    );

    let mut wrapper_parameters = Vec::new();
    let mut original_arguments = Vec::new();
    let mut console_input_type = None;

    for function_argument in &command_function.sig.inputs {
        let FnArg::Typed(typed_argument) = function_argument else {
            return Err(syn::Error::new_spanned(
                function_argument,
                "console commands cannot use self parameters",
            ));
        };

        if let Some(input_type) = console_inputs_inner_type(&typed_argument.ty) {
            if console_input_type.is_some() {
                return Err(syn::Error::new_spanned(
                    &typed_argument.ty,
                    "console commands can only have one ConsoleInputs<T> parameter",
                ));
            }
            console_input_type = Some(input_type.clone());
            let Pat::Ident(_) = typed_argument.pat.as_ref() else {
                return Err(syn::Error::new_spanned(
                    &typed_argument.pat,
                    "ConsoleInputs<T> parameters must use a simple identifier pattern",
                ));
            };
            original_arguments.push(quote! {
                foundation_runtime_library::console::ConsoleInputs::new(console_command_inputs)
            });
        } else {
            wrapper_parameters.push(wrapper_parameter_tokens(typed_argument)?);
            original_arguments.push(argument_pattern_tokens(&typed_argument.pat)?);
        }
    }

    let (system_input_type, parse_inputs, parameter_metadata) = match console_input_type {
        Some(input_type) => (
            quote! { #input_type },
            quote! { <#input_type as foundation_runtime_library::console::ConsoleCommandInput>::parse(&console_command_arguments)? },
            quote! { <#input_type as foundation_runtime_library::console::ConsoleCommandInput>::parameters },
        ),
        None => (
            quote! { () },
            quote! { <() as foundation_runtime_library::console::ConsoleCommandInput>::parse(&console_command_arguments)? },
            quote! { <() as foundation_runtime_library::console::ConsoleCommandInput>::parameters },
        ),
    };

    Ok(quote! {
        #command_function

        #[allow(non_snake_case)]
        fn #wrapper_identifier(
            bevy::prelude::In(console_command_inputs): bevy::prelude::In<#system_input_type>,
            #(#wrapper_parameters,)*
        ) -> foundation_runtime_library::console::ConsoleCommandResult<()> {
            let command_result = #command_function_identifier(
                #(#original_arguments,)*
            );
            foundation_runtime_library::console::IntoConsoleCommandResult::into_console_command_result(
                command_result,
            )
        }

        #[allow(non_snake_case)]
        fn #executor_identifier(
            world: &mut bevy::prelude::World,
            console_command_arguments: foundation_runtime_library::console::ConsoleCommandArguments,
        ) -> foundation_runtime_library::console::ConsoleCommandResult<()> {
            let console_command_inputs = #parse_inputs;
            bevy::ecs::system::RunSystemOnce::run_system_once_with(
                world,
                #wrapper_identifier,
                console_command_inputs,
            )
            .map_err(foundation_runtime_library::console::ConsoleCommandError::from_run_system_error)??;
            Ok(())
        }

        #[foundation_runtime_library::console::__private::linkme::distributed_slice(foundation_runtime_library::console::FOUNDATION_CONSOLE_COMMANDS)]
        static #descriptor_identifier: foundation_runtime_library::console::ConsoleCommandDescriptor =
            foundation_runtime_library::console::ConsoleCommandDescriptor {
                name: #command_name,
                parameters: #parameter_metadata,
                execute: #executor_identifier,
            };
    })
}

fn command_name(
    attribute_tokens: proc_macro2::TokenStream,
    command_function: &ItemFn,
) -> syn::Result<String> {
    if attribute_tokens.is_empty() {
        return Ok(command_function.sig.ident.to_string());
    }

    if let Ok(command_name) = syn::parse2::<LitStr>(attribute_tokens.clone()) {
        return Ok(command_name.value());
    }

    let Ok(meta) = syn::parse2::<Meta>(attribute_tokens.clone()) else {
        return Err(syn::Error::new_spanned(
            attribute_tokens,
            "console_command accepts zero arguments, a string literal, or name = \"...\"",
        ));
    };

    let Meta::NameValue(name_value) = meta else {
        return Err(syn::Error::new_spanned(
            meta,
            "console_command accepts zero arguments, a string literal, or name = \"...\"",
        ));
    };

    if !name_value.path.is_ident("name") {
        return Err(syn::Error::new_spanned(
            name_value.path,
            "console command name override must use name = \"...\"",
        ));
    }

    let Expr::Lit(expression_literal) = name_value.value else {
        return Err(syn::Error::new_spanned(
            name_value.path,
            "console command name must be a string literal",
        ));
    };

    let Lit::Str(command_name) = expression_literal.lit else {
        return Err(syn::Error::new_spanned(
            expression_literal.lit,
            "console command name must be a string literal",
        ));
    };

    Ok(command_name.value())
}

fn console_inputs_inner_type(console_input_type: &Type) -> Option<&Type> {
    let Type::Path(type_path) = console_input_type else {
        return None;
    };

    let last_segment = type_path.path.segments.last()?;
    if last_segment.ident != "ConsoleInputs" {
        return None;
    }

    let PathArguments::AngleBracketed(arguments) = &last_segment.arguments else {
        return None;
    };

    let Some(GenericArgument::Type(input_type)) = arguments.args.first() else {
        return None;
    };

    Some(input_type)
}

fn wrapper_parameter_tokens(
    typed_argument: &syn::PatType,
) -> syn::Result<proc_macro2::TokenStream> {
    let Pat::Ident(parameter_identifier) = typed_argument.pat.as_ref() else {
        return Err(syn::Error::new_spanned(
            &typed_argument.pat,
            "console command Bevy parameters must use simple identifier patterns",
        ));
    };

    let parameter_identifier = &parameter_identifier.ident;
    let parameter_type = &typed_argument.ty;
    Ok(quote! { #parameter_identifier: #parameter_type })
}

fn argument_pattern_tokens(pattern: &Pat) -> syn::Result<proc_macro2::TokenStream> {
    match pattern {
        Pat::Ident(parameter_identifier) => {
            let identifier = &parameter_identifier.ident;
            Ok(quote! { #identifier })
        }
        _ => Err(syn::Error::new_spanned(
            pattern,
            "console command Bevy parameters must use simple identifier patterns",
        )),
    }
}
