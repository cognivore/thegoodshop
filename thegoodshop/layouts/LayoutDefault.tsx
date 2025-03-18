import React from 'react'

export default function LayoutDefault({ children }: { children: React.ReactNode }) {
  return (
    <div>
      <header className="p-4 bg-gray-200">
        <h1>Header</h1>
      </header>
      <main>{children}</main>
      <footer className="p-4 bg-gray-200 text-center">
        © {new Date().getFullYear()} The Good Shop
      </footer>
    </div>
  )
}
