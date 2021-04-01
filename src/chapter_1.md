# Chapter 1

A simple demonstration of latex and gnuplot integration into `mdbook`.

The current mdbook-science

These lines related to hyperplane is not working

Other stuff works:

$$equation, energy
E = m C^2
$$

$$equation, svm
\begin{aligned}
0 \leq \lambda_i^{\ast} \leq c &\implies y_i(\mathbf{a}^{\ast^T}\mathbf{x}_i+b^{\ast}) = 1 \quad \text{e.g. support vector} \\
\lambda_i^{\ast} = c &\implies y_i(\mathbf{a}^{\ast^T}\mathbf{x}_i+b^{\ast}) \leq 1 \quad \text{e.g. outlier vector} \\
\lambda_i^{\ast} = 0 &\implies y_i(\mathbf{a}^{\ast^T}\mathbf{x}_i+b^{\ast}) \geq 1
\end{aligned}
$$

$$gnuplot, contour, Contour test plot
set title "contours on both base and surface"
set contour both
set hidden3d
set ztics  -100,40,100
splot x**2-y**2 with lines title "$x^2 - y^2$", x**2-y**2 with labels boxed notitle
$$

We should reference this $ref:bib:legendreintegral$ and inline math $\sum_i \frac{a}{b}$ should also work and this $ref:equ:svm$.
