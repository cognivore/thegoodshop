import type { Product } from '../../src/types/generated'
import { usePageContext } from 'vike-react/usePageContext'

export default function Page() {
  // Type the pageContext to indicate that data contains products
  const pageContext = usePageContext()
  const { products } = pageContext.data as { products: Product[] }

  console.log('pageContext.data', pageContext.data)

  return (
    <div className="p-8">
      <h1 className="text-3xl font-bold mb-4">The Good Shop</h1>
      <ul className="list-disc pl-5 space-y-2">
        {products.map((p) => (
          <li key={p.id} className="p-2 bg-base-200 rounded">
            <span className="font-semibold">{p.name}</span>
            {' — '}
            <span className="text-secondary">${p.price.toFixed(2)}</span>
          </li>
        ))}
      </ul>
    </div>
  )
}
