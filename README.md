# Metarustistic: a meta heuristic optimization toy project

## Architecture

An optimization problem is formalized by:
- A function to optimize, giving the score (single objective) for the candidate point. We know the boundaries of the function, but not if it has a single minima / if it is convex
## Functions:

- Rastrigin
    - Boundaries [-5.12,5.12] for each dimension. Multiple local minimums but  single global minimum at $\vec{x}=\vec{0}$

## Optimizers:

- RandomWalk
    - Initial candidate point, move by a random increment $\delta x$ on each dimension. 
    - If $x+\delta x \notin F$, increment by $x-\delta x$ instead to stay within the boundaries of the function. 
    - Save the point with the best score.
- GA
- PSO

## Visualization:

- CL
- ASCII
- plotter
- rerun