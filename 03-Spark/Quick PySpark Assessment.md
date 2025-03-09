# Step 1: Basic PySpark Qualifier        
        
Part 1: Multiple Choice Assessment (Basic PySpark Concepts)        
        
Here are five multiple-choice questions to test fundamental PySpark knowledge:        
        
1.  Which of the following is NOT a valid Spark transformation?        
    *   a) `map`        
    *   b) `filter`        
    *   c) `reduceByKey`        
    *   d) `collect`        
    *   e) `flatMap`        
        
    *   Answer: d) `collect`        
    *   Explanation: `collect` is an *action* in Spark, not a transformation.  Transformations are lazy and define operations on DataFrames/RDDs, while actions trigger the computation and return a result to the driver. `map`, `filter`, `reduceByKey`, and `flatMap` are all transformations that create new RDDs/DataFrames based on the input.        
        
2.  What is the primary purpose of the Spark Catalyst Optimizer?        
    *   a) To manage cluster resources.        
    *   b) To optimize query execution plans.        
    *   c) To handle data serialization and deserialization.        
    *   d) To provide a user-friendly web interface.        
    *   e) To load data        
	  
    *   Answer: b) To optimize query execution plans.        
    *   Explanation: The Catalyst Optimizer is a crucial component of Spark SQL. It analyzes logical plans (how the query is written) and transforms them into optimized physical plans (how the query will actually be executed), using techniques like predicate pushdown, column pruning, and choosing efficient join strategies.        
        
3.  What is the difference between `persist()` and `cache()` in PySpark?        
    *   a) `persist()` allows you to specify a storage level, while `cache()` always uses `MEMORY_AND_DISK`.        
    *   b) `cache()` allows you to specify a storage level, while `persist()` always uses `MEMORY_ONLY`.        
    *   c) `persist()` allows you to specify a storage level, while `cache()` always uses `MEMORY_ONLY`.        
    *   d) There is no functional difference; they are synonyms.        
    *   e) `persist()` is used for RDD while `cache()` is for DataFrame        
        
    *   Answer: c) `persist()` allows you to specify a storage level, while `cache()` always uses `MEMORY_ONLY`.        
    *   Explanation:  `cache()` is a shorthand for `persist(StorageLevel.MEMORY_ONLY)`. `persist()` provides more flexibility, letting you choose storage levels like `MEMORY_AND_DISK`, `DISK_ONLY`, etc., to balance memory usage and performance.        
        
4.  What does the `repartition()` transformation do?        
    *   a) Filters rows based on a condition.        
    *   b) Changes the number of partitions in an RDD or DataFrame, potentially causing a full shuffle.        
    *   c) Groups data by a key.        
    *   d) Sorts data within each partition.        
    *   e) Renames a column.        
        
    *   Answer: b) Changes the number of partitions in an DataFrame or RDD, potentially causing a full shuffle.        
    *   Explanation: `repartition()` is used to increase or decrease the parallelism of your data.  It redistributes data across the cluster, which involves a full shuffle (data is moved between nodes).  This is a relatively expensive operation.        
        
5.  Which of the following is the most efficient way to perform a join between two large DataFrames in Spark?        
    *   a)  Using a cross join followed by a filter.        
    *   b)  Using a broadcast join if one DataFrame is significantly smaller than the other.        
    *   c)  Using a Cartesian product.        
    *   d)  Always using a sort-merge join, regardless of DataFrame size.        
    *   e)  Converting to RDD and using groupByKey        
        
    *   Answer: b) Using a broadcast join if one DataFrame is significantly smaller than the other.        
    *   Explanation:  A broadcast join is highly efficient when one DataFrame is small enough to fit in memory on each executor. Spark broadcasts the smaller DataFrame to all nodes, avoiding the need to shuffle the larger DataFrame.  Cross joins and Cartesian products are *extremely* inefficient for large datasets. Sort-merge join is the default and a good general-purpose join, but broadcast join is superior when applicable.  Converting to RDDs and using `groupByKey` is generally less efficient than DataFrame operations for joins.        
        
        
        
# Step 2: PySpark Code Questions        
        
Use the accompanying `customer_orders.csv` data for answering the following questions.        
        
Schema Description: `customer_orders` Dataset        
        
The `customer_orders` dataset represents e-commerce transactions and contains information about customers, products, and orders.            
It's structured as a single table with the following columns:          
           
| Column Name          | Data Type      | Description                                                                                               |        
|-----------------------|----------------|----------------------------------------------------------------------------------------------------------|        
| `customer_id`        | String (UUID)  | Unique identifier for each customer.                                                                      |        
| `customer_name`       | String         | The full name of the customer.                                                                           |        
| `customer_email`       | String         | The email address of the customer.                                                                      |        
| `customer_join_date` | Timestamp      | The date and time when the customer joined the platform.                                                  |        
| `product_id`         | String (UUID)  | Unique identifier for each product.                                                                       |        
| `product_name`       | String         | The name of the product (e.g., "Laptop", "Smartphone").                                                   |        
| `product_category`   | String         | The category the product belongs to (e.g., "Electronics", "Accessories").                                 |        
| `product_price`      | Integer        | The price of a single unit of the product.                                                                |        
| `order_id`           | String (UUID)  | Unique identifier for each order.                                                                         |        
| `order_date`         | Timestamp      | The date and time when the order was placed.  Guaranteed to be after `customer_join_date`                 |        
| `order_quantity`     | Integer        | The number of units of the product ordered in this specific order line.                                   |        
| `order_total`        | Integer        | The total cost of the order line (`product_price` * `order_quantity`).                                    |        
        
Key Relationships and Constraints:        
        
*   One-to-Many Relationships:        
    *   A single customer (`customer_id`) can have multiple orders (`order_id`).        
    *   A single product (`product_id`) can be part of multiple orders (`order_id`).        
*   Data Integrity:        
    *   `order_date` is always greater than or equal to `customer_join_date` for the corresponding customer.        
    *    `order_total` is calculated correctly based on `product_price` and `order_quantity`.        
* Data Type Notes: The Timestamp datetype contains both date and time information.        
        
                
---        
Assume the following setup code for spark:        
        
```        
# import pyspark        
import pyspark        
from pyspark.sql import SparkSession        
from pyspark.sql.types import StructType, StructField, StringType, IntegerType, TimestampType, DoubleType        
        
# Define the Schema        
schema = StructType([        
    StructField("customer_id", StringType(), True),        
    StructField("customer_name", StringType(), True),        
    StructField("customer_email", StringType(), True),        
    StructField("customer_join_date", TimestampType(), True),        
    StructField("product_id", StringType(), True),        
    StructField("product_name", StringType(), True),        
    StructField("product_category", StringType(), True),        
    StructField("product_price", IntegerType(), True),        
    StructField("order_id", StringType(), True),        
    StructField("order_date", TimestampType(), True),        
    StructField("order_quantity", IntegerType(), True),        
    StructField("order_total", IntegerType(), True),        
])        
        
# Create a SparkSession        
spark = SparkSession.builder.appName("LoadCustomerOrders").getOrCreate()        
        
# Load the CSV data with the defined schema        
# Assuming 'customer_orders.csv' is in the current directory        
# Adjust the path if necessary.        
try:        
    df = spark.read.csv("customer_orders.csv", header=True, schema=schema)        
        
    # Verify the schema and show some data        
    df.printSchema()        
    df.show(5)        
except Exception as e:        
    print(f"Error loading or processing data: {e}")        
    print("Please make sure 'customer_orders.csv' exists in the correct path.")        
```	        
	
---                  
        
Answer the following questions to the best of your ability:      
        
1.  Find the total number of orders for each product category.  Display the category and the order count, sorted by order count in descending order.        
        
    ```python        
    from pyspark.sql import functions as F        
        
    # Using countDistinct is important to avoid double-counting if the same order appears multiple times (e.g., due to multiple items in the order).        
    result = spark_df.groupBy("product_category").agg(        
        F.countDistinct("order_id").alias("order_count")        
    ).orderBy(F.col("order_count").desc())        
        
    result.show()        
    ```        
        
2.  Calculate the average order total for each customer. Display the customer ID, customer name, and their average order total, rounded to two decimal places.        
        
    ```python        
    from pyspark.sql import functions as F        
        
    # Calculates the average of the "order_total" for each customer, rounds it to two decimal places, and renames the column.        
    result = spark_df.groupBy("customer_id", "customer_name").agg(        
        F.round(F.avg("order_total"), 2).alias("average_order_total")        
    )        
        
    result.show()        
    ```        
        
3.  Find the customers who joined in the last year (from today) and have placed more than 3 orders. Display customer ID, join date, and total order count.        
        
    ```python        
    from pyspark.sql import functions as F        
    from datetime import datetime, timedelta        
        
    # Calculate one_year_ago:  Gets the date one year prior to today.        
    one_year_ago = datetime.now() - timedelta(days=365)        
        
    # F.lit() is used to create a literal value from the Python datetime object for comparison.        
    result = spark_df.groupBy("customer_id", "customer_join_date").agg(        
        F.countDistinct("order_id").alias("total_orders")        
    ).filter(        
        (F.col("customer_join_date") >= F.lit(one_year_ago)) & (F.col("total_orders") > 3)        
    )        
        
    result.show()        
    ```        
        
4.  What is the most popular product (highest total quantity ordered)? Display the product name and total quantity ordered.        
        
    ```python        
    from pyspark.sql import functions as F        
        
    # Crucially, limit(1) takes only the top row (the most popular product).          
	# Without limit(), you'd get the sorted list of all products.        
    result = spark_df.groupBy("product_name").agg(        
        F.sum("order_quantity").alias("total_quantity")        
    ).orderBy(F.col("total_quantity").desc()).limit(1)        
        
    result.show()        
    ```        
        
5. Create a new column called `order_year_month` that represents the year and month of each order in the format "YYYY-MM".        
        
    ```python        
    from pyspark.sql import functions as F        
        
    result_df = spark_df.withColumn("order_year_month", F.date_format(F.col("order_date"), "yyyy-MM"))        
    result_df.show(5)        
    ```        
