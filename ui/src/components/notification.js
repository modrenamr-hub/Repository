export function renderNotification({ from, to, confidence }) {
  return `
    <section class="notification glass">
      <h2>اقتراح التصحيح</h2>
      <p><strong>${from}</strong> ← <strong>${to}</strong></p>
      <p>الثقة: ${confidence}%</p>
      <div class="actions">
        <button>تصحيح</button>
        <button class="ghost">تجاهل</button>
        <button class="ghost">تجاهل دائمًا</button>
      </div>
    </section>
  `;
}
