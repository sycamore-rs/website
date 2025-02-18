module.exports = {
	content: ["./src/**/*.rs"],
	darkMode: "class", // or 'media' or 'class'
	theme: {
		extend: {
			zIndex: {
				neg: -1,
			},
			typography: (_theme) => ({
				DEFAULT: {
					css: {
						maxWidth: "80ch",
						"--tw-prose-pre-bg": "var(--color-codeblock)",
						"--tw-prose-pre-code": "var(--color-codeblock-code)",
						pre: {
							// Keep in sync with PrismJS theme.
							padding: "1em",
							margin: "0.5em 0",
							lineHeight: "1.5 !important",
						},
					},
				},
			}),
		}
	}
};
