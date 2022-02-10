/**
 * Spade default theme entrypoint.
 * Features/responsibilities:
 *
 * - display a graph of connected articles
 *
 * - parse internal links and display tooltips with previews
 *   on hover
 */
import "core-js/stable"
import "regenerator-runtime/runtime"

import graph from "./graph"
import create_tooltip from "./tooltips"
import "./styles.css"

// article_id may be supplied inline within the various article pages
declare global {
  interface Window {
    article_id?: string
  }
}

window.addEventListener("DOMContentLoaded", () => {
  fetch("/assets/graph.json")
    .then((data) => data.json())
    .then((data) => graph(data, window.article_id || ""))

  document.querySelectorAll('#main a[href^="/"]').forEach(create_tooltip)
})
