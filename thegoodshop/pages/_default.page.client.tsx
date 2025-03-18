export { render }

import React from 'react'
import ReactDOM from 'react-dom/client'
import { PageLayout } from './PageLayout'

async function render(pageContext: any) {
  const { Page, pageProps } = pageContext
  const page = (
    <PageLayout>
      <Page {...pageProps} />
    </PageLayout>
  )
  const container = document.getElementById('page-view')!
  ReactDOM.createRoot(container).render(page)
}
