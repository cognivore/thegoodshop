import React from 'react';
import '../renderer/layout.css';
import { CartProvider } from '../src/contexts/CartContext';
import { CartIcon } from '../components/CartIcon';

export default function LayoutDefault({ children }: { children: React.ReactNode }) {
  const currentYear = new Date().getFullYear();
  return (
    <CartProvider>
      <div className="min-h-screen flex flex-col">
        <header className="sticky top-0 z-50 py-6 bg-base-200 shadow-sm">
          <div className="max-w-7xl mx-auto px-4 relative">
            <div className="flex justify-between items-center">
              <div className="w-24" /> {/* Spacer to balance the cart icon */}
              <h1 className="text-2xl md:text-4xl font-bold absolute left-1/2 -translate-x-1/2">
                The Good Shop
              </h1>
              <CartIcon />
            </div>
          </div>
        </header>
        <main className="flex-grow">{children}</main>
        <footer className="py-3 mt-12 bg-base-200/50 text-center text-sm text-base-content/70">
          {currentYear > 2025 ? (
            <p>CC-BY-SA 2025-{currentYear} youmna and friends</p>
          ) : (
            <p>CC-BY-SA {currentYear} youmna and friends</p>
          )}
        </footer>
      </div>
    </CartProvider>
  )
}
