import 'core-js/stable';
import 'regenerator-runtime/runtime';

import graph from './graph';
import './styles.css';

window.addEventListener('DOMContentLoaded', () => {
  fetch('/assets/graph.json')
    .then((data) => data.json())
    .then((data) => graph(data, window.article_id));

  document.querySelectorAll('#main a[href^="/"]').forEach((element) => element.addEventListener(
    'mouseenter',
    async (e) => {
      const html = await fetch(e?.target?.href).then((result) => result.text());
      const doc = new DOMParser().parseFromString(html, 'text/html');
      const article = doc.querySelector('#main .article');
      if (article) {
        const tooltip = document.createElement('div').appendChild(article);
        tooltip.className = 'tooltip';
        e?.target?.appendChild(tooltip);
      }
    },
    {
      once: true,
    },
  ));
});
