// CADASTRO
const formCadastro = document.getElementById("formCadastro");

if (formCadastro) {
  formCadastro.addEventListener("submit", function(e) {
    e.preventDefault();

    const senha = document.getElementById("cadSenha").value;
    const confirmar = document.getElementById("cadConfirmaSenha").value;

    if (senha !== confirmar) {
      alert("Erro: As senhas não coincidem!");
      return;
    }

    alert("Cadastro realizado com sucesso!");
    formCadastro.reset(); 
  });
}

// LOGIN
const formLogin = document.getElementById("formLogin");

if (formLogin) {
  formLogin.addEventListener("submit", function(e) {
    e.preventDefault();

    alert("Login realizado!");
    formLogin.reset(); 
  });
}