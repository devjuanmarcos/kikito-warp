# Critérios de qualidade

1. **Inicialização offline:** o aplicativo abre com rede indisponível e não tenta acessar host externo no fluxo básico; evidência por execução monitorada ou bloqueio de tráfego.
2. **Sem login obrigatório:** abre diretamente no terminal sem página de login, handoff de navegador, refresh de token ou modal repetido.
3. **Uso local preservado:** shell, abas, sessões, preferências e restauração sobrevivem ao encerramento e reinício sem apagar o banco SQLite.
4. **Limites claros dos recursos remotos:** no perfil offline, recursos que dependem de servidores não fazem chamadas ocultas e ficam removidos/desativados com explicação local em pt-BR.
5. **Português completo:** todos os textos visíveis e acessíveis do perfil suportado estão em pt-BR, incluindo menus, onboarding, erros, tooltips, atalhos, diálogos e estados vazios.
6. **Sem dependência Warp no caminho local:** configuração padrão, links e serviços em background não contêm endpoints Warp ativos no perfil offline.
7. **Migração segura:** preferências e dados já existentes são lidos sem exclusão automática; mudanças de formato têm migração e rollback documentados.
