# How to create "Open in Colab" links

## Basics  
  
Colab works really well with GitHub (at least as of now).  

Taking the example of my github account ('shauryashaurya'):  
* **GitHub** links start with:  
	https://github.com/[username]/[repo]/blob/main/[notebook]  
	for e.g., https://github.com/shauryashaurya/learn-data-munging/blob/main/mySampleNotebook.ipynb    
* Corresponding **Colab** links would start with:  
	https://colab.research.google.com/github/[username]/[repo]/blob/main/[notebook]  
	following example from above, we change the domain to Colab:  
	https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/mySampleNotebook.ipynb    
	
  
  
So it's easy to convert the link to a GitHub notebook to a link to that same notebook in Colab. We can then put that link under a "Open in Colab" badge supplied by Google, available at: https://colab.research.google.com/assets/colab-badge.svg - or use our own graphic or icon.  

With this in mind, let's look at the 2 common usecases.  

## README or other file  
  
This is simple. Take a GitHub link, replace the domain with Colab, then use the badge to house the hyperlink.

```[![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/01.001%20-%2010%2B%20minutes%20to%20pandas.ipynb)```  
  
renders as:  
  
[![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/01.001%20-%2010%2B%20minutes%20to%20pandas.ipynb)  
  
  
## From within a notebook  
  
This is fairly straightforward too. Create an HTML or Markdown cell. Embed the Colab URL you created by replacing the GitHub domain within an ```ahref``` tag.  
  
```<a href="https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/01.001%20-%2010%2B%20minutes%20to%20pandas.ipynb" target="_parent"><img src="https://colab.research.google.com/assets/colab-badge.svg" alt="Open In Colab"/></a>```  

renders to:  

<a href="https://colab.research.google.com/github/shauryashaurya/learn-data-munging/blob/main/01.001%20-%2010%2B%20minutes%20to%20pandas.ipynb" target="_parent"><img src="https://colab.research.google.com/assets/colab-badge.svg" alt="Open In Colab"/></a>  
  
  
That's all there is to it.  
  
  