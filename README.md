# phubble

Do you have a persistent chain complex that is not filtered (i.e. the chain maps are not inclusions)?
Would you like to convert this into a filtered chain complex so that you can pass it along to a persistent homology library, such as [lophat](https://github.com/tomchaplin/lophat)?
Look no further!
`phubble` realises your wildest persistent homology dreams by constructing the [mapping telescope](https://ncatlab.org/nlab/show/mapping+telescope).

The only function of interest is `build_telescope` which is exported as a Python binding.
Documentation is forthcoming (hopefully) but get in touch if you need advice on how to use.

Install via
```shell
pip install phubble
```
