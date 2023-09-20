#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import json
import re
import subprocess

def main():
    print("hi")
    with open("test.txt", 'r') as txt_file:
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
                cur = get_keyboard(cur)
                arr.append(cur)
                cur = {}
            
    data["poets"] = arr

    with open('poet.json', 'w', encoding='utf-8') as json_file:
        json.dump(data, json_file, ensure_ascii=False, indent=4)

def get_zhuyin(cur):
    result = subprocess.run(['python3', 'bopomofo/main.py', cur["content"]], stdout=subprocess.PIPE, text=True)
    text = result.stdout.strip()
    text_without_spaces = re.sub(r'\s', '', text)
    cleaned_text = re.sub(r'[，。]+', ' ', text_without_spaces)
    cur["zhuyin"] = cleaned_text
    return cur

def get_keyboard(cur):
    cur["keyboard"] = "qwe"
    return cur

if __name__ == "__main__":
    main()
