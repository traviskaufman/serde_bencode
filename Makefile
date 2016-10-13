.PHONY: test-unit
test-unit:
	cd bencode && cargo test

.PHONY: test-integration
test-integration:
	cd bencode_tests && cargo test

.PHONY: test
test: test-unit test-integration
