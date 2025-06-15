# Política de Proteção de Branch

Este documento descreve as políticas de proteção de branch implementadas no repositório R5 Flowlight.

## Visão Geral

Para manter a qualidade do código e um fluxo de trabalho eficiente, implementamos proteções de branch que garantem:

1. **Qualidade do Código**: Todo o código deve passar por revisão e testes antes de ser mesclado.
2. **Rastreabilidade**: Todo o trabalho é rastreado por meio de pull requests.
3. **Estabilidade**: A branch principal (`main`) está sempre em um estado estável.

## Branches Protegidas

### `main`

- **Proposta**: Branch de produção
- **Proteções**:
  - Push direto desabilitado
  - Requer revisão de código
  - Requer verificações de status de CI bem-sucedidas
  - Requer resolução de conversa
  - Requer histórico linear
  - Não permite exclusão

### `develop`

- **Proposta**: Branch de desenvolvimento
- **Proteções**:
  - Push direto desabilitado
  - Requer revisão de código (1 aprovador)
  - Requer verificações de status de CI bem-sucedidas
  - Requer resolução de conversa
  - Requer histórico linear
  - Não permite exclusão

## Processo de Revisão de Código

1. **Criar um Pull Request (PR)**
   - Descreva as alterações
   - Referencie as issues relacionadas
   - Atribua revisores apropriados

2. **Revisão do Código**
   - Os revisores devem:
     - Verificar a qualidade do código
     - Garantir que os testes estejam presentes
     - Verificar se a documentação foi atualizada
     - Aprovar ou solicitar alterações

3. **Integração Contínua**
   - O PR deve passar em todas as verificações de CI
   - Os testes devem estar passando
   - A compilação deve ser bem-sucedida

4. **Merge**
   - Apenas após aprovação e verificação de CI
   - Use "Squash and merge" para manter o histórico limpo
   - Remova a branch de recurso após o merge

## Exceções

Em casos raros, pode ser necessário fazer push direto para branches protegidas. Isso deve ser feito apenas em situações de emergência e deve ser comunicado à equipe imediatamente.

## Solução de Problemas

Se você encontrar problemas com as proteções de branch:

1. Verifique se seu PR atende a todos os requisitos
2. Certifique-se de que todos os testes estão passando
3. Verifique se você tem as permissões necessárias
4. Consulte um administrador do repositório se o problema persistir

## Documentação Adicional

- [GitHub Docs - Sobre branches protegidas](https://docs.github.com/pt/repositories/configuring-branches-and-merges-in-your-repository/defining-the-mergeability-of-pull-requests/about-protected-branches)
- [GitHub Docs - Gerenciando uma proteção de branch](https://docs.github.com/pt/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/managing-a-branch-protection-rule)
