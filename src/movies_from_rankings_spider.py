# encoding=utf-8
# Time：2022/12/9 19:15
# by Yoake

import json
import time
import os

import requests

from movie_robot import DoubanMovieRobot


def get_movie_json(i, cookie):
    if os.path.exists('./clc/localmovies'):
        os.mkdir('./clc/localmovies')
    if os.path.exists(f'./clc/localmovies/{type}movies.json'):
        print('本地已存在相应的数据，从本地读取中...')
        result = json.load(open(f'./clc/localmovies/{type}movies.json', 'r', encoding='utf-8'))
        return result
    else:
        params = {
            'start': (i - 1) * 20,
            'limit': 20
        }
        headers_dict = {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) '
                          'Chrome/104.0.5112.81 Safari/537.36 Edg/104.0.1293.47',
            'cookie': cookie
        }

        url = url_base
        response = requests.get(url, params=params, headers=headers_dict)
        print(response.url)
        content = response.text
        with open(f'./clc/localmovies/{type}movies.json', 'w', encoding='utf-8') as fp:
            fp.write(content)
        result = json.loads(content)
        return result


def get_movies_id(result):
    import jsonpath
    movie_id_list = jsonpath.jsonpath(result, '$..id')

    return movie_id_list


if __name__ == "__main__":
    bot = DoubanMovieRobot()

    start_page = int(input('请输入起始的页码：'))
    end_page = int(input('请输入结束的页码：'))
    count = 1
    index = 1
    # 排行榜的页面的数据

    url_base = f'https://movie.douban.com/j/chart/top_list?type={type}&interval_id=100%3A90&action='

    for item in range(start_page, end_page + 1):
        content=get_movie_json(item,)
        movieid_ls=get_movies_id(content)
        print(f'已获取{count*20}部电影的数据')
        bot.run(movieid_ls)
