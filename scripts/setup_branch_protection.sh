#!/bin/bash
# Script para configurar a proteção de branch no GitHub
# Requer: gh CLI instalado e autenticado

# Nome do repositório no formato "dono/repositorio"
REPO="rafaelreis-r5/flowlight"

# Configuração para a branch main
echo "Configurando proteção para a branch main..."
gh api \
  --method PUT \
  -H "Accept: application/vnd.github.v3+json" \
  "/repos/$REPO/branches/main/protection" \
  --input - <<EOF
{
  "required_status_checks": {
    "strict": true,
    "contexts": ["check-formatting", "test", "build"]
  },
  "enforce_admins": false,
  "required_pull_request_reviews": {
    "required_approving_review_count": 1,
    "require_code_owner_reviews": true,
    "dismiss_stale_reviews": true,
    "require_last_push_approval": true
  },
  "restrictions": null,
  "required_linear_history": true,
  "allow_force_pushes": false,
  "allow_deletions": false,
  "required_conversation_resolution": true,
  "lock_branch": false,
  "allow_fork_syncing": true
}
EOF

# Configuração para a branch develop
echo "Configurando proteção para a branch develop..."
gh api \
  --method PUT \
  -H "Accept: application/vnd.github.v3+json" \
  "/repos/$REPO/branches/develop/protection" \
  --input - <<EOF
{
  "required_status_checks": {
    "strict": true,
    "contexts": ["check-formatting", "test", "build"]
  },
  "enforce_admins": false,
  "required_pull_request_reviews": {
    "required_approving_review_count": 1,
    "require_code_owner_reviews": false,
    "dismiss_stale_reviews": true,
    "require_last_push_approval": true
  },
  "restrictions": null,
  "required_linear_history": true,
  "allow_force_pushes": false,
  "allow_deletions": false,
  "required_conversation_resolution": true,
  "lock_branch": false,
  "allow_fork_syncing": true
}
EOF

echo "Configuração de proteção de branch concluída!"
