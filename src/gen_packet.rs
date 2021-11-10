macro_rules! packet {
    ($name:ident, $($field_name:ident : $field_type:ty),*) => {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            $(pub $field_name: $field_type),*
        }
        #[async_trait::async_trait]
        impl crate::parsable::Parsable for $name {
            fn default() -> $name {
                $name {
                    $(
                        $field_name: Default::default()
                    ),*
                }
            }
        }
    };
}
