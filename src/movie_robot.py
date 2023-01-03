# encoding=utf-8
# Time：2022/12/9 0:02
# by Yoake

import re
import sys
import os
import time
import json

import requests
from lxml import etree


class DoubanMovie:
    def __init__(self, name, director, actor, release_time, writer, score, genre, region, duration, year, imdb,
                 language, rank_no, src):
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
        self.rank_no = rank_no
        self.src = src


class DoubanMovieRobot():



    def get_config(self,isbulk=None):
        if not os.path.exists('./clc'):
            os.mkdir('./clc')
            os.mkdir('./clc/keys')
        else:
            if not os.path.exists('./clc/keys'):
                os.mkdir('./clc/keys')

        if os.path.exists('./clc/keys/use_key.json'):
            with open('./clc/keys/use_key.json', 'r', encoding='utf-8') as fp:
                config_data = json.load(fp)
                page_id = config_data["pageid"]
                token = config_data["token"]
                cookie = config_data["cookie"]

        else:
            print('未找到配置文件，请先进行配置')
            page_id = input('请输入notion页面id：')
            token = input('请输入操作该页面的机器人token：')
            if isbulk:
                cookie = input("请输入登录豆瓣后的cookie:")
            else:
                cookie = ''
            print("初始化配置成功")
            config_data = {"pageid": page_id, "token": token, "cookie": cookie}
            with open('./clc/keys/use_key.json', 'w', encoding='utf-8') as fp:
                config_data = json.dumps(config_data)
                fp.write(config_data)

        return page_id, token,cookie

    def crawl_movie_info(self, movie_id, cookie=None):

        url = f"https://movie.douban.com/subject/{movie_id}/"
        # 默认不添加cookie，如果请求过多就需要添加防止限制访问
        if cookie == '':
            headers = {
                'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) '
                              'Chrome/108.0.0.0 Safari/537.36',
            }
        else:
            headers = {
                'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) '
                              'Chrome/108.0.0.0 Safari/537.36',
                'Cookie': cookie
            }
        # 发送 HTTP 请求并获取网页内容
        error_count = 1
        while True:
            try:
                response = requests.get(url, headers=headers)
                if response.status_code == 200:
                    html = etree.HTML(response.text)
                    movie = self.parse_response(html)
                    return movie
                else:
                    time.sleep(1)
                    continue

            except requests.exceptions.ConnectionError :
                if error_count <= 10:
                    print(f"\033[0;91;40m获取电影信息过于频繁的请求，正在等待重试...第{error_count}次\033[0m")
                    time.sleep(2)
                    error_count += 1
                    continue
                else:
                    print(f"\033[0;91;40m该电影请求重试超过10次，请检查网络，程序终止！\033[0m")
                    sys.exit()

    @staticmethod
    def parse_response(html):
        # 使用 lxml 库解析网页内容
        # 提取电影名称
        name = html.xpath('//span[@property="v:itemreviewed"]/text()')
        name = name[0]

        # 提取导演
        director = html.xpath('//a[@rel="v:directedBy"]/text()')
        director += [" ", " ", " ", " ", " "]

        # 提取编剧
        writer = html.xpath('//*[@id="info"]/span[2]/span[2]/a/text()')
        writer += [" ", " ", " ", " ", " "]

        # 提取主演
        actor = html.xpath('//a[@rel="v:starring"]/text()')
        actor += [" ", " ", " ", " ", " "]

        # 提取上映时间
        release_time = '\n'.join(html.xpath('//span[@property="v:initialReleaseDate"]/text()'))
        release_time = release_time.strip()

        # 提取评分
        score = ''.join(html.xpath('//strong[@property="v:average"]/text()'))
        score = score.strip()
        if score == '':
            raters = None
            score = None
            rlist = [None, None, None, None, None]
        else:
            score = float(score)
            raters = ''.join(html.xpath('//*[@pclass="rate_people"]/span/text()'))

            score = float(score)
            # raters=int(raters)

        # 提取所在的榜单和排名
        rank_li = ''.join(html.xpath('//*[@class="top250-link"]/a/text()'))
        if rank_li != '':
            rank_no = int(''.join(html.xpath('//span[@class="top250-no"]/text()')).replace('No.', ''))
        else:
            rank_li = ''
            rank_no = None

        # 提取地区
        region = ''.join(html.xpath('//span[./text()="制片国家/地区:"]/following::text()[1]'))
        region = region.split('/')
        region = [i.strip() for i in region]
        region += [" ", " ", " ", " ", " "]

        # 提取语种
        language = ''.join(html.xpath('//span[@class="pl" and text()="语言:"]/following-sibling::text()[1]'))
        language = language.split('/')
        language = [i.strip() for i in language]
        language += [" ", " ", " ", " ", " "]

        # 提取类型
        genre = html.xpath("//span[./text()='类型:']/following::span[@property='v:genre']/text()")
        genre += [" ", " ", " ", " ", " "]

        # 提取时长
        duration = ''.join(html.xpath('//span[@property="v:runtime"]/@content'))
        duration = duration.strip()
        duration = int(duration)

        # 提取年份
        year = "".join(html.xpath('//span[@class="year"]/text()'))
        year = re.findall(r'-?\d+\.?\d*', year)[0]
        year = int(year)

        # 提取IMDB号
        imdb = ''.join(html.xpath('//span[./text()="IMDb:"]/following::text()[1]'))
        imdb = imdb.strip()

        # 提取封面
        src = "".join(html.xpath("//*[@id='mainpic']/a/img/@src"))

        movie = DoubanMovie(name, director, actor, release_time, writer, score, genre, region, duration, year, imdb,
                            language, rank_no, src)
        return movie

    def run(self,fromexternal=None):
        if fromexternal:
            config= self.get_config(True)
            page_id, token= config[:2]
            cookie = config[2]
            for single_id in fromexternal:
                movie= self.crawl_movie_info(single_id,cookie)
                p = {
                    "properties": {
                        "Movie": {"title": [{"type": "text", "text": {"content": movie.name}}]},
                        "Director": {"multi_select": [{"name": "{}".format(movie.director[0])},
                                                      {"name": "{}".format(movie.director[1])},
                                                      {"name": "{}".format(movie.director[2])},
                                                      {"name": "{}".format(movie.director[3])},
                                                      {"name": "{}".format(movie.director[4])}
                                                      ]},
                        "Actor": {"multi_select": [{"name": "{}".format(movie.actor[0])},
                                                   {"name": "{}".format(movie.actor[1])},
                                                   {"name": "{}".format(movie.actor[2])},
                                                   {"name": "{}".format(movie.actor[3])},
                                                   {"name": "{}".format(movie.actor[4])}
                                                   ]},
                        "UpDate": {"rich_text": [{"type": "text", "text": {"content": movie.release_time}}]},
                        "Score": {"number": movie.score},
                        "Region": {"multi_select": [{"name": "{}".format(movie.region[0])},
                                                    {"name": "{}".format(movie.region[1])},
                                                    {"name": "{}".format(movie.region[2])},
                                                    {"name": "{}".format(movie.region[3])},
                                                    {"name": "{}".format(movie.region[4])}]},
                        "Writer": {"multi_select": [{"name": "{}".format(movie.writer[0])},
                                                    {"name": "{}".format(movie.writer[1])},
                                                    {"name": "{}".format(movie.writer[2])},
                                                    {"name": "{}".format(movie.writer[3])},
                                                    {"name": "{}".format(movie.writer[4])}
                                                    ]},
                        "Genre": {"multi_select": [{"name": "{}".format(movie.genre[0])},
                                                   {"name": "{}".format(movie.genre[1])},
                                                   {"name": "{}".format(movie.genre[2])},
                                                   {"name": "{}".format(movie.genre[3])},
                                                   {"name": "{}".format(movie.genre[4])}
                                                   ]},
                        "Cover": {"files": [{"name": "封面", "type": "external", "external": {"url": movie.src}}]},
                        "Language": {"multi_select": [{"name": "{}".format(movie.language[0])},
                                                      {"name": "{}".format(movie.language[1])},
                                                      {"name": "{}".format(movie.language[2])},
                                                      {"name": "{}".format(movie.language[3])},
                                                      {"name": "{}".format(movie.language[4])}
                                                      ]},
                        "Year": {"number": movie.year},
                        "imdb": {"rich_text": [{"type": "text", "text": {"content": movie.imdb}}]},
                        "Duraiton": {"number": movie.duration},
                        "Rank": {"number": movie.rank_no}
                    },
                    "parent": {
                        "type": "database_id",
                        "database_id": page_id
                    }
                }
                headers = {
                    "Accept": "application/json",
                    "Notion-Version": "2022-06-28",
                    "Content-Type": "application/json",
                    "Authorization": "Bearer " + token
                }
                self.read_post(page_id,p,headers,movie)

        else:
            movie_id = input('请输入电影ID：')
            if movie_id == '':
                print('请输入正确的电影ID')
            else:
                movie_id = movie_id
            page_id, token = self.get_config()[:2]
            cookie = ''
            movie = self.crawl_movie_info(movie_id,cookie)
            p = {
                "properties": {
                    "Movie": {"title": [{"type": "text", "text": {"content": movie.name}}]},
                    "Director": {"multi_select": [{"name": "{}".format(movie.director[0])},
                                                  {"name": "{}".format(movie.director[1])},
                                                  {"name": "{}".format(movie.director[2])},
                                                  {"name": "{}".format(movie.director[3])},
                                                  {"name": "{}".format(movie.director[4])}
                                                  ]},
                    "Actor": {"multi_select": [{"name": "{}".format(movie.actor[0])},
                                               {"name": "{}".format(movie.actor[1])},
                                               {"name": "{}".format(movie.actor[2])},
                                               {"name": "{}".format(movie.actor[3])},
                                               {"name": "{}".format(movie.actor[4])}
                                               ]},
                    "UpDate": {"rich_text": [{"type": "text", "text": {"content": movie.release_time}}]},
                    "Score": {"number": movie.score},
                    "Region": {"multi_select": [{"name": "{}".format(movie.region[0])},
                                                {"name": "{}".format(movie.region[1])},
                                                {"name": "{}".format(movie.region[2])},
                                                {"name": "{}".format(movie.region[3])},
                                                {"name": "{}".format(movie.region[4])}]},
                    "Writer": {"multi_select": [{"name": "{}".format(movie.writer[0])},
                                                {"name": "{}".format(movie.writer[1])},
                                                {"name": "{}".format(movie.writer[2])},
                                                {"name": "{}".format(movie.writer[3])},
                                                {"name": "{}".format(movie.writer[4])}
                                                ]},
                    "Genre": {"multi_select": [{"name": "{}".format(movie.genre[0])},
                                               {"name": "{}".format(movie.genre[1])},
                                               {"name": "{}".format(movie.genre[2])},
                                               {"name": "{}".format(movie.genre[3])},
                                               {"name": "{}".format(movie.genre[4])}
                                               ]},
                    "Cover": {"files": [{"name": "封面", "type": "external", "external": {"url": movie.src}}]},
                    "Language": {"multi_select": [{"name": "{}".format(movie.language[0])},
                                                  {"name": "{}".format(movie.language[1])},
                                                  {"name": "{}".format(movie.language[2])},
                                                  {"name": "{}".format(movie.language[3])},
                                                  {"name": "{}".format(movie.language[4])}
                                                  ]},
                    "Year": {"number": movie.year},
                    "imdb": {"rich_text": [{"type": "text", "text": {"content": movie.imdb}}]},
                    "Duraiton": {"number": movie.duration},
                    "Rank": {"number": movie.rank_no}
                },
                "parent": {
                    "type": "database_id",
                    "database_id": page_id
                }
            }
            headers = {
                "Accept": "application/json",
                "Notion-Version": "2022-06-28",
                "Content-Type": "application/json",
                "Authorization": "Bearer " + token
            }
            self.read_post(page_id,p,headers,movie)

    @staticmethod
    def read_post(page_id,p,headers,movie):
        url = "https://api.notion.com/v1/pages"
        while True:
            try:
                r = requests.post(url, json=p, headers=headers)
                if r.status_code == 200:
                    print(f'电影《{movie.name}》导入notion成功！')
                    break
                else:
                    print('导入notion失败！')
                print(r.text.encode('utf-8'))
                check_connection = input('输入000以检查连接')
                if check_connection == "000":
                    check_url = f"https://api.notion.com/v1/databases/{page_id}"
                    r = requests.get(check_url, headers=headers)
                if r.status_code == 200:
                    print('通讯正常，请检查表格内容')
                    break
                else:
                    print('通讯失败，检查使用配置好机器人')
                    break
            except requests.exceptions.ConnectionError:
                print("\033[0;93;101m上传notion过于频繁的请求，正在等待重试...\033[0m")
                continue

if __name__ == "__main__":
    try:
        bot = DoubanMovieRobot()
        bot.run()
    except Exception as ex:
        print(ex)

    finally:
        check = input('请按回车键确认并结束程序')
        if check != 'afsgsfaagsada':
            sys.exit()
