# encoding=utf-8
# Time：2022/12/9 19:15
# by Yoake

import json
import time
import os
import sys

import requests
import jsonpath

from movie_robot import DoubanMovieRobot


def get_movie_json(page, cookie=''):
    if not os.path.exists('./clc/localmovies'):
        os.mkdir('./clc/localmovies')
    if os.path.exists(f'./clc/localmovies/{type}movies{page}.json'):
        print('本地已存在相应的数据，从本地读取中...')
        result = json.load(open(f'./clc/localmovies/{type}movies{page}.json', 'r', encoding='utf-8'))
        return result
    else:
        print('本地没有相应的数据，从网页拉取数据')
        params = {
            'start': (page - 1) * 20,
            'limit': 20
        }
        headers_dict = {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) '
                          'Chrome/104.0.5112.81 Safari/537.36 Edg/104.0.1293.47',
            'cookie': cookie
        }

        url = url_base
        error_count = 1
        while True:
            try:
                response = requests.get(url, params=params, headers=headers_dict)
                if response.status_code==200:
                    content = response.text
                    with open(f'./clc/localmovies/{type}movies{page}.json', 'w', encoding='utf-8') as fp:
                        fp.write(content)
                    result = json.loads(content)
                    return result
                else:
                    time.sleep(1)
                    continue
            except requests.exceptions.ConnectionError:
                if error_count <= 10:
                    print(f"\033[0;91;40m请求获取电影链接失败，正在等待重试...第{error_count}次\033[0m")
                    time.sleep(2)
                    error_count += 1
                    continue
                else:
                    print(f"\033[0;91;40m该页请求重试超过10次，程序终止，请检查网络！\033[0m")
                    sys.exit()


def get_movies_id(result):

    movie_id_list = jsonpath.jsonpath(result, '$..id')

    return movie_id_list


if __name__ == "__main__":

    bot = DoubanMovieRobot(True)
    cookie = bot.cookie
    start_page = int(input('请输入起始的页码（每页20部电影）：'))
    end_page = int(input('请输入结束的页码：'))

    index = 1
    total = (end_page - start_page + 1)*20
    # 排行榜的页面的数据
    type = input("请输入排行榜的type：")
    url_base = f'https://movie.douban.com/j/chart/top_list?type={type}&interval_id=100%3A90&action='
    for page in range(start_page, end_page + 1):
        content = get_movie_json(page,cookie)
        movieid_ls = get_movies_id(content)
        print(f'已获取到20部电影的数据')
        bot.run(movieid_ls)
