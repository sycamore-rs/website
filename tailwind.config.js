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
						maxWidth: "75ch",
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
		},
	},
	plugins: [typography],
};
