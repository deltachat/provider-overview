---
name: Fastmail
status: PREPARATION
domains: 
  - fastmail.com
# Servers list from https://www.fastmail.help/hc/en-us/articles/1500000278342
server:
  - type: imap
    socket: SSL
    hostname: imap.fastmail.com
    port: 993
  - type: smtp
    socket: SSL
    hostname: smtp.fastmail.com
    port: 465
  - type: smtp
    socket: STARTTLS
    hostname: smtp.fastmail.com
    port: 587
before_login_hint: "You must create an app-specific password for Delta Chat before you can log in."
last_checked: 2020-01
website: https://fastmail.com
---

To use Delta Chat with your Fastmail email address
you have to generate a specific password for it.

You can do that in the Fastmail web interface
at "Settings / Account: Password & Security / App Passwords: Manage":

1. Click "New App Password".
2. Click "Generate password". You can optionally select a custom "Name" for this password to make it easier to find later.
3. Enter the 16-digit password shown there into the Delta Chat password field.

Afterwards you can use Delta Chat as usual.

## Custom domains

If your login email does not end with `@fastmail.com`, you will need to manually type server information by clicking "Advanced" in Delta Chat and entering the details specified in the "Technical specs" link below.
