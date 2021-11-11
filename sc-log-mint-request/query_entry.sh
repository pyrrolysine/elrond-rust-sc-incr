#!/bin/bash

query=get
index=$1
contract=$(cat sc_address)

(
	curl \
		--silent \
		-X POST \
		https://devnet-gateway.elrond.com/vm-values/query \
		-H "Content-Type: application/json" \
		-d '{"scAddress": "'$contract'", "funcName": "'$query'", "args": ["'$index'"]}'
) | (
	python3 \
		-c "json = eval(input().replace('null', 'None')); print(json['data']['data']['returnData'][0])"
) | (
	base64 -d
) | (
	xxd -ps -c 4096
) | (
	cat
	#python3 \
	#	-c "x=input(); print(eval('0x' + x))"
)





