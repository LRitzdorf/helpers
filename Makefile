.DEFAULT: all
.PHONY: all clean

CFLAGS = -Wall
LDFLAGS = 

HEADERS = $(wildcard *.h)
SRC = $(wildcard *.c)
OBJ = $(SRC:.c=.o)


all: conserve

conserve: conserve.o
	$(CC) $(LDFLAGS) $< -o $@

%.o: %.c $(HEADERS)
	$(CC) $(CFLAGS) -c $< -o $(<:.c=.o)

clean:
	rm *.o
