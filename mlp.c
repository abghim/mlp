#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include "matrix.h"
#include "mlp.h"

nn *nnalloc(int nlayers, int *nnodes, double (*activate)(int, double *))
{
	nn *temp = (nn *) malloc(sizeof(nn));
	temp->nlayers = nlayers;
	temp->nnodes = nnodes;
	return temp;
}

/* ----- example case ----------------------------*
 | int mlp1_nnodes[] = {4, 10, 8, 3};             |
 | nn *mlp1 = nnalloc(3, mlp1_nnodes, mlp_ReLU);  |
 * -----------------------------------------------*/




