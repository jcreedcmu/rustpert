Rustpert
========

I want to try some brute-force search for
[Rupert-property](https://en.wikipedia.org/wiki/Prince_Rupert%27s_cube)
satisfaction for (a slight variant of) the snub cube, whose Rupertness
is, at time of writing, an open problem.

My working definition of the Rupert property is: a subset $X$ of $\mathbb{R}^3$ is
Rupert if there exist two transformations $t_{inn}, t_{out} \in SE(3)$ (that is, rotations
and translations are allowed) such that the closure of the 'inner shadow' $\pi t_{inn} X$
is a subset of the interior of the 'outer shadow' $\pi t_{out} X$, where $\pi$ is the
evident projection $\mathbb{R}^3 \to \mathbb{R}^2$. See dwrensha's [Lean development](https://github.com/dwrensha/Rupert.lean) for more details.

The file [data/rational-snub.json](data/rational-snub.json) is a rational approximation to the snub cube, given to me by tom7, [whose code is here](https://sourceforge.net/p/tom7misc/svn/HEAD/tree/trunk/ruperts/). Empirically, based on his experiments, it seems not to have the Rupert property.
