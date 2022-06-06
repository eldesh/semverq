
PANDOC := pandoc

MAN    := doc/man/semverq.1

all: man


man: $(MAN)


$(MAN): semverq.md
	@mkdir -p $(dir $@)
	$(PANDOC) $^ -s -t man > $@


.PHONY: clean
clean:
	$(RM) $(MAN)

