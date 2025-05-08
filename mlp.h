#ifndef _MLP_H_
#define _MLP_H_

typedef struct {
	int nlayers;
	int *nnodes;

	double *(*activate)(int, double *);
} nn;

nn *nnalloc(int nlayers, int *nnodes, double (*activate)(int, double *));



#endif
