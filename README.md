# A const-ready version of core::convert / std::convert traits ((+ const_ops))

All traits are the same as in std / core but prefixed with the #[const_trait] attribute to allow usage in const situations.

This crate is designed in such a way that, as soon as the #[const_trait] attribute is used in core / std all imports of this crate can simply be deleted.