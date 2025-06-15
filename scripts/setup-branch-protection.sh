#!/bin/bash

# Script para configurar as regras de proteção de branch no GitHub
# Requer a GitHub CLI (gh) instalada e autenticada

REPO="rafaelreis-r5/flowlight"
BASE_CONFIG=".github/branch-protection.yml"
TMP_DIR="/tmp/branch-protection-$(date +%s)"

# Criar diretório temporário
mkdir -p "$TMP_DIR"

# Função para limpar arquivos temporários
cleanup() {
    rm -rf "$TMP_DIR"
}
trap cleanup EXIT

# Criar configuração para a branch main
cat > "$TMP_DIR/main.yml" << 'EOF'
required_status_checks:
  strict: true
  contexts:
    - "check-formatting"
    - "test"
    - "build"
enforce_admins: false
required_pull_request_reviews:
  required_approving_review_count: 1
  require_code_owner_reviews: true
  dismiss_stale_reviews: true
  require_last_push_approval: true
restrictions: null
required_linear_history: true
allow_force_pushes: false
allow_deletions: false
required_conversation_resolution: true
lock_branch: false
allow_fork_syncing: true
EOF

# Criar configuração para a branch develop
cat > "$TMP_DIR/develop.yml" << 'EOF'
required_status_checks:
  strict: true
  contexts:
    - "check-formatting"
    - "test"
    - "build"
enforce_admins: false
required_pull_request_reviews:
  required_approving_review_count: 1
  require_code_owner_reviews: false
  dismiss_stale_reviews: true
  require_last_push_approval: true
restrictions: null
required_linear_history: true
allow_force_pushes: false
allow_deletions: false
required_conversation_resolution: true
lock_branch: false
allow_fork_syncing: true
EOF

# Verificar se os arquivos foram criados
if [ ! -f "$TMP_DIR/main.yml" ] || [ ! -f "$TMP_DIR/develop.yml" ]; then
    echo "Erro ao criar arquivos de configuração temporários"
    exit 1
fi

# Verifica se o gh está instalado
if ! command -v gh &> /dev/null; then
    echo "GitHub CLI (gh) não encontrado. Por favor, instale-o primeiro."
    echo "https://cli.github.com/"
    exit 1
fi

# Verifica se o usuário está autenticado
if ! gh auth status &> /dev/null; then
    echo "Por favor, faça login no GitHub CLI primeiro com: gh auth login"
    exit 1
fi

# Aplica as configurações para cada branch
echo "Aplicando configurações de proteção de branch..."
echo "Repositório: $REPO"

# Para a branch main
echo -e "\n=== Configurando proteção para a branch main ==="
cat "$TMP_DIR/main.yml" | gh api \
  -X PUT \
  -H "Accept: application/vnd.github.v3+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  /repos/$REPO/branches/main/protection \
  --input - \
  --jq 'del(.url,.required_status_checks.url)' \
  || { echo "Erro ao configurar proteção para a branch main"; exit 1; }

echo -e "\n=== Configuração concluída para a branch main ==="

# Para a branch develop
echo -e "\n=== Configurando proteção para a branch develop ==="
cat "$TMP_DIR/develop.yml" | gh api \
  -X PUT \
  -H "Accept: application/vnd.github.v3+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  /repos/$REPO/branches/develop/protection \
  --input - \
  --jq 'del(.url,.required_status_checks.url)' \
  || { echo "Erro ao configurar proteção para a branch develop"; exit 1; }

echo -e "\n=== Configuração concluída para a branch develop ==="

echo "Configuração concluída com sucesso!"
