# ucpikms-v1-smartcontract

decentralized kms(build on secretnetwork) operated using web2 authentication
steps to test:
setup secretcli(from secretnetwork docs)
make build-mainnet
deploy

# Demo Test secretcli commands
INIT="{\"owner\":\"any@universe\",\"idtype\":\"universal\"}"
AK="{\"addkey\":{\"key\":\"anykey\",\"token\":\"1112cc1a19ca49970b1989d92bc9babbcb433a6d17efcea0d496f7b325ff3683\"}}"
secretcli tx compute execute $CONTRACT "$AK" --from sn1
NV="{\"nodevot\":{\"jwt\":\"1112cc1a19ca49970b1989d92bc9babbcb433a6d17efcea0d496f7b325ff3683\",\"nodeadress\":\"secret1r4gnwa5f33nyjragmr6uu7ls42vkst5jdm4ngc\"}}"
VF="{\"votefor\":{\"owner\":\"any@universe\",\"token\":\"1112cc1a19ca49970b1989d92bc9babbcb433a6d17efcea0d496f7b325ff368\",\"vote\":true}}"
secretcli tx compute execute $CONTRACT "$VF" --from sn1
GV="{\"getvoting\":{\"jwt\":\"1112cc1a19ca49970b1989d92bc9babbcb433a6d17efcea0d496f7b325ff368\"}}"
secretcli q compute query "$CONTRACT" "$GV" 
 secretcli tx compute instantiate $CODE_ID "$INIT" --from sn1 --label "ram9" -y
GK="{\"showkey\":{\"vik\":\"1112cc1a19ca49970b1989d92bc9babbcb433a6d17efcea0d496f7b325ff3683\",\"vk\":\"shivi\"}}"
SKK="{\"skey\":{\"vik\":\"shivi\"}}"
CKK="{\"showkey\":{\"vik\":\"shivi\"}}"
SK="{\"login\":{\"jw\":\"1112cc1a19ca49970b1989d92bc9babbcb433a6d17efcea0d496f7b325ff368\",\"vk\":\"shivi\"}}"
GK="{\"showkey\":{\"vik\":\"1112cc1a19ca49970b1989d92bc9babbcb433a6d17efcea0d496f7b325ff368\",\"vk\":\"shivi\"}}"
V="{\"vote\":{\"msg\":\"1112cc1a19ca49970b1989d92bc9babbcb433a6d17efcea0d496f7b325ff368\"}}"
