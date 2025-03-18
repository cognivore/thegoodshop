import React from 'react'

export function PageLayout({ children }: { children: React.ReactNode }) {
  return (
    <div>
      <header>My Site Header</header>
      <main>{children}</main>
      <footer>My Site Footer</footer>
    </div>
  )
}
