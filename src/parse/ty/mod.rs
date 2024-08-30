use crate::ast::*;
use chumsky::prelude::*;
use text::ident;

pub fn type_parser() -> impl Parser<char, Type, Error = Simple<char>> {
    recursive(|type_parser| {
        let type_kind = choice((
            just("Byte").to(TypeKind::Byte),
            just("UByte").to(TypeKind::UByte),
            just("Short").to(TypeKind::Short),
            just("UShort").to(TypeKind::UShort),
            just("Int").to(TypeKind::Int),
            just("UInt").to(TypeKind::UInt),
            just("Long").to(TypeKind::Long),
            just("ULong").to(TypeKind::ULong),
            just("Float").to(TypeKind::Float),
            just("Double").to(TypeKind::Double),
            just("Char").to(TypeKind::Char),
            just("String").to(TypeKind::String),
            just("Array").to(TypeKind::Array),
            just("ByteArray").to(TypeKind::ByteArray),
            just("UByteArray").to(TypeKind::UByteArray),
            just("ShortArray").to(TypeKind::ShortArray),
            just("UShortArray").to(TypeKind::UShortArray),
            just("IntArray").to(TypeKind::IntArray),
            just("UIntArray").to(TypeKind::UIntArray),
            just("LongArray").to(TypeKind::LongArray),
            just("ULongArray").to(TypeKind::ULongArray),
            just("FloatArray").to(TypeKind::FloatArray),
            just("DoubleArray").to(TypeKind::DoubleArray),
            just("BooleanArray").to(TypeKind::BooleanArray),
            just("CharArray").to(TypeKind::CharArray),
        ))
        .or(choice((
            just("Map").to(TypeKind::Map),
            just("MutableMap").to(TypeKind::MutableMap),
            just("EmptyMap").to(TypeKind::EmptyMap),
            just("Set").to(TypeKind::Set),
            just("MutableSet").to(TypeKind::MutableSet),
            just("EmptySet").to(TypeKind::EmptySet),
            just("List").to(TypeKind::List),
            just("MutableList").to(TypeKind::MutableList),
            just("EmptyList").to(TypeKind::EmptyList),
            just("Boolean").to(TypeKind::Boolean),
            just("Any").to(TypeKind::Any),
            just("Nothing").to(TypeKind::Nothing),
            just("Unit").to(TypeKind::Unit),
        )))
        .or(ident().map(TypeKind::Object))
        .padded();

        let attributes = type_parser
            .clone()
            .separated_by(just(',').padded())
            .delimited_by(just('<'), just('>'))
            .padded()
            .or_not()
            .map(|attrs| attrs.unwrap_or_default());

        let nullable = just('?').or_not().map(|n| n.is_some());

        type_kind.then(attributes).then(nullable).map(
            |((kind, attributes), nullable)| Type {
                kind,
                nullable,
                attributes: attributes.into_iter().map(Box::new).collect(),
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_parser() {
        assert_eq!(
            type_parser()
                .parse("Triple<UInt, Array<String>, HashMap<Key, Value>>?"),
            Ok(Type {
                kind: TypeKind::Object("Triple".to_string(),),
                nullable: true,
                attributes: vec![
                    Box::new(Type {
                        kind: TypeKind::UInt,
                        nullable: false,
                        attributes: vec![],
                    }),
                    Box::new(Type {
                        kind: TypeKind::Array,
                        nullable: false,
                        attributes: vec![Box::new(Type {
                            kind: TypeKind::String,
                            nullable: false,
                            attributes: vec![],
                        })],
                    }),
                    Box::new(Type {
                        kind: TypeKind::Object("HashMap".to_string()),
                        nullable: false,
                        attributes: vec![
                            Box::new(Type {
                                kind: TypeKind::Object("Key".to_string()),
                                nullable: false,
                                attributes: vec![],
                            }),
                            Box::new(Type {
                                kind: TypeKind::Object("Value".to_string()),
                                nullable: false,
                                attributes: vec![],
                            }),
                        ],
                    }),
                ],
            }),
        );
    }
}
