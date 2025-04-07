macro_rules! create {
    ($t:ident) => {
        paste::paste! {
            #[derive(::std::fmt::Debug, ::serde::Deserialize)]
            pub struct [<Create $t>] {
                data: [<Create $t Data>]
            }

            #[derive(::std::fmt::Debug, ::serde::Deserialize)]
            struct [<Create $t Data>] {
                #[serde(rename = "type")]
                _type: String,
                attributes: $t,
            }

            // TODO: Implement validation of "type" field

            impl ::std::convert::From<[<Create $t>]> for $t {
                fn from(value: [<Create $t>]) -> Self {
                    value.data.attributes
                }
            }
        }
    };
}

pub(crate) use create;
