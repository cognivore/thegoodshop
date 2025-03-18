pnpm create vite@latest thegoodshop --template react

cd thegoodshop || exit 2

pnpm install

pnpm add vite-plugin-ssr@latest react-dom@latest
