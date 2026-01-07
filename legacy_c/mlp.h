#ifndef _MLP_H_
#define _MLP_H_

typedef struct {
	int nlayers;
	int *nnodes;

	matrix *(*activate)(int, matrix *);
} nn;

matrix *derv(double (*f)(matrix *), matrix *x);




#endif
