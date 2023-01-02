# encoding=utf-8
# Time：2022/12/9 19:15
# by Yoake

import requests
import movierobot
import json
import time
import os

url_base = 'https://movie.douban.com/j/chart/top_list?type=11&interval_id=100%3A90&action=' #type=11表示剧情分类电影
i = 0
def creat_request(i):
    if os.path.exists('movie'+str(i)+'.json'):
        print('本地已存在相应的数据，从本地读取中...')
        result=json.load(open('movie' + str(i) + '.json','r',encoding='utf-8'))
        time.sleep(7)
        return result
    else:
        params={
            'start': (i - 1) * 20,
            'limit': 20
        }
        # proxies= {'https':'47.104.98.46:16819'}
        headers_dict={
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.81 Safari/537.36 Edg/104.0.1293.47',
            'Cookie':'bid=k6Pj241b3kQ; __gads=ID=d69f77c43a68dba4-2247532544d3009b:T=1652964112:RT=1652964112:S=ALNI_MaIAjWw6nMtMHD0y93OSwmpfISK_g; gr_user_id=957e5ec7-450c-462b-bd51-b7200d864bd2; ll="118269"; _vwo_uuid_v2=D6D2730D8F0A07C4A2ED4BA6242725300|50e1523345e65aa44805473e366d8a00; douban-fav-remind=1; viewed="35917530_25976544_1115748_25740171_26922299_1639355_20498923_26821429_25766679_25783486"; Hm_lvt_561e920ead59795187ebf52984852ce7=1666499160,1666624753,1666792386,1666940691; __utmc=30149280; __utmc=223695111; dbcl2="186788043:C6tEPrHz+SI"; ck=c1xR; _pk_ref.100001.4cf6=%5B%22%22%2C%22%22%2C1670597299%2C%22https%3A%2F%2Faccounts.douban.com%2F%22%5D; _pk_ses.100001.4cf6=*; __utma=30149280.344676693.1652964115.1670591962.1670597299.48; __utmb=30149280.0.10.1670597299; __utmz=30149280.1670597299.48.36.utmcsr=accounts.douban.com|utmccn=(referral)|utmcmd=referral|utmcct=/; __utma=223695111.992594575.1653490285.1670591962.1670597299.22; __utmb=223695111.0.10.1670597299; __utmz=223695111.1670597299.22.14.utmcsr=accounts.douban.com|utmccn=(referral)|utmcmd=referral|utmcct=/; push_noty_num=0; push_doumail_num=0; frodotk_db="e0abf663827035fea203e32a8750a1fa"; __gpi=UID=0000058fdd717e12:T=1652964112:RT=1670597307:S=ALNI_MbO4PnzsgwRIDVbvqmAESGTT_Y0_g; _pk_id.100001.4cf6=354210c1ada82ebc.1653490285.22.1670597977.1670591962.'
        }

        url=url_base
        response=requests.get(url,params=params,headers=headers_dict)
        print(response.url)
        content=response.text
        with open('movie'+str(i)+'.json','w',encoding='utf-8') as fp:
            fp.write(content)
        result=json.loads(content)
        return result

def getmovieid(result):
    import jsonpath
    movieidls=jsonpath.jsonpath(result,'$..id')

    return movieidls

if __name__=="__main__":
    with open('mykey.txt', 'r', encoding='utf-8') as fp:
        page_id = (''.join(fp.readlines(1))).strip('\n')
        # token=
        token = (''.join(fp.readlines(2))).strip('\n')
    start_page = int(input('请输入起始的页码：'))
    end_page = int(input('请输入结束的页码：'))
    count = 1
    index=1
    for item in range(start_page, end_page + 1):
        content=creat_request(item)
        movieid_ls=getmovieid(content)
        print(f'已获取{count*20}部电影的数据')
        for id in movieid_ls:
            movie=movierobot.crawl_movie_info(id)
            print(str(index)+'.'+movie.name)
            movierobot.post_notion(database_id=page_id, token=token, movie=movie)
            index+=1
            time.sleep(2)
        count+=1