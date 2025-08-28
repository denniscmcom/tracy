use proc_macro::TokenStream;
use quote::quote;
use tracy_macros_helper::StructInput;

pub fn impl_color_macro(input: StructInput) -> TokenStream {
    let ident = input.ident;
    let to_u8_fields = input.fields.iter().map(|f| {
        quote! {#f: (self.#f.clamp(0.0, 1.0) * 255.0) as u8}
    });

    let to_gamma_fields = input.fields.iter().map(|f| {
        quote! {#f: f64::max(self.#f, 0.0).sqrt()}
    });

    let color_impl = quote! {
        impl Color for #ident<f64> {
            type U8 = #ident<u8>;

            fn to_u8(self) -> Self::U8 {
                #ident::<u8> {
                   #(#to_u8_fields),*
                }
            }

            fn to_gamma(&self) -> Self {
                #ident::<f64> {
                    #(#to_gamma_fields),*
                }
            }
        }
    };

    color_impl.into()
}
