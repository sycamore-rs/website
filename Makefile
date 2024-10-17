serve:
	npx concurrently "trunk watch" "npx http-server dist -c-1"
