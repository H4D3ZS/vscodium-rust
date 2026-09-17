import urllib.request
import re

html = urllib.request.urlopen('https://www.apple.com/certificateauthority/').read().decode()
links = re.findall(r'href="([^"]+\.cer)"', html)
for l in links:
    print(l)
