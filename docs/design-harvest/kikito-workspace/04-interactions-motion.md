# Interações e movimento

## Regra de movimento

O kit usa transições curtas em hover e seleção; não usa animação de entrada para menus ou diálogos. Não acrescentar fade, spring, escala ou deslocamento que não esteja no JSX. Movimento detectado: rail marker/tile 120 ms; `ProjectRow`, cards, tabs e menus mudam aparência imediatamente salvo o estado local de hover; botão Git de refresh gira durante o timer da maquete; slider/segmented control do painel Tweaks possui movimento próprio e pertence apenas à ferramenta de autoria.

| Componente | Estado | Resposta do ponteiro | Teclado/saída |
|---|---|---|---|
| RailItem | hover | marcador lateral muda para 12 px; tooltip em 120 ms conforme componente Tooltip; superfície/ícone muda | `cmd+0` Home; `cmd+1..9` organizações até 9; `cmd+,` configurações segundo tooltip |
| Organização | ativa | avatar passa a radius10, opacity1 e ring colorido; marcador ativo 28 px | seleção persiste ao trocar para projeto; remover limpa seleção inválida |
| Tile de organização | botão direito | previne menu do sistema e abre menu relativo ao cursor | Escape fecha; setas mudam seleção; Enter/Space ativa |
| Projeto | hover | fundo foreground5; ações Nova aba e overflow substituem contador/estado | busca por nome/caminho; clique na linha seleciona; botão não deve propagar |
| Tab | hover ou ativa | fundo fg5/fg10 e borda fg15/fg20; X surge | seleção ativa projeto/aba; X fecha a aba sem selecionar outra ação |
| Botão | hover/down | primary usa accent/overlay; naked usa fg10 e press fg15; sem scale | foco visível e shortcut exibido onde definido |
| Campo | focus | borda accent e caret accent | foco inicial no nome; Enter salva quando válido; Escape cancela |
| HomeAsk | foco e resultados | borda Accent; item atual accent25 | ↑/↓, Enter, Escape; não perder a seleção ao renderizar resultados novos |
| Menu | hover | linha inteira accent sólido; disabled mantém cor disabled | Up/Down, Enter/Space, Escape; fecha clique fora |
| Cropper | dragging | cursor grab→grabbing; grid 3×3 aparece | pointer capture mantém arraste até soltar fora |
| Cropper | wheel/slider | zoom mantém ponto sob cursor; clamp impede áreas vazias | limites 1× e 4×; Cancelar retorna ao diálogo |
| Dialog | aberto | backdrop preto 70%; conteúdo fica centrado; sem animação | X/Escape/cancelar; foco inicial no nome; clique dentro não fecha |
| Git row | hover | mostra stage/unstage/discard e preserva status no extremo direito | confirmação para descarte destrutivo |
| Commit | ocupado/sucesso | ícone de refresh gira 0.8s na maquete; notificação dura 2.4 s | produção substitui timers por estado async real e mensagens de resultado |
| Commit graph | hover/expanded | hover fg5; expandido accent25, fundo informativo fg5 | clique expande hash/autor/data/refs/pais |

## Estados que precisam de capturas separadas

Para cada fase, guardar baseline, hover, selecionado, diálogo/menu aberto, disabled, vazio e erro. Capturar com o cursor fora depois do hover para detectar estado preso. Para cropper, capturar zoom mínimo/máximo, centro e pan nos quatro limites. Para Git, capturar staged vazio/preenchido, alterações, working tree limpa, branch atual e grafo expandido. Nenhum screenshot pode ocultar toolbar ou hover por uma posição acidental do cursor.

## Movimento e acessibilidade

O protótipo não define comportamento `prefers-reduced-motion`. A implementação só deve preservar os 120 ms observados, e pode reduzir essa transição respeitando configuração de movimento reduzido. Não modificar animações globais do app. View do menu deve capturar foco; foco visível, ordem de Tab, labels acessíveis e retorno ao acionador após fechar devem ser implementados/testados apesar da limitação de focusability anotada no JSX.

Todos os alvos que hoje dependem de hover devem continuar executáveis por teclado e, quando aplicável, por toque. O botão de overflow deve ter nome acessível. O X da aba deve ser alvo separado e não parte do acionador de seleção. Atualizações async do Git e validação de caminho devem ser anunciadas sem roubar o foco.
