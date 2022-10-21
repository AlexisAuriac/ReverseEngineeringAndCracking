# HTTPS

Decrypt HTTPS with wireshark: https://support.f5.com/csp/article/K50557518#OnLinux

Search for something that looks like this:
```
25	7.099731051	192.168.1.13	34.78.251.81	HTTP2	190	HEADERS[1]: GET /key/B42brPAq1KnMB3KVPfI27wtR
```

Get the second key:
```
curl https://keyvalidator.reverse.blackfoot.io/key/B42brPAq1KnMB3KVPfI27wtR
```

Get the flag:
```
curl https://keyvalidator.reverse.blackfoot.io/validate/B42brPAq1KnMB3KVPfI27wtR/b57ab317235d05d9615ee4053357bc700ad7e12e0d3b51ddbd94de04dd796ab5
```
