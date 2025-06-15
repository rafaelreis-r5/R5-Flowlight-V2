# Scripts de Configuração do Projeto

Este diretório contém scripts úteis para configurar e manter o ambiente de desenvolvimento do R5 Flowlight.

## Configuração de Proteção de Branch

### Pré-requisitos

1. GitHub CLI (gh) instalado
2. Autenticação no GitHub CLI (`gh auth login`)
3. Permissões de administrador no repositório

### Como usar

1. Navegue até o diretório do projeto
2. Torne o script executável:
   ```bash
   chmod +x scripts/setup-branch-protection.sh
   ```
3. Execute o script:
   ```bash
   ./scripts/setup-branch-protection.sh
   ```

### O que o script faz

1. Verifica se o GitHub CLI está instalado
2. Verifica a autenticação do usuário
3. Aplica as configurações de proteção de branch definidas em `.github/branch-protection.yml`
   - Para a branch `main`
   - Para a branch `develop`

### Configurações Aplicadas

- **Proteção contra push direto**
- Revisão obrigatória de código
  - Pelo menos 1 aprovação necessária
  - Aprovação de proprietários de código obrigatória para mudanças em áreas críticas
- Verificações de status obrigatórias
  - Formatação de código
  - Testes
  - Build
- Histórico linear obrigatório
- Resolução de conversa obrigatória
- Proteção contra exclusão de branch

## Personalização

Você pode personalizar as configurações editando os seguintes arquivos:

- `.github/branch-protection.yml`: Configurações de proteção de branch
- `.github/CODEOWNERS`: Definição dos proprietários de código

## Solução de Problemas

Se encontrar erros ao executar o script:

1. Verifique se você está autenticado no GitHub CLI:
   ```bash
   gh auth status
   ```
2. Verifique se você tem permissões de administrador no repositório
3. Verifique se o arquivo de configuração `.github/branch-protection.yml` existe e está formatado corretamente
