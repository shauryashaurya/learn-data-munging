# Data Munging Using **\*X\***  - Outline for PyData Global 2023
  
<html> 
	<img src="./../images/data_munging_pydata2023.png" width="95%" align="center" alt="Data Wrangling with Python, A Tutorial for PyData Global 2023" />  
	<p font-size="8px">Data Wrangling with Python, A Tutorial for PyData Global 2023</p> 
</html>

This page lists the notebooks we will cover during [the tutorial session for PyData Global 2023 ](https://youtu.be/AHgio9PG5Cw)  
tip: *use ```ctrl+click``` (Windows) or ```cmd+click``` (Mac) to watch the full YouTube video in a new window*  

There are other notebooks in the repo - these will mature over time.  
These are typically a part of a longer engagement (like a 3 to 5 day data engineering boot camp).  
  
**BTW - Big Thank you [PyData](https://pydata.org/global2023/schedule#2023-12-07:~:text=Shaurya%20Agarwal)! :)**
  
---  
## Video from the PyData Global 2023 session  
  
Click on the image below to watch the full video from the PyData Global 2023 session.  
  
[![Click on the image below to watch the full video from the PyData Global 2023 session](https://img.youtube.com/vi/AHgio9PG5Cw/0.jpg)](https://youtu.be/AHgio9PG5Cw)  
  
<br />  
  
---
### 00 Python Collections  
  
This set of notebooks works through examples of how some pretty sophisticated data engineering can be done using Python Collections, Itertools and Functools. It uses the [small MovieLens dataset](https://grouplens.org/datasets/movielens/#:~:text=Small%3A%20100%2C000%20ratings%20and%203%2C600%20tag%20applications). 
  
* Basic Collections and the ```Collections``` Module: [Notebook](https://github.com/shauryashaurya/learn-data-munging/blob/main/00-Python-Collections/01.01%20Data-Wrangling-with-Plain-Old-Python.ipynb) also [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/00-Python-Collections/01.01%20Data-Wrangling-with-Plain-Old-Python.ipynb)  
  
    
### 01 [Numpy](https://github.com/shauryashaurya/learn-data-munging/tree/main/01-Numpy)    
  
* NumPy vs Python Collections [Notebook](https://github.com/shauryashaurya/learn-data-munging/blob/main/01-Numpy/02.01-Numpy-over-Python-Collections.ipynb) also [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/01-Numpy/02.01-Numpy-over-Python-Collections.ipynb)   
	  
### 02 [Pandas](https://github.com/shauryashaurya/learn-data-munging/tree/main/02-Pandas)    
    
* Wrangling MovieLens with Pandas - Part 1: Getting Started, Load the MovieLens dataset: [Notebook](https://github.com/shauryashaurya/learn-data-munging/blob/main/02-Pandas/02.01-Data-Wrangling-with-MovieLens-and-Pandas.ipynb) also [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/02-Pandas/02.01-Data-Wrangling-with-MovieLens-and-Pandas.ipynb)
* Wrangling MovieLens with Pandas - Part 2: Playing with the Movies and Ratings data: [Notebook](https://github.com/shauryashaurya/learn-data-munging/blob/main/02-Pandas/02.02-Data-Wrangling-with-MovieLens-and-Pandas.ipynb) also [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/02-Pandas/02.02-Data-Wrangling-with-MovieLens-and-Pandas.ipynb)
	  
### 03 [Spark](https://github.com/shauryashaurya/learn-data-munging/tree/main/03-Spark)  
	  
#### 02 - A set of notebooks exploring data wrangling in depth using the MovieLens dataset
* Part 01: Overview, Starting Spark and Loading the data: [Notebook](https://github.com/shauryashaurya/learn-data-munging/blob/main/03-Spark/002.01-Analyze-MovieLens-using-PySpark.ipynb) or [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/03-Spark/002.01-Analyze-MovieLens-using-PySpark.ipynb)
    
* Part 02: Data Analysis basics using tags.csv from the MovieLens dataset: [Notebook](https://github.com/shauryashaurya/learn-data-munging/blob/main/03-Spark/002.02-Analyze-MovieLens-using-PySpark.ipynb) or [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/03-Spark/002.02-Analyze-MovieLens-using-PySpark.ipynb)
    
    
### 04 [Dask](https://github.com/shauryashaurya/learn-data-munging/tree/main/04-Dask)    
    
* Distributed Data Analysis with Dask - Part 1: Getting Started, Load the MovieLens dataset: [Notebook](https://github.com/shauryashaurya/learn-data-munging/blob/main/04-Dask/02.01-MovieLens-and-Dask.ipynb) also [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/04-Dask/02.01-MovieLens-and-Dask.ipynb) 
* Distributed Data Analysis with Dask - Part 2: Playing with the Movies data: [Notebook](https://github.com/shauryashaurya/learn-data-munging/blob/main/04-Dask/02.02-MovieLens-and-Dask.ipynb) also [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/04-Dask/02.02-MovieLens-and-Dask.ipynb) 

    
### 05 [Polars](https://github.com/shauryashaurya/learn-data-munging/tree/main/05-Polars)    
    
* Polars with the MovieLens dataset - Getting Started, Load the MovieLens dataset, A quick look at Arrow, and some analysis: [Notebook](https://github.com/shauryashaurya/learn-data-munging/blob/main/05-Polars/02.01-Polars-on-MovieLens.ipynb) also [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/05-Polars/02.01-Polars-on-MovieLens.ipynb)  
  
  	  
  
## Other notes  
#### Formatting  
I tried multiple variations to embed the PyData session YouTube video here. Seems like there are multiple limitations. After several trial-and-error attempts, I need to park this for a later date. 
<html>
	<body>
		<p font-size="8px">N.B.  </p>
		<p font-size="8px">
			<ul>
				<li>adding <code>&lt;p font-size=&quot;8px&quot;&gt;</code> to any .md page does&#39;t seem to work as of Feb 2024, when the page is viewed in GitHub. The Jekyll page can support custom CSS, but that's not my focus for now, I want to continue adding more data engines to the repo. So will defer this task to some future state (or ask the community for a PR).</li>  
				<li><code>open in new window</code> doesn&#39;t work just yet per <a href="https://stackoverflow.com/questions/41915571/open-link-in-new-tab-with-github-markdown-using-target-blank">this stackoverflow discussion</a>, the <a href="https://kramdown.gettalong.org/">Kramdown</a> solution of adding <code>{:target=&quot;_blank&quot;}</code> didn&#39;t work either. So the best bet is to use <code>ctrl+click</code> (Windows) or <code>cmd+click</code> (Mac) to open links in a new window (tab) and not lose this page.</li>
				<li>Seems like it&#39;s not possible to embed YouTube vidoes in GitHub markdown just yet (they say &#39;security&#39; etc.), <a href="https://stackoverflow.com/questions/11804820/how-can-i-embed-a-youtube-video-on-github-wiki-pages">ref1</a>, <a href="https://github.com/adam-p/markdown-here/wiki/Markdown-Cheatsheet#youtube-videos">ref2</a>, so just embedding a clickable preview instead.</li>
				<li>Editing custom CSS for these pages will have to be a project for another day...</li>
			</ul>
		</p>
	</body>
</html>
  
.