# Data Munging Using **\*X\***  
  
Data Engineering Workshops on some of the more popular libraries, frameworks and tech circa 2022-2023.  
  
<img src="data_munging_01.png" width="500" />  

---
  
# Notebooks  

Note - the *"10+ minutes to XX"* notebooks are just references, not to be run as actual workshop material. These are there to carry toy examples that "getting started" pages for XX carry. I have tried to ensure there's a 10+ minutes notebook for each data engineering library/framework considered here. While it may be interesting to go through these to quickly refresh the syntax and other idiosyncracies, the actual data munging happens in other notebooks.  
  
  
### 00 Python Collections  
  
	* [TODO]  
	
### 01 Numpy    
  
	* [TODO]  
	  
### 02 Pandas    
  
	* 02.*002* [10+ minutes to pandas](02.001%20-%2010%2B%20minutes%20to%20pandas.ipynb) or [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/02.001%20-%2010%2B%20minutes%20to%20pandas.ipynb) 
	* [TODO: Indexing and Selecting Data]
	* [TODO: Pivot Tables]
	* [TODO: Grouping, Windowing]
	* [TODO: Time]
	* [TODO: Possibly more real-world examples]
	  
### 03 Spark  
    
	* 03.*000* Optional notebook to demonstrate how to run Apache Spark based notebooks in Google Colab [Setup Spark in Google Colab](https://github.com/shauryashaurya/learn-data-munging/blob/main/03.000%20(optional)%20Setup_Spark_in_Google_Colab.ipynb) also [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/03.000%20(optional)%20Setup_Spark_in_Google_Colab.ipynb)
	* 03.*001* [10+ minutes to pyspark](03.001%20-%2010%2B%20minutes%20to%20pyspark.ipynb) or [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/03.001%20-%2010%2B%20minutes%20to%20pyspark.ipynb)
	* 03.*002* [Exploratory Data Analysis: Fun with the MovieLens Dataset using PySpark](03.002%20-%20Exploratory%20Data%20Analysis%20-%20Fun%20with%20the%20Movielens%20dataset%20using%20PySpark.ipynb) or [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/03.002%20-%20Exploratory%20Data%20Analysis%20-%20Fun%20with%20the%20Movielens%20dataset%20using%20PySpark.ipynb)  [TODO: *Maybe* break this into multiple notebooks for easier access...]  
    
### 04 Dask    
    
	* [WIP] 04.*001* [10+ minutes to dask](https://github.com/shauryashaurya/learn-data-munging/blob/main/04.001%20-%2010%2B%20minutes%20to%20dask.ipynb), also [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/04.001%20-%2010%2B%20minutes%20to%20dask.ipynb)
	* [TODO: real world analysis using Dask]  
	* [TODO: possibly compare groupby and foldby]
    
### 05 Polars    
    
	* [WIP] 05.*001* [10+ minutes to polars.ipynb](https://github.com/shauryashaurya/learn-data-munging/blob/main/05.001%20-%2010%2B%20minutes%20to%20polars.ipynb), also on [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/05.001%20-%2010%2B%20minutes%20to%20polars.ipynb) 
	* [TODO]
  	  
### 06 Apache Arrow and DataFusion    
    
	* [WIP] 06.*001* [10+ minutes to pyArrow and DataFusion.ipynb](https://github.com/shauryashaurya/learn-data-munging/blob/main/06.001%20-%2010%2B%20minutes%20to%20pyArrow%20and%20DataFusion.ipynb), also on [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/06.001%20-%2010%2B%20minutes%20to%20pyArrow%20and%20DataFusion.ipynb)	 
	* [TODO]  
    
### 07 Ray    
    
	* [TODO]  
    
### *99 Static: The TPC Benchmark Queries*    
    
	* [TODO] Also, question: can I just put the queries here? like is that ok? or will I get a slap on my wrist? Lemme try. 
  
  
# References:  
    
#### 01 [Numpy](https://numpy.org/doc/stable/user/index.html)  
    
* [Numpy User Guide (v1.23 as of this)](https://numpy.org/doc/stable/user/index.html#user)  
* [Numpy Tutorials](https://numpy.org/numpy-tutorials/features.html)  
* [_NumPy Basics: Arrays and Vectorized Computation_ from Wes Mckinney's Python for Data Analysis, 3E:](https://wesmckinney.com/book/numpy-basics.html)  
* [Numpy is absurd](https://gist.github.com/Moelf/59d6312c51c250ba251125e54bea7282)
* [100 Numpy Exercises](https://github.com/rougier/numpy-100)
* [From Python to Numpy](https://www.labri.fr/perso/nrougier/from-python-to-numpy/)
    
    
#### 02 [Pandas](https://pandas.pydata.org/pandas-docs/stable/user_guide/index.html)  
    
* [Pandas _(current stable version)_ User Guide](https://pandas.pydata.org/pandas-docs/stable/user_guide/index.html)
* [10 minutes to pandas](https://pandas.pydata.org/pandas-docs/stable/user_guide/10min.html)
* [_Data Cleaning and Preparation_ from Wes Mckinney's Python for Data Analysis, 3E:](https://wesmckinney.com/book/data-cleaning.html)  
* [_Data Wrangling: Join, Combine, and Reshape_ from Wes Mckinney's Python for Data Analysis, 3E:](https://wesmckinney.com/book/data-wrangling.html)  
* [_Data Aggregation and Group Operations_ from Wes Mckinney's Python for Data Analysis, 3E:](https://wesmckinney.com/book/data-aggregation.html)  
* [Effective Pandas | Matt Harrison](https://www.youtube.com/watch?v=zgbUk90aQ6A&t=309s), also [here](https://www.youtube.com/watch?v=UURvPeczxJI)  
* _...also from Matt harrison on github_: [effective pandas (book)](https://github.com/mattharrison/effective_pandas_book) and [idiomatic pandas tutorial](https://github.com/mattharrison/Idiomatic-Pandas-Tutorial/blob/main/Idiomatic%20Pandas.ipynb)  
* [Pandas Exercises](https://github.com/guipsamora/pandas_exercises)
* [100 Pandas Puzzles](https://github.com/ajcr/100-pandas-puzzles)
    
    
#### 03 [Spark](https://spark.apache.org/docs/latest/api/python/user_guide/index.html)  
   
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
    
    
#### 04 [Dask](https://docs.dask.org/en/stable/10-minutes-to-dask.html)  
    
The approach is different: Dask focuses on Task scheduling vs Spark's Map-Reduce  
* [10 minutes to Dask](https://docs.dask.org/en/stable/10-minutes-to-dask.html)  
* [90-minute Dask tutorial video](https://www.youtube.com/watch?v=_u0OQm9qf_A)  
* [Talks and tutorials page](https://docs.dask.org/en/latest/presentations.html)  
* [The Dask tutorial notebooks](https://github.com/dask/dask-tutorial)
* [The SciPy 2022 tutorial talk](https://www.youtube.com/watch?v=J0NcbvkYPoE)
    
     
#### 05 [Polars](https://www.pola.rs/)
    
* [Polars User Guide](https://pola-rs.github.io/polars-book/user-guide/) and [Getting Started](https://pola-rs.github.io/polars-book/getting-started/intro/)  
    
    
#### 06 [Arrow](https://arrow.apache.org/overview/),  [Arrow DataFusion](https://arrow.apache.org/datafusion/) and [Ballista](https://arrow.apache.org/ballista/)
    
* [Apache Arrow Official Native Rust Implementation](https://docs.rs/crate/arrow/latest)
* [pyArrow](https://arrow.apache.org/docs/python/)
* [Ballista on GitHub](https://github.com/apache/arrow-ballista)
* [Apache Arrow Python Cookbook](https://arrow.apache.org/cookbook/py/)
* [DataFusion User Guide](https://arrow.apache.org/datafusion/user-guide/introduction.html)
* [Arrow DataFusion Python](https://github.com/apache/arrow-datafusion-python)
* [Arrow NumPy Integration](https://arrow.apache.org/docs/python/numpy.html)
* [Arrow Pandas Integration](https://arrow.apache.org/docs/python/pandas.html)
* [DataFusion Roadmap Epics](https://github.com/apache/arrow-datafusion/issues?q=is%3Aissue+is%3Aopen+epic)
    
    
#### 07 [Ray](https://www.ray.io/)  
    
* [Ray Core](https://docs.ray.io/en/latest/ray-core/user-guide.html)
* [Ray Dataset Quickstart](https://docs.ray.io/en/latest/data/getting-started.html#datasets-getting-started)
* [Ray Data](https://docs.ray.io/en/latest/data/user-guide.html)
* [Ray with Spark](https://github.com/oap-project/raydp)
* [Ray with Dask]()
    
    
# Future State / Miscellany  
    
Datasets we use:  
* [MovieLens 25M Dataset](https://grouplens.org/datasets/movielens/25m/)
* [Wikipedia Movie Plots](https://www.kaggle.com/datasets/jrobischon/wikipedia-movie-plots)  
* [CMU Movie Summary Corpus](https://paperswithcode.com/dataset/cmu-movie-summary-corpus) also [here](http://www.cs.cmu.edu/~ark/personas/)
* [MoviePlotEvents (CMU Movie Summary Corpus with Events)](https://paperswithcode.com/dataset/movieplotevents) also [here](https://www.dropbox.com/s/i5dsk92735jpunf/EventRepresentationDataRelease.zip?dl=0)
    
    
There's a lot of interesting (interesting to me) tools, datasets and papers out there.   
When there's time or need, we'll get to them as well.  
    
* [Arrow](https://arrow.apache.org/) and [pyArrow](https://arrow.apache.org/cookbook/py/) really warrant a deeper study. Maybe a gateway to Rust based data processing. Not really *emerging* anymore, a lot of very cool stuff is being done with this and datafusion, very interesting to explore.  
* [Apache Arrow Ballista](https://arrow.apache.org/ballista/) is looking very interesting from a next gen distributed processing PoV  
* [PRQL](https://prql-lang.org/), [*on github*](https://github.com/PRQL/prql) and [PRQL Query](https://github.com/PRQL/prql-query). Also the [PRQL Book](https://prql-lang.org/book/).  
* [Mars](https://docs.pymars.org/en/latest/) and [Project Mars on GitHub](https://github.com/mars-project/mars)
* [Modin](https://github.com/modin-project/modin)  
* [Polars](https://www.pola.rs/). Also, [Polars Github Repo](https://github.com/pola-rs/polars/)  
* [Danfo.js - pandas like dataframes in JavaScript](https://danfo.jsdata.org/)
* [Orchest](https://www.orchest.io/) for data pipelines - maybe?  Update: as of Jly 2023, Nah!
* I think there's something to be said about leveraging TPC benchmarks - we'll attend to this in due time. There's got to be a .md readme in this repo that'll list all the queries anyway. Yea, lemme do that soonish.
* Is there value in comparing formats? (Parquet)[https://parquet.apache.org/docs/], (Zarr)[https://zarr.readthedocs.io/en/stable/tutorial.html] etc.?
* [Papers and Data - Scifi TV Shows (Scifi TV Show Plot Summaries & Events)](https://paperswithcode.com/dataset/scifi-tv-plots)    
* [Papers and Data - Story Cloze](https://paperswithcode.com/dataset/storycloze)   
* [State Of The Art on paperswithcode (](https://paperswithcode.com/sota)  
* [Only cause LMMs have been trending for a while - A Survey of Large Language Models](https://paperswithcode.com/paper/a-survey-of-large-language-models)  
* [SST (Stanford Sentiment Treebank)](https://paperswithcode.com/dataset/sst), [also](https://nlp.stanford.edu/sentiment/)
* ...    
    
	   
# MOAR GIMME MOAR LINKS!!!  
  
Kitchen sink of all other references I've found useful (or wonderful). There's so much to learn I tell you!  
* [How Query Engines Work](https://howqueryengineswork.com/00-introduction.html)
* Carnegie Mellon's Advanced Database Systems Playlist: 
	* [Spring 2020](https://www.youtube.com/playlist?list=PLSE8ODhjZXjasmrEd2_Yi1deeE360zv5O)
	* [Spring 2023 session](https://youtube.com/playlist?list=PLSE8ODhjZXjYzlLMbX3cR0sxWnRM7CLFn), also [CMU 15-721 (2023)](https://15721.courses.cs.cmu.edu/spring2023/schedule.html)
* Go here if the advanced database systems feels hard - [CMU Intro to Database Systems (15-445/645 - Fall 2022)](https://www.youtube.com/playlist?list=PLSE8ODhjZXjaKScG3l0nuOiDTTqpfnWFf), also [course site](https://15445.courses.cs.cmu.edu/fall2022/schedule.html)
* [Database Query Optimizers](https://www.youtube.com/playlist?list=PLSE8ODhjZXjYPyrUG_YxqYPS7wjWY6gYN)
* [¡Databases! – A Database Seminar Series (Fall 2022)](https://www.youtube.com/playlist?list=PLSE8ODhjZXjZKp-oX_75aBnznulk7nubu), also on [CMU](https://db.cs.cmu.edu/seminar2022/)
* [Hardware Accelerated Database Lectures (Fall 2018)](https://db.cs.cmu.edu/seminar2018/)
* [Time Series Database Lectures (Fall 2017)](https://www.youtube.com/playlist?list=PLSE8ODhjZXjY0GMWN4X8FIkYNfiu8_Wl9)
* [The Databaseology Lectures (Fall 2015)](https://www.youtube.com/playlist?list=PLSE8ODhjZXjakeQR57ZdN5slUu2oPUr1Y)
* [Seven Databases in Seven Weeks (Fall 2014)](https://www.youtube.com/playlist?list=PLSE8ODhjZXjY2xvwxuKjZT5qFH0sQga8_)
    
    
.    
