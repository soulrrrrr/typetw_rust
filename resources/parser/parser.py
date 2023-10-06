#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import json
import re
import subprocess

def generate_json_file(poems):
    # Extract poems with the style "五言絕句" including title, author, style, and content
    five_character_quatrains_details = []

    zhuyin_to_keyboard = {
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
    
    for poem in poems:
        if "詩體:五言絕句" in poem:
            # Extract the poem title, author, style and content
            title = poem.split("詩名:")[1].split("\n")[0].strip()
            author = poem.split("作者:")[1].split("\n")[0].strip()
            style = poem.split("詩體:")[1].split("\n")[0].strip()
            content = poem.split("詩文:")[-1].strip()
            
            # Remove the rhyme part in brackets if present
            content = content.split(')')[-1] if '(' in content else content
            
            zhuyin = get_zhuyin(content)
            keyboard = get_keyboard(zhuyin, zhuyin_to_keyboard)
            poem_details = {
                "title": title,
                "author": author,
                "style": style,
                "content": content,
                "zhuyin": zhuyin,
                "keyboard": keyboard
            }
            five_character_quatrains_details.append(poem_details)


    data = {
        "count" : len(five_character_quatrains_details),
        "poems": five_character_quatrains_details
    }


    # Save the extracted details into a JSON file
    with open("./poems_5.json", "w", encoding="utf-8") as file:
        json.dump(data, file, ensure_ascii=False, indent=4)


def get_zhuyin(content):
    result = subprocess.run(['python3', 'bopomofo/main.py', content], stdout=subprocess.PIPE, text=True)
    text = result.stdout.strip()
    text_without_spaces = re.sub(r'\s', '', text)
    cleaned_text = re.sub(r'[，。]+', ' ', text_without_spaces)
    return cleaned_text

def get_keyboard(zhuyin, mapping):
    result = ""
    for char in zhuyin:
        if char in mapping:
            result += mapping[char]
        else:
            result += char  # 如果找不到映射，保留原字符

    return result


def main():
    # Load poems from the provided poet300.txt file
    with open("./poet300.txt", "r", encoding="utf-8") as file:
        content = file.read()
        poems = content.split("\n\n")

    # Generate the JSON file using the loaded poems
    generate_json_file(poems)

if __name__ == "__main__":
    main()


