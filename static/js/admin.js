const statusMeta = {
  pending: { label: 'Pendente', class: 'pending' },
  'in-progress': { label: 'Em andamento', class: 'in-progress' },
  done: { label: 'Resolvido', class: 'done' },
  closed: { label: 'Fechado', class: 'closed' },
};

const statusSteps = [
  { key: 'in-progress', label: 'Em andamento' },
  { key: 'done', label: 'Resolvido' },
  { key: 'closed', label: 'Fechado' },
];

const hardcodedChamados = [
  { id: '001', user: 'Maria', tipo: 'Iluminação', local: 'Av. Paulista, SP', status: 'pending', foto: 'https://imgs.search.brave.com/xN-R00Iw32mrp3e4uYJSoUACmlCIUeRVC1HzzpuWT6g/rs:fit:500:0:1:0/g:ce/aHR0cHM6Ly93d3cu/ZGVza3RvcC5jb20u/YnIvYmxvZy93cC1j/b250ZW50L3dlYnAt/ZXhwcmVzcy93ZWJw/LWltYWdlcy91cGxv/YWRzLzIwMjQvMTEv/UXVhbmRvLXJlYWxp/emFyLXVtLXRlc3Rl/LWRlLXZlbG9jaWRh/ZGUtZGUtaW50ZXJu/ZXQuanBnLndlYnA' },
  { id: '002', user: 'João', tipo: 'Buracos', local: 'Rua das Flores, RJ', status: 'in-progress', foto: 'https://imgs.search.brave.com/dGicBYK5dU30d9rAltPGQIWxRZlnHOp8WU6JVM-Ctfc/rs:fit:500:0:1:0/g:ce/aHR0cHM6Ly9zdC5k/ZXBvc2l0cGhvdG9z/LmNvbS8xMDMyNTc3/LzMyMzgvaS80NTAv/ZGVwb3NpdHBob3Rv/c18zMjM4MjYxMS1z/dG9jay1waG90by10/ZXN0LmpwZw' },
  { id: '003', user: 'Ana', tipo: 'Limpeza', local: 'Centro, BH', status: 'done', foto: '' },
  { id: '004', user: 'Carlos', tipo: 'Sinalização', local: 'Av. Brasil, RJ', status: 'pending', foto: 'https://imgs.search.brave.com/3f0tAfCDD-iB4CzTXkU6Fjhffjl3FFdwIIBs7WBG1-s/rs:fit:500:0:1:0/g:ce/aHR0cHM6Ly9pbWcu/ZnJlZXBpay5jb20v/Zm90b3MtcHJlbWl1/bS9jbG9zZS11cC1k/ZS11bS1wYXRvXzEw/NDg5NDQtMjk1NzA0/NzYuanBnP3NlbXQ9/YWlzX2luY29taW5n/Jnc9NzQwJnE9ODA' },
];

const chamadaBody = document.querySelector('.chamados-table tbody');
const URL_CHAMADOS = '/api/chamados'; // backend endpoint a ser implementado

let chamadoSelecionado = null; // id do chamado selecionado no painel

async function fetchChamados() {
  try {
    const response = await fetch(URL_CHAMADOS);
    if (!response.ok) throw new Error('Erro ao carregar chamados no backend');
    const data = await response.json();
    if (!Array.isArray(data)) throw new Error('Formato inválido de payload');
    return data;
  } catch (err) {
    console.warn('Falha no fetch. Usando dados hardcoded:', err.message);
    return hardcodedChamados;
  }
}

function setStatusButtons(chamadoId, newStatus, stepsWrapper) {
  atualizarStatusBackend(chamadoId, newStatus);

  stepsWrapper.querySelectorAll('.status-step').forEach((btn) => {
    const isSelected = btn.dataset.status === newStatus;
    btn.classList.toggle('active', isSelected);
    btn.querySelector('.step-check').textContent = isSelected ? '✓' : '';
  });
}

const imageModal = (() => {
  const modal = document.createElement('div');
  modal.className = 'image-modal';
  modal.innerHTML = `
    <div class="image-modal-backdrop"></div>
    <div class="image-modal-content">
      <button type="button" class="image-modal-close" aria-label="Fechar imagem">×</button>
      <img class="image-modal-img" src="" alt="Imagem do chamado" />
    </div>
  `;

  document.body.appendChild(modal);
  const close = modal.querySelector('.image-modal-close');
  const backdrop = modal.querySelector('.image-modal-backdrop');
  const img = modal.querySelector('.image-modal-img');

  const hide = () => {
    modal.classList.remove('open');
    img.src = '';
    img.alt = 'Imagem do chamado';
  };

  close.addEventListener('click', hide);
  backdrop.addEventListener('click', hide);

  window.addEventListener('keydown', (event) => {
    if (event.key === 'Escape' && modal.classList.contains('open')) hide();
  });

  return {
    open(src, alt) {
      img.src = src;
      img.alt = alt;
      modal.classList.add('open');
    },
    hide,
  };
})();

function createStatusSteps(currentStatus, chamadoId) {
  const container = document.createElement('div');
  container.className = 'status-steps';

  statusSteps.forEach((step) => {
    const stepButton = document.createElement('button');
    stepButton.type = 'button';
    stepButton.className = 'status-step';
    stepButton.dataset.status = step.key;
    const selected = step.key === currentStatus;
    if (selected) stepButton.classList.add('active');

    stepButton.innerHTML = `
      <span class="step-check">${selected ? '✓' : ''}</span>
      <span class="step-label">${step.label}</span>
    `;

    stepButton.addEventListener('click', (event) => {
      event.stopPropagation();
      setStatusButtons(chamadoId, step.key, container);
    });

    container.appendChild(stepButton);
  });

  return container;
}

function renderChamados(items) {
  chamadaBody.innerHTML = '';

  items.forEach(chamado => {
    const row = document.createElement('tr');
    row.dataset.id = chamado.id;

    row.innerHTML = `
      <td>#${chamado.id}</td>
      <td>${chamado.user}</td>
      <td>${chamado.tipo}</td>
      <td>${chamado.local}</td>
      <td class="photo-cell"></td>
      <td class="status-cell"></td>
    `;

    const photoCell = row.querySelector('.photo-cell');
    if (chamado.foto) {
      const img = document.createElement('img');
      img.src = chamado.foto;
      img.alt = `Foto do chamado ${chamado.id}`;
      img.className = 'chamado-photo';
      img.addEventListener('click', (event) => {
        event.stopPropagation();
        imageModal.open(chamado.foto, `Foto do chamado ${chamado.id}`);
      });
      photoCell.appendChild(img);
    } else {
      photoCell.textContent = 'Sem imagem';
      photoCell.classList.add('no-photo');
    }

    const statusCell = row.querySelector('.status-cell');
    const statusStepsEl = createStatusSteps(chamado.status || 'in-progress', chamado.id);
    statusCell.appendChild(statusStepsEl);

    row.addEventListener('click', () => {
      if (chamadoSelecionado) {
        const prev = document.querySelector('tr.selected-row');
        if (prev) prev.classList.remove('selected-row');
      }
      chamadoSelecionado = chamado.id;
      row.classList.add('selected-row');
    });

    chamadaBody.appendChild(row);
  });
}

async function atualizarStatusBackend(id, status) {
  try {
    const response = await fetch(`${URL_CHAMADOS}/${id}/status`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ status }),
    });

    if (!response.ok) {
      throw new Error('Erro ao atualizar status no backend');
    }

    const resultado = await response.json();
    console.log('Status atualizado no backend:', resultado);
  } catch (err) {
    console.warn('Não foi possível enviar status ao backend:', err.message);
  }
}



(async () => {
  const chamados = await fetchChamados();
  renderChamados(chamados);
})();