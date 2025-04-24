export TRANSLATABLE_LOCALES_PATH=${PWD}/translatable/tests/fixtures/translations

test:
	cargo test -p translatable -- --nocapture --color=always --test-threads=1
