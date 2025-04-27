use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::macro_input::context::{ContextMacroArgs, ContextMacroStruct};

pub fn context_macro(base_path: ContextMacroArgs, macro_input: ContextMacroStruct) -> TokenStream2 {
    quote! { struct Thing {} }
}
