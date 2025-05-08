all: mlp

matrix.o: matrix.c matrix.h
	clang -c matrix.c

main.o: main.c matrix.h mlp.h
	clang -c main.c

mlp.o: mlp.c matrix.h
	clang -c mlp.c

mlp: mlp.o main.o matrix.o
	clang main.o matrix.o mlp.o -o mlp

.PHONY: clean

clean:
	rm *.o
	rm mlp
	echo clean done


