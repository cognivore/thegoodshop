export async function onBeforeRender() {
  console.log('onBeforeRender')
  const res = await fetch('http://127.0.0.1:5526/api/products')
  const products = await res.json()
  console.log('products', products)

  return {
    pageContext: {
      // Instead of "pageProps", we return our data under "data"
      data: { products, title: 'Home' },
    },
  }
}
