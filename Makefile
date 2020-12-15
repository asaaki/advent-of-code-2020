MODE   ?= --release
RUN     = cargo run
RUN_FOR = $(RUN) $(MODE) --
QUIET   = 2>/dev/null
piece   = $(word $2,$(subst ., ,$1))
TASKS   = $(shell ls src/days/day*.rs | sed -e 's/[^0-9]//g' | sort -h | xargs -i echo {}.1 {}.2)

# neat trick if you want all recipes invoke a single shell for the whole body!
# https://www.gnu.org/software/make/manual/html_node/One-Shell.html
# WARNING! This cannot be done for individual recipes. :-(
# .ONESHELL:
# .SHELLFLAGS: -ec

default: fast

fast:
	@$(MAKE) build-and-tasks -j16 --output-sync=recurse --no-print-directory

build-and-tasks: | build $(TASKS)

build:
	cargo build $(MODE)

$(TASKS): %:
	@echo @@ Run day $(call piece,$@,1) / part $(call piece,$@,2)
	@echo
	@echo -- Test case
	@echo $(RUN_FOR) $(call piece,$@,1) $(call piece,$@,2) test $(QUIET)
	@$(RUN_FOR)      $(call piece,$@,1) $(call piece,$@,2) test $(QUIET)
	@echo
	@echo -- Challenge
	@echo $(RUN_FOR) $(call piece,$@,1) $(call piece,$@,2)      $(QUIET)
	@$(RUN_FOR)      $(call piece,$@,1) $(call piece,$@,2)      $(QUIET)
	@echo

day:
	@(echo -n "Day? ";\
		read DAY;\
		echo "day is $${DAY}";\
		cp src/days/template.rs src/days/day$${DAY}.rs;\
		touch inputs/day_$${DAY}_{1,2}{,_test}.txt)
