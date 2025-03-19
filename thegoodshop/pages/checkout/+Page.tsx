import React, { useState } from 'react';
import { useCart } from '../../src/contexts/CartContext';

export default function CheckoutPage() {
  const { items } = useCart();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  // Compute total amount in pounds (or your local currency)
  const totalAmountPounds = items
    .reduce((sum, item) => sum + item.product.price * item.quantity, 0)
    .toFixed(2);
  // Convert pounds to pence (Stripe expects the smallest currency unit)
  const totalAmountPence = Math.round(parseFloat(totalAmountPounds) * 100);

  const handleCheckout = async () => {
    setLoading(true);
    setError('');
    try {
      const res = await fetch('/api/create-checkout-session', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          products: items.map(item => ({
            id: item.product.id,
            name: item.product.name,
            price: item.product.price,
            quantity: item.quantity
          }))
        }),
      });
      if (!res.ok) {
        throw new Error('Failed to create checkout session');
      }
      const data = await res.json();
      // Redirect to the Checkout Session URL returned by the backend.
      window.location.href = data.url;
    } catch (err) {
      console.error(err);
      setError('Checkout failed. Please try again.');
    }
    setLoading(false);
  };

  return (
    <div className="p-8 flex flex-col items-center">
      <h1 className="text-4xl font-bold mb-6">Checkout</h1>

      {/* Order Summary Card */}
      <div className="w-full max-w-4xl">
        <div className="card bg-white shadow-xl rounded-lg p-6">
          <h2 className="card-title text-2xl font-semibold mb-4">Order Summary</h2>
          <div className="space-y-4">
            {items.map(item => (
              <div
                key={item.product.id}
                className="flex items-center gap-4 border p-4 rounded-md"
              >
                {/* Placeholder for Product Image */}
                <div className="w-16 h-16 bg-gray-300 rounded flex items-center justify-center">
                  <span className="text-xs text-gray-600">Image</span>
                </div>
                {/* Product details */}
                <div className="flex-1">
                  <h3 className="text-lg font-medium">{item.product.name}</h3>
                  <p className="text-sm text-gray-500">Quantity: {item.quantity}</p>
                </div>
                {/* Price for this item */}
                <div className="text-xl font-bold">
                  £{(item.product.price * item.quantity).toFixed(2)}
                </div>
              </div>
            ))}
          </div>
          <div className="mt-6 text-right">
            <p className="text-2xl font-bold">Total: £{totalAmountPounds}</p>
          </div>
        </div>
      </div>

      {error && <p className="text-red-500 mt-4">{error}</p>}
      <button
        className="btn btn-primary mt-8"
        onClick={handleCheckout}
        disabled={loading || items.length === 0}
      >
        {loading ? 'Processing...' : 'Proceed to Payment'}
      </button>
    </div>
  );
}
