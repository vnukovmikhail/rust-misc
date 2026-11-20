.PHONY: FORCE

Makefile makefile GNUmakefile: ;

%: FORCE
	cargo run -p $@
