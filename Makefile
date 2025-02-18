serve:
	mkdir -p target_ssr && npx concurrently "CARGO_TERM_COLOR=always TRUNK_AUTO_RELOAD=true trunk serve" "npx serve dist -p 8080"
