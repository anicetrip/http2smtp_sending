# email http api to smtp
It's used for zero2prod chapter 7, to replace Postmark to your own mail server incase you face with network problem or don't  willing to share your info to postmark. 

It will be updated with the time I learning the this part.
I have't write test now because I found that in chapter 7, it is teaching how to use rwqest and mook email sever, so I decided to leave it here and just wait until I learned how to do it.



API form:
```
curl -i "http://localhost:8080" \
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
## use multi-git 
`git remote set-url --add origin https://`
## scan what's in the head.
```
let key_list = req_headers.keys();
for key in key_list{
    let a = key.as_str();
    println!("{}:{}",a,req_headers.get(a).unwrap().to_str().unwrap());
}
```
