# Handoff: paridade pixel a pixel do workspace do Kikito Terminal

Data: 20/10/2026. Repositório: `D:\DEVJUANMARCOS\PROJETOS\KIKITO WARP`, branch `master`, remoto `git@github.com:devjuanmarcos/kikito-warp.git`.
Último commit enviado: `d163adf09` ("feat(workspace): align rail, sidebar, tabs and Home with the design kit"). Tudo que está descrito como "não commitado" abaixo está só no disco.

## 1. Objetivo

Deixar organizações, projetos, abas, Home, menus e modais do app Rust/WarpUI **idênticos** ao kit de design `C:\Users\Marcos\Downloads\✅ Kikito Terminal.zip` (pasta `ui_kits/workspace/`), mantendo o modelo local (SQLite), as sessões de terminal e o Git local já existentes.

Regra do dono do projeto (Marcos): **só avançar para o próximo item quando o item atual estiver com no mínimo 95% de paridade.** Ele aceitou esta métrica: geometria e cor exatas, e texto comparado com leve desfoque (ver seção 5). Ele pediu atalhos fora da paridade (⌘1–9 já trocam de aba no produto).

Idioma de resposta: português do Brasil, com acentos. O `AGENTS.md` do repositório vale (comentários só de "porquê", `cargo check` pontual, formatador por último, sem presubmit completo).

## 2. Estado por item (ordem do plano)

| # | Item | Estado | Paridade medida |
| - | --- | --- | --- |
| 1 | Hover e press dos botões | fechado, commitado | 95,7% (fundos 98,7%; botões com glifo 96,6%; rodapé com texto 89%) |
| 2 | Tooltips | fechado pela métrica combinada, commitado | geometria/cor exatas; pixels 73–78% (texto) |
| 3 | Cores das organizações vindas do tema + tiles do trilho | fechado, **não commitado** | 95,8% (com desfoque σ=1, tol 16; 93% bruto) |
| 4 | Menu de contexto (org e projeto) | fechado, **não commitado** | posição, tamanho e cor exatos; 87–91% com desfoque |
| 5 | Modal "Nova organização" | **em andamento** | primeira captura bem próxima; segunda versão não medida |
| 6 | Sidebar (busca, imagem da organização, textos vazios, linhas) | pendente | — |
| 7 | Estado "projeto sem aba" | pendente | — |
| 8 | Badge/status de agente | pendente, precisa decisão de dados | — |

Também pendentes do relatório `10-hover-tooltip-parity-audit.md`: recorte de imagem (janela de 240px), diálogo de projeto, diálogo de remoção, painel Git, e a faixa de abas duplicada (ver seção 6).

## 3. O que está no disco e não foi commitado

Compila (`cargo check -p warp --lib` passou), mas o binário com as últimas edições **não foi construído nem medido** (o build foi interrompido por falta de memória, não por erro).

- `app/src/local_workspace/model.rs`: `context_menu_keyboard_active` (destaque de teclado só depois de usar as setas).
- `app/src/workspace/view.rs`: altura do modal por tipo (`dialog_height`: 395 org, 330 projeto, 190 remoção — as duas últimas são palpite, não medidas).
- `.../local_workspace/org_rail.rs`: cores do tema (`organization_accent(theme, nome)`), tile sem borda de hover, borda de 1px, logo com raio 8 e opacidade 0,85 quando inativo, trilho com 64px de conteúdo mais 1px de borda, tooltip escondido com o menu aberto.
- `.../project_sidebar.rs`: sidebar com 248px mais 1px de borda, menu do projeto à esquerda do botão `⋯` (offset `-38, 3`), `WindowByPosition`.
- `.../context_menu.rs`: menu reescrito (186px, padding vertical 9, itens 5/14, hover com cor de destaque, perigo em vermelho ANSI do tema, sombra 0 8 24).
- `.../home.rs`: logo com raio 10.
- `.../create_dialog.rs` + novo `.../dialog_ui.rs`: modal redesenhado (botões 32px, X + ESC, campo de imagem 56px, bolinhas com anel, ordem Nome / Imagem / Cor, cor padrão amarela, fonte da interface no campo, foco automático, borda de destaque no foco).
- `docs/design-harvest/kikito-workspace/harness/`: scripts de captura e comparação (seção 4). `output/` (capturas) continua fora do git de propósito.

Primeiro passo sugerido: rodar `cargo check -p warp --lib`, depois commitar e dar push (Marcos já autorizou push para `master` nesta sessão; confirme de novo se o contexto mudou).

## 4. Como medir (harness)

Tudo em `docs/design-harvest/kikito-workspace/harness/`. Os scripts têm caminhos absolutos da sessão antiga: ajuste `$s`/`S` para a sua pasta de trabalho.

1. **Referência (Chrome):** extrair o zip em pasta própria, servir com `python -I -m http.server 8765 --bind 127.0.0.1 --directory <pasta>`. `cdp.mjs` fala com o Chrome por DevTools (Node 24, sem dependências). Os scripts `ref_*.mjs` abrem `http://127.0.0.1:8765/ui_kits/workspace/index.html`, forçam `data-theme="dark"` no quadro e salvam PNGs 1280×800.
2. **Nativo:** `target\debug\warp-oss.exe` com `WARP_DATA_PROFILE=parity` (perfil isolado em `%LOCALAPPDATA%\warp\WarpOss-parity`). Só funciona em build debug. `seed.py` insere organizações/projetos no `warp.sqlite` do perfil (fechar o app antes). `native.ps1` tem as ações (`launch`, `front`, `move`, `down`, `up`, `rclick`, `shot`, `back`); `native_states.ps1`, `native_tooltips.ps1` e `native_menu.ps1` rodam cenários completos.
3. **Comparar:** `compare.py spec.json` (recortes pareados, tolerância de canal 10, opção `ignore` para mascarar o glifo) e `tt_compare.py`. Para texto, use desfoque gaussiano σ=1 e tolerância 16, com busca de deslocamento ±2px.

Avisos que custaram tempo:
- Mensagens de mouse simuladas (`PostMessage`) não funcionam no app. É preciso mouse **real** (`SetCursorPos` + `mouse_event`), então o usuário não pode mexer no mouse durante a captura (cerca de 1–2 minutos). Avise antes de cada rodada.
- Cliques logo após abrir o app falham: espere ~3s, passe o mouse sobre o alvo (~700ms) e só então clique. Os scripts já fazem isso.
- `native.ps1` só mata processos cujo caminho é o do `warp-oss.exe` do projeto. O Warp instalado do Marcos é `C:\Program Files\Warp\warp.exe`: **nunca mexer nele**.
- A janela do app tem borda de 1px: a primeira coluna/linha de conteúdo começa em x=1, y=1. Os recortes nativos precisam desse deslocamento.
- Build debug leva 5 a 15 minutos. O sistema já matou um build por falta de memória: feche programas pesados antes de compilar.

## 5. Métrica e limites conhecidos

- Pixel a pixel contra o Chrome nunca passa de ~90% quando há texto pequeno (o Chrome usa sub-pixel colorido, o app usa cinza; a largura do texto difere 1–2px). Por isso o critério combinado: geometria e cor exatas, texto com desfoque.
- Ícones: o design usa os SVGs de `assets/icons/` do zip (ex.: `git-branch-02`); o app usa o conjunto `warp_core::ui::Icon` (ex.: `Icon::GitBranch`). Diferem em forma. Resolver exige empacotar os SVGs do zip como assets do app.
- Tooltips acima dos botões do cabeçalho: no design ficam cortados pela moldura e não aparecem; no app aparecem abaixo do botão (decisão minha, avisada ao Marcos).
- Arrastar arquivo para a imagem da organização não existe no app; o texto do modal diz só "PNG, JPG ou SVG." (o design diz "Arraste um arquivo para cá.").

## 6. Lacunas conhecidas fora do item atual

- **Faixa de abas duplicada:** dentro do canvas do projeto aparece a barra de abas nativa do Warp (com seu `+`, divisores e botões de diff) abaixo da nova barra do projeto. O design só tem a barra do projeto. Resolver exige esconder a barra nativa quando o workspace local está ativo (código em `app/src/workspace/view.rs`, `render_panels`/tab bar).
- **Controles da janela:** minimizar/maximizar/fechar ficam sobre o canto direito do cabeçalho; reservei 138px (`WINDOW_CONTROLS_INSET` em `project_tabs.rs`).
- Linha do projeto ativo mostra texto extra do Git ("master · 0 preparados · 74 alterações"), que não existe no design.
- Linhas de projeto não ativas mostram o caminho em vez do branch com ícone.
- Botões "vincular aba" (ícone de link) e reordenar abas: o primeiro mantive como ícone; as setas de reordenação foram removidas. Decisão de produto pendente.
- Home: falta o campo "Jarvis" (Cmd+K), o nome "Juan", o status de agentes, branch nos cards e hover nos cards.
- Badge vermelho do trilho e glifo de status das abas: o modelo local não tem status de agente.
- O painel Source Control, o recorte de imagem e os diálogos de projeto/remoção continuam como o Codex deixou.

## 7. Armadilhas de código (WarpUI)

- Hover/press: use `Hoverable` com `MouseStateHandle` persistente. `ws_style::mouse_state(tipo, id)` devolve o mesmo handle por chave entre quadros; criar o handle dentro do `render` quebra o hover.
- Dois tipos `Fill`: `warp_core::ui::theme::Fill` (temas) e `warpui::elements::Fill` (elementos). Converter com `.into()`. `theme.foreground().with_opacity(n)` dá os overlays do design.
- `Flex` com `MainAxisSize::Max` dentro de scroll vertical ou sem largura limitada causa panic ("flex that should expand to a max space can't be rendered in an infinite max constraint"). Foi a causa do crash ao abrir uma organização.
- `Shrinkable` não expande; use `Expanded` quando o filho precisa ocupar o espaço (cabeçalhos com botão à direita).
- Cabeçalhos de 34px com borda inferior de 1px: adicionar `padding_top(1.)` para o conteúdo cair em pixel inteiro (senão borda e botões ficam borrados).
- Tooltips e menus usam overlay de `Stack` com `ParentOffsetBounds::WindowByPosition`; `ParentByPosition` prende o menu à largura do pai (o menu de 186px ficava preso ao trilho de 64px).
- Textos `Text::new_inline` com `.soft_wrap(false)` e `ClipConfig::ellipsis()` para truncar.
- Arquivos do projeto misturam LF e CRLF; ao editar por script, normalize como em `harness/ed.py`.

## 8. Arquivos principais

- `app/src/workspace/view/local_workspace/`: `mod.rs` (composição), `org_rail.rs`, `project_sidebar.rs`, `project_tabs.rs`, `home.rs`, `context_menu.rs`, `create_dialog.rs`, `dialog_ui.rs`, `ws_style.rs` (helpers de overlay, botões, tooltip, estado de mouse), `image_cropper.rs`, `source_control/`.
- `app/src/local_workspace/model.rs` (modelo), `app/src/workspace/action.rs`, `app/src/workspace/view.rs` (handlers e modal).
- Documentos: `docs/design-harvest/kikito-workspace/00..09`, `10-hover-tooltip-parity-audit.md` (auditoria com placar), `docs/superpowers/plans/2026-10-19-kikito-workspace-pixel-parity.md`, `impl/kikito-workspace-pixel-parity.md`.

## 9. Próximos passos sugeridos (em ordem)

1. `cargo check`, commit e push do estado atual.
2. Construir o binário, capturar o modal "Nova organização" (`ref_modal.mjs` para a referência, `native_menu.ps1` como modelo para o cenário nativo, clicando no `+` do trilho em `(32,250)`) e medir. Corrigir diferenças até ≥95% pela métrica combinada. Confira: foco automático do campo, placeholder em fonte da interface, ordem Nome / Imagem / Cor, rodapé com borda `neutral-4`.
3. Modal de projeto (campo "Diretório" em fonte mono) e de remoção; alturas por tipo (hoje palpite).
4. Sidebar (item 6): imagem da organização no cabeçalho, borda e foco da busca, textos vazios do design, linhas com branch e ícone.
5. Estado "projeto sem aba" (texto 14px centralizado "Projeto quietinho. Abro um terminal?" e botão "Nova aba").
6. Esconder a barra de abas nativa duplicada.
7. Ícones do design como assets.
8. Decisões pendentes com o Marcos: dados de agente (badge/status), botão "vincular aba" e reordenação, atalhos.

## 10. Atualização (20/10/2026, segunda sessão)

Meta confirmada pelo Marcos: **95%** pela métrica combinada (não 98%). Fluxo autônomo: medir, corrigir, commitar e dar push sem pedir aprovação a cada passo.

| # | Item | Estado | Paridade medida |
| - | --- | --- | --- |
| 5 | Modal "Nova organização" | fechado | 95,6% (desfoque σ=1, tol 16; era 84,3%) |
| 5b | Modal "Novo projeto" | fechado | 96,2% |
| 6 | Sidebar | fechado | 96,1% (era 79,9%) |
| 7 | Projeto sem aba | fechado | geometria e cor exatas; fundo 100%; texto 76,7% (só glifos); teclas diferem por plataforma |

O que mudou:
- Modal: caixas de linha de 18px no corpo, cabeçalho de 32px, sem borda e sem margem superior do `Modal` genérico (`with_top_margin`), `PixelAligned` (em `dialog_ui.rs`) pinta o diálogo em pixel inteiro (395px numa janela de 800 caía em y=202,5 e borrava as bordas). Tile mostra a inicial do nome; "Remover" ao lado de "Trocar imagem"; teclas semibold e ⏎ como ícone `CornerDownLeft`; ícone `Icon::Upload` (`upload-01.svg`, idêntico ao do kit).
- Modal de projeto: título "Novo projeto em {org}", campo "Diretório" mono 13px/37px com placeholder `~/slug` (vazio usa o slug; `~` é expandido). Mantive um ícone de pasta dentro do campo para o seletor nativo (desvio pequeno, consciente). Alturas: 318 (novo projeto), 243 (editar projeto). Remoção: o kit não tem diálogo de confirmação; continua 190.
- Sidebar: busca com borda 20%/foco accent, imagem da organização (18px, raio 4) no cabeçalho, branch em todas as linhas (lido de `.git/HEAD` junto com a checagem de pastas; ativo usa o status Git ao vivo), sem a linha extra "master · N preparados", gap 2 entre linhas, linha de 43px, rodapé fixo embaixo, textos vazios do tom Jarvis ("Nada aqui ainda. Quer que eu crie um projeto?" / "Nenhum resultado encontrado").
- Projeto sem aba: "Projeto quietinho. Abro um terminal?" 14px centralizado + botão secundário "Nova aba" com o atalho real (`workspace:new_tab`). O atalho de nova aba, no modo Terminal e com projeto selecionado, agora abre o terminal do projeto (como o kit promete).

Achados úteis:
- Vários ícones do kit já existem em `app/assets/bundled/svg/` com o mesmo desenho; antes de empacotar SVGs novos (item "ícones como assets"), procure lá e só crie a variante em `Icon`.
- `cargo nextest` não está instalado nesta máquina; use `cargo test -p warp --lib <filtro>` (compilar o binário de teste leva ~20 min).
- Fixtures de Git para a captura: repositórios vazios com commit em `scratchpad/repos` (branches `main`, `feat/checkout`); o `seed.py` aponta para lá para os textos de branch baterem com o kit.
- Scripts novos no harness: `native_modal.ps1`, `native_project.ps1`, `native_empty.ps1`, `ref_modal_project.mjs`, `ref_empty.mjs`, `modal_compare.py`, `project_compare.py`, `sidebar_compare.py`, `empty_compare.py`.

Também nesta sessão:
- **Bug corrigido** (`b95b1e30e`): "Falha ao salvar as abas do projeto" aparecia ao abrir uma aba nova. A identidade da aba só é ligada à sessão no próximo snapshot do app, e um segundo `ReplaceProjectTabs` antes disso era rejeitado. Agora basta a identidade existir (o snapshot já remove associações que continuam soltas). Teste de regressão em `persistence/local_workspace_tests.rs`.
- **Faixa de abas duplicada removida** (`ddbe2010a`): a barra empilhada de sessões não é mais desenhada sob a barra do projeto. Controles da janela e arrastar pelo cabeçalho continuam funcionando (verificado: a janela moveu 60px ao arrastar em y=17). Abas fora de projeto aparecem na Home ("Terminais sem projeto").
- **Organização sem projetos** (`8d2254aa1`): cabeçalho com nome + busca (título 98,3%, busca 96,5%), fundo 100%, sidebar 94,7%, prompt central com botão secundário "Novo projeto". Diferença restante: o kit usa `text-wrap: pretty` no texto vazio da sidebar (quebra antes de "um"); o WarpUI não tem equivalente.
- Log do app no perfil de paridade: `%LOCALAPPDATA%\warp\WarpOss-parity\data\logs\warp-oss.log` (o stderr fica vazio).
- Artefato visto em várias capturas: uma linha de texto apagado em y≈798, na borda inferior da área do terminal. Não investigado.

Decisões do Marcos (20/10, fim da sessão): saudação sem nome ("Boa noite. Tudo pronto."), status/badge de agente fora por enquanto, botão "vincular aba" removido por enquanto.

- **Home** (`a34d9a731`, `58a41e218`): barra "Início", campo "O que vamos fazer?" que filtra projetos e abas (ignora "abrir/ir para"), setas + Enter ou clique abrem o resultado (testado no app real), placeholder com o projeto mais recente, cards com hover/caminho com `~`/branch, hover nas abas recentes. Medição contra o kit alimentado com os mesmos dados (`match_kit.py` gera `kit_matched` com o seed nativo e as decisões acima, servido na porta 8766): conteúdo 95,1%, campo 96,4%, cards 96,1%, barra 97,9%. Diferenças: sem tecla ⌘K no campo (não há atalho ligado); logo 1px à esquerda (o conteúdo centra em x=192,5 e o WarpUI arredonda para baixo); a linha "Ver Codex / Retomar" do kit depende de agentes (fora).
- **Ícones**: todos os SVGs usados pelo kit já existem idênticos em `app/assets/bundled/svg`, e as variantes `Icon` usadas no workspace já apontam para eles (`GitBranch` → `git-branch-02.svg`, `X` → `x-close.svg`, etc.). Item fechado sem mudança.

Próximos passos: (1) recorte de imagem ("Ajustar imagem", janela de 240px no kit); (2) painel de controle de versão (`GitPanel.jsx`); (3) diálogo de remoção (o kit remove direto, sem confirmação; decidir com o Marcos se mantém a confirmação); (4) quando houver dados de agente: badge do trilho, glifos nas abas e linha de ações da Home.

## 11. Estado ao interromper (20/10/2026, ~23:52, desligamento agendado do Windows)

Commits enviados nesta parte: recorte "Ajustar imagem" (`cc58481fc`, 97,8%), correção do crash do painel Git (`17ca14277`). Decisão do Marcos: **manter a confirmação de remoção** e seguir os itens sem pausas.

**Painel de controle de versão (em andamento, commit local `wip`, NÃO enviado, NÃO medido, testes NÃO rodados):**
- Reescrito no layout do kit: abre abaixo da barra do projeto (34% da largura, 300–380px), cabeçalho "Controle de versão", seção ALTERAÇÕES (branch clicável abre as branches locais, fetch/pull/push com badge de commits à frente, mensagem multilinha com Ctrl+Enter e placeholder com a branch, botão dividido "Commit"/"Commit de tudo" + menu com Commit e push, Preparar tudo, Remover tudo da preparação e, abaixo de um separador, alterar/desfazer/reverter último commit), listas "Preparadas"/"Alterações" com letra de status colorida, GRAFO (layout de faixas portado do `gitLayout` do kit em `source_control/graph.rs`, cada linha desenhada como SVG gerado, pílulas de ref, filtro "Todas/Só a atual", detalhes ao clicar), rodapé de sincronização (↑↓ ou "não publicada" + Publicar/Sincronizar).
- `local_git`: `%D` (refs) no histórico, `LocalCommit.refs`, `read_history`, `fetch_remote`, `pull_branch`, `push_branch`; push com `-u` para publicar com upstream.
- Novas ações: `ToggleLocalGitPanelSection`, `ToggleLocalGitGraphFilter`, `ToggleLocalGitCommitMenu`, `ToggleLocalGitBranches`, `ToggleLocalGitCommitDetails`, `RunLocalGitRemote`, `CommitAndPushLocalGit`.
- `context_menu::menu(...)` genérico (o menu do commit reutiliza).
- **Crash achado e corrigido**: `Flex` com `MainAxisSize::Max` em largura infinita (contador de arquivos dentro de uma linha). O processo aborta sem gravar o panic no log; diagnóstico feito por bisseção com uma variável de ambiente temporária (`bisect_git.ps1`, já removida do código). Regra: todo `Flex` Max precisa de um pai com a dimensão limitada.
- Próximo: capturar (`native_git.ps1`), medir com `git_compare.py` contra `ref/git_matched.png` (kit alimentado por `match_kit.py` + `GitData` do repositório de teste), corrigir até ≥95%, rodar `cargo test -p warp --lib workspace::view::local_workspace` e `local_git`, clippy, formatar, enviar.

**Harness — segurança:** `native.ps1` agora recusa qualquer ação de mouse/teclado se o `warp-oss.exe` do projeto não estiver em primeiro plano e sob o ponteiro (`Assert-AppAt`; ação `keys` para digitar). Antes disso, numa rodada, cliques caíram na janela do Warp do Marcos, porque o app de teste não estava na frente.

Outras pendências vistas: o estado "organização selecionada sem projeto selecionado" mostra "Nada aqui ainda…" mesmo quando há projetos (deveria abrir o mais recente ou pedir para escolher); avisos repetidos `Position not set for "workspace_view:tab_bar"` no log depois de remover a barra empilhada (algo ainda se posiciona relativo a ela).

## 12. Fila de ajustes pedida pelo Marcos (21/10/2026, após testar a versão instalada)

Instalado: `v0.2026.10.21.09.03.oss_00` (commit `17ca14277`) em `D:\Program Files\KikitoTerminal`, via `script\windows\bundle.ps1 -CHANNEL oss` (worktree `D:\DEVJUANMARCOS\PROJETOS\KIKITO-WARP-release`). O build de release leva ~1 h e não pode rodar junto com outro build (falta memória).

1. [feito, sem build] Abas novas falhavam com "Falha ao salvar as abas do projeto" e iam para a Home. Causa (log do Kikito instalado): `UNIQUE constraint failed: local_workspace_tab_identities.id` — o snapshot cria identidades com id do banco para todas as abas abertas, e o modelo escolhe `maior id conhecido + 1`, que já pode existir. `claim_tab_identity` (persistence/local_workspace.rs) renumera essas linhas do snapshot (nunca associadas a projeto). Teste `new_project_tab_takes_its_model_id_over_snapshot_created_identities`.
2. [feito, sem build] Painel de abas verticais escondido no workspace (`render_panels(..., true)`): só trilho + sidebar de projetos.
3. [feito, sem build] X da aba no cabeçalho alinhado (vaga fixa na linha, não overlay).
4. [feito, sem build] "+" e "Opções de visualização" no cabeçalho do projeto; o "+" abre o menu de nova sessão (shells, configurações de aba) ancorado nele; o popup perdeu "Exibir como" e "Densidade".
5. [feito, sem build] Todo "nova aba" abre o menu: "+" da linha na sidebar, "⋯ → Nova aba", botão do projeto vazio (`OpenLocalWorkspaceNewTabMenu`, `WorkspaceAction::at_position`).
6. [feito, sem build] Imagem da organização não aparecia no trilho: o `Image` ficava 0×0 dentro da linha; agora 36px com anel do kit.
7. [pendente] Abas criadas pelo menu de sessões devem abrir na pasta do projeto selecionado.
8. [pendente] Tela de Configurações sumiu (abre como aba sem projeto e a ativação manda para a Home): mostrar como tela principal.
9. [pendente] Botões de notificações e de configurações no canto direito do cabeçalho, ao lado dos controles da janela.
10. [pendente] Remover "Terminais sem projeto" da Home (fluxo deixa de existir).
11. [pendente] Testes completos do fluxo de nova aba (cabeçalho, sidebar, menu ⋯, projeto vazio, clique na aba, reabrir o app) com o perfil `parity` usando as configurações do Marcos (abas verticais ligadas, `default_session_mode = "agent"`). A automação precisa da máquina parada (trava de primeiro plano).
12. [pendente] Títulos das abas do cabeçalho seguindo "Título do painel como" (comando/diretório/branch) para o popup de opções ter efeito.

## 13. Fila do dia 21/10 — implementado (compila; falta teste no app)

- Abas novas abrem na pasta do projeto selecionado (`selected_local_project_directory` em `startup_directory.rs`; WSL fica de fora).
- Toda aba nova que não é restauração entra no projeto selecionado (antes só `SingleTerminal`; configs de aba e agente caíam sem dono e a rota mandava para a Home).
- Aba sem projeto não muda mais a tela (`TabActivationRoute::Home` removido).
- Fechar a última aba não fecha a janela: `remove_tab` cria uma aba reserva oculta, fora de qualquer projeto (`add_local_workspace_stand_in_tab`).
- Setting `appearance.local_workspace.tabs_in_sidebar` + botão "Layout das abas" ao lado do Git + comando na paleta. No modo sidebar o painel vertical lista só as abas do projeto selecionado (`local_workspace_lists_tab`).
- Clique direito na aba do header: Renomear (editor inline), mover esquerda/direita, fechar.
- Menus do workspace (`context_menu::menu`) no estilo do `Menu` do app (menu de nova aba).
- Configurações: quando a aba de Configurações está ativa, vira a tela principal com barra própria e botão fechar.
- Botões de notificações e configurações à esquerda dos controles da janela, em todas as telas.
- "Terminais sem projeto" removido da Home.

Pendências: teste completo do fluxo no app (perfil `parity`), menu de clique direito do painel vertical ainda é o do Warp ("Fechar outras abas" afeta outros projetos), clippy, format, commit, push, novo instalador.

## 14. Painel Git: paridade com o VS Code (21/10/2026, noite)

- Paridade visual do painel Git aceita pelo Marcos em 94,0% (média por região, `git_compare.py`); o resto é rasterização de texto.
- Mapeamento completo dos fluxos do VS Code: `12-git-flows-vscode.md` (fases 1–3).
- **Fase 1 (commit `7a9755239`)**: vários repositórios por projeto (varredura até 2 níveis), seção "REPOSITÓRIOS", troca do ativo, branch do rodapé abre a lista de branches.
- **Fase 2 (checkpoint `113e33df4`, wip)**: backend com testes (`discard_all`, `delete_local_branch`, `read_remote_branches`, `checkout_remote_branch`, `merge_branch`, `abort_merge`, `merge_in_progress`, stash push/list/apply/pop/drop, `cherry_pick`, `revert_commit`, `read_commit_files`, `resolve_conflict`, `init_repository`); UI em `source_control/extras.rs` e `refs.rs` (lista de branches refeita com ações no hover e branches remotas), "Descartar todas as alterações" com confirmação, seção STASHES, faixa de merge com "Abortar merge", aceitar nossa/deles em conflitos, menu de contexto do arquivo (copiar caminho, .gitignore, descartar), ações e arquivos do commit no grafo, "Commit e sincronizar", "Inicializar repositório", badge de alterações no botão Git; executor genérico `run_local_workspace_git_task` + `read_local_git_snapshot`.
- Testado no app (perfil `parity`, monorepo/svc-pagamentos): badge, stash guardar/pop, lista de branches, merge limpo (fast-forward). Faltam testar: merge com conflito (stash antes; o clique do merge em feature-conflito não pegou), aceitar lado, abortar, excluir branch, checkout remoto, cherry-pick/reverter, menu do arquivo, descartar tudo, init, commit e sincronizar.
- Pendências: lembrar o repositório ativo entre reinícios (hoje só em memória); grafo exclui `refs/stash` (feito, falta build).
- Fixture: `scratchpad/repos/monorepo/{svc-pagamentos,apps/painel}`; svc-pagamentos tem branches `feature-x` (já mesclada) e `feature-conflito` (conflita no README).

### 14.1 Fase 2 concluída (22/10/2026)

Commits `addfe8d01` e `d0c9ebbbd`. Testados no app: stash guardar/pop, merge limpo, merge com conflito (aceitar deles, abortar), excluir branch, checkout de branch remota, reverter commit, menu do arquivo (.gitignore), descartar tudo, commit e sincronizar (remoto bare em `scratchpad/repos/remotes/svc-pagamentos.git`), repositório ativo lembrado após reiniciar (`%APPDATA%\warp\<perfil>\data\local_workspace_git_repositories.json`). Só "Inicializar repositório" ficou sem teste no app. Mapeamento atualizado em `12-git-flows-vscode.md`; próximo: Fase 3.
