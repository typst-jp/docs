import argparse
import openai
import tiktoken

def translate_markdown(api_key, model, base_url, sysprompt, input_file, last_output_file=None):
    # OpenAI API の認証情報
    openai.api_key = api_key
    openai.base_url = base_url

    # Markdown ファイルを読み込む
    with open(input_file, 'r', encoding='utf-8') as file:
        content = file.read()

    # 途中で切れた前回の出力を Markdown から読み込む
    if last_output_file is not None:
        with open(last_output_file, 'r', encoding='utf-8') as file:
            last_output = file.read()
    
    prompt_template = f"""
以下は英語の技術文書の一部です。これを日本語に翻訳してください。ただし、次のsystem promptの要求に注意してください: {content}
"""
    messages = [
            {"role": "system", "content": sysprompt},
            {"role": "user", "content": prompt_template}
        ]
    if last_output_file is not None:
        # see: https://zenn.dev/lambta/articles/1124a6aa22ff91
        messages.append({"role": "assistant", "content": last_output})

    # OpenAI API の呼び出し
    stream = openai.chat.completions.create(
        model=model,
        messages=messages,
        stream=True,
        max_tokens=4096,
    )
    response = ""
    for chunk in stream:
        if chunk.choices[0].delta.content is not None:
          print(chunk.choices[0].delta.content, end="")
          response += chunk.choices[0].delta.content
    # overwrite the original file
    # with open(input_file, 'w', encoding='utf-8') as file:
    #     file.write(response)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Translate Markdown files using OpenAI API.")
    parser.add_argument("-k", "--api-key", required=True, help="OpenAI API key")
    parser.add_argument("-m", "--model", default="gpt-3.5-turbo-16k", help="Model to be used for translation. gpt-3.5-turbo-16k and gpt-4-1106-preview are recommended.")
    parser.add_argument("-b", "--base-url", default="https://api.openai.com/v1/", help="Base URL for the OpenAI API")
    parser.add_argument("-s", "--sysprompt", default="""
以下の条件に従い、英文の技術文書を日本語に翻訳してください。訳文のみを返してください。
1. 日本語における標準的なIT技術用語・プログラミング用語を使用して翻訳してください。
2. 日本語の表現習慣を考慮して、語順を変更し、文の成分を補足して、文が日本語の文脈に合った流暢さを保つようにしてください。
3. 文脈に応じて固有名詞や多義語の翻訳を決定してください。直接的な日本語の対応語がない単語は原文を保持し、可能な場合は注釈を提供してください。
4. 流暢で読みやすく、元の情報が完全に保持されるように翻訳してください。
5. コードブロック中の記述は翻訳しないでください。
""", help="System prompt to be used for translation")
    parser.add_argument("-f", "--file", required=True, help="Path to the Markdown file to be translated")
    parser.add_argument("-l", "--last-output", default=None, required=False, help="Path to the Markdown file that ChatGPT generated last time. If this is specified, ChatGPT will continue from the last output.")

    args = parser.parse_args()

    translate_markdown(args.api_key, args.model, args.base_url, args.sysprompt, args.file, args.last_output)

"""
使い方：

# 初回実行。これだけだと出力が途中で途切れてしまうので、次回以降の実行に利用するために途中までの出力を保存しておく
$ python ./translate.py -k ${api-key} -f input.md > chunk_output.md

# 2回目以降。ファイル全体を翻訳し終えるまで、このワンライナーを繰り返し実行し続ける
# これを続けていくと、いずれ chunk_output.md が input.md 全体の翻訳になる。その時点でワンライナーの再実行を取りやめる
$ python ./translate.py -k ${api-key} -f input.md -l chunk_output.md >> chunk_output.md
"""