// Copyright (c) 2017-present PyO3 Project and Contributors

use std::borrow::Cow;

use crate::attributes::{
    self, kw, take_pyo3_options, CrateAttribute, ExtendsAttribute, FreelistAttribute,
    ModuleAttribute, NameAttribute, NameLitStr, TextSignatureAttribute,
};
use crate::deprecations::{Deprecation, Deprecations};
use crate::konst::{ConstAttributes, ConstSpec};
use crate::method::FnSpec;
use crate::pyimpl::{gen_py_const, PyClassMethodsType};
use crate::pymethod::{
    impl_py_getter_def, impl_py_setter_def, MethodAndMethodDef, MethodAndSlotDef, PropertyType,
    SlotDef, __INT__, __REPR__, __RICHCMP__,
};
use crate::utils::{self, get_pyo3_crate, PythonDoc};
use crate::PyFunctionOptions;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_quote, spanned::Spanned, Result, Token};

/// If the class is derived from a Rust `struct` or `enum`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PyClassKind {
    Struct,
    Enum,
}

/// The parsed arguments of the pyclass macro
pub struct PyClassArgs {
    pub class_kind: PyClassKind,
    pub options: PyClassPyO3Options,
    pub deprecations: Deprecations,
}

impl PyClassArgs {
    fn parse(input: ParseStream<'_>, kind: PyClassKind) -> Result<Self> {
        Ok(PyClassArgs {
            class_kind: kind,
            options: PyClassPyO3Options::parse(input)?,
            deprecations: Deprecations::new(),
        })
    }

    pub fn parse_stuct_args(input: ParseStream<'_>) -> syn::Result<Self> {
        Self::parse(input, PyClassKind::Struct)
    }

    pub fn parse_enum_args(input: ParseStream<'_>) -> syn::Result<Self> {
        Self::parse(input, PyClassKind::Enum)
    }
}

#[derive(Default)]
pub struct PyClassPyO3Options {
    pub krate: Option<CrateAttribute>,
    pub dict: Option<kw::dict>,
    pub extends: Option<ExtendsAttribute>,
    pub freelist: Option<FreelistAttribute>,
    pub frozen: Option<kw::frozen>,
    pub mapping: Option<kw::mapping>,
    pub module: Option<ModuleAttribute>,
    pub name: Option<NameAttribute>,
    pub sequence: Option<kw::sequence>,
    pub subclass: Option<kw::subclass>,
    pub text_signature: Option<TextSignatureAttribute>,
    pub unsendable: Option<kw::unsendable>,
    pub weakref: Option<kw::weakref>,

    pub deprecations: Deprecations,
}

enum PyClassPyO3Option {
    Crate(CrateAttribute),
    Dict(kw::dict),
    Extends(ExtendsAttribute),
    Freelist(FreelistAttribute),
    Frozen(kw::frozen),
    Mapping(kw::mapping),
    Module(ModuleAttribute),
    Name(NameAttribute),
    Sequence(kw::sequence),
    Subclass(kw::subclass),
    TextSignature(TextSignatureAttribute),
    Unsendable(kw::unsendable),
    Weakref(kw::weakref),

    DeprecatedGC(kw::gc),
}

impl Parse for PyClassPyO3Option {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![crate]) {
            input.parse().map(PyClassPyO3Option::Crate)
        } else if lookahead.peek(kw::dict) {
            input.parse().map(PyClassPyO3Option::Dict)
        } else if lookahead.peek(kw::extends) {
            input.parse().map(PyClassPyO3Option::Extends)
        } else if lookahead.peek(attributes::kw::freelist) {
            input.parse().map(PyClassPyO3Option::Freelist)
        } else if lookahead.peek(attributes::kw::frozen) {
            input.parse().map(PyClassPyO3Option::Frozen)
        } else if lookahead.peek(attributes::kw::mapping) {
            input.parse().map(PyClassPyO3Option::Mapping)
        } else if lookahead.peek(attributes::kw::module) {
            input.parse().map(PyClassPyO3Option::Module)
        } else if lookahead.peek(kw::name) {
            input.parse().map(PyClassPyO3Option::Name)
        } else if lookahead.peek(attributes::kw::sequence) {
            input.parse().map(PyClassPyO3Option::Sequence)
        } else if lookahead.peek(attributes::kw::subclass) {
            input.parse().map(PyClassPyO3Option::Subclass)
        } else if lookahead.peek(attributes::kw::text_signature) {
            input.parse().map(PyClassPyO3Option::TextSignature)
        } else if lookahead.peek(attributes::kw::unsendable) {
            input.parse().map(PyClassPyO3Option::Unsendable)
        } else if lookahead.peek(attributes::kw::weakref) {
            input.parse().map(PyClassPyO3Option::Weakref)
        } else if lookahead.peek(attributes::kw::gc) {
            input.parse().map(PyClassPyO3Option::DeprecatedGC)
        } else {
            Err(lookahead.error())
        }
    }
}

impl PyClassPyO3Options {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut options: PyClassPyO3Options = Default::default();

        for option in Punctuated::<PyClassPyO3Option, syn::Token![,]>::parse_terminated(input)? {
            options.set_option(option)?;
        }

        Ok(options)
    }

    pub fn take_pyo3_options(&mut self, attrs: &mut Vec<syn::Attribute>) -> syn::Result<()> {
        take_pyo3_options(attrs)?
            .into_iter()
            .try_for_each(|option| self.set_option(option))
    }

    fn set_option(&mut self, option: PyClassPyO3Option) -> syn::Result<()> {
        macro_rules! set_option {
            ($key:ident) => {
                {
                    ensure_spanned!(
                        self.$key.is_none(),
                        $key.span() => concat!("`", stringify!($key), "` may only be specified once")
                    );
                    self.$key = Some($key);
                }
            };
        }

        match option {
            PyClassPyO3Option::Crate(krate) => set_option!(krate),
            PyClassPyO3Option::Dict(dict) => set_option!(dict),
            PyClassPyO3Option::Extends(extends) => set_option!(extends),
            PyClassPyO3Option::Freelist(freelist) => set_option!(freelist),
            PyClassPyO3Option::Frozen(frozen) => set_option!(frozen),
            PyClassPyO3Option::Mapping(mapping) => set_option!(mapping),
            PyClassPyO3Option::Module(module) => set_option!(module),
            PyClassPyO3Option::Name(name) => set_option!(name),
            PyClassPyO3Option::Sequence(sequence) => set_option!(sequence),
            PyClassPyO3Option::Subclass(subclass) => set_option!(subclass),
            PyClassPyO3Option::TextSignature(text_signature) => set_option!(text_signature),
            PyClassPyO3Option::Unsendable(unsendable) => set_option!(unsendable),
            PyClassPyO3Option::Weakref(weakref) => set_option!(weakref),

            PyClassPyO3Option::DeprecatedGC(gc) => self
                .deprecations
                .push(Deprecation::PyClassGcOption, gc.span()),
        }
        Ok(())
    }
}

pub fn build_py_class(
    class: &mut syn::ItemStruct,
    mut args: PyClassArgs,
    methods_type: PyClassMethodsType,
) -> syn::Result<TokenStream> {
    args.options.take_pyo3_options(&mut class.attrs)?;
    let doc = utils::get_doc(
        &class.attrs,
        args.options
            .text_signature
            .as_ref()
            .map(|attr| (get_class_python_name(&class.ident, &args), attr)),
    );
    let krate = get_pyo3_crate(&args.options.krate);

    if let Some(lt) = class.generics.lifetimes().next() {
        bail_spanned!(
            lt.span() =>
            "#[pyclass] cannot have lifetime parameters. \
            For an explanation, see https://pyo3.rs/latest/class.html#no-lifetime-parameters"
        );
    }

    ensure_spanned!(
        class.generics.params.is_empty(),
        class.generics.span() =>
            "#[pyclass] cannot have generic parameters. \
            For an explanation, see https://pyo3.rs/latest/class.html#no-generic-parameters"
    );

    let field_options = match &mut class.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter_mut()
            .map(|field| {
                FieldPyO3Options::take_pyo3_options(&mut field.attrs)
                    .map(move |options| (&*field, options))
            })
            .collect::<Result<_>>()?,
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter_mut()
            .map(|field| {
                FieldPyO3Options::take_pyo3_options(&mut field.attrs)
                    .map(move |options| (&*field, options))
            })
            .collect::<Result<_>>()?,
        syn::Fields::Unit => {
            // No fields for unit struct
            Vec::new()
        }
    };

    impl_class(&class.ident, &args, doc, field_options, methods_type, krate)
}

/// `#[pyo3()]` options for pyclass fields
struct FieldPyO3Options {
    get: bool,
    set: bool,
    name: Option<NameAttribute>,
}

enum FieldPyO3Option {
    Get(attributes::kw::get),
    Set(attributes::kw::set),
    Name(NameAttribute),
}

impl Parse for FieldPyO3Option {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(attributes::kw::get) {
            input.parse().map(FieldPyO3Option::Get)
        } else if lookahead.peek(attributes::kw::set) {
            input.parse().map(FieldPyO3Option::Set)
        } else if lookahead.peek(attributes::kw::name) {
            input.parse().map(FieldPyO3Option::Name)
        } else {
            Err(lookahead.error())
        }
    }
}

impl FieldPyO3Options {
    fn take_pyo3_options(attrs: &mut Vec<syn::Attribute>) -> Result<Self> {
        let mut options = FieldPyO3Options {
            get: false,
            set: false,
            name: None,
        };

        for option in take_pyo3_options(attrs)? {
            match option {
                FieldPyO3Option::Get(kw) => {
                    ensure_spanned!(
                        !options.get,
                        kw.span() => "`get` may only be specified once"
                    );
                    options.get = true;
                }
                FieldPyO3Option::Set(kw) => {
                    ensure_spanned!(
                        !options.set,
                        kw.span() => "`set` may only be specified once"
                    );
                    options.set = true;
                }
                FieldPyO3Option::Name(name) => {
                    ensure_spanned!(
                        options.name.is_none(),
                        name.span() => "`name` may only be specified once"
                    );
                    options.name = Some(name);
                }
            }
        }

        Ok(options)
    }
}

fn get_class_python_name<'a>(cls: &'a syn::Ident, args: &'a PyClassArgs) -> Cow<'a, syn::Ident> {
    args.options
        .name
        .as_ref()
        .map(|name_attr| Cow::Borrowed(&name_attr.value.0))
        .unwrap_or_else(|| Cow::Owned(cls.unraw()))
}

fn impl_class(
    cls: &syn::Ident,
    args: &PyClassArgs,
    doc: PythonDoc,
    field_options: Vec<(&syn::Field, FieldPyO3Options)>,
    methods_type: PyClassMethodsType,
    krate: syn::Path,
) -> syn::Result<TokenStream> {
    let pytypeinfo_impl = impl_pytypeinfo(cls, args, Some(&args.options.deprecations));

    let py_class_impl = PyClassImplsBuilder::new(
        cls,
        args,
        methods_type,
        descriptors_to_items(cls, field_options)?,
        vec![],
    )
    .doc(doc)
    .impl_all()?;

    Ok(quote! {
        const _: () = {
            use #krate as _pyo3;

            #pytypeinfo_impl

            #py_class_impl
        };
    })
}

struct PyClassEnumVariant<'a> {
    ident: &'a syn::Ident,
    options: EnumVariantPyO3Options,
}

impl<'a> PyClassEnumVariant<'a> {
    fn python_name(&self) -> Cow<'_, syn::Ident> {
        self.options
            .name
            .as_ref()
            .map(|name_attr| Cow::Borrowed(&name_attr.value.0))
            .unwrap_or_else(|| Cow::Owned(self.ident.unraw()))
    }
}

struct PyClassEnum<'a> {
    ident: &'a syn::Ident,
    // The underlying #[repr] of the enum, used to implement __int__ and __richcmp__.
    // This matters when the underlying representation may not fit in `isize`.
    repr_type: syn::Ident,
    variants: Vec<PyClassEnumVariant<'a>>,
}

impl<'a> PyClassEnum<'a> {
    fn new(enum_: &'a mut syn::ItemEnum) -> syn::Result<Self> {
        fn is_numeric_type(t: &syn::Ident) -> bool {
            [
                "u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "u128", "i128", "usize",
                "isize",
            ]
            .iter()
            .any(|&s| t == s)
        }
        let ident = &enum_.ident;
        // According to the [reference](https://doc.rust-lang.org/reference/items/enumerations.html),
        // "Under the default representation, the specified discriminant is interpreted as an isize
        // value", so `isize` should be enough by default.
        let mut repr_type = syn::Ident::new("isize", proc_macro2::Span::call_site());
        if let Some(attr) = enum_.attrs.iter().find(|attr| attr.path.is_ident("repr")) {
            let args =
                attr.parse_args_with(Punctuated::<TokenStream, Token![!]>::parse_terminated)?;
            if let Some(ident) = args
                .into_iter()
                .filter_map(|ts| syn::parse2::<syn::Ident>(ts).ok())
                .find(is_numeric_type)
            {
                repr_type = ident;
            }
        }

        let variants = enum_
            .variants
            .iter_mut()
            .map(extract_variant_data)
            .collect::<syn::Result<_>>()?;
        Ok(Self {
            ident,
            repr_type,
            variants,
        })
    }
}

pub fn build_py_enum(
    enum_: &mut syn::ItemEnum,
    mut args: PyClassArgs,
    method_type: PyClassMethodsType,
) -> syn::Result<TokenStream> {
    args.options.take_pyo3_options(&mut enum_.attrs)?;

    if let Some(extends) = &args.options.extends {
        bail_spanned!(extends.span() => "enums can't extend from other classes");
    } else if let Some(subclass) = &args.options.subclass {
        bail_spanned!(subclass.span() => "enums can't be inherited by other classes");
    } else if enum_.variants.is_empty() {
        bail_spanned!(enum_.brace_token.span => "#[pyclass] can't be used on enums without any variants");
    }

    let doc = utils::get_doc(
        &enum_.attrs,
        args.options
            .text_signature
            .as_ref()
            .map(|attr| (get_class_python_name(&enum_.ident, &args), attr)),
    );
    let enum_ = PyClassEnum::new(enum_)?;
    impl_enum(enum_, &args, doc, method_type)
}

/// `#[pyo3()]` options for pyclass enum variants
struct EnumVariantPyO3Options {
    name: Option<NameAttribute>,
}

enum EnumVariantPyO3Option {
    Name(NameAttribute),
}

impl Parse for EnumVariantPyO3Option {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(attributes::kw::name) {
            input.parse().map(EnumVariantPyO3Option::Name)
        } else {
            Err(lookahead.error())
        }
    }
}

impl EnumVariantPyO3Options {
    fn take_pyo3_options(attrs: &mut Vec<syn::Attribute>) -> Result<Self> {
        let mut options = EnumVariantPyO3Options { name: None };

        for option in take_pyo3_options(attrs)? {
            match option {
                EnumVariantPyO3Option::Name(name) => {
                    ensure_spanned!(
                        options.name.is_none(),
                        name.span() => "`name` may only be specified once"
                    );
                    options.name = Some(name);
                }
            }
        }

        Ok(options)
    }
}

fn impl_enum(
    enum_: PyClassEnum<'_>,
    args: &PyClassArgs,
    doc: PythonDoc,
    methods_type: PyClassMethodsType,
) -> Result<TokenStream> {
    let krate = get_pyo3_crate(&args.options.krate);
    impl_enum_class(enum_, args, doc, methods_type, krate)
}

fn impl_enum_class(
    enum_: PyClassEnum<'_>,
    args: &PyClassArgs,
    doc: PythonDoc,
    methods_type: PyClassMethodsType,
    krate: syn::Path,
) -> Result<TokenStream> {
    let cls = enum_.ident;
    let ty: syn::Type = syn::parse_quote!(#cls);
    let variants = enum_.variants;
    let pytypeinfo = impl_pytypeinfo(cls, args, None);

    let (default_repr, default_repr_slot) = {
        let variants_repr = variants.iter().map(|variant| {
            let variant_name = variant.ident;
            // Assuming all variants are unit variants because they are the only type we support.
            let repr = format!(
                "{}.{}",
                get_class_python_name(cls, args),
                variant.python_name(),
            );
            quote! { #cls::#variant_name => #repr, }
        });
        let mut repr_impl: syn::ImplItemMethod = syn::parse_quote! {
            fn __pyo3__repr__(&self) -> &'static str {
                match self {
                    #(#variants_repr)*
                }
            }
        };
        let repr_slot = generate_default_protocol_slot(&ty, &mut repr_impl, &__REPR__).unwrap();
        (repr_impl, repr_slot)
    };

    let repr_type = &enum_.repr_type;

    let (default_int, default_int_slot) = {
        // This implementation allows us to convert &T to #repr_type without implementing `Copy`
        let variants_to_int = variants.iter().map(|variant| {
            let variant_name = variant.ident;
            quote! { #cls::#variant_name => #cls::#variant_name as #repr_type, }
        });
        let mut int_impl: syn::ImplItemMethod = syn::parse_quote! {
            fn __pyo3__int__(&self) -> #repr_type {
                match self {
                    #(#variants_to_int)*
                }
            }
        };
        let int_slot = generate_default_protocol_slot(&ty, &mut int_impl, &__INT__).unwrap();
        (int_impl, int_slot)
    };

    let (default_richcmp, default_richcmp_slot) = {
        let mut richcmp_impl: syn::ImplItemMethod = syn::parse_quote! {
            fn __pyo3__richcmp__(
                &self,
                py: _pyo3::Python,
                other: &_pyo3::PyAny,
                op: _pyo3::basic::CompareOp
            ) -> _pyo3::PyResult<_pyo3::PyObject> {
                use _pyo3::conversion::ToPyObject;
                use ::core::result::Result::*;
                match op {
                    _pyo3::basic::CompareOp::Eq => {
                        let self_val = self.__pyo3__int__();
                        if let Ok(i) = other.extract::<#repr_type>() {
                            return Ok((self_val == i).to_object(py));
                        }
                        if let Ok(other) = other.extract::<_pyo3::PyRef<Self>>() {
                            return Ok((self_val == other.__pyo3__int__()).to_object(py));
                        }

                        return Ok(py.NotImplemented());
                    }
                    _pyo3::basic::CompareOp::Ne => {
                        let self_val = self.__pyo3__int__();
                        if let Ok(i) = other.extract::<#repr_type>() {
                            return Ok((self_val != i).to_object(py));
                        }
                        if let Ok(other) = other.extract::<_pyo3::PyRef<Self>>() {
                            return Ok((self_val != other.__pyo3__int__()).to_object(py));
                        }

                        return Ok(py.NotImplemented());
                    }
                    _ => Ok(py.NotImplemented()),
                }
            }
        };
        let richcmp_slot =
            generate_default_protocol_slot(&ty, &mut richcmp_impl, &__RICHCMP__).unwrap();
        (richcmp_impl, richcmp_slot)
    };

    let default_slots = vec![default_repr_slot, default_int_slot, default_richcmp_slot];

    let pyclass_impls = PyClassImplsBuilder::new(
        cls,
        args,
        methods_type,
        enum_default_methods(cls, variants.iter().map(|v| (v.ident, v.python_name()))),
        default_slots,
    )
    .doc(doc)
    .impl_all()?;

    Ok(quote! {
        const _: () = {
            use #krate as _pyo3;

            #pytypeinfo

            #pyclass_impls

            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl #cls {
                #default_repr
                #default_int
                #default_richcmp
            }
        };
    })
}

fn generate_default_protocol_slot(
    cls: &syn::Type,
    method: &mut syn::ImplItemMethod,
    slot: &SlotDef,
) -> syn::Result<MethodAndSlotDef> {
    let spec = FnSpec::parse(
        &mut method.sig,
        &mut Vec::new(),
        PyFunctionOptions::default(),
    )
    .unwrap();
    let name = spec.name.to_string();
    slot.generate_type_slot(
        &syn::parse_quote!(#cls),
        &spec,
        &format!("__default_{}__", name),
    )
}

fn enum_default_methods<'a>(
    cls: &'a syn::Ident,
    unit_variant_names: impl IntoIterator<Item = (&'a syn::Ident, Cow<'a, syn::Ident>)>,
) -> Vec<MethodAndMethodDef> {
    let cls_type = syn::parse_quote!(#cls);
    let variant_to_attribute = |var_ident: &syn::Ident, py_ident: &syn::Ident| ConstSpec {
        rust_ident: var_ident.clone(),
        attributes: ConstAttributes {
            is_class_attr: true,
            name: Some(NameAttribute {
                kw: syn::parse_quote! { name },
                value: NameLitStr(py_ident.clone()),
            }),
            deprecations: Default::default(),
        },
    };
    unit_variant_names
        .into_iter()
        .map(|(var, py_name)| gen_py_const(&cls_type, &variant_to_attribute(var, &py_name)))
        .collect()
}

fn extract_variant_data(variant: &mut syn::Variant) -> syn::Result<PyClassEnumVariant<'_>> {
    use syn::Fields;
    let ident = match variant.fields {
        Fields::Unit => &variant.ident,
        _ => bail_spanned!(variant.span() => "Currently only support unit variants."),
    };
    let options = EnumVariantPyO3Options::take_pyo3_options(&mut variant.attrs)?;
    Ok(PyClassEnumVariant { ident, options })
}

fn descriptors_to_items(
    cls: &syn::Ident,
    field_options: Vec<(&syn::Field, FieldPyO3Options)>,
) -> syn::Result<Vec<MethodAndMethodDef>> {
    let ty = syn::parse_quote!(#cls);
    field_options
        .into_iter()
        .enumerate()
        .flat_map(|(field_index, (field, options))| {
            let name_err = if options.name.is_some() && !options.get && !options.set {
                Some(Err(err_spanned!(options.name.as_ref().unwrap().span() => "`name` is useless without `get` or `set`")))
            } else {
                None
            };

            let getter = if options.get {
                Some(impl_py_getter_def(&ty, PropertyType::Descriptor {
                    field_index,
                    field,
                    python_name: options.name.as_ref()
                }))
            } else {
                None
            };

            let setter = if options.set {
                Some(impl_py_setter_def(&ty, PropertyType::Descriptor {
                    field_index,
                    field,
                    python_name: options.name.as_ref()
                }))
            } else {
                None
            };

            name_err.into_iter().chain(getter).chain(setter)
        })
        .collect::<syn::Result<_>>()
}

fn impl_pytypeinfo(
    cls: &syn::Ident,
    attr: &PyClassArgs,
    deprecations: Option<&Deprecations>,
) -> TokenStream {
    let cls_name = get_class_python_name(cls, attr).to_string();

    let module = if let Some(ModuleAttribute { value, .. }) = &attr.options.module {
        quote! { ::core::option::Option::Some(#value) }
    } else {
        quote! { ::core::option::Option::None }
    };

    quote! {
        unsafe impl _pyo3::type_object::PyTypeInfo for #cls {
            type AsRefTarget = _pyo3::PyCell<Self>;

            const NAME: &'static str = #cls_name;
            const MODULE: ::std::option::Option<&'static str> = #module;

            #[inline]
            fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
                #deprecations

                use _pyo3::type_object::LazyStaticType;
                static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
                TYPE_OBJECT.get_or_init::<Self>(py)
            }
        }
    }
}

/// Implements most traits used by `#[pyclass]`.
///
/// Specifically, it implements traits that only depend on class name,
/// and attributes of `#[pyclass]`, and docstrings.
/// Therefore it doesn't implement traits that depends on struct fields and enum variants.
struct PyClassImplsBuilder<'a> {
    cls: &'a syn::Ident,
    attr: &'a PyClassArgs,
    methods_type: PyClassMethodsType,
    default_methods: Vec<MethodAndMethodDef>,
    default_slots: Vec<MethodAndSlotDef>,
    doc: Option<PythonDoc>,
}

impl<'a> PyClassImplsBuilder<'a> {
    fn new(
        cls: &'a syn::Ident,
        attr: &'a PyClassArgs,
        methods_type: PyClassMethodsType,
        default_methods: Vec<MethodAndMethodDef>,
        default_slots: Vec<MethodAndSlotDef>,
    ) -> Self {
        Self {
            cls,
            attr,
            methods_type,
            default_methods,
            default_slots,
            doc: None,
        }
    }

    fn doc(self, doc: PythonDoc) -> Self {
        Self {
            doc: Some(doc),
            ..self
        }
    }

    fn impl_all(&self) -> Result<TokenStream> {
        let tokens = vec![
            self.impl_pyclass(),
            self.impl_extractext(),
            self.impl_into_py(),
            self.impl_pyclassimpl()?,
            self.impl_freelist(),
        ]
        .into_iter()
        .collect();
        Ok(tokens)
    }

    fn impl_pyclass(&self) -> TokenStream {
        let cls = self.cls;

        let frozen = if self.attr.options.frozen.is_some() {
            quote! { _pyo3::pyclass::boolean_struct::True }
        } else {
            quote! { _pyo3::pyclass::boolean_struct::False }
        };

        quote! {
            impl _pyo3::PyClass for #cls {
                type Frozen = #frozen;
            }
        }
    }
    fn impl_extractext(&self) -> TokenStream {
        let cls = self.cls;
        if self.attr.options.frozen.is_some() {
            quote! {
                impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a #cls
                {
                    type Holder = ::std::option::Option<_pyo3::PyRef<'py, #cls>>;

                    #[inline]
                    fn extract(obj: &'py _pyo3::PyAny, holder: &'a mut Self::Holder) -> _pyo3::PyResult<Self> {
                        _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
                    }
                }
            }
        } else {
            quote! {
                impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a #cls
                {
                    type Holder = ::std::option::Option<_pyo3::PyRef<'py, #cls>>;

                    #[inline]
                    fn extract(obj: &'py _pyo3::PyAny, holder: &'a mut Self::Holder) -> _pyo3::PyResult<Self> {
                        _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
                    }
                }

                impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py> for &'a mut #cls
                {
                    type Holder = ::std::option::Option<_pyo3::PyRefMut<'py, #cls>>;

                    #[inline]
                    fn extract(obj: &'py _pyo3::PyAny, holder: &'a mut Self::Holder) -> _pyo3::PyResult<Self> {
                        _pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
                    }
                }
            }
        }
    }

    fn impl_into_py(&self) -> TokenStream {
        let cls = self.cls;
        let attr = self.attr;
        // If #cls is not extended type, we allow Self->PyObject conversion
        if attr.options.extends.is_none() {
            quote! {
                impl _pyo3::IntoPy<_pyo3::PyObject> for #cls {
                    fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
                        _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
                    }
                }
            }
        } else {
            quote! {}
        }
    }
    fn impl_pyclassimpl(&self) -> Result<TokenStream> {
        let cls = self.cls;
        let doc = self.doc.as_ref().map_or(quote! {"\0"}, |doc| quote! {#doc});
        let is_basetype = self.attr.options.subclass.is_some();
        let base = self
            .attr
            .options
            .extends
            .as_ref()
            .map(|extends_attr| extends_attr.value.clone())
            .unwrap_or_else(|| parse_quote! { _pyo3::PyAny });
        let is_subclass = self.attr.options.extends.is_some();
        let is_mapping: bool = self.attr.options.mapping.is_some();
        let is_sequence: bool = self.attr.options.sequence.is_some();

        ensure_spanned!(
            !(is_mapping && is_sequence),
            self.cls.span() => "a `#[pyclass]` cannot be both a `mapping` and a `sequence`"
        );

        let dict_offset = if self.attr.options.dict.is_some() {
            quote! {
                fn dict_offset() -> ::std::option::Option<_pyo3::ffi::Py_ssize_t> {
                    ::std::option::Option::Some(_pyo3::impl_::pyclass::dict_offset::<Self>())
                }
            }
        } else {
            TokenStream::new()
        };

        // insert space for weak ref
        let weaklist_offset = if self.attr.options.weakref.is_some() {
            quote! {
                fn weaklist_offset() -> ::std::option::Option<_pyo3::ffi::Py_ssize_t> {
                    ::std::option::Option::Some(_pyo3::impl_::pyclass::weaklist_offset::<Self>())
                }
            }
        } else {
            TokenStream::new()
        };

        let thread_checker = if self.attr.options.unsendable.is_some() {
            quote! { _pyo3::impl_::pyclass::ThreadCheckerImpl<#cls> }
        } else if self.attr.options.extends.is_some() {
            quote! {
                _pyo3::impl_::pyclass::ThreadCheckerInherited<#cls, <#cls as _pyo3::impl_::pyclass::PyClassImpl>::BaseType>
            }
        } else {
            quote! { _pyo3::impl_::pyclass::ThreadCheckerStub<#cls> }
        };

        let (pymethods_items, inventory, inventory_class) = match self.methods_type {
            PyClassMethodsType::Specialization => (quote! { collector.py_methods() }, None, None),
            PyClassMethodsType::Inventory => {
                // To allow multiple #[pymethods] block, we define inventory types.
                let inventory_class_name = syn::Ident::new(
                    &format!("Pyo3MethodsInventoryFor{}", cls.unraw()),
                    Span::call_site(),
                );
                (
                    quote! {
                        ::std::boxed::Box::new(
                            ::std::iter::Iterator::map(
                                _pyo3::inventory::iter::<<Self as _pyo3::impl_::pyclass::PyClassImpl>::Inventory>(),
                                _pyo3::impl_::pyclass::PyClassInventory::items
                            )
                        )
                    },
                    Some(quote! { type Inventory = #inventory_class_name; }),
                    Some(define_inventory_class(&inventory_class_name)),
                )
            }
        };

        let pyproto_items = if cfg!(feature = "pyproto") {
            Some(quote! {
                collector.object_protocol_items(),
                collector.number_protocol_items(),
                collector.iter_protocol_items(),
                collector.gc_protocol_items(),
                collector.descr_protocol_items(),
                collector.mapping_protocol_items(),
                collector.sequence_protocol_items(),
                collector.async_protocol_items(),
                collector.buffer_protocol_items(),
            })
        } else {
            None
        };

        let default_methods = self
            .default_methods
            .iter()
            .map(|meth| &meth.associated_method)
            .chain(
                self.default_slots
                    .iter()
                    .map(|meth| &meth.associated_method),
            );

        let default_method_defs = self.default_methods.iter().map(|meth| &meth.method_def);
        let default_slot_defs = self.default_slots.iter().map(|slot| &slot.slot_def);
        let freelist_slots = self.freelist_slots();

        let deprecations = &self.attr.deprecations;

        let class_mutability = if self.attr.options.frozen.is_some() {
            quote! {
                ImmutableChild
            }
        } else {
            quote! {
                MutableChild
            }
        };

        let cls = self.cls;
        let attr = self.attr;
        let dict = if attr.options.dict.is_some() {
            quote! { _pyo3::impl_::pyclass::PyClassDictSlot }
        } else {
            quote! { _pyo3::impl_::pyclass::PyClassDummySlot }
        };

        // insert space for weak ref
        let weakref = if attr.options.weakref.is_some() {
            quote! { _pyo3::impl_::pyclass::PyClassWeakRefSlot }
        } else {
            quote! { _pyo3::impl_::pyclass::PyClassDummySlot }
        };

        let base_nativetype = if attr.options.extends.is_some() {
            quote! { <Self::BaseType as _pyo3::impl_::pyclass::PyClassBaseType>::BaseNativeType }
        } else {
            quote! { _pyo3::PyAny }
        };

        Ok(quote! {
            impl _pyo3::impl_::pyclass::PyClassImpl for #cls {
                const DOC: &'static str = #doc;
                const IS_BASETYPE: bool = #is_basetype;
                const IS_SUBCLASS: bool = #is_subclass;
                const IS_MAPPING: bool = #is_mapping;
                const IS_SEQUENCE: bool = #is_sequence;

                type Layout = _pyo3::PyCell<Self>;
                type BaseType = #base;
                type ThreadChecker = #thread_checker;
                #inventory
                type PyClassMutability = <<#base as _pyo3::impl_::pyclass::PyClassBaseType>::PyClassMutability as _pyo3::impl_::pycell::PyClassMutability>::#class_mutability;
                type Dict = #dict;
                type WeakRef = #weakref;
                type BaseNativeType = #base_nativetype;

                fn items_iter() -> _pyo3::impl_::pyclass::PyClassItemsIter {
                    use _pyo3::impl_::pyclass::*;
                    let collector = PyClassImplCollector::<Self>::new();
                    #deprecations;
                    static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                        methods: &[#(#default_method_defs),*],
                        slots: &[#(#default_slot_defs),* #(#freelist_slots),*],
                    };
                    PyClassItemsIter::new(
                        &INTRINSIC_ITEMS,
                        #pymethods_items,
                        #pyproto_items
                    )
                }

                #dict_offset

                #weaklist_offset
            }

            #[doc(hidden)]
            #[allow(non_snake_case)]
            impl #cls {
                #(#default_methods)*
            }

            #inventory_class
        })
    }

    fn impl_freelist(&self) -> TokenStream {
        let cls = self.cls;

        self.attr.options.freelist.as_ref().map_or(quote!{}, |freelist| {
            let freelist = &freelist.value;
            quote! {
                impl _pyo3::impl_::pyclass::PyClassWithFreeList for #cls {
                    #[inline]
                    fn get_free_list(_py: _pyo3::Python<'_>) -> &mut _pyo3::impl_::freelist::FreeList<*mut _pyo3::ffi::PyObject> {
                        static mut FREELIST: *mut _pyo3::impl_::freelist::FreeList<*mut _pyo3::ffi::PyObject> = 0 as *mut _;
                        unsafe {
                            if FREELIST.is_null() {
                                FREELIST = ::std::boxed::Box::into_raw(::std::boxed::Box::new(
                                    _pyo3::impl_::freelist::FreeList::with_capacity(#freelist)));
                            }
                            &mut *FREELIST
                        }
                    }
                }
            }
        })
    }

    fn freelist_slots(&self) -> Vec<TokenStream> {
        let cls = self.cls;

        if self.attr.options.freelist.is_some() {
            vec![
                quote! {
                    _pyo3::ffi::PyType_Slot {
                        slot: _pyo3::ffi::Py_tp_alloc,
                        pfunc: _pyo3::impl_::pyclass::alloc_with_freelist::<#cls> as *mut _,
                    }
                },
                quote! {
                    _pyo3::ffi::PyType_Slot {
                        slot: _pyo3::ffi::Py_tp_free,
                        pfunc: _pyo3::impl_::pyclass::free_with_freelist::<#cls> as *mut _,
                    }
                },
            ]
        } else {
            Vec::new()
        }
    }
}

fn define_inventory_class(inventory_class_name: &syn::Ident) -> TokenStream {
    quote! {
        #[doc(hidden)]
        pub struct #inventory_class_name {
            items: _pyo3::impl_::pyclass::PyClassItems,
        }
        impl #inventory_class_name {
            pub const fn new(items: _pyo3::impl_::pyclass::PyClassItems) -> Self {
                Self { items }
            }
        }

        impl _pyo3::impl_::pyclass::PyClassInventory for #inventory_class_name {
            fn items(&self) -> &_pyo3::impl_::pyclass::PyClassItems {
                &self.items
            }
        }

        _pyo3::inventory::collect!(#inventory_class_name);
    }
}
