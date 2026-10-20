# Tamanhos de tela e comportamento adaptativo

## Evidência do protótipo

O único viewport declarado para `ui_kits/workspace/index.html` é 1280 × 800. Não há breakpoint de mobile ou tablet nas telas do workspace. A moldura tem `width:100%`, `max-width:1400px`, `height:100%`, `max-height:900px`, padding externo 24 e overflow hidden. Trilho de 64 px e sidebar de 248 px são não encolhíveis; o canvas usa `flex:1` e `min-width:0`.

## Regras flexíveis observadas

- Project tab: min 72/max 220 px, flex 1; faixa clip/overflow horizontal.
- Busca da barra: até 240 px, encolhe até 120 px.
- Sidebar Git: clamp entre 300 e 380 px, 34% da coluna disponível.
- Home: max-width 960 px, padding horizontal40 e cards com min 220 px; grade auto-fill.
- Texto longo de projeto, caminho, organização, tab e branch truncado com ellipsis; não empurrar botões fixos.
- Menu e resultados de busca usam posição absoluta dentro do frame e precisam respeitar bordas/clip.

## Regras a preservar e perguntas de validação

1. 1280 × 800 é o baseline obrigatório e único valor diretamente documentado.
2. Em janela estreita, conservar o trilho, permitir sidebar Git fechar, manter alvos de 24/32 px e evitar tab width negativa. Documentar o limite mínimo após medir o app.
3. Tablet e mobile não têm design correspondente no ZIP. Não inventar comportamento como parte de “pixel perfect”; validar no desktop e registrar como decisão futura se for necessário criar layout adaptativo novo.
4. Scale diferente de 100%, alto DPI, janela maximizada, barra de título real e área útil menor devem ser capturados como QA de integração, separados da comparação base 1280×800.
5. Testar com caminhos longos, branch longa, muitos projetos/organizações/abas, lista vazia e conteúdo com acento/Unicode. Os mesmos truncamentos devem ocorrer em ambos os lados.
