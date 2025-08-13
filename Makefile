.DEFAULT: all
.PHONY: all permissions notify nvim-hosted stmux clean


all: conserve notify nvim-hosted stmux

permissions: conserve
	strip $<
	chown root:root $<
	chmod u+s $<

conserve: conserve-rs/src/*
	cd $@-rs && cargo build --release
	cp $@-rs/target/release/$@ ./

notify:
	@echo "\"notify\" is a shell script, and does not require compilation."

stmux:
	@echo "\"stmux\" is a shell script, and does not require compilation."

nvim-hosted:
	@echo "\"nvim-hosted\" is a shell script, and does not require compilation."

clean:
	cd conserve-rs && cargo clean
	rm -f conserve
