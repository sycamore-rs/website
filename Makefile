serve:
	npx concurrently "CARGO_TERM_COLOR=always trunk watch" "npx http-server dist -c-1"
