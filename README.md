# Data munging using **X**
* 00 [Numpy](https://numpy.org/doc/stable/user/index.html)
* 01 [Pandas](https://pandas.pydata.org/pandas-docs/stable/user_guide/index.html)
* 02 [Spark](https://spark.apache.org/docs/latest/api/python/user_guide/index.html)
* 03 [Dask](https://docs.dask.org/en/stable/10-minutes-to-dask.html)
* 04 [Ray](https://www.ray.io/)


# 00 Numpy

Foundations  
* [Numpy User Guide (v1.23 as of this)](https://numpy.org/doc/stable/user/index.html#user)  
* [Numpy Tutorials](https://numpy.org/numpy-tutorials/features.html)  
* [_NumPy Basics: Arrays and Vectorized Computation_ from Wes Mckinney's Python for Data Analysis, 3E:](https://wesmckinney.com/book/numpy-basics.html)  
* [Numpy is absurd](https://gist.github.com/Moelf/59d6312c51c250ba251125e54bea7282)


# 01 Pandas

The go to data munging approach  
* [Pandas 1.4.3 User Guide](https://pandas.pydata.org/pandas-docs/stable/user_guide/index.html)
* [10 minutes to pandas](https://pandas.pydata.org/pandas-docs/stable/user_guide/10min.html)
* [_Data Cleaning and Preparation_ from Wes Mckinney's Python for Data Analysis, 3E:](https://wesmckinney.com/book/data-cleaning.html)  
* [_Data Wrangling: Join, Combine, and Reshape_ from Wes Mckinney's Python for Data Analysis, 3E:](https://wesmckinney.com/book/data-wrangling.html)  
* [_Data Aggregation and Group Operations_ from Wes Mckinney's Python for Data Analysis, 3E:](https://wesmckinney.com/book/data-aggregation.html)  


TODO:
* Gotchas and a bit more indepth on some topics
* Real world analysis example instead of toy dataframes


# 02 Spark

Distributed, Map-Reduce based jobs.  
Using Pyspark.  
Future State: if time permits will try a bunch of Scala Notebooks too.  
* [Spark User Guide](https://spark.apache.org/docs/latest/api/python/user_guide/index.html)
* [**The Internals of Apache Spark** online book](https://books.japila.pl/apache-spark-internals/overview/)
* [PySpark User Guide](https://spark.apache.org/docs/latest/api/python/user_guide/index.html)
	* This is also available as live binder notebooks:
		* [Live Notebook: DataFrame](https://mybinder.org/v2/gh/apache/spark/f74867bddf?filepath=python%2Fdocs%2Fsource%2Fgetting_started%2Fquickstart_df.ipynb)
		* [Live Notebook: Pandas API on Spark](https://mybinder.org/v2/gh/apache/spark/f74867bddf?filepath=python%2Fdocs%2Fsource%2Fgetting_started%2Fquickstart_ps.ipynb)
* [Spark SQL and Built-in Functions Reference](https://spark.apache.org/docs/latest/api/sql/index.html)


# 03 Dask

The approach is different: Dask focuses on Task scheduling vs Spark's Map-Reduce  
* [10 minutes to Dask](https://docs.dask.org/en/stable/10-minutes-to-dask.html)
* [90-minute Dask tutorial video](https://www.youtube.com/watch?v=_u0OQm9qf_A)
* [Talks and tutorials page](https://docs.dask.org/en/latest/presentations.html)


# 04 Ray

* [Ray Core](https://docs.ray.io/en/latest/ray-core/user-guide.html)
* [Ray Dataset Quickstart](https://docs.ray.io/en/latest/data/getting-started.html#datasets-getting-started)
* [Ray Data](https://docs.ray.io/en/latest/data/user-guide.html)
* [Ray with Spark](https://github.com/oap-project/raydp)
* [Ray with Dask]()


# Future State / Miscellany
There's a lot of interesting tools out there.  
When there's time or need, we'll get to them as well.  
* [Arrow](https://arrow.apache.org/) and [pyArrow](https://arrow.apache.org/cookbook/py/) really warrant a deeper study. Maybe a gateway to Rust based data processing. 
* [Mars](https://docs.pymars.org/en/latest/)
* [Modin](https://github.com/modin-project/modin)
* [Polars](https://www.pola.rs/). Also, [Polars Github Repo](https://github.com/pola-rs/polars/)
* [Danfo.js - pandas like dataframes in JavaScript](https://danfo.jsdata.org/)
* [Orchest](https://www.orchest.io/) for data pipelines - maybe?
* ...  



.  
