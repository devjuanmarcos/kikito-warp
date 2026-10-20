# Evidências de inspeção

## Pacote de origem

- Arquivo analisado: `C:\Users\Marcos\Downloads\✅ Kikito Terminal.zip`.
- SHA-256: `8CCDB225255F422E72B13915DE815911B62092345B73F31374B98BA6A8D09474`.
- 275 arquivos; 12.694.287 bytes descompactados.
- Conteúdo: 40 JSX, 27 HTML, 27 Markdown, 24 `.d.ts`, 121 SVG, 11 PNG, 10 TTF, 6 CSS, 5 JPG, 2 JSON e bundle JS.
- Viewport: `ui_kits/workspace/index.html` declara `@dsCard viewport="1280x800"`.
- Fonte visual principal: JSX inline e tokens do CSS. Fonte comportamental do protótipo: handlers React nos mesmos arquivos.
- Screenshot complementar VS Code: `uploads/pasted-1792329106410-0.png`; dimensions 366 × 1000 px.

## Estados inspecionados no código

Trilho ativo/hover/inativo, organização com imagem/inicial e badge, modo com nomes; sidebar de busca/sem resultados/sem projetos/ativa/hover/abas aninhadas; tabs ativa/inativa/hover/close; Home tones/launcher/results/cards/recentes/empty; diálogo create/edit/confirmation, imagem e seleção de cor; crop drag/zoom/grid; Git staged/unstaged/clean/commit menu/graph filter/expanded/sync toolbar.

## Captura válida do protótipo

Uma sessão Chromium controlada serviu a extração limpa por HTTP em `127.0.0.1:8731`; nenhum arquivo-fonte foi alterado. A captura ocorreu em viewport CSS de 1280×800, DPR 1, tema escuro, movimento reduzido, contexto isolado sem armazenamento e preset inicial `Jarvis / railNames false / nestedTabs false`.

- Screenshot Home: [reference-home-1280x800.png](../../../output/design-harvest/kikito-workspace/reference-home-1280x800.png), SHA-256 `6a6f48d88f0431dd765128b4785ba94a2ec47d6f5ed592182f2bfa1b8426697c`.
- Evidência controlada: `ev_49641ae7be03581eab1adc2cf794d70442e1484a269dde6315a425cdae6a836e`.
- Relatório estruturado: [inspection-report.json](../../../output/design-harvest/kikito-workspace/inspection-report.json).
- Referência Source Control VS Code: [vscode-source-control-reference.png](../../../output/design-harvest/kikito-workspace/vscode-source-control-reference.png), 366×1000 px.

A captura do protótipo não é uma screenshot original fornecida pelo usuário; ela é uma renderização reproduzível da referência executável. O ZIP não contém screenshot integral renderizado para comparar com o produto. O screenshot inicial de `file://` foi descartado e não entra em nenhuma medição. A captura Home não valida superfícies de projeto nem demonstra paridade do app nativo.

## Critérios da futura sessão de captura

1. Usar cópia extraída em HTTP local, sem editar o conteúdo de referência e sem acessar o perfil instalado do Kikito Terminal.
2. Viewport CSS 1280×800, zoom 100%, DPR documentado, tema escuro `Phenomenon` como padrão; recapturar tema claro separadamente.
3. Congelar horário/dados do fixture e limpar apenas `kt-workspace-v1` em perfil isolado.
4. Ocultar TweaksPanel somente no replay de captura, se ficar visível; registrar preset ativo `Jarvis / railNames false / nestedTabs false`.
5. Guardar screenshot e DOM/box metrics para Home, cada etapa sequencial e estados hover, selected, menu, dialog, crop, Git.
6. Guardar console errors, fontes efetivamente carregadas, assets pendentes e dimensões de cada bloco. Falha externa de CDN invalida a captura.
7. Capturar o app nativo no mesmo viewport e estado; emitir changed-pixel ratio, erro de canal e lista de regiões mascaradas. Todo mask precisa justificar conteúdo dinâmico.

## Resultado desta rodada

O ZIP foi inventariado; JSX, tokens, componentes e a imagem complementar foram lidos. Home tem captura de referência válida e artefatada. A comparação de implementação permanece pendente; nenhuma taxa de paridade do app foi declarada. A captura das outras superfícies segue a ordem serial definida em [plano de implementação](07-implementation-plan.md).
