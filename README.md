# Data Munging Using **\*X\***  
  
Data Engineering Workshops on some of the more popular libraries circa 2022-2023.  
  
<img src="data_munging_01.png" width="500" />  

---
  
# Notebooks:  
  
* **00 Numpy**
	* [TODO]
* **01 Pandas**
	* **01**.*001* [10+ minutes to pandas](01.001%20-%2010%2B%20minutes%20to%20pandas.ipynb) or [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/01.001%20-%2010%2B%20minutes%20to%20pandas.ipynb)
	* [TODO: Indexing and Selecting Data]
	* [TODO: Pivot Tables]
	* [TODO: Grouping, Windowing]
	* [TODO: Time]
	* [TODO: Possibly more real-world examples]
* **02 Spark**
	* **02**.*000* Optional notebook to demonstrate how to run Apache Spark based notebooks in Google Colab [Setup Spark in Google Colab](https://github.com/shauryashaurya/learn-data-munging/blob/main/02.000%20(optional)%20Setup_Spark_in_Google_Colab.ipynb) also [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/02.000%20(optional)%20Setup_Spark_in_Google_Colab.ipynb)
	* **02**.*001* [10+ minutes to pyspark](02.001%20-%2010%2B%20minutes%20to%20pyspark.ipynb) or [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/02.001%20-%2010%2B%20minutes%20to%20pyspark.ipynb)
	* **02**.*002* [Exploratory Data Analysis: Fun with the MovieLens Dataset using PySpark](02.002%20-%20Exploratory%20Data%20Analysis%20-%20Fun%20with%20the%20Movielens%20dataset%20using%20PySpark.ipynb) or [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/02.002%20-%20Exploratory%20Data%20Analysis%20-%20Fun%20with%20the%20Movielens%20dataset%20using%20PySpark.ipynb)  [**TODO**: *Maybe* break this into multiple notebooks for easier access...]  
	
* **03 Dask**  
	* **03**.**001** [10+ minutes to dask](https://github.com/shauryashaurya/learn-data-munging/blob/main/03.001%20-%2010%2B%20minutes%20to%20dask.ipynb)[![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/03.001%20-%2010%2B%20minutes%20to%20dask.ipynb)
	* [TODO: real world analysis using Dask]  
* **04 Ray**  
	* [TODO]  
* **05 Apache Arrow and DataFusion**  
	* [TODO]  
* **06 Polars**
	* [TODO]
* *99 Static: The **TPC Benchmark** Queries*
	* [TODO] Also, question: can I just put the queries here? like is that ok? or will I get a slap on my wrist? Lemme try. 


# References:  
  
## 00 [Numpy](https://numpy.org/doc/stable/user/index.html)  
  
Foundations  
* [Numpy User Guide (v1.23 as of this)](https://numpy.org/doc/stable/user/index.html#user)  
* [Numpy Tutorials](https://numpy.org/numpy-tutorials/features.html)  
* [_NumPy Basics: Arrays and Vectorized Computation_ from Wes Mckinney's Python for Data Analysis, 3E:](https://wesmckinney.com/book/numpy-basics.html)  
* [Numpy is absurd](https://gist.github.com/Moelf/59d6312c51c250ba251125e54bea7282)
* [100 Numpy Exercises](https://github.com/rougier/numpy-100)
* [From Python to Numpy](https://www.labri.fr/perso/nrougier/from-python-to-numpy/)
  
  
## 01 [Pandas](https://pandas.pydata.org/pandas-docs/stable/user_guide/index.html)  
  
The go to data munging approach  
* [Pandas _(current stable version)_ User Guide](https://pandas.pydata.org/pandas-docs/stable/user_guide/index.html)
* [10 minutes to pandas](https://pandas.pydata.org/pandas-docs/stable/user_guide/10min.html)
* [_Data Cleaning and Preparation_ from Wes Mckinney's Python for Data Analysis, 3E:](https://wesmckinney.com/book/data-cleaning.html)  
* [_Data Wrangling: Join, Combine, and Reshape_ from Wes Mckinney's Python for Data Analysis, 3E:](https://wesmckinney.com/book/data-wrangling.html)  
* [_Data Aggregation and Group Operations_ from Wes Mckinney's Python for Data Analysis, 3E:](https://wesmckinney.com/book/data-aggregation.html)  
* [Effective Pandas | Matt Harrison](https://www.youtube.com/watch?v=zgbUk90aQ6A&t=309s), also [here](https://www.youtube.com/watch?v=UURvPeczxJI)  
* _...also from Matt harrison on github_: [effective pandas (book)](https://github.com/mattharrison/effective_pandas_book) and [idiomatic pandas tutorial](https://github.com/mattharrison/Idiomatic-Pandas-Tutorial/blob/main/Idiomatic%20Pandas.ipynb)  
* [Pandas Exercises](https://github.com/guipsamora/pandas_exercises)
* [100 Pandas Puzzles](https://github.com/ajcr/100-pandas-puzzles)
  
  
TODO:  
* Gotchas and a bit more indepth on some topics
* Real world analysis example instead of toy dataframes
  
  
## 02 [Spark](https://spark.apache.org/docs/latest/api/python/user_guide/index.html)  

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
* _weak references, dated but interesting_
	* [Spark Python Notebooks](https://github.com/jadianes/spark-py-notebooks)
	* [Data Science Engineering, your way](https://github.com/jadianes/data-science-your-way)
	* [A scalable on-line movie recommender using Spark and Flask](https://github.com/jadianes/spark-movie-lens)
* [PySpark Cheatsheet](https://github.com/kevinschaich/pyspark-cheatsheet)
  
  
## 03 [Dask](https://docs.dask.org/en/stable/10-minutes-to-dask.html)  
  
The approach is different: Dask focuses on Task scheduling vs Spark's Map-Reduce  
* [10 minutes to Dask](https://docs.dask.org/en/stable/10-minutes-to-dask.html)  
* [90-minute Dask tutorial video](https://www.youtube.com/watch?v=_u0OQm9qf_A)  
* [Talks and tutorials page](https://docs.dask.org/en/latest/presentations.html)  
  
  
## 04 [Ray](https://www.ray.io/)  
  
* [Ray Core](https://docs.ray.io/en/latest/ray-core/user-guide.html)
* [Ray Dataset Quickstart](https://docs.ray.io/en/latest/data/getting-started.html#datasets-getting-started)
* [Ray Data](https://docs.ray.io/en/latest/data/user-guide.html)
* [Ray with Spark](https://github.com/oap-project/raydp)
* [Ray with Dask]()

## 05 [Arrow](https://arrow.apache.org/overview/) and [Arrow DataFusion](https://arrow.apache.org/datafusion/)
* [Apache Arrow Official Native Rust Implementation](https://docs.rs/crate/arrow/latest)
* [pyArrow](https://arrow.apache.org/docs/python/)
* [Apache Arrow Python Cookbook](https://arrow.apache.org/cookbook/py/)
* [DataFusion User Guide](https://arrow.apache.org/datafusion/user-guide/introduction.html)
* [Arrow DataFusion Python](https://github.com/apache/arrow-datafusion-python)

## 06 [Polars](https://www.pola.rs/)
* [Polars User Guide](https://pola-rs.github.io/polars-book/user-guide/) and [Getting Started](https://pola-rs.github.io/polars-book/getting-started/intro/)
  
  
# Future State / Miscellany  
There's a lot of interesting tools emerging.   
When there's time or need, we'll get to them as well.  
* [Arrow](https://arrow.apache.org/) and [pyArrow](https://arrow.apache.org/cookbook/py/) really warrant a deeper study. Maybe a gateway to Rust based data processing. Not really *emerging* anymore, a lot of very cool stuff is being done with this and datafusion, very interesting to explore.  
* [Apache Arrow Ballista](https://arrow.apache.org/ballista/) is looking very interesting from a next gen distributed processing PoV
* [Mars](https://docs.pymars.org/en/latest/)  
* [Modin](https://github.com/modin-project/modin)  
* [Polars](https://www.pola.rs/). Also, [Polars Github Repo](https://github.com/pola-rs/polars/)  
* [Danfo.js - pandas like dataframes in JavaScript](https://danfo.jsdata.org/)
* [Orchest](https://www.orchest.io/) for data pipelines - maybe?  
* I think there's something to be said about leveraging TPC benchmarks - we'll attend to this in due time. There's got to be a .md readme in this repo that'll list all the queries anyway. Yea, lemme do that soonish.
* ...    
  
# MOAR GIMME MOAR LINKS!!!  
Kitchen sink of all other references I've found useful (or wonderful). There's so much to learn I tell you!  
* [How Query Engines Work](https://howqueryengineswork.com/00-introduction.html)
* [Carnegie Mellon's Advanced Database Systems Playlist on YouTube](https://www.youtube.com/playlist?list=PLSE8ODhjZXjasmrEd2_Yi1deeE360zv5O)



.  
