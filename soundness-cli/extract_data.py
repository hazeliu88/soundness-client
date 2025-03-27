import json
import csv

# 定义输入和输出文件路径
json_file_path = './target/debug/key_store_with_mnemonic.json'
csv_file_path = 'output.csv'

try:
    # 打开并读取 JSON 文件
    with open(json_file_path, 'r', encoding='utf-8') as json_file:
        data = json.load(json_file)

    # 准备写入 CSV 文件
    with open(csv_file_path, 'w', newline='', encoding='utf-8') as csv_file:
        fieldnames = ['public_key_string', 'mnemonic']
        writer = csv.DictWriter(csv_file, fieldnames=fieldnames)

        # 写入 CSV 文件的表头
        writer.writeheader()

        # 提取 keys 字典下的数据
        keys_data = data.get('keys', {})
        for key_info in keys_data.values():
            writer.writerow({
                'public_key_string': key_info.get('public_key_string', ''),
                'mnemonic': key_info.get('mnemonic', '')
            })

    print(f"数据已成功写入 {csv_file_path}")

except FileNotFoundError:
    print(f"未找到 {json_file_path} 文件，请检查文件路径。")
except json.JSONDecodeError:
    print(f"{json_file_path} 文件不是有效的 JSON 格式，请检查文件内容。")