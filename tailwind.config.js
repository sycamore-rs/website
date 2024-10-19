const colors = require("tailwindcss/colors");
const typography = require("@tailwindcss/typography");

module.exports = {
	content: ["./src/**/*.rs"],
	darkMode: "class", // or 'media' or 'class'
	theme: {
		fontFamily: {
			body: ["Inter", "system-ui", "sans-serif"],
			mono: ["IBM Plex Mono", "Menlo", "monospace"],
			code: ["ui-monospace", "monospace"],
		},
		extend: {
			zIndex: {
				neg: -1,
			},
			typography: (theme) => ({
				DEFAULT: {
					css: {
						maxWidth: "80ch",
						"--tw-prose-pre-bg": theme("colors.codeblock"),
						"--tw-prose-pre-code": theme("colors.codeblock-code"),
						pre: {
							// Keep in sync with PrismJS theme.
							padding: "1em",
							margin: "0.5em 0",
							"line-height": "1.5",
						},
					},
				},
			}),
		},
		colors: {
			transparent: "transparent",
			current: "currentColor",
			black: colors.black,
			red: colors.red,
			gray: colors.gray,
			orange: colors.orange,
			amber: colors.amber,
			yellow: colors.yellow,
			white: colors.white,
			// Keep in sync with PrismJS theme.
			codeblock: "#1d2021",
			"codeblock-code": "#ebdbb2",
		},
	},
	plugins: [typography],
};
