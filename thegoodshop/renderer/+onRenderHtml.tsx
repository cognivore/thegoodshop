import ReactDOMServer from 'react-dom/server'
import { PageContextServer } from 'vike/types'

export { onRenderHtml }

function onRenderHtml(pageContext: PageContextServer & { Page: any }) {
  const { Page } = pageContext
  const pageHtml = ReactDOMServer.renderToString(<Page />)
  return {
    documentHtml: `<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <title>The Good Shop</title>
  </head>
  <body>
    <div id="page-view">${pageHtml}</div>
  </body>
</html>`,
  }
}
