.DEFAULT: all
.PHONY: all permissions notify stmux clean

CFLAGS = -Wall
LDFLAGS = 

HEADERS = $(wildcard *.h)
SRC = $(wildcard *.c)
OBJ = $(SRC:.c=.o)


all: conserve notify stmux

permissions: conserve
	strip $<
	chown root:root $<
	chmod u+s $<

conserve: conserve.o
	$(CC) $(LDFLAGS) $< -o $@

notify:
	@echo "\"notify\" is a shell script, and does not require compilation."

stmux:
	@echo "\"stmux\" is a shell script, and does not require compilation."

%.o: %.c $(HEADERS)
	$(CC) $(CFLAGS) -c $< -o $(<:.c=.o)

clean:
	rm *.o
