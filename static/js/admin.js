// ============================================================
// admin.js — Painel Administrativo (MVC View)
// ============================================================

const token = localStorage.getItem('token');
const role  = localStorage.getItem('role');
const nome  = localStorage.getItem('nome');

// Verificar autenticacao
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

const authHeaders = {
  'Content-Type': 'application/json',
  'Authorization': `Bearer ${token}`,
};

// ---------- CHAMADOS ----------

async function loadChamados() {
  try {
    const res  = await fetch('/api/chamados', { headers: authHeaders });
    const data = await res.json();
    if (data.success) renderChamados(data.data);
    else document.getElementById('tbodyChamados').innerHTML =
      `<tr><td colspan="6" style="color:#e53e3e">${data.message}</td></tr>`;
  } catch {
    document.getElementById('tbodyChamados').innerHTML =
      '<tr><td colspan="6">Erro ao carregar chamados.</td></tr>';
  }
}

const statusLabel = {
  pending: 'Pendente',
  'in-progress': 'Em andamento',
  done: 'Resolvido',
  closed: 'Fechado',
};

function renderChamados(items) {
  const tbody = document.getElementById('tbodyChamados');
  tbody.innerHTML = '';
  if (!items || !items.length) {
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

async function deleteChamado(id) {
  if (!confirm('Deseja realmente excluir este chamado?')) return;
  const res  = await fetch(`/api/chamados/${id}`, { method: 'DELETE', headers: authHeaders });
  const data = await res.json();
  alert(data.message);
  if (data.success) loadChamados();
}

// ---------- EDIT ----------
let currentChamado = null;

async function openEdit(id) {
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
  const id   = document.getElementById('editId').value;
  const body = {
    titulo:   document.getElementById('editTitulo').value,
    tipo:     document.getElementById('editTipo').value,
    descricao: currentChamado ? currentChamado.descricao : '',
    endereco: document.getElementById('editEndereco').value,
    status:   document.getElementById('editStatus').value,
  };

  // Salvar campos editaveis
  const resPut = await fetch(`/api/chamados/${id}`, {
    method: 'PUT', headers: authHeaders, body: JSON.stringify(body),
  });
  const dataPut = await resPut.json();

  // Atualizar status separadamente se admin
  if (role === 'admin') {
    await fetch(`/api/chamados/${id}/status`, {
      method: 'PUT', headers: authHeaders,
      body: JSON.stringify({ status: body.status }),
    });
  }

  alert(dataPut.message);
  document.getElementById('modalEdit').classList.remove('open');
  if (dataPut.success) loadChamados();
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
  if (!users || !users.length) {
    tbody.innerHTML = '<tr><td colspan="6">Nenhum usuário encontrado.</td></tr>';
    return;
  }
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
