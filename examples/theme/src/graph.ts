/**
 * graph.ts
 *
 * sets up the network graph of connected articles.
 * currently uses cytoscape.js for the actual graph,
 * which is huuuuge. we definitely want to move to
 * something smaller at some point, since we utilize
 * like half a percent of its features.
 */
import cytoscape from "cytoscape"

const graph = (elements: cytoscape.ElementsDefinition, currentID: string) => {
  const cy = cytoscape({
    elements,
    container: document.getElementById("cy"),
    layout: {
      name: "cose",
      animate: false,
      stop() {},
      fit: true,
      idealEdgeLength: () => 50,
      nodeOverlap: 6,
      componentSpacing: 100,
      nodeRepulsion: () => 500000,
      edgeElasticity: () => 120,
      nestingFactor: 5,
      gravity: 80,
      numIter: 1000,
    },
    zoom: 1.2,
    zoomingEnabled: false,
    style: [
      {
        selector: "edge",
        style: {
          "line-color": "#ddd",
          width: 1,
          "curve-style": "bezier",
        },
      },
      {
        selector: `edge[source="${currentID}"], edge[target="${currentID}"]`,
        style: {
          "line-color": "#3d3d3d",
          width: 1,
          "curve-style": "bezier",
        },
      },
      {
        selector: "node",
        style: {
          color: "#ccc",
          "text-background-color": "#fff",
          "background-color": "#ccc",
          label: "data(id)",
          width: "15px",
          height: "15px",
        },
      },
      {
        selector: `node[id="${currentID}"]`,
        style: {
          color: "#3d3d3d",
          "background-color": "#3d3d3d",
        },
      },
    ],
  })
  const currentNode = cy.$(`node[id="${currentID}"]`)
  cy.center(currentNode)
  currentNode
    .neighborhood()
    .nodes()
    .forEach((ele) => {
      ele
        .neighborhood()
        .nodes()
        .forEach((nEle) => {
          nEle.style({
            color: "#3d3d3d",
            "background-color": "#bbb",
          })
        })
      ele.style({
        color: "#3d3d3d",
        "background-color": "#999",
      })
    })
  currentNode.style({
    "background-color": "#3d3d3d",
  })
  cy.on("tap", "node", (evt) => {
    const node = evt.target
    window.location.href = node.data("url")
  })
  cy.on("mouseover", "node", (event) => {
    const container = event.cy.container()
    if (container) {
      container.style.cursor = "pointer"
    }
  })
  cy.on("mouseout", "node", (event) => {
    const container = event.cy.container()
    if (container) {
      container.style.cursor = "default"
    }
  })
}

export default graph
