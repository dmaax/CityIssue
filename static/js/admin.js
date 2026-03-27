const statusMeta = {
  pending: { label: 'Pendente', class: 'pending' },
  'in-progress': { label: 'Em andamento', class: 'in-progress' },
  done: { label: 'Resolvido', class: 'done' },
};

const hardcodedChamados = [
  { id: '001', user: 'Maria', tipo: 'Iluminação', local: 'Av. Paulista, SP', status: 'pending' },
  { id: '002', user: 'João', tipo: 'Buracos', local: 'Rua das Flores, RJ', status: 'in-progress' },
  { id: '003', user: 'Ana', tipo: 'Limpeza', local: 'Centro, BH', status: 'done' },
  { id: '004', user: 'Carlos', tipo: 'Sinalização', local: 'Av. Brasil, RJ', status: 'pending' },
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

function getStatusSelect(selectedStatus) {
  const select = document.createElement('select');
  const optionNone = document.createElement('option');
  optionNone.value = '';
  optionNone.textContent = '--- sem check ---';
  select.appendChild(optionNone);

  for (const key of ['pending', 'in-progress', 'done']) {
    const opt = document.createElement('option');
    opt.value = key;
    opt.textContent = statusMeta[key].label;
    if (key === selectedStatus) opt.selected = true;
    select.appendChild(opt);
  }
  return select;
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
      <td></td>
    `;

    const statusCell = row.children[4];
    const select = getStatusSelect(chamado.status);
    select.addEventListener('change', () => {
      atualizarStatusBackend(chamado.id, select.value);
    });
    statusCell.appendChild(select);

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