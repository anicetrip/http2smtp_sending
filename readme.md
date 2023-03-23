# email http api to smtp
It's used for zero2prod chapter 7, to replace Postmark to your own mail server incase you face with network problem or don't  willing to share your info to postmark. 

It will be updated with the time I learning the this part .



API form:
```
curl "http://localhost:8080" \
-X POST \
-H "Accept: application/json" \
-H "Content-Type: application/json" \
-H "X-Postmark-Server-Token: <password>" \
-d '{
"From": "a@send.com",
"To": "b@receive.com",
"Subject": "From html2stmp",
"TextBody": "The program success.",
"HtmlBody": "<html><body><strong>The program</strong>  success.</body></html>"
}'

```



lisene: MIT or GPL.

# useful things
`git remote set-url --add origin https://`