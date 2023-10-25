
all: libsome.a

some.o: some.c
	$(CC) -c some.c -o some.o

libsome.a: some.o
	$(AR) rcs libsome.a some.o
