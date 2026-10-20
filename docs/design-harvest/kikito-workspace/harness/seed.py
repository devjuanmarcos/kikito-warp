import sqlite3, time
db = r'C:\Users\Marcos\AppData\Local\warp\WarpOss-parity\data\warp.sqlite'
c = sqlite3.connect(db)
c.execute('PRAGMA foreign_keys=OFF')
for t in ('local_workspace_project_tabs', 'local_workspace_projects', 'local_workspace_organizations'):
    c.execute(f'DELETE FROM {t}')
orgs = [(1, 'Keener', 0, 'blue'), (2, 'Pessoal', 1, 'green'), (3, 'Freelas', 2, 'magenta')]
for i, n, o, col in orgs:
    c.execute('INSERT INTO local_workspace_organizations (id,name,sort_order,accent_color,image_data) VALUES (?,?,?,?,NULL)', (i, n, o, col))
now = int(time.time() * 1000)
projects = [
    (1, 1, 'keener-api', r'D:\DEVJUANMARCOS\PROJETOS\KIKITO WARP', 0, now - 4 * 60000),
    (2, 1, 'keener-web', r'D:\DEVJUANMARCOS', 1, now - 55 * 60000),
    (3, 1, 'infra', r'C:\Users\Marcos\Downloads', 2, now - 26 * 3600000),
    (4, 2, 'kikito-terminal', r'D:\DEVJUANMARCOS\PROJETOS', 0, now - 12 * 60000),
    (5, 3, 'loja-ana', r'C:\Users\Marcos\Documents', 0, now - 5 * 3600000),
]
for i, o, n, p, s, ts in projects:
    c.execute('INSERT INTO local_workspace_projects (id,organization_id,name,path,path_key,sort_order,last_opened_ts) VALUES (?,?,?,?,?,?,?)', (i, o, n, p, p.lower(), s, ts))
c.execute('UPDATE local_workspace_state SET selected_organization_id=1, selected_project_id=1 WHERE id=1')
c.commit()
print(c.execute('select count(*) from local_workspace_projects').fetchone())
