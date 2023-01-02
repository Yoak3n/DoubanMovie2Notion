# encoding=utf-8
# Time：2022/12/9 0:02
# by Yoake

import re
import sys
import requests
import os
from lxml import etree

if os.path.exists('mykey.txt') :
    with open('mykey.txt','r',encoding='utf-8') as fp:
        page_id=(''.join(fp.readlines(1))).strip('\n')
        # token=
        token=(''.join(fp.readlines(2))).strip('\n')
else:
    page_id = '1f0e2952197e4c71bf35b6726f418b4e'
    token = 'secret_2ma7sspxZAchUs28j8smkuHARL2N38s7xUhozqz1TKI'

class Movie:
    def __init__(self, name, director, actor, release_time, writer, score, genre, region, duration, year, imdb,language,rank_no,src):
        self.name = name
        self.director = director
        self.actor = actor
        self.release_time = release_time
        self.writer = writer
        self.score = score
        self.genre = genre
        self.region = region
        self.duration = duration
        self.year = year
        self.imdb = imdb
        self.language = language
        self.rank_no=rank_no
        self.src=src


def crawl_movie_info(movie_id):

    url = f"https://movie.douban.com/subject/{movie_id}/"

    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36',
        'Cookie': 'bid=k6Pj241b3kQ; __gads=ID=d69f77c43a68dba4-2247532544d3009b:T=1652964112:RT=1652964112:S=ALNI_MaIAjWw6nMtMHD0y93OSwmpfISK_g; gr_user_id=957e5ec7-450c-462b-bd51-b7200d864bd2; ll="118269"; _vwo_uuid_v2=D6D2730D8F0A07C4A2ED4BA6242725300|50e1523345e65aa44805473e366d8a00; douban-fav-remind=1; viewed="35917530_25976544_1115748_25740171_26922299_1639355_20498923_26821429_25766679_25783486"; Hm_lvt_561e920ead59795187ebf52984852ce7=1666499160,1666624753,1666792386,1666940691; __utmc=30149280; __utmc=223695111; dbcl2="186788043:C6tEPrHz+SI"; ck=c1xR; _pk_ref.100001.4cf6=%5B%22%22%2C%22%22%2C1670597299%2C%22https%3A%2F%2Faccounts.douban.com%2F%22%5D; _pk_ses.100001.4cf6=*; __utma=30149280.344676693.1652964115.1670591962.1670597299.48; __utmb=30149280.0.10.1670597299; __utmz=30149280.1670597299.48.36.utmcsr=accounts.douban.com|utmccn=(referral)|utmcmd=referral|utmcct=/; __utma=223695111.992594575.1653490285.1670591962.1670597299.22; __utmb=223695111.0.10.1670597299; __utmz=223695111.1670597299.22.14.utmcsr=accounts.douban.com|utmccn=(referral)|utmcmd=referral|utmcct=/; push_noty_num=0; push_doumail_num=0; frodotk_db="e0abf663827035fea203e32a8750a1fa"; __gpi=UID=0000058fdd717e12:T=1652964112:RT=1670597307:S=ALNI_MbO4PnzsgwRIDVbvqmAESGTT_Y0_g; _pk_id.100001.4cf6=354210c1ada82ebc.1653490285.22.1670597977.1670591962.'
    }
    # 发送 HTTP 请求并获取网页内容
    response = requests.get(url,headers=headers)
    html = response.text

     # 使用 lxml 库解析网页内容
    html = etree.HTML(html)

    # 提取电影名称
    name = html.xpath('//span[@property="v:itemreviewed"]/text()')
    name=name[0]

    # 提取导演
    director = html.xpath('//a[@rel="v:directedBy"]/text()')
    director+=[" "," "," "," "," "]

    # 提取编剧
    writer=html.xpath('//*[@id="info"]/span[2]/span[2]/a/text()')
    writer+= [" "," "," "," "," "]

    # 提取主演
    actor = html.xpath('//a[@rel="v:starring"]/text()')
    actor += [" "," "," "," "," "]

    # 提取上映时间
    release_time = '\n'.join(html.xpath('//span[@property="v:initialReleaseDate"]/text()'))
    release_time=release_time.strip()


    # 提取评分
    score=''.join(html.xpath('//strong[@property="v:average"]/text()'))
    score=score.strip()
    if score =='':
        raters=None
        score=None
        rlist=[None,None,None,None,None]
    else:
        score=float(score)
        raters=''.join(html.xpath('//*[@pclass="rate_people"]/span/text()'))

        score=float(score)
        # raters=int(raters)

    #提取所在的榜单和排名
    rank_li=''.join(html.xpath('//*[@class="top250-link"]/a/text()'))
    if rank_li !='':
        rank_no=int(''.join(html.xpath('//span[@class="top250-no"]/text()')).replace('No.',''))
    else:
        rank_li=''
        rank_no=None

    #提取地区
    region=''.join(html.xpath('//span[./text()="制片国家/地区:"]/following::text()[1]'))
    region=region.split('/')
    region=[i.strip() for i in region]
    region+=[" "," "," "," "," "]


    #提取语种
    language=''.join(html.xpath('//span[@class="pl" and text()="语言:"]/following-sibling::text()[1]'))
    language=language.split('/')
    language=[i.strip() for i in language]
    language+= [" "," "," "," "," "]

    #提取类型
    genre = html.xpath("//span[./text()='类型:']/following::span[@property='v:genre']/text()")
    genre+=[" "," "," "," "," "]

    #提取时长
    duration=''.join(html.xpath('//span[@property="v:runtime"]/@content'))
    duration=duration.strip()
    duration=int(duration)

    #提取年份
    year="".join(html.xpath('//span[@class="year"]/text()'))
    year=re.findall(r'-?\d+\.?\d*',year)[0]
    year=int(year)


    #提取IMDB号
    imdb=''.join(html.xpath('//span[./text()="IMDb:"]/following::text()[1]'))
    imdb=imdb.strip()

    # 提取封面
    src = "".join(html.xpath("//*[@id='mainpic']/a/img/@src"))


    movie = Movie(name, director, actor, release_time,writer,score,genre,region,duration,year,imdb,language,rank_no,src)
    return movie



def post_notion(database_id,token,movie):
    url="https://api.notion.com/v1/pages"

    p = {
        "properties": {
            "Movie": {"title": [{"type": "text", "text": {"content": movie.name}}]},
            "Director": {"multi_select": [{"name": "{}".format(movie.director[0])},
                                         {"name": "{}".format(movie.director[1])},
                                         {"name": "{}".format(movie.director[2])},
                                         {"name": "{}".format(movie.director[3])},
                                         {"name": "{}".format(movie.director[4])}
                                         ]},
            "Actor":{"multi_select":[{"name": "{}".format(movie.actor[0])},
                                    {"name": "{}".format(movie.actor[1])},
                                    {"name": "{}".format(movie.actor[2])},
                                    {"name": "{}".format(movie.actor[3])},
                                    {"name": "{}".format(movie.actor[4])}
                                    ]},
            "UpDate":{"rich_text":[{"type":"text","text":{"content":movie.release_time}}]},
            "Score":{"number":movie.score},
            "Region":{"multi_select": [{"name":"{}".format(movie.region[0])},
                                      {"name":"{}".format(movie.region[1])},
                                      {"name":"{}".format(movie.region[2])},
                                      {"name":"{}".format(movie.region[3])},
                                      {"name":"{}".format(movie.region[4])}]},
            "Writer":{"multi_select":[{"name": "{}".format(movie.writer[0])},
                                     {"name": "{}".format(movie.writer[1])},
                                     {"name": "{}".format(movie.writer[2])},
                                     {"name": "{}".format(movie.writer[3])},
                                     {"name": "{}".format(movie.writer[4])}
                                     ]},
            "Genre":{"multi_select":[{"name": "{}".format(movie.genre[0])},
                                    {"name": "{}".format(movie.genre[1])},
                                    {"name": "{}".format(movie.genre[2])},
                                    {"name": "{}".format(movie.genre[3])},
                                    {"name": "{}".format(movie.genre[4])}
                                    ]},
            "Cover": {"files": [{"name": "封面", "type": "external", "external": {"url": movie.src}}]},
            "Language":{"multi_select":[{"name": "{}".format(movie.language[0])},
                                       {"name": "{}".format(movie.language[1])},
                                       {"name": "{}".format(movie.language[2])},
                                       {"name": "{}".format(movie.language[3])},
                                       {"name": "{}".format(movie.language[4])}
                                       ]},
            "Year":{"number":movie.year},
            "imdb":{"rich_text":[{"type":"text","text":{"content":movie.imdb}}]},
            "Duraiton":{"number":movie.duration},
            "Rank":{"number":movie.rank_no}
        },
        "parent": {
            "type": "database_id",
            "database_id": database_id
        }
    }
    headers = {
        "Accept": "application/json",
        "Notion-Version": "2022-06-28",
        "Content-Type": "application/json",
        "Authorization": "Bearer " + token
    }
    r = requests.post(url, json=p, headers=headers)
    if r.status_code==200:
        print(f'电影《{movie.name}》导入notion成功！')
    else:
        print('导入notion失败！')
        print(r.text.encode('utf-8'))
        check=str(input('输入000以检查连接'))
        if check=="000":
            url=f"https://api.notion.com/v1/databases/{database_id}"
            headers={
                "Accept": "application/json",
                "Notion-Version": "2022-06-28",
                "Content-Type": "application/json",
                "Authorization": "Bearer " + token
            }
        r = requests.get(url,headers=headers)
        if r.status_code==200:
            print('通讯正常，请检查表格内容')
        else:
            print('通讯失败，检查使用配置好机器人')
    return


if __name__=="__main__":
    try:
        movie_id = input('请输入电影ID：')
        if movie_id=='':
            print('请输入正确的电影ID')
        else:
            movie_id=movie_id
        movie = crawl_movie_info(movie_id)
        post_notion(database_id=page_id,token=token,movie=movie)

# 输出电影信息
    except Exception as ex :
        print(ex)

    finally:
        check = input('请按回车键确认并结束程序')
        if check=='':
            sys.exit()