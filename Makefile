RUN     = cargo run
RUN_FOR = $(RUN) --
QUIET   = 2>/dev/null
piece   = $(word $2,$(subst ., ,$1))
TASKS   = 1.1 1.2 \
					2.1 2.2 \
					3.1 3.2 \
					4.1 4.2 \
					5.1 5.2 \
					6.1 6.2 \
					7.1 7.2 \
					8.1 8.2 \
					9.1 9.2

default: $(TASKS)

$(TASKS): %:
	@echo @@ Run day $(call piece,$@,1) / step $(call piece,$@,2)
	@echo -- Test
	@echo $(RUN_FOR) $(call piece,$@,1) $(call piece,$@,2) test $(QUIET)
	@$(RUN_FOR) $(call piece,$@,1) $(call piece,$@,2) test $(QUIET)
	@echo
	@echo -- Challenge
	@echo $(RUN_FOR) $(call piece,$@,1) $(call piece,$@,2)      $(QUIET)
	@$(RUN_FOR) $(call piece,$@,1) $(call piece,$@,2)      $(QUIET)
	@echo

create-inputs:
	@touch inputs/day_$(DAY)_{1,2}{,_test}.txt
