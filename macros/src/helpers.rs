macro_rules! code {
    ($($tt:tt)*) => {
        quote::quote_spanned!(proc_macro2::Span::mixed_site() => $($tt)*)
    };
}
