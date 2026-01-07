#include <stdio.h>
#include "matrix.h"

static void matprint(matrix *m, const char *name)
{

    printf("%s (%ux%u):\n", name, m->row, m->col);
    for (size_t r = 0; r < m->row; ++r) {
        for (size_t c = 0; c < m->col; ++c)
            printf("%8.2f ", matget(m, r, c));
        putchar('\n');
    } putchar('\n');
	return;
}

int main()
{

	return 0;
}

