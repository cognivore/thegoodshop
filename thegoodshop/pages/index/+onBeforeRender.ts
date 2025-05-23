export async function onBeforeRender() {
  console.log('onBeforeRender')

  let products = []

  try {
    // Try to fetch products, but don't fail if the server isn't running
    const res = await fetch('http://127.0.0.1:5526/api/products', {
      // Short timeout to avoid long wait during build
      signal: AbortSignal.timeout(1000)
    })
    products = await res.json()
    console.log('products', products)
  } catch (error) {
    console.warn('Failed to fetch products, using placeholder data instead')
    // Use placeholder data for build or if API is unavailable
    products = [
      { id: 1, name: "Emergency Food Package", price: 25.0, created_at: 1685454400 },
      { id: 2, name: "Medical Supplies Kit", price: 50.0, created_at: 1685454400 }
    ]
  }

  return {
    pageContext: {
      // Instead of "pageProps", we return our data under "data"
      data: { products, title: 'Home' },
    },
  }
}
