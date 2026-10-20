import sys
def edit(p, pairs):
    raw=open(p,encoding='utf-8',newline='').read()
    crlf = raw.count('\r\n') > raw.count('\n')/2
    s=raw.replace('\r\n','\n')
    for a,b in pairs:
        assert a in s,(p,a[:80])
        s=s.replace(a,b,1)
    if crlf: s=s.replace('\n','\r\n')
    open(p,'w',encoding='utf-8',newline='').write(s)
