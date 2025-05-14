#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include "matrix.h"
#include "mlp.h"

#define DELTA 0.001

matrix *derv(double (*f)(matrix *), matrix *x)
{
	matrix *grad = matnew(x->row, x->col);
	for (int i=0; i<(grad->row)*(x->col); i++) {
		double tmp = x->data[i];
		x->data[i] = tmp+DELTA;
		double fx1 = f(x);
		x->data[i] = tmp-DELTA;
		double fx2 = f(x);
		grad->data[i] = (fx1-fx2)/(2*DELTA);
		x->data[i] = tmp;
	} return grad;
}

