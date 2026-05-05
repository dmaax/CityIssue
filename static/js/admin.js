// ============================================================
// admin.js — View: Painel Administrativo (CRUD de Chamados)
// ============================================================

const token = localStorage.getItem('token');
const role  = localStorage.getItem('role');
const nome  = localStorage.getItem('nome');

// Verificar autenticacao e role admin
if (!token) {
  alert('Acesso restrito. Faça login.');
  window.location.href = 'login-cadastro.html';
}

document.getElementById('adminWelcome').textContent =
  `Bem-vindo, ${nome || 'Admin'} | Role: ${role || '?'}`;

// Logout
document.getElementById('logoutBtn').addEventListener('click', (e) => {
  e.preventDefault();
  localStorage.clear();
  window.location.href = 'login-cadastro.html';
});

// Tabs
document.querySelectorAll('.tab-btn').forEach(btn => {
  btn.addEventListener('click', () => {
    document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
    document.querySelectorAll('.tab-panel').forEach(p => p.classList.remove('active'));
    btn.classList.add('active');
    document.getElementById('tab-' + btn.dataset.tab).classList.add('active');
    if (btn.dataset.tab === 'usuarios') loadUsers();
  });
});

// ---------- AUTH HEADER ----------
const authHeaders = {
  'Content-Type': 'application/json',
  'Authorization': `Bearer ${token}`,
};

// ---------- CHAMADOS ----------

async function loadChamados() {
  try {
    const res = await fetch('/api/chamados', { headers: authHeaders });
    const data = await res.json();
    if (data.success) renderChamados(data.data);
    else document.getElementById('tbodyChamados').innerHTML =
      `<tr><td colspan="6" style="color:#e53e3e">${data.message}</td></tr>`;
  } catch {
    document.getElementById('tbodyChamados').innerHTML =
      '<tr><td colspan="6">Erro ao carregar chamados.</td></tr>';
  }
}

const statusLabel = { pending:'Pendente', 'in-progress':'Em andamento', done:'Resolvido', closed:'Fechado' };

function renderChamados(items) {
  const tbody = document.getElementById('tbodyChamados');
  tbody.innerHTML = '';
  if (!items.length) {
    tbody.innerHTML = '<tr><td colspan="6">Nenhum chamado encontrado.</td></tr>';
    return;
  }
  items.forEach(c => {
    const tr = document.createElement('tr');
    tr.innerHTML = `
      <td>#${c.id.slice(0,8)}</td>
      <td>${c.user_nome}</td>
      <td>${c.tipo}</td>
      <td>${c.endereco}</td>
      <td><span class="status-badge status-${c.status}">${statusLabel[c.status] || c.status}</span></td>
      <td class="actions-cell">
        <button class="btn-edit"   onclick="openEdit('${c.id}')">Editar</button>
        <button class="btn-delete" onclick="deleteChamado('${c.id}')">Excluir</button>
      </td>`;
    tbody.appendChild(tr);
  });
}

// ---------- DELETE Chamado ----------
async function deleteChamado(id) {
  if (!confirm('Deseja realmente excluir este chamado?')) return;
  const res = await fetch(`/api/chamados/${id}`, { method: 'DELETE', headers: authHeaders });
  const data = await res.json();
  alert(data.message);
  if (data.success) loadChamados();
}

// ---------- EDIT Chamado ----------
let currentChamado = null;

async function openEdit(id) {
  // Buscar na lista atual
  const res  = await fetch('/api/chamados', { headers: authHeaders });
  const data = await res.json();
  currentChamado = (data.data || []).find(c => c.id === id);
  if (!currentChamado) return;

  document.getElementById('editId').value       = currentChamado.id;
  document.getElementById('editTitulo').value   = currentChamado.titulo;
  document.getElementById('editTipo').value     = currentChamado.tipo;
  document.getElementById('editEndereco').value = currentChamado.endereco;
  document.getElementById('editStatus').value   = currentChamado.status;
  document.getElementById('modalEdit').classList.add('open');
}

document.getElementById('btnCancelEdit').addEventListener('click', () => {
  document.getElementById('modalEdit').classList.remove('open');
});

document.getElementById('btnSaveEdit').addEventListener('click', async () => {
  const id = document.getElementById('editId').value;
  const body = {
    titulo:   document.getElementById('editTitulo').value,
    tipo:     document.getElementById('editTipo').value,
    endereco: document.getElementById('editEndereco').value,
    status:   document.getElementById('editStatus').value,
  };
  const res  = await fetch(`/api/chamados/${id}`, {
    method: 'PUT', headers: authHeaders, body: JSON.stringify(body),
  });
  const data = await res.json();
  alert(data.message);
  document.getElementById('modalEdit').classList.remove('open');
  if (data.success) loadChamados();
});

// ---------- USUARIOS ----------
async function loadUsers() {
  if (role !== 'admin') {
    document.getElementById('tbodyUsers').innerHTML =
      '<tr><td colspan="6" style="color:#e53e3e">Acesso restrito a administradores.</td></tr>';
    return;
  }
  try {
    const res  = await fetch('/api/users', { headers: authHeaders });
    const data = await res.json();
    if (data.success) renderUsers(data.data);
    else document.getElementById('tbodyUsers').innerHTML =
      `<tr><td colspan="6">${data.message}</td></tr>`;
  } catch {
    document.getElementById('tbodyUsers').innerHTML =
      '<tr><td colspan="6">Erro ao carregar usuários.</td></tr>';
  }
}

function renderUsers(users) {
  const tbody = document.getElementById('tbodyUsers');
  tbody.innerHTML = '';
  users.forEach(u => {
    const tr = document.createElement('tr');
    tr.innerHTML = `
      <td>${u.id.slice(0,8)}</td>
      <td>${u.nome}</td>
      <td>${u.email}</td>
      <td>${u.role}</td>
      <td>${u.criado_em ? u.criado_em.slice(0,10) : '-'}</td>
      <td class="actions-cell">
        <button class="btn-delete" onclick="deactivateUser('${u.id}')">Desativar</button>
      </td>`;
    tbody.appendChild(tr);
  });
}

async function deactivateUser(id) {
  if (!confirm('Desativar este usuário?')) return;
  const res  = await fetch(`/api/users/${id}`, { method: 'DELETE', headers: authHeaders });
  const data = await res.json();
  alert(data.message);
  if (data.success) loadUsers();
}

// ---------- INIT ----------
loadChamados();

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