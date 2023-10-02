#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import json
import re
import subprocess

def main():
    print("hi")
    bopomofo_to_keyboard = {
        'ㄅ': '1',
        'ㄆ': 'q',
        'ㄇ': 'a',
        'ㄈ': 'z',
        'ㄉ': '2',
        'ㄊ': 'w',
        'ㄋ': 's',
        'ㄌ': 'x',
        'ㄍ': 'e',
        'ㄎ': 'd',
        'ㄏ': 'c',
        'ㄐ': 'r',
        'ㄑ': 'f',
        'ㄒ': 'v',
        'ㄓ': '5',
        'ㄔ': 't',
        'ㄕ': 'g',
        'ㄖ': 'b',
        'ㄗ': 'y',
        'ㄘ': 'h',
        'ㄙ': 'n',
        'ㄧ': 'u',
        'ㄨ': 'j',
        'ㄩ': 'm',
        'ㄚ': '8',
        'ㄛ': 'i',
        'ㄜ': 'k',
        'ㄝ': ',',
        'ㄞ': '9',
        'ㄟ': 'o',
        'ㄠ': 'l',
        'ㄡ': '.',
        'ㄢ': '0',
        'ㄣ': 'p',
        'ㄤ': ';',
        'ㄥ': '/',
        'ㄦ': '-',
        '˙': '7',
        'ˊ': '6',
        'ˇ': '3',
        'ˋ': '4',
        '-': ' ',
    }

    with open("poet300.txt", 'r') as txt_file:
        data = {}
        data["count"] = 300
        arr = []
        cur = {}

        for line in txt_file:
            line = line.strip()

            if "詩名" in line:
                key, value = line.strip().split(":")
                cur["name"] = value

            if "作者" in line:
                key, value = line.strip().split(":")
                cur["author"] = value

            if "詩文" in line:
                key, value = line.strip().split(":")
                result = re.sub(r'\([^)]*\)', '', value)
                #result = value
                cur["content"] = result
                cur = get_zhuyin(cur)
                cur = get_keyboard(cur, bopomofo_to_keyboard)
                arr.append(cur)
                cur = {}
            
    data["poets"] = arr

    with open('poet300.json', 'w', encoding='utf-8') as json_file:
        json.dump(data, json_file, ensure_ascii=False, indent=4)

def get_zhuyin(cur):
    result = subprocess.run(['python3', 'bopomofo/main.py', cur["content"]], stdout=subprocess.PIPE, text=True)
    text = result.stdout.strip()
    text_without_spaces = re.sub(r'\s', '', text)
    cleaned_text = re.sub(r'[，。]+', ' ', text_without_spaces)
    cur["zhuyin"] = cleaned_text
    return cur

def get_keyboard(cur, mapping):
    result = ""
    for char in cur["zhuyin"]:
        if char in mapping:
            result += mapping[char]
        else:
            result += char  # 如果找不到映射，保留原字符

    cur["keyboard"] = result
    return cur


if __name__ == "__main__":
    main()
