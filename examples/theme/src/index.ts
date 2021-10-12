import graph from './graph';
import './styles.css';

window.addEventListener('DOMContentLoaded', (event) => {
  fetch('/assets/graph.json').then((data) => data.json()).then((data) => graph(data, window.article_id));
});
