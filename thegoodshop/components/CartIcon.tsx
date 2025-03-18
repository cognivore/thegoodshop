import React, { useState } from 'react';
import { useCart } from '../src/contexts/CartContext';

export function CartIcon() {
  const { items } = useCart();
  const [showPopup, setShowPopup] = useState(false);

  const totalItems = items.reduce((sum, item) => sum + item.quantity, 0);

  return (
    <div className="relative"
      onMouseEnter={() => setShowPopup(true)}
      onMouseLeave={() => setShowPopup(false)}>
      <div className="flex items-center gap-2">
        <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z" />
        </svg>
        {totalItems > 0 && (
          <span className="badge badge-sm badge-primary">{totalItems}</span>
        )}
      </div>

      {showPopup && items.length > 0 && (
        <div className="absolute right-0 mt-2 w-64 bg-base-100 shadow-xl rounded-box p-4 z-50">
          {items.map(item => (
            <div key={item.product.id} className="flex justify-between items-center mb-2">
              <span className="truncate">{item.product.name}</span>
              <span className="text-sm">x{item.quantity}</span>
            </div>
          ))}
          <div className="border-t pt-2 mt-2">
            <span className="font-bold">Total: </span>
            ${items.reduce((sum, item) => sum + (item.product.price * item.quantity), 0).toFixed(2)}
          </div>
        </div>
      )}
    </div>
  );
}
