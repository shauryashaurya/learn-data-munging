use arrow::datatypes::{DataType, Field, Schema};
use ballista::prelude::*;
use datafusion::logical_plan::Expr;
use datafusion::prelude::*;

fn main() {
    // Define the schema
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::UInt32, false),
        Field::new("name", DataType::Utf8, false),
    ]));

    // Create an empty DataFusion ExecutionContext
    let mut ctx = ExecutionContext::new();

    // Create an example DataFusion table
    let table = create_example_table(&schema);

    // Register the DataFusion table in the context
    ctx.register_table("my_table", table);

    // Define a query using DataFusion
    let query = ctx
        .sql("SELECT name FROM my_table WHERE id > 5")
        .unwrap();

    // Convert the DataFusion query plan to Ballista logical plan
    let plan = BallistaLogicalPlanBuilder::from_datafusion_query(&query, &schema)
        .unwrap()
        .build()
        .unwrap();

    // Create a Ballista context and execute the plan
    let mut ballista_ctx = BallistaContext::remote("localhost", 50050).unwrap();
    let result = ballista_ctx
        .execute(&ExecutionConfig::new(), vec![plan])
        .unwrap();

    // Display the result
    for batch in result {
        println!("{:?}", batch);
    }
}

fn create_example_table(schema: &Schema) -> Arc<dyn TableProvider> {
    // Create an example Arrow Table
    let data = vec![
        vec![1u32, 2u32, 3u32, 4u32, 5u32],
        vec!["Alice", "Bob", "Charlie", "Dave", "Eve"],
    ];
    let array_data: Vec<ArrayRef> = data
        .iter()
        .enumerate()
        .map(|(i, values)| {
            let field = schema.field(i);
            let array: ArrayRef = match field.data_type() {
                DataType::UInt32 => Arc::new(UInt32Array::from(values.clone())),
                DataType::Utf8 => Arc::new(StringArray::from(values.clone())),
                _ => unreachable!(),
            };
            array
        })
        .collect();

    let batch = RecordBatch::try_new(schema.clone(), array_data).unwrap();
    let table = MemTable::try_new(schema.clone(), vec![vec![batch]]).unwrap();
    Arc::new(table)
}
