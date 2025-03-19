import type { Product } from '../../src/types/generated'
import { usePageContext } from 'vike-react/usePageContext'
import { useCart } from '../../src/contexts/CartContext'

export default function Page() {
  const pageContext = usePageContext()
  const { products } = pageContext.data as { products: Product[] }
  const { addToCart } = useCart()

  return (
    <div className="p-4">
      <div className="max-w-7xl mx-auto">
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-4 gap-4">

          {products.map((p) => (
            <div key={p.id} className="card bg-base-200 shadow hover:shadow-lg transition-all duration-200">
              <div className="card-body p-2">
                <h2 className="card-title truncate mt-4 mb-2 ml-4 mr-4">{p.name}</h2>
                <div className="aspect-[3/4] bg-primary-content rounded-lg mb-2 ml-4 mr-4" />
                <p className="text-base text-lg">£{p.price.toFixed(2)}</p>
                <div className="card-actions justify-end">
                  <button
                    className="btn btn-primary m-2"
                    onClick={() => addToCart(p)}
                  >
                    Add to Cart
                  </button>
                </div>
              </div>
            </div>
          ))}

          {products.map((p) => (
            <div key={p.id} className="card bg-base-200 shadow hover:shadow-lg transition-all duration-200">
              <div className="card-body p-2">
                <h2 className="card-title truncate mt-4 mb-2 ml-4 mr-4">{p.name}</h2>
                <div className="aspect-[3/4] bg-primary-content rounded-lg mb-2 ml-4 mr-4" />
                <p className="text-base text-lg">£{p.price.toFixed(2)}</p>
                <div className="card-actions justify-end">
                  <button
                    className="btn btn-primary m-2"
                    onClick={() => addToCart(p)}
                  >
                    Add to Cart
                  </button>
                </div>
              </div>
            </div>
          ))}

          {products.map((p) => (
            <div key={p.id} className="card bg-base-200 shadow hover:shadow-lg transition-all duration-200">
              <div className="card-body p-2">
                <h2 className="card-title truncate mt-4 mb-2 ml-4 mr-4">{p.name}</h2>
                <div className="aspect-[3/4] bg-primary-content rounded-lg mb-2 ml-4 mr-4" />
                <p className="text-base text-lg">£{p.price.toFixed(2)}</p>
                <div className="card-actions justify-end">
                  <button
                    className="btn btn-primary m-2"
                    onClick={() => addToCart(p)}
                  >
                    Add to Cart
                  </button>
                </div>
              </div>
            </div>
          ))}

          {products.map((p) => (
            <div key={p.id} className="card bg-base-200 shadow hover:shadow-lg transition-all duration-200">
              <div className="card-body p-2">
                <h2 className="card-title truncate mt-4 mb-2 ml-4 mr-4">{p.name}</h2>
                <div className="aspect-[3/4] bg-primary-content rounded-lg mb-2 ml-4 mr-4" />
                <p className="text-base text-lg">£{p.price.toFixed(2)}</p>
                <div className="card-actions justify-end">
                  <button
                    className="btn btn-primary m-2"
                    onClick={() => addToCart(p)}
                  >
                    Add to Cart
                  </button>
                </div>
              </div>
            </div>
          ))}

          {products.map((p) => (
            <div key={p.id} className="card bg-base-200 shadow hover:shadow-lg transition-all duration-200">
              <div className="card-body p-2">
                <h2 className="card-title truncate mt-4 mb-2 ml-4 mr-4">{p.name}</h2>
                <div className="aspect-[3/4] bg-primary-content rounded-lg mb-2 ml-4 mr-4" />
                <p className="text-base text-lg">£{p.price.toFixed(2)}</p>
                <div className="card-actions justify-end">
                  <button
                    className="btn btn-primary m-2"
                    onClick={() => addToCart(p)}
                  >
                    Add to Cart
                  </button>
                </div>
              </div>
            </div>
          ))}

        </div>
      </div>
    </div>
  )
}
