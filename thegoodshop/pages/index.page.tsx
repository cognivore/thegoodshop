import React from 'react'
import type { Product } from '../src/types/generated'

export { Page }

function Page({ products }: { products: Product[] }) {
  return (
    <div>
      <h1>The Good Shop</h1>
      <ul>
        {products.map((p) => (
          <li key={p.id}>
            {p.name} - ${p.price.toFixed(2)}
          </li>
        ))}
      </ul>
    </div>
  )
}
