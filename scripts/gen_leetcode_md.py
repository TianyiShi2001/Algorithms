import yaml
from functools import reduce

INP = "leetcode.yaml"
OUT = "leetcode.md"

tag2q = {}
with open(INP) as f:
    inp = yaml.load(f)

for id, info in inp.items():
    info = reduce(lambda x, y: {**x, **y}, info, {})
    q = f'- [{id}. {info["title"]}](info["url"]) (Difficulty: {info["difficulty"]})'
    for tag in info["tags"]:
        if not tag2q.get(tag):
            tag2q[tag] = []
        tag2q[tag].append(q)

buffer = ""
for tag, questions in tag2q.items():
    buffer += f"## {tag}\n\n"
    for q in questions:
        buffer += f"{q}\n"
    buffer += '\n\n'

with open(OUT, "w") as f:
    f.write(buffer)
