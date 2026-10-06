const milestones = [
  { title: 'Architecture & prototype', description: 'Evidence submitted · approved on-chain', amount: '600 POT', done: true },
  { title: 'Testnet launch & docs', description: 'Awaiting builder evidence', amount: '600 POT', done: false }
];

const milestonesElement = document.querySelector('#milestones');
const demoButton = document.querySelector('#demo');
const connectButton = document.querySelector('#connect');
const statusElement = document.querySelector('#status');
const networkElement = document.querySelector('#network');

function renderMilestones() {
  milestonesElement.innerHTML = milestones.map((milestone, index) => `
    <article class="milestone">
      <span class="dot ${milestone.done ? '' : 'pending'}" aria-hidden="true"></span>
      <div><h3>${index + 1}. ${milestone.title}</h3><p>${milestone.description}</p></div>
      <span class="amount">${milestone.amount}</span>
    </article>
  `).join('');
}

demoButton.addEventListener('click', () => {
  renderMilestones();
  statusElement.textContent = 'Demo loaded. This view is illustrative; connect a provider before signing transactions.';
  demoButton.textContent = 'Demo loaded ✓';
});

connectButton.addEventListener('click', () => {
  const hasInjectedProvider = Boolean(window.injectedWeb3 || window.ethereum);
  if (hasInjectedProvider) {
    networkElement.textContent = 'PROVIDER DETECTED';
    networkElement.classList.add('live');
    statusElement.textContent = 'A wallet provider is available. Configure the Portaldot RPC and contract address before sending a transaction.';
    connectButton.textContent = 'Provider detected ✓';
  } else {
    statusElement.textContent = 'No wallet provider detected. The dashboard remains in safe demo mode.';
    connectButton.textContent = 'Demo only';
  }
});

renderMilestones();
