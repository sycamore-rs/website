serve:
	mkdir -p target_ssr && npx concurrently "CARGO_TERM_COLOR=always trunk serve" "npx serve dist -p 8080"
