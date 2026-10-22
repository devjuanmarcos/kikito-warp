# Fluxos Git: VS Code (Git + GitHub) × painel "Controle de versão" do Kikito

Referência: painel Source Control do VS Code com a extensão Git embutida, a extensão GitHub (autenticação, publicar) e o Source Control Graph. Pedido do Marcos em 21/10/2026: ter os mesmos fluxos no painel do Kikito, incluindo a listagem de vários repositórios dentro da pasta do projeto.

Legenda: ✅ já existe · 🟡 parcial · ❌ falta · ⏸ fora de escopo por enquanto (depende de decisão)

## 1. Repositórios (vários `.git` numa pasta)

| Fluxo no VS Code | Kikito | Observação |
|---|---|---|
| Detectar o repositório da pasta aberta (ou de uma pasta pai) | ✅ | `detect_possible_local_git_repo` sobe a árvore. |
| Detectar sub-repositórios dentro da pasta (`git.autoRepositoryDetection: subFolders`, `git.repositoryScanMaxDepth`) | ✅ | Até 2 níveis, ignora `node_modules`/`target`/etc. Varrer a pasta do projeto até profundidade 2, ignorando `node_modules`, `target`, `dist`, `.venv` etc. Aceitar `.git` pasta ou arquivo (worktree/submódulo). |
| Lista "Repositórios" com nome, branch atual e status de sincronização (↑↓) de cada um | ✅ | Fase 1. Seção no topo do painel quando houver mais de um repositório. |
| Clicar num repositório para torná-lo o ativo (alterações, commit, grafo e rodapé passam a ser dele) | ✅ | Lembrado por projeto, inclusive após reiniciar. |
| Contagem de alterações por repositório na lista | ✅ | |
| Fechar/ocultar um repositório da lista | ❌ | Fase 3. |
| Abrir repositório de uma pasta pai (`git.openRepositoryInParentFolders`) | ✅ | Já sobe a árvore. |
| Pasta sem repositório: "Inicializar repositório" | ✅ | Falta testar no app. |
| Clonar repositório | ⏸ | Faz mais sentido no fluxo de criar projeto. |

## 2. Alterações

| Fluxo | Kikito | Observação |
|---|---|---|
| Grupos Preparadas / Alterações / Conflitos de merge / Não rastreados | ✅ | Não rastreados entram em Alterações com `U`, como no kit. |
| Preparar / retirar arquivo | ✅ | |
| Preparar tudo / retirar tudo | ✅ | |
| Descartar arquivo (com confirmação) | ✅ | |
| Descartar todas as alterações | ✅ | Com confirmação. |
| Abrir diff do arquivo (lado a lado) | 🟡 | Hoje é um preview inline no painel; o VS Code abre editor de diff. Avaliar abrir o diff na área principal (Fase 3). |
| Preparar / retirar bloco (hunk) | ✅ | "Preparar bloco". |
| Preparar linhas selecionadas | ⏸ | Depende de seleção no diff. |
| Abrir o arquivo / revelar no explorador | ❌ | Fase 3. |
| Copiar caminho / caminho relativo | ✅ | Menu de clique direito do arquivo. |
| Adicionar ao `.gitignore` | ✅ | |
| Ver como lista / árvore | ❌ | Fase 3. |
| Seleção múltipla de arquivos | ⏸ | |

## 3. Commit

| Fluxo | Kikito | Observação |
|---|---|---|
| Mensagem + Ctrl+Enter | ✅ | |
| Commit / Commit de tudo (sem nada preparado) | ✅ | |
| Amend (alterar último commit) | ✅ | Menu do botão de commit, com confirmação. |
| Commit e push / Commit e sincronizar | ✅ | |
| Desfazer último commit | ✅ | |
| Reverter commit | ✅ | Qualquer commit, pelo grafo. |
| Histórico de mensagens (seta para cima) | ❌ | Fase 3. |
| Commit vazio / `--no-verify` / `--signoff` | ⏸ | |
| Gerar mensagem com IA | ⏸ | Depende da decisão sobre IA no Kikito. |

## 4. Branches

| Fluxo | Kikito | Observação |
|---|---|---|
| Trocar de branch (locais) | ✅ | Lista de branches. |
| Trocar para branch remota (cria a local rastreando) | ✅ | |
| Criar branch / criar a partir de outra | ✅ | |
| Renomear branch | ✅ | |
| Excluir branch local | ✅ | Com confirmação. |
| Excluir branch remota | ❌ | Fase 3. |
| Publicar branch (push `-u`) | ✅ | "Publicar". |
| Merge de branch na atual | ✅ | |
| Rebase da atual sobre outra | ❌ | Fase 3. |
| Busca na lista de branches | ❌ | Fase 3. |
| Clicar no branch do rodapé para trocar | ✅ | Abre a lista de branches. |

## 5. Sincronização

| Fluxo | Kikito | Observação |
|---|---|---|
| Fetch | ✅ | |
| Pull | ✅ | |
| Push | ✅ | |
| Sincronizar (pull + push) com contagem ↑↓ | ✅ | Testado com remoto local. |
| Fetch com prune / de todos os remotos | ❌ | Fase 3. |
| Pull com rebase | ❌ | Fase 3. |
| Force push (`--force-with-lease`) | ❌ | Fase 3, com confirmação. |
| Fetch automático periódico | ❌ | Fase 3 (configurável). |
| Mensagem clara sem remoto | ✅ | Corrigido em 21/10. |

## 6. Remotos e GitHub

| Fluxo | Kikito | Observação |
|---|---|---|
| Adicionar / remover remoto | ❌ | Fase 3. |
| Publicar no GitHub (criar repositório) | ⏸ | Precisa de autenticação GitHub. |
| Abrir no GitHub (arquivo, commit, branch) | ❌ | Fase 3: montar a URL a partir do remoto `origin`. |
| Criar Pull Request | ⏸ | Extensão separada no VS Code. |

## 7. Stash

| Fluxo | Kikito | Observação |
|---|---|---|
| Stash (com e sem não rastreados, só preparados) | ✅ | Com e sem não rastreados. |
| Aplicar / pop do último ou de um específico | ✅ | |
| Descartar stash / todos | ✅ | Um por vez, com confirmação. |
| Ver stashes | ✅ | Seção STASHES. |

## 8. Tags

| Fluxo | Kikito | Observação |
|---|---|---|
| Criar tag (leve/anotada) | ❌ | Fase 3. |
| Excluir tag | ❌ | Fase 3. |
| Push de tags | ❌ | Fase 3. |
| Tags no grafo | ✅ | Pílulas com ícone de tag. |

## 9. Grafo

| Fluxo | Kikito | Observação |
|---|---|---|
| Histórico com faixas, refs como pílulas, HEAD em anel | ✅ | |
| Filtro: todas as branches / só a atual | ✅ | |
| Clicar no commit: detalhes | ✅ | Com a lista de arquivos alterados. |
| Menu do commit: copiar hash, copiar mensagem | ✅ | |
| Menu do commit: criar branch, criar tag, checkout (HEAD destacado) | 🟡 | Criar branch feito; tag e checkout destacado na Fase 3. |
| Menu do commit: cherry-pick, reverter | ✅ | |
| Carregar mais histórico (paginação) | 🟡 | Limite fixo de 200. Fase 3. |

## 10. Conflitos de merge

| Fluxo | Kikito | Observação |
|---|---|---|
| Grupo "Conflitos de merge" | ✅ | Arquivos com `!`. |
| Aceitar atual / recebida / ambas | 🟡 | Atual e recebida por arquivo; "ambas" exige editor de merge. |
| Marcar como resolvido (preparar) | ✅ | Aceitar um lado já prepara. |
| Abortar merge / continuar rebase | ✅ | Faixa para merge, cherry-pick e revert pendentes, com abortar. |

## 11. Barra de status / indicadores

| Fluxo | Kikito | Observação |
|---|---|---|
| Branch atual e ↑↓ na barra | ✅ | Rodapé do painel e chip do terminal. |
| Contagem de alterações no ícone do painel | ✅ | Badge no botão Git do cabeçalho. |
| Blame inline / histórico do arquivo (Timeline) | ⏸ | Fora do painel. |

## Ordem de implementação

1. **Fase 1 — vários repositórios:** varredura de sub-repositórios, seção "Repositórios" com branch, ↑↓ e alterações de cada um, troca do repositório ativo (lembrada por projeto) e branch do rodapé clicável.
2. **Fase 2 — fluxos do dia a dia:** descartar tudo, menu de contexto do arquivo, commit e sincronizar, excluir branch, checkout de branch remota, merge, busca de branch, stash completo, menu do commit no grafo (copiar, cherry-pick, reverter), arquivos do commit, conflitos (aceitar lado, abortar/continuar), remotos, `git init`, badge de alterações.
3. **Fase 3 — avançados:** tags, rebase, force push, prune, fetch automático, árvore de arquivos, histórico de mensagens, abrir no GitHub, paginação do grafo, diff na área principal.

Cada fase termina com testes de unidade nas operações Git, teste do fluxo no app (perfil `parity`) e medição de paridade quando houver tela do kit correspondente.
