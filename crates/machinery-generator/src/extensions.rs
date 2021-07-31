use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{char, none_of, satisfy, space0, space1},
    multi::fold_many1,
    sequence::delimited,
    IResult,
};
use proc_macro2::Span;
use quote::quote;
use syn::{File, FnArg, ForeignItem, GenericArgument, Ident, ImplItem, Item, ItemStruct, PathArguments, ReturnType, Type};

use crate::config::Project;

pub fn generate(project: &Project, target_headers: &[PathBuf], blocklist: &mut HashSet<String>) {
    // Load the input file
    let target_path = Path::new(&project.target);
    let mut module = fs::read_to_string(target_path).unwrap();
    let mut file = syn::parse_file(&module).unwrap();

    // Store all identifiers that aren't internal so that we can avoid duplicates
    for item in &file.items {
        let ident = if let Some(ident) = get_raw_ident(item) {
            ident
        } else {
            continue;
        };

        // Store the type if it's not an internal utility
        if !ident.starts_with("__") && !ident.to_lowercase().contains("bindgen") {
            blocklist.insert(ident);
        }
    }

    // Rename items where relevant
    for item in &mut file.items {
        rename_item(item);
    }

    // Re-write the file so far
    module.clear();
    module.push_str(&quote! { #file }.to_string());

    module.push_str("\n\n// Extensions generated by machinery-generator\n\n");
    module.push_str("use const_cstr::{const_cstr, ConstCStr};\n\n");

    // Add explicitly defined uses
    if let Some(ref uses) = project.uses {
        for entry in uses {
            module.push_str("use ");
            module.push_str(entry);
            module.push_str(";\n");
        }
    }
    module.push('\n');

    // Individual generation steps
    generate_apis(&file, &mut module);
    generate_define_macros(target_headers, &mut module);

    // Write the generated code back to the output
    fs::write(target_path, module).unwrap();

    // Run rustfmt on the output file
    Command::new("rustfmt")
        .args(&[target_path.to_str().unwrap()])
        .status()
        .expect("Failed to run rustfmt");
}

fn get_raw_ident(item: &Item) -> Option<String> {
    Some(match item {
        Item::Const(item) => item.ident.to_string(),
        Item::Struct(item) => item.ident.to_string(),
        Item::Type(item) => item.ident.to_string(),
        _ => return None,
    })
}

fn rename_item(item: &mut Item) {
    match item {
        Item::Const(item) => rename_ty(&mut item.ty),
        Item::ForeignMod(item) => {
            for item in &mut item.items {
                match item {
                    ForeignItem::Fn(item) => {
                        for input in &mut item.sig.inputs {
                            if let FnArg::Typed(ty) = input {
                                rename_ty(&mut ty.ty);
                            }
                        }
                        if let ReturnType::Type(_, ty) = &mut item.sig.output {
                            rename_ty(ty);
                        }
                    }
                    ForeignItem::Static(item) => {
                        rename_ty(&mut item.ty);
                    }
                    _ => {}
                }
            }
        }
        Item::Impl(item) => {
            rename_ty(&mut item.self_ty);
            for item in &mut item.items {
                if let ImplItem::Method(item) = item {
                    for input in &mut item.sig.inputs {
                        if let FnArg::Typed(ty) = input {
                            rename_ty(&mut ty.ty);
                        }
                    }
                    if let ReturnType::Type(_, ty) = &mut item.sig.output {
                        rename_ty(ty);
                    }
                }
            }
        }
        Item::Struct(item) => {
            for field in &mut item.fields {
                rename_ty(&mut field.ty);
            }
            rename_ident(&mut item.ident);
        }
        Item::Type(item) => {
            rename_ident(&mut item.ident);
            rename_ty(&mut item.ty);
        }
        Item::Union(item) => {
            rename_ident(&mut item.ident);
            for field in &mut item.fields.named {
                rename_ty(&mut field.ty);
            }
        }
        _ => {}
    }
}

fn rename_ty(ty: &mut Type) {
    match ty {
        Type::Array(ty) => {
            rename_ty(&mut ty.elem);
        }
        Type::BareFn(ty) => {
            for input in &mut ty.inputs {
                rename_ty(&mut input.ty);
            }
            if let ReturnType::Type(_, ty) = &mut ty.output {
                rename_ty(ty);
            }
        }
        Type::Ptr(ty) => {
            rename_ty(&mut ty.elem);
        }
        Type::Path(ty) => {
            for segment in &mut ty.path.segments {
                rename_ident(&mut segment.ident);
                if let PathArguments::AngleBracketed(args) = &mut segment.arguments {
                    for arg in &mut args.args {
                        if let GenericArgument::Type(ty) = arg {
                            rename_ty(ty);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

fn rename_ident(ident: &mut Ident) {
    let value = ident.to_string();
    if let Some(value) = value.strip_prefix("tm_") {
        let new_name = value.to_camel_case();
        *ident = Ident::new(&new_name, Span::call_site());
    }
}

fn generate_apis(file: &File, module: &mut String) {
    // Find all defined names, so we know what can have an Api trait
    let mut defined_names = HashSet::new();
    for item in &file.items {
        if let Item::Const(cons) = item {
            let name = cons.ident.to_string();
            if name.ends_with("_NAME") {
                defined_names.insert(name);
            }
        }
    }

    for item in &file.items {
        if let Item::Struct(item) = item {
            let name = item.ident.to_string();

            // Skip anything that isn't an API
            if !name.ends_with("Api") {
                continue;
            }

            generate_api(module, item, &defined_names);
        }
    }
}

fn generate_api(src: &mut String, item: &ItemStruct, defined_names: &HashSet<String>) {
    let struct_name = item.ident.clone();

    src.push_str(&format!("impl {} {{\n", struct_name));

    for field in &item.fields {
        let field_name = field.ident.clone().unwrap();

        // Skip internals
        if field_name.to_string().starts_with("internal__") {
            continue;
        }

        let path_type = if let Type::Path(path_type) = &field.ty {
            path_type
        } else {
            continue;
        };

        // Extract the type
        let segment = path_type.path.segments.last().unwrap();
        if segment.ident != "Option" {
            // Skip non-options, they're not what we're looking for
            continue;
        }

        let arguments = if let PathArguments::AngleBracketed(ref arguments) = segment.arguments {
            arguments
        } else {
            continue;
        };

        let type_arg = if let GenericArgument::Type(ref type_arg) = arguments.args[0] {
            type_arg
        } else {
            continue;
        };

        let fn_type = if let Type::BareFn(fn_type) = type_arg {
            fn_type
        } else {
            continue;
        };

        // Generate in/out args code
        let mut in_args = vec![quote!(&self)];
        let mut out_args = Vec::new();

        for input in &fn_type.inputs {
            let name = &input.name.as_ref().unwrap().0;
            let raw_ty = &input.ty;
            in_args.push(quote! { #name: #raw_ty });
            out_args.push(quote! { #name });
        }

        let output = &fn_type.output;
        let function = quote! {
            pub unsafe fn #field_name(#(#in_args),*) #output
        };

        src.push_str(&format!("{} {{\n", function));

        // Call into the field
        let call = quote! {
            self.#field_name.unwrap()(#(#out_args),*)
        };
        src.push_str(&format!("{}\n", call));

        src.push_str("}\n\n");
    }

    src.push_str("}\n\n");

    // Trait implementation for fetching from the registry
    let define_name = format!("TM_{}_NAME", struct_name.to_string().to_shouty_snake_case());
    if defined_names.contains(&define_name) {
        // Infer the actual name (this may be incorrect due to user error)
        let api_name = format!("tm_{}", struct_name.to_string().to_snake_case());

        let reg_impl = quote! {
            impl crate::Api for #struct_name {
                const NAME: ConstCStr = const_cstr!(#api_name);
            }
        };
        src.push_str(&reg_impl.to_string());
    }

    src.push_str("\n\n");
}

fn generate_define_macros(target_headers: &[PathBuf], module: &mut String) {
    for header in target_headers {
        let header = fs::read_to_string(header).unwrap();

        for line in header.lines() {
            let line = line.trim_start();

            // We're only looking for defines with static hashes
            if !line.starts_with("#define")
                || !line.contains("TM_STATIC_HASH")
                // This is the line that defines the macro itself
                || line.contains("TM_STRHASH")
            {
                continue;
            }

            // Parse the line to extract the data
            let parsed = match static_hash_define(line) {
                Ok((_, parsed)) => parsed,
                Err(err) => {
                    println!("Failed to parse found TM_STATIC_HASH:\n{}", err);
                    continue;
                }
            };

            // Create a definition for this line
            let ident = Ident::new(&parsed.name, Span::call_site());
            let hash = parsed.hash;
            let define = quote! {
                pub const #ident: StrhashT = StrhashT { u64_: #hash };
            };
            module.push_str(&define.to_string());
        }
    }
}

fn static_hash_define(input: &str) -> IResult<&str, StaticHashDefine> {
    let (input, _) = tag("#define")(input)?;
    let (input, _) = space1(input)?;

    let (input, name) = identifier(input)?;
    let (input, _) = space1(input)?;

    let (input, _) = tag("TM_STATIC_HASH")(input)?;

    let (input, _) = delimited(space0, char('('), space0)(input)?;
    let (input, value) = string_literal(input)?;
    let (input, _) = delimited(space0, char(','), space0)(input)?;
    let (input, hex) = delimited(tag("0x"), take_while1(is_hex_digit), tag("ULL"))(input)?;
    let (input, _) = delimited(space0, char(')'), space0)(input)?;

    Ok((
        input,
        StaticHashDefine {
            name,
            value,
            hash: u64::from_str_radix(hex, 16).unwrap(),
        },
    ))
}

fn identifier(input: &str) -> IResult<&str, String> {
    fold_many1(alphanumeric_or_underscore, String::new(), |mut acc, c| {
        acc.push(c);
        acc
    })(input)
}

fn alphanumeric_or_underscore(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_ascii_alphanumeric() || c == '_')(input)
}

fn string_literal(input: &str) -> IResult<&str, String> {
    let (input, _) = char('"')(input)?;
    let (input, value) = fold_many1(none_of("\""), String::new(), |mut acc, c| {
        acc.push(c);
        acc
    })(input)?;
    let (input, _) = char('"')(input)?;

    Ok((input, value))
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

#[derive(Debug)]
struct StaticHashDefine {
    name: String,
    value: String,
    hash: u64,
}
