// renderer/+onRenderClient.tsx
import React from 'react'
import ReactDOM from 'react-dom/client'
import type { PageContextClient } from 'vike/types'

export { onRenderClient }

function onRenderClient(pageContext: PageContextClient & { Page: any }) {
  const { Page } = pageContext
  const container = document.getElementById('page-view')
  if (!container) throw new Error('#page-view not found')
  ReactDOM.createRoot(container).render(<Page />)
}
