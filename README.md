# Sample Elrond smart contracts


## Non-payable

### **empty**
An empty contract. Useful as a template.

### **clock**
Responds with the current Unix time at execution when its method _tick_ is called.

### **monotonic_counter**
You increase the counter by calling _inc_ and fetch the current value with _get_.

## Payable

### **deposit**
A contract which can be paid eGLD.

### **deposit_to_owner**
Sends any received eGLD amount to its owner.

### **sc-bid-owner**
Allows the owner to sequence NFT auctions and accept bids.

### **sc-log-mint-request**
Logs addresses and IDs of tokens to be minted later.

