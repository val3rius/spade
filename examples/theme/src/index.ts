import cytoscape from 'cytoscape';
import './styles.css';

var cy = cytoscape({ 
  elements: window.elements, 
  container: document.getElementById('cy'),
  layout: {
    name: 'cose',
    animate: true,
    animationThreshold: 500,
    stop: function() {
      var current_node = cy.$('node[id="'+window.article_id+'"]');
      cy.center(current_node);
      current_node.neighborhood().nodes().map(function(ele){
        ele.neighborhood().nodes().map(function(ele) {
          ele.style({
            'color': '#3d3d3d',
            'background-color': '#bbb',
        });
        });
        ele.style({
            'color': '#3d3d3d',
            'background-color': '#999',
        });
      });
      current_node.style({
        'background-color': '#3d3d3d'
      });
      
    },
    fit: true,
    idealEdgeLength: 50,
    nodeOverlap: 6,
    componentSpacing: 100,
    nodeRepulsion: 500000,
    edgeElasticity: 120,
    nestingFactor: 5,
    gravity: 80,
    numIter: 1000,
  },
  zoom: 1.2,
  zoomingEnabled: false,
  style: [
    {
      selector: 'edge',
      style: {
        'line-color': '#ddd',
        'width': 1,
        'curve-style': 'bezier'
      }
    },
    {
      selector: 'edge[source="'+window.article_id+'"], edge[target="'+window.article_id+'"]',
      style: {
        'line-color': '#3d3d3d',
        'width': 1,
        'curve-style': 'bezier'
      }
    },
    {
      selector: 'node',
      style: {
        'color': '#ccc',
        'text-background-color': '#fff',
        'background-color': '#ccc',
        'label': 'data(id)',
        'width': '15px',
        'height': '15px'
      }
    },
    {
      selector: 'node[id="'+window.article_id+'"]',
      style: {
        'color': '#3d3d3d',
        'background-color': '#3d3d3d',
      }
    },
  ]
});
cy.on('tap', 'node', function(evt){
  var node = evt.target;
  window.location.href = node.data("url");
});
cy.on('mouseover', 'node', (event) => {
  if(event.cy.container()) {
    event.cy.container().style.cursor = 'pointer';
  }
});
cy.on('mouseout', 'node', (event) => {
  if(event.cy.container()) {
    event.cy.container().style.cursor = 'default';
  }
});