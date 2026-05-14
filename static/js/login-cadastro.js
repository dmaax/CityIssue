// ============================================================
// login-cadastro.js — View: Autenticacao e Cadastro
// ============================================================

const API = '';  // mesmo host

// ---------- HELPERS ----------
function showMsg(id, msg, isError = false) {
  const el = document.getElementById(id);
  if (!el) return;
  el.textContent = msg;
  el.style.color = isError ? '#e53e3e' : '#00cc55';
  el.style.display = 'block';
}

// ---------- CADASTRO ----------
const formCadastro = document.getElementById('formCadastro');
if (formCadastro) {
  formCadastro.addEventListener('submit', async function (e) {
    e.preventDefault();

    const nome   = document.getElementById('cadNome').value.trim();
    const email  = document.getElementById('cadEmail').value.trim();
    const senha  = document.getElementById('cadSenha').value;
    const conf   = document.getElementById('cadConfirmaSenha').value;

    if (senha !== conf) {
      return showMsg('msgCadastro', 'As senhas não coincidem!', true);
    }

    try {
      const res = await fetch(`${API}/api/auth/register`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ nome, email, senha }),
      });
      const data = await res.json();

      if (data.success) {
        localStorage.setItem('token', data.data.token);
        localStorage.setItem('role',  data.data.role);
        localStorage.setItem('nome',  data.data.nome);
        showMsg('msgCadastro', 'Cadastro realizado! Redirecionando...');
        setTimeout(() => { window.location.href = 'abrir-chamado.html'; }, 1200);
      } else {
        showMsg('msgCadastro', data.message, true);
      }
    } catch (err) {
      showMsg('msgCadastro', 'Erro de conexão com o servidor.', true);
    }
  });
}

// ---------- LOGIN ----------
const formLogin = document.getElementById('formLogin');
if (formLogin) {
  formLogin.addEventListener('submit', async function (e) {
    e.preventDefault();

    const email = document.getElementById('loginEmail').value.trim();
    const senha = document.getElementById('loginSenha').value;

    try {
      const res = await fetch(`${API}/api/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email, senha }),
      });
      const data = await res.json();

      if (data.success) {
        localStorage.setItem('token', data.data.token);
        localStorage.setItem('role',  data.data.role);
        localStorage.setItem('nome',  data.data.nome);
        showMsg('msgLogin', 'Login realizado! Redirecionando...');
        const dest = data.data.role === 'admin' ? 'admin.html' : 'abrir-chamado.html';
        setTimeout(() => { window.location.href = dest; }, 1000);
      } else {
        showMsg('msgLogin', data.message, true);
      }
    } catch (err) {
      showMsg('msgLogin', 'Erro de conexão com o servidor.', true);
    }
  });
}
