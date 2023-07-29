import type {Config} from 'tailwindcss'

export default {
    content: ['./src/**/*.{html,rs}'],
    theme: {},
    plugins: [require("rippleui")],
} satisfies Config;
