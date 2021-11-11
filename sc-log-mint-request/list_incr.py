#!/usr/bin/python3

import os

with os.popen('./query_count.sh', mode = 'r') as fd: count = int(fd.read().strip())
print(count - 1, 'NFTs minted to date')

data = []
for i in range(1, count + 1):
    pos = hex(i)[2:].rjust(8, '0')
    print('Fetching', pos)
    with os.popen('./query_entry.sh ' + pos, mode = 'r') as fd: spec = fd.read().strip()
    hex_address = spec[0x02:0x42]
    lon = int(spec[0x42:0x52])
    lat = int(spec[0x52:0x62])
    latlon = str(lon) + ' ' + str(lat)
    with os.popen('erdpy wallet bech32 --encode ' + hex_address, mode = 'r') as fd: address = fd.read().strip()
    data.append(latlon + ' ' + address)

with open('../owners', mode = 'w') as fd:
    for x in data:
        fd.write(x + '\n')


