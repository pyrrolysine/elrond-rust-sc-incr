#!/bin/bash

query=get_min_bid

(
	curl \
		--silent \
		-X POST \
		https://devnet-gateway.elrond.com/vm-values/query \
		-H "Content-Type: application/json" \
		-d '{"scAddress": "erd1qqqqqqqqqqqqqpgq0s0a7yurg0gq8u874k253ce874yy2a3y7j5qmzsu0z", "funcName": "'$query'", "args": []}'
) | (
	python3 \
		-c "json = eval(input().replace('null', 'None')); print(json['data']['data']['returnData'][0])"
) | (
	base64 -d
) | (
	xxd -ps
) | (
	python3 \
		-c "print(eval('0x' + input()) / 1e18, 'eGLD')"
)





