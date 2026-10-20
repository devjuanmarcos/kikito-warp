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

Próximos passos: (1) estado de organização sem projetos (texto + "Novo projeto" no centro, como o kit); (2) esconder a faixa de abas nativa duplicada; (3) ícones do kit via `Icon` (procurar em `bundled/svg` primeiro); (4) Home (Jarvis, nome, cards); (5) decisões com o Marcos: dados de agente (badge/status e contagem de abas), botão "vincular aba", arrastar arquivo para a imagem (o terminal sob o modal também recebe `DragAndDropFiles`).
