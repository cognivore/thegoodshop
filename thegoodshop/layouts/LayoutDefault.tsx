import React from 'react';
import '../renderer/layout.css';

export default function LayoutDefault({ children }: { children: React.ReactNode }) {
  const currentYear = new Date().getFullYear();
  return (
    <div className="min-h-screen flex flex-col">
      <header className="py-6 mb-8 bg-base-200 shadow-sm text-center">
        <h1 className="text-4xl font-bold">
          The Good Shop
        </h1>
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
  )
}
