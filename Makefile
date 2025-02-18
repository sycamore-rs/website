serve:
	mkdir -p target_ssr && npx concurrently "CARGO_TERM_COLOR=always trunk watch" "npx serve dist -p 8080"
