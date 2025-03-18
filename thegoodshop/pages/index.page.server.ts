export async function onBeforeRender() {
  const res = await fetch('http://127.0.0.1:5526/api/products')
  const products = await res.json()

  return {
    pageContext: {
      pageProps: {
        products,
      },
    },
  }
}
