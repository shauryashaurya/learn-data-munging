# A Note on Numbered Headings  

*UPDATE* - none of these seem to be working.    
  
Oh well! I've got to let the cosmetic be and focus on the actual work.  
Will debug this when I get the time, or maybe GitHub will enable this feature.
All in all, this was an interesting detour.  
Can always Toggle auto-numbering in Jupyter Lab.   
  
## Numbered headings in notebooks

Added a cell at the top of the notebook with the following code:
```
from IPython.display import HTML
HTML(''.format(open('./numbered_headings.css').read()))
```
  
Tried the following solutions:
* [From Kaggle](https://www.kaggle.com/discussions/getting-started/129348)
* [This CSS Gist](https://gist.github.com/patik/89ee6092c72a9e39950445c01598517a)
  
  
## numbered headings on GitHub Readme file  
As of August 2022, Github README files do not have numbered headings.  
So we needed to add a custom CSS.  
But Github does not allow that either (for good reason).  
So, tried embeding CSS in a SVG and embed that SVG in README.md by adding the following code at the top:
```
<div>
	<a href="https://github.com/shauryashaurya/learn-data-munging/blob/main/numbered_headings.svg">
		<img src="numbered_headings.svg" width="1" height="1" alt="">
	</a>
</div>
```

Links I tried:  
* [This Stackoverflow Answer](https://stackoverflow.com/a/66981634)  
* [This CSS Gist](https://gist.github.com/patik/89ee6092c72a9e39950445c01598517a)
* [This reference implementation on GitHub](https://github.com/sindresorhus/css-in-readme-like-wat)  
    
    