"""Rewrites the scratch copy of the kit's WorkspaceData.jsx with the native parity seed, so the
Home can be compared on layout alone. usage: match_kit.py <WorkspaceData.jsx>"""
import re
import sys

path = sys.argv[1]
source = open(path, encoding='utf-8').read()

parts = ['AppData', 'Local', 'Temp', 'claude', 'D--DEVJUANMARCOS-PROJETOS-KIKITO-WARP',
         '594f3941-77bf-483a-a5e1-364a7ec5d42c', 'scratchpad', 'repos']
js_sep = '\\\\'  # one escaped backslash inside a JS string literal
base = '~' + js_sep + js_sep.join(parts) + js_sep


def project(key, name, branch, opened):
    return (f"{{id:'{key}',name:'{name}',path:'{base}{name}',branch:'{branch}',"
            f"opened:{opened},tabs:[]}}")


seed = (
    "const WS_SEED = [\n"
    f"  {{id:'keener',name:'Keener',color:'blue',projects:["
    f"{project('keener-api', 'keener-api', 'main', 'm(4)')},"
    f"{project('keener-web', 'keener-web', 'feat/checkout', 'm(55)')},"
    f"{project('keener-infra', 'infra', 'main', 'm(60*26)')}]}},\n"
    f"  {{id:'pessoal',name:'Pessoal',color:'green',projects:["
    f"{project('kikito', 'kikito-terminal', 'main', 'm(12)')}]}},\n"
    f"  {{id:'freelas',name:'Freelas',color:'magenta',projects:["
    f"{project('loja-ana', 'loja-ana', 'main', 'm(60*5)')}]}}\n"
    "];"
)
start = source.index('const WS_SEED = [')
end = source.index('];', start) + 2
source = source[:start] + seed + source[end:]

jarvis = re.compile(r"  return \{title:wsGreeting\(\)\+', Juan\. Tudo pronto\.',sub:.*?,ask:")
assert jarvis.search(source), 'Jarvis copy not found'
source = jarvis.sub(
    "  return {title:wsGreeting()+'. Tudo pronto.',sub:'Retome de onde parou.',ask:", source, count=1)
open(path, 'w', encoding='utf-8').write(source)
print('ok')
