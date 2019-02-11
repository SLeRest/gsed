test1:
	cargo build
	@echo BEFORE
	@echo /Users/ouralgan/program/rust/gsed/test/test1/word_text.txt
	@cat /Users/ouralgan/program/rust/gsed//test/test1/word_text.txt
	@echo /Users/ouralgan/program/rust/gsed/test/test1/f1
	@cat /Users/ouralgan/program/rust/gsed/test/test1/f1
	@echo /Users/ouralgan/program/rust/gsed/test/test1/d1/f11
	@cat /Users/ouralgan/program/rust/gsed/test/test1/d1/f11
	@echo /Users/ouralgan/program/rust/gsed/test/test1/d1/f12
	@cat /Users/ouralgan/program/rust/gsed/test/test1/d1/f12
	@echo /Users/ouralgan/program/rust/gsed/test/test1/d1/d12/f11
	@cat /Users/ouralgan/program/rust/gsed/test/test1/d1/d12/f11
	@echo /Users/ouralgan/program/rust/gsed/test/test1/d1/d12/f12
	@cat /Users/ouralgan/program/rust/gsed/test/test1/d1/d12/f12
	target/debug/gsed -R mdr lol test/test1
	@echo AFTER
	@echo /Users/ouralgan/program/rust/gsed/test/test1/word_text.txt
	@cat /Users/ouralgan/program/rust/gsed/test/test1/word_text.txt
	@echo /Users/ouralgan/program/rust/gsed/test/test1/f1
	@cat /Users/ouralgan/program/rust/gsed/test/test1/f1
	@echo /Users/ouralgan/program/rust/gsed/test/test1/d1/f11
	@cat /Users/ouralgan/program/rust/gsed/test/test1/d1/f11
	@echo /Users/ouralgan/program/rust/gsed/test/test1/d1/f12
	@cat /Users/ouralgan/program/rust/gsed/test/test1/d1/f12
	@echo /Users/ouralgan/program/rust/gsed/test/test1/d1/d12/f11
	@cat /Users/ouralgan/program/rust/gsed/test/test1/d1/d12/f11
	@echo /Users/ouralgan/program/rust/gsed/test/test1/d1/d12/f12
	@cat /Users/ouralgan/program/rust/gsed/test/test1/d1/d12/f12
	@echo mdr > /Users/ouralgan/program/rust/gsed/test/test1/word_text.txt
	@echo mdr > /Users/ouralgan/program/rust/gsed/test/test1/f1
	@echo mdr > /Users/ouralgan/program/rust/gsed/test/test1/d1/f11
	@echo mdr > /Users/ouralgan/program/rust/gsed/test/test1/d1/f12
	@echo mdr > /Users/ouralgan/program/rust/gsed/test/test1/d1/d12/f11
	@echo mdr > /Users/ouralgan/program/rust/gsed/test/test1/d1/d12/f12

print_test1:
	@echo /Users/ouralgan/program/rust/gsed/test/test1/word_text.txt
	@cat /Users/ouralgan/program/rust/gsed//test/test1/word_text.txt
	@echo /Users/ouralgan/program/rust/gsed/test/test1/f1
	@cat /Users/ouralgan/program/rust/gsed/test/test1/f1
	@echo /Users/ouralgan/program/rust/gsed/test/test1/d1/f11
	@cat /Users/ouralgan/program/rust/gsed/test/test1/d1/f11
	@echo /Users/ouralgan/program/rust/gsed/test/test1/d1/f12
	@cat /Users/ouralgan/program/rust/gsed/test/test1/d1/f12
	@echo /Users/ouralgan/program/rust/gsed/test/test1/d1/d12/f11
	@cat /Users/ouralgan/program/rust/gsed/test/test1/d1/d12/f11
	@echo /Users/ouralgan/program/rust/gsed/test/test1/d1/d12/f12
	@cat /Users/ouralgan/program/rust/gsed/test/test1/d1/d12/f12

write_test1:
	@echo mdr > /Users/ouralgan/program/rust/gsed/test/test1/word_text.txt
	@echo mdr > /Users/ouralgan/program/rust/gsed/test/test1/f1
	@echo mdr > /Users/ouralgan/program/rust/gsed/test/test1/d1/f11
	@echo mdr > /Users/ouralgan/program/rust/gsed/test/test1/d1/f12
	@echo mdr > /Users/ouralgan/program/rust/gsed/test/test1/d1/d12/f11
	@echo mdr > /Users/ouralgan/program/rust/gsed/test/test1/d1/d12/f12

