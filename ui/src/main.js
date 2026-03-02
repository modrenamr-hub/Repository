import { renderSettings } from './components/settings.js';
import { renderNotification } from './components/notification.js';

const root = document.querySelector('#app');
root.innerHTML = `
  <main class="layout">
    ${renderNotification({ from: 'ghbdk', to: 'مرحبا', confidence: 84 })}
    ${renderSettings()}
  </main>
`;
