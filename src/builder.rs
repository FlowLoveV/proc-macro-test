use proc_macro2::Indet;
use syn::{punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput, Field, Fields};

#[derive(Debug)]
pub struct BuuildContext {
    name: Indet,
    field: Punctuated<Field, Comma>,
}

impl BuuildContext {
    // from DeriveInput
    fn from(input: DeriveInput) -> Self {
        let name = input.ident;

        let fields = if let Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) = input.data
        {
            named
        } else {
            panic!("Unsupported data type");
        };

        let fds = fields.into_iter().map(Fd::from).collect();
        Self { name, fields: fds }
    }

    // into TokeStream
    pub fn render(&self) -> TokenStream {
        let name = &self.name;
        // 生成 XXXBuilder 的 ident
        let builder_name = Ident::new(&format!("{}Builder", name), name.span());

        let optionized_fields = self.gen_optionized_fields();
        let methods = self.gen_methods();
        let assigns = self.gen_assigns();

        quote! {
            /// Builder 结构
            #[derive(Debug, Default)]
            struct #builder_name {
                #(#optionized_fields,)*
            }

            /// Builder 结构每个字段赋值的方法，以及 build() 方法
            impl #builder_name {
                #(#methods)*

                pub fn build(mut self) -> Result<#name, &'static str> {
                    Ok(#name {
                        #(#assigns,)*
                    })
                }
            }

            /// 为使用 Builder 的原结构提供 builder() 方法，生成 Builder 结构
            impl #name {
                fn builder() -> #builder_name {
                    Default::default()
                }
            }
        }
    }
}
