diesel-build:
	diesel migration generate vf

diesel-run:
	diesel migration run

test-db:
	cargo test test_connection

test-agent:
	cargo test test_insert_agent -- --nocapture