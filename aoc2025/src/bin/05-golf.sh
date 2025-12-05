sort -n|(while IFS=- read a b;do((b&&(b+=1),b>c&&(t+=b-(a<c?c:a),c=b)))done;echo $t)
