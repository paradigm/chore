# TODO: comment, copyright
# https://github.com/actions-rs
# https://www.reddit.com/r/rust/comments/ga80lj/example_github_actions_with_rust_build_test/

check: check-lint check-format check-test

check-lint:
	cargo clippy --all-targets --all-features -- -D warnings

check-format:
	rustfmt --check $$(find src -name "*.rs")

check-test:
	cargo clean
	# maybe put this in cargo test profile?
	CARGO_INCREMENTAL=0 RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" RUSTDOCFLAGS="-Cpanic=abort" cargo test
	grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/ --excl-line ".*match [a-z.*]* \\{$$"
	cd target/debug/coverage && \
		awk -F'[<>]' -vq="'" \
		'/<span class="counterNoCov">/\
		{\
			sub(/\.html$$/, "", FILENAME);\
			printf "Missing test coverage: %s:%04d ", FILENAME, $$7;\
			sub(/^([^>]*>){8}/, "");\
			sub(/(<[^<]*){2}$$/, "");\
			gsub(/&lt;/, "<");\
			gsub(/&gt;/, ">");\
			gsub(/&amp;/, "\\&");\
			gsub(/&#39;/, q);\
			gsub(/&quot;/, "\"");\
			print;\
			rv=1\
		}\
		END {exit(rv)}' \
		$$(find . -name "*.rs.html")
	firefox $$(pwd)/target/debug/coverage/index.html
