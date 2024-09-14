# GitHub Markdown ideosyncracies and fun/unfun applesauce  

<html> 
	<img src="./../images/GitHubMarkdownIdiosyncracies.png" width="95%" align="center" alt="GitHub Markdown ideosyncracies and fun/unfun applesauce..." />  
	<p font-size="8px">Distributed Data wrangling with Dask</p> 
</html>   


* Reference to [GitHub Markdown syntax highlighting](https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/creating-and-highlighting-code-blocks#syntax-highlighting)  
* but, you cannot add spaces after ```````shell``````` in code below or this breaks
````
```shell    
pip upgrade mypackage
```
````  
  
```shell    
pip upgrade mypackage
```  
  
* Don't add spaces, and it seems to work better.
````
```shell
pip upgrade mypackage
```
````
```shell
pip upgrade mypackage
```  

* you need to add two spaces at the end of a sentance to insert a line break, but here it's done automatically, because we are doing code highlighting but also being literal about everything being written in the 3backtick-bracekts.

* I feel dumb and confused...it really shouldn't take me 5 commit-push-refresh page cycles to get the formatting right. I am going to allow that this is mostly my shortcomings, not going to allow that we can't do better.