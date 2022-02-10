/**
 * tooltips.ts
 *
 * create_tooltip fetches the contents of the supplied <a> element,
 * stashes the resulting article in a div and appends it to the inner
 * contents of the <a> tag. Our CSS in `styles.css` is then responsible
 * for actually displaying it on hover.
 */
const create_tooltip = (element: Element) =>
  element.addEventListener(
    "mouseenter",
    async (e: Event) => {
      if (e instanceof MouseEvent && e.target instanceof HTMLAnchorElement) {
        const html = await fetch(e?.target?.href).then((result) => result.text())
        const doc = new DOMParser().parseFromString(html, "text/html")
        const article = doc.querySelector("#main .article")

        if (article) {
          const tooltip = document.createElement("div").appendChild(article)
          tooltip.className = "tooltip"
          e?.target?.appendChild(tooltip)
        }
      }
    },
    {
      once: true,
    }
  )

export default create_tooltip
