#![recursion_limit = "128"]
#![feature(proc_macro_diagnostic)]
// #![feature(box_syntax)]

extern crate proc_macro;
extern crate proc_macro2;

#[proc_macro]
pub fn rsx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if input.is_empty() {
        Span::call_site().unstable().warning(EMPTY_TEMPLATE).emit();
        TokenStream::from(quote! {
            None
        }).into()
    } else {
        syn::parse2(input.into())
            .map(|ctx: Context| {
                let output = quote! {
                    #ctx
                };
                println!("{:?}", output);
                TokenStream::from(output)
            })
            .map(|ts| ts.into())
            .map_err(|err| {
                Span::call_site().unstable().error(format!("{:?}", err)).emit();
                err
            })
            .unwrap_or(TokenStream::from(quote! {
                None
            }).into())
    }
}