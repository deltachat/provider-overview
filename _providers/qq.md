---
name: QQ Mail
status: OK
domains:
- qq.com
- foxmail.com
server:
  - type: imap
    socket: SSL
    hostname: imap.qq.com
    port: 993
    username_pattern: emaillocalpart
  - type: smtp
    socket: SSL/TLS
    hostname: smtp.qq.com
    port: 465
before_login_hint: "You must create an app-specific password for Delta Chat, like as any other mail clients."
last_checked: 2021-06
website: https://mail.qq.com/
---