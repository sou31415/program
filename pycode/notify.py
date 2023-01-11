import requests
from  bs4 import BeautifulSoup

TOKEN = "E5jHCsd70okYgYG6WmCoJ3xxb4IF0wRRqzhjE1WFaTS"
api_url = "https://notify-api.line.me/api/notify"
send_contents = "https://forms.gle/2iPTW6X4XjHCu4ar7"

TOKEN_dic = {"Authorization" : "Bearer" + " " + TOKEN}
send_dic = {"message" : send_contents}
print(TOKEN_dic)
print(send_dic)

requests.post(api_url , headers = TOKEN_dic , data = send_dic)