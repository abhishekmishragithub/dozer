use std::time::Duration;

use dozer_types::{
    chrono::DateTime,
    types::{
        DozerDuration, Field, FieldDefinition, FieldType, Lifetime, Record, Schema,
        SourceDefinition, TimeUnit,
    },
};

use crate::pipeline::{
    expression::execution::Expression,
    table_operator::{lifetime::LifetimeTableOperator, operator::TableOperator},
};

#[test]
fn test_lifetime() {
    let schema = Schema::empty()
        .field(
            FieldDefinition::new(
                "id".to_string(),
                FieldType::Int,
                false,
                SourceDefinition::Alias {
                    name: "alias".to_string(),
                },
            ),
            false,
        )
        .field(
            FieldDefinition::new(
                "ref".to_string(),
                FieldType::Timestamp,
                false,
                SourceDefinition::Alias {
                    name: "alias".to_string(),
                },
            ),
            false,
        )
        .to_owned();

    let record = Record::new(
        None,
        vec![
            Field::Int(0),
            Field::Timestamp(DateTime::parse_from_rfc3339("2020-01-01T00:13:00Z").unwrap()),
        ],
    );

    let table_operator = LifetimeTableOperator::new(
        None,
        Expression::Column { index: 1 },
        // Expression::new(
        //     ExpressionType::BinaryExpression {
        //         operator: BinaryOperator::Add,
        //         left: Box::new(Expression::new(ExpressionType::Field("ref".to_string()))),
        //         right: Box::new(Expression::new(ExpressionType::Literal(
        //             Literal::Duration(DozerDuration(
        //                 Duration::from_secs(60),
        //                 TimeUnit::Seconds,
        //             )),
        //         ))),
        //     },
        //     "ref".to_string(),
        // ),
        DozerDuration(Duration::from_secs(60), TimeUnit::Seconds),
    );

    let result = table_operator.execute(&record, &schema).unwrap();
    assert_eq!(result.len(), 1);
    let lifetime_record = result.get(0).unwrap();

    let mut expected_record = Record::new(
        None,
        vec![
            Field::Int(0),
            Field::Timestamp(DateTime::parse_from_rfc3339("2020-01-01T00:13:00Z").unwrap()),
        ],
    );

    expected_record.set_lifetime(Some(Lifetime {
        reference: Field::Timestamp(DateTime::parse_from_rfc3339("2020-01-01T00:13:00Z").unwrap()),
        duration: DozerDuration(Duration::from_secs(60), TimeUnit::Seconds),
    }));

    assert_eq!(*lifetime_record, expected_record);
}
