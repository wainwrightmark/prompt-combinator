## Prompt Combinator

This is a tool to create parameterized prompts for AI image generators.  
[Try it here](https://wainwrightmark.github.io/prompt-combinator/)  

It has the following key features
- *Disjunction* of prompts e.g. `a {cat|dog}` will generate `a cat` and `a dog`
- *Ranges* of values e.g.  `a (blue:{0;1;0.1}) dog` will generate `a (blue:0) dog`, `a (blue:0.1) dog` etc. This is obviously most useful if you are on a platform that supports prompt multipliers
- *Variables* e.g. `a {animal:cat|dog} with another {animal}` will generate `a cat with another cat` and `a dog with another dog`
- *Hidden Variables* Use an exclamation mark `{animal:dog|cat}! a {animal}`
- *Ordering* If you have multiple disjunctions or ranges you can prefix with a number to control the order they are iterated. Try Comparing `{1:a|b} {2:c|d}` with `{2:a|b} {1:c|d}`
- *Math Expressions* Coming soon!  

Please have fun, share, and post feature requests.

mark