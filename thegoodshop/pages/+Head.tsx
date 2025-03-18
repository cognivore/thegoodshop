import { usePageContext } from "vike-react/usePageContext";
import logoUrl from "../assets/logo.svg";
import { PageContext } from "vike/types";

function title(pageContext: PageContext) {
  if (typeof pageContext.data === 'object' && pageContext.data !== null && 'title' in pageContext.data) {
    const { title } = pageContext.data as { title: string }
    return `The Good Shop - ${title}`
  }
  return 'The Good Shop'
}

export default function HeadDefault() {
  const pageContext = usePageContext()
  return (
    <>
      <link rel="icon" href={logoUrl} />
      <title>{title(pageContext)}</title>
    </>
  );
}
