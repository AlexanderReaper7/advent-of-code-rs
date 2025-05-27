use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ItemFn};

/// Attribute macro to instrument functions for visualization
#[proc_macro_attribute]
pub fn visualize(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    
    // For now, just add basic instrumentation without complex AST transformation
    let original_block = &input_fn.block;
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let attrs = &input_fn.attrs;
    
    quote! {
        #(#attrs)*
        #vis #sig {
            let mut __viz_ctx = viz_lib::VizContext::new(stringify!(#fn_name));
            __viz_ctx.add_step("Function started");
            
            let result = (|| #original_block)();
            
            __viz_ctx.add_step("Function completed");
            __viz_ctx.finalize();
            result
        }
    }.into()
}

/// Macro to manually track values at specific points
#[proc_macro]
pub fn track(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    quote! {
        {
            let value = #expr;
            __viz_ctx.track_value(stringify!(#expr), &format!("{:?}", value));
            value
        }
    }.into()
}

/// Macro to mark algorithm steps
#[proc_macro]
pub fn step(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as syn::LitStr);
    quote! {
        __viz_ctx.add_step(#expr);
    }.into()
}
