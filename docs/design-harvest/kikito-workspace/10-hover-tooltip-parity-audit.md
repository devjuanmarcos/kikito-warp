# Auditoria de paridade: hover, tooltip, animação e modal (rail, sidebar, abas, corpo)

Data: 20/10/2026. Fonte da verdade: `ui_kits/workspace/*.jsx` e `components/*` do zip `✅ Kikito Terminal.zip`.

## Método e limite da medição

- **Análise estática** do código Rust contra o JSX/CSS do zip. Não é diff de pixels: o app não foi aberto nem capturado nesta rodada.
- Cada linha é um item verificável do design. Nota: ✔ = 1, ½ = 0,5, ✘ = 0. O percentual é a soma das notas dividida pelo número de itens da área.
- O número mede **fidelidade ao design**, não qualidade do código. Itens sem equivalente nativo (chrome da janela) ficam fora da conta.
- Depois de cada correção, o placar deve ser refeito com captura pareada 1280×800 antes de qualquer afirmação de 98%.

## Placar

| Área | Itens | Nota | Paridade |
| --- | ---: | ---: | ---: |
| 1. Trilho de organizações | 17 | 9,0 | **53%** |
| 2. Sidebar de projetos | 22 | 12,5 | **57%** |
| 3. Cabeçalho de abas do projeto | 16 | 11,0 | **69%** |
| 4. Corpo do terminal (moldura) | 2 | 1,0 | **50%** (só moldura; blocos nativos não medidos) |
| 5. Modal nova organização | 15 | 6,75 | **45%** |
| 6. Menu de contexto (org/projeto) | 10 | 4,0 | **40%** |
| 7. Tooltips (componente global) | — | — | **0%** (nenhum uso no workspace) |

## Causas-raiz transversais

1. **Sem estado hover/press em botões.** O `Button` do design troca fundo em hover (`fg-overlay-2`) e press (`fg-overlay-3`). Os botões do workspace são `EventHandler` sem estado visual. Correção: `Hoverable` + `MouseStateHandle` criados uma vez e guardados na view.
2. **Sem tooltips.** O design usa `WithTooltip` (pílula invertida, 10px, `surface-tooltip`, atalho em `neutral-4`). O WarpUI já tem `crates/ui_components/src/tooltip.rs` (e `Button::with_tooltip`), mas nenhum item do workspace o usa.
3. **Cores de organização inventadas.** `org_rail.rs` usa RGB fixos (`78,131,211` etc.). O design usa `--ansi-*` do tema (Phenomenon: blue `#3780E9`, green `#1CA05A`, magenta `#BF409D`, yellow `#E5A01A`, cyan `#799C92`, red `#D22D1E`). Devem vir de `theme.terminal_colors()`.
4. **Modal e menu feitos com `Container`/`Text` soltos**, sem os componentes `Button` (primary/secondary/naked), `Dialog` e `Menu`. Por isso botões de 12px com fundo cinza em vez de 32px semibold com borda `neutral-4` e CTA primário.
5. **Sem dado de status de agente** no modelo local: badge vermelho do trilho e glifo de status nas linhas/abas não têm origem.

## 1. Trilho de organizações (53%)

| # | Item do design | Estado no código | Nota |
| - | --- | --- | ---: |
| 1 | 64px, fundo `neutral-1`, borda direita outline | Corrigido nesta sessão | 1 |
| 2 | Tile Home 36px, radius 8 | Imagem sem radius | ½ |
| 3 | Home com opacidade .85 quando inativo | Sempre opaco | 0 |
| 4 | Divisor 24×1 | OK | 1 |
| 5 | Tile 36px, radius 8 (10 ativo) | OK | 1 |
| 6 | Fundo tingido 18% / 32% (ativo) | OK (46/82 de 255) | 1 |
| 7 | Borda 1px (ativo: accent sólido; inativo: tint 30%) | Ativo usa 1,5px | ½ |
| 8 | Sem mudança de borda em hover | Código adiciona borda outline em hover | 0 |
| 9 | Letra 15px bold na cor `--ansi-*` do tema | Tamanho OK, matiz errado (RGB fixo) | ½ |
| 10 | Marcador 3px: 28px ativo, 12px hover, 0 repouso | Geometria OK | 1 |
| 11 | Transição 120ms (marcador, radius, opacidade) | Instantâneo | 0 |
| 12 | Tooltip à direita (10px) com nome e ⌘1..9 / ⌘0 / ⌘, em Início, orgs, Configurações | Inexistente | 0 |
| 13 | Badge vermelho 8px (agente bloqueado) | Inexistente | 0 |
| 14 | `+` tracejado 36px, borda `neutral-4` | Tracejado OK, cor outline (10% em vez de 20%) | ½ |
| 15 | Engrenagem 36px, ícone 16 | OK | 1 |
| 16 | Tile com imagem: anel 1px outline / 1,5px accent, opacidade .85 | Sem anel, sem radius | 0 |
| 17 | Ordem e espaçamento (gap 8; `+` após orgs; engrenagem no rodapé) | OK | 1 |

## 2. Sidebar de projetos (57%)

| # | Item do design | Estado no código | Nota |
| - | --- | --- | ---: |
| a | 248px, `neutral-1`, borda direita | OK | 1 |
| b | Cabeçalho 34px: bolinha 8 ou imagem 18 (radius 4), nome 12 semibold | Só bolinha | ½ |
| c | Botão `+` naked 24px | OK | 1 |
| d | Hover do `+` (fundo `fg-overlay-2`) | Inexistente | 0 |
| e | Tooltip "Novo projeto" | Inexistente | 0 |
| f | Busca: ícone 14, borda 1px fg 20%, raio 4, padding 6/8, placeholder "Buscar projetos..." | Fundo `surface_2` sem borda | ½ |
| g | Foco da busca com borda accent | Inexistente | 0 |
| h | Sobrelinha "PROJETOS" 10px + contagem | OK | 1 |
| i | Linha: margem 8, padding 7/8/7/10, raio 4 | OK | 1 |
| j | Ativa: `fg-overlay-2` + borda `fg-overlay-3` | OK | 1 |
| k | Hover: `fg-overlay-1` | OK | 1 |
| l | Texto: pasta 14, nome 12 (600 ativo), cores main/sub | OK | 1 |
| m | Linha de branch (ícone 10, mono 10, hint) em todo projeto | Branch real só no ativo; demais mostram o caminho | ½ |
| n | Hover troca contagem por `+` e `⋯` (24px, margem −4) | OK | 1 |
| o | Fundo hover/press desses botões | Inexistente | 0 |
| p | Tooltip "Nova aba" | Inexistente | 0 |
| q | Glifo de status do agente + contagem em repouso | Contagem OK, sem glifo | ½ |
| r | Menu `⋯` abre à esquerda do botão | Ancorado no canto da linha | ½ |
| s | Vazio: "Nenhum projeto nesta organização." / sem resultado: "Nenhum resultado encontrado" | Textos diferentes | ½ |
| t | Rodapé: botão secundário small, largura total, ícone + "Novo projeto" | Estático OK | 1 |
| u | Hover/press do botão do rodapé | Inexistente | 0 |
| v | Rolagem fina (`scrollbar-width: thin`) | Scrollbar padrão | ½ |

## 3. Cabeçalho de abas (69%)

| # | Item do design | Estado no código | Nota |
| - | --- | --- | ---: |
| a | Barra 34px, `neutral-1`, borda inferior, padding 8 | OK | 1 |
| b | Aba 26px, min 72, max 220, padding 0/24/0/10, raio 4 | OK | 1 |
| c | Ativa: `fg-overlay-2` + borda `fg-overlay-4` | OK | 1 |
| d | Hover: `fg-overlay-1` + borda `fg-overlay-3` | OK | 1 |
| e | Título 12px, main/sub, reticências | OK | 1 |
| f | Glifo: terminal 12px ou status de agente com anel | Só terminal | ½ |
| g | X 12px no hover/ativa, à direita 4px | OK | 1 |
| h | `+` naked small, tooltip "Nova aba ⌘T" | Ícone OK; sem hover, sem tooltip | ½ (ícone 1; hover 0; tooltip 0 → contado como 3 sub-itens abaixo) |
| i | Pílula de busca 24px, fundo `fg-overlay-1`, ícone 14, hint | Largura por proporção (0,4), não 120–240 | ½ |
| j | Botão Git naked, ativo com `fg-overlay-2`, tooltip "Controle de versão ⌃⇧G" | Ícone e estado ativo OK; sem tooltip/hover | 1 + 0 + 0 |
| k | Rótulo "projeto · organização" abre a Home | OK | 1 |
| l | Títulos reais das abas | OK | 1 |

(Contagem usada no placar: a, b, c, d, e, f, g, h-ícone, h-hover, h-tooltip, i, j-estático, j-tooltip, j-hover, k, l = 16 itens, 11,0 pontos.)

Itens do código sem equivalente no design: botão "vincular aba" (ícone `Link`) e reordenação por setas (removida). Precisam de decisão de produto.

## 4. Corpo do terminal (moldura)

| # | Item | Estado | Nota |
| - | --- | --- | ---: |
| a | Canvas ocupa o resto abaixo da barra, sem borda extra | OK | 1 |
| b | Estado sem aba: texto 14 centralizado "Projeto quietinho. Abro um terminal?" + botão "Nova aba ⌘T" | Texto 13 no canto, sem botão, copy diferente | 0 |
| c | Painel Git à direita ao abrir | Existe; não avaliado aqui | — |

Blocos, prompt, input e zero-state do terminal são renderização **nativa do Warp**, da qual o design foi derivado. Não foram alterados e não foram medidos por falta de captura pareada. Isso exige screenshot nativo vs `reference-*.png`.

## 5. Modal "Nova organização" (45%)

| # | Item do design | Estado no código | Nota |
| - | --- | --- | ---: |
| a | Fundo `blur-backdrop` (preto 70%), clique fora fecha | Depende do `Modal` nativo; não verificado | ½ |
| b | 420px, `surface-1`, borda 1px `neutral-4`, raio 8 | Sem borda/raio no `Container` raiz | ¼ |
| c | Cabeçalho 24/24/12, título 18 semibold | OK | 1 |
| d | Fechar: botão naked 32px, ícone `x-close` 16 + keycap ESC | Texto "×" em 24px | 0 |
| e | Descrição 12px sub | OK | 1 |
| f | Rótulo 12px main, gap 6 | OK | 1 |
| g | Input: padding 10, borda fg 20%, raio 4, 12px, foco accent | `TextInput` de 36px sem borda definida | ½ |
| h | Campo Imagem: tile 56px raio 12 tingido, inicial 22px, botão "Escolher imagem" small com ícone upload, dica "PNG, JPG ou SVG. Arraste um arquivo para cá." | Botão cinza "Escolher imagem..." | 0 |
| i | Cores: 6 pontos 16px, anel 2px accent fora (20px total), gap 6 | Anel interno em cor de texto, gap 8, matizes fixos | ½ |
| j | Rodapé: padding 12/24, borda superior `neutral-4`, botões à direita gap 8 | Borda com cor outline | ½ |
| k | Cancelar secundário 32px; Criar primário (accent) 14px semibold com keycap ⏎, desabilitado sem nome | Ambos cinza 12px | 0 |
| l | Hover/press dos botões | Inexistente | 0 |
| m | Recorte: janela 240px, saída 128px, zoom 1–4×, pan | Pré-visualização 128px com −/+ | 0 |
| n | Textos pt-BR do design | OK | 1 |
| o | Enter cria, Esc fecha | Parcial (Esc/Enter dependem do foco) | ½ |

## 6. Menu de contexto (40%)

| # | Item do design | Estado | Nota |
| - | --- | --- | ---: |
| a | Largura 186 | OK | 1 |
| b | `surface-2`, borda outline, raio 5 | OK | 1 |
| c | Padding vertical 9 | 4 | 0 |
| d | Item padding 5/14, gap 8 | Padding 5 uniforme | 0 |
| e | Hover = preenchimento sólido accent na linha inteira | Só destaque de teclado em `surface_3`; sem hover de mouse | 0 |
| f | Ícone 14 + rótulo 12px | OK | 1 |
| g | Perigo em `--ansi-red` | `ui_error_color` (#BC362A) | ½ |
| h | Separador 1px, margem 4/0, largura total | Margem lateral 10 | ½ |
| i | Sombra overlay `0 8 24 rgba(0,0,0,.4)` | Sem sombra | 0 |
| j | Atalho à direita ("ctrl+shift+g") | Inexistente | 0 |

## 7. Animações

O README do design diz que o chrome é essencialmente instantâneo. Exceção declarada no workspace: **transição de 120ms** no marcador do trilho, no raio e na opacidade do tile. Hoje tudo é instantâneo. Reproduzir exige animação por frame no WarpUI (sem equivalente a `transition` de CSS). Prioridade baixa; o resto (hover, menus, abas) é instantâneo no design também.

## Ordem de correção proposta

1. Infra de hover/press para botões naked/secundário/primário (um único helper em `ws_style.rs`, `MouseStateHandle` guardado na view).
2. Tooltips (`ui_components::Tooltip`) no trilho, `+` do cabeçalho, `+`/`⋯` da linha, `+` das abas e botão Git.
3. Cores `--ansi-*` do tema para organizações e pontos.
4. Menu: padding 9, itens 5/14, hover accent, sombra, atalhos.
5. Modal: botões primário/secundário, X com ESC, campo de imagem 56px, pontos com anel, borda `neutral-4`.
6. Sidebar: imagem da organização, borda/foco da busca, textos vazios.
7. Estado sem aba do projeto.
8. Badge/status de agente: precisa decisão de dados antes.

## Resultados medidos (captura pareada, 1280×800, tema dark, perfil isolado)

Método: o app (`warp-oss`, build debug, `WARP_DATA_PROFILE=parity`) e o protótipo do zip (Chrome, `data-theme="dark"`) foram capturados nos mesmos estados; as regiões foram recortadas e comparadas pixel a pixel (tolerância de canal 10). Estados de hover/press usam mouse real.

### Item 1 — hover e press dos botões: **95,7%** (fechado)

| Medida | Resultado |
| --- | ---: |
| Fundo em repouso, hover e press (header `+`, `+` das abas, Git, rodapé) | 98,7% |
| Botões naked com glifo (header `+`, `+` das abas, Git) | 96,6% (Git 94%: o ícone do design é outro SVG) |
| Botão secundário do rodapé com texto | 89% (texto do Chrome é 2px mais largo; renderização de fonte) |
| Cursor de mão e persistência do estado de hover entre quadros | implementado (`MouseStateHandle` por chave) |

Efeitos colaterais corrigidos no caminho: queda do app ao abrir uma organização (`Flex` com tamanho máximo dentro da lista rolável), altura da linha mudando em hover, meio pixel de desalinhamento nos cabeçalhos de 34px, hover preso nas linhas.

### Item 2 — tooltips: em andamento

| Medida | Resultado |
| --- | --- |
| Cor, borda, raio, fonte 10px | iguais (fundo 157 vs 155 de 255) |
| Posição (tooltip à direita do trilho, +10px do tile; acima e à direita do `+` da linha) | dentro de 1–2px |
| Tamanho dos tooltips só com rótulo ("Nova aba", "Nova organização") | 56×19 vs 56×20 e 93×20 vs 94×19 |
| Pixels do tooltip com rótulo, alinhado, tolerância 24 | 73–78% (texto em escala de cinza vs sub-pixel do Chrome; bordas superior e inferior com antialias por posição fracionária) |
| Atalhos nos tooltips (⌘0, ⌘1–9, ⌘T, ⌃⇧G, ⌘,) | não implementados: ⌘1–9 já trocam de aba no produto |
| Tooltips acima dos botões do cabeçalho | no design ficam cortados pela moldura e não aparecem; no app aparecem abaixo do botão |
