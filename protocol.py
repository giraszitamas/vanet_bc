from py_ecc.secp256k1 import secp256k1 as ec
import random
import hashlib
import json

def is_on_curve(point):
    if point is None:
        return False
    x, y = point
    return (y**2 - x**3 - 7) % ec.P == 0

def hash_list(l):
    l_json = json.dumps(l, sort_keys=True).encode()
    h = hashlib.sha256(l_json).hexdigest()
    return h

def gen_inc_report(y_obu, yG_obu, ayG_obu,aG, ayGs, MSG):
    try:
        i_obu = ayGs.index(ayG_obu)
        
        idx = set()
        #### select obus
        idx.add(i_obu)
        while len(idx) != K:
            idx.add(random.randint(0, 99))
        idx = list(idx)
        
        random.shuffle(idx)
        i_obu = idx.index(i_obu)
        print("The curremt obu list:", list(idx))
        print("OBU:", idx[i_obu])
        print("OBU place:", i_obu)
        #### r_j and d_choose
        r_js = [random.randint(0, ec.P-1) for _ in range(K)]
        d_js = [random.randint(0, ec.P-1) for _ in range(K)]
        
        W_list = [
            ec.add(ec.multiply(aG, r_js[i]), ec.multiply(ayGs[idx[i]], d_js[i])) for i in range(K)
        ]
       
        #### 
        
        w = (y_obu * d_js[i_obu] + r_js[i_obu]) % ec.N
        
        #### c 
        to_hash = idx + W_list
        to_hash.append(MSG)
        
        #print(to_hash)
        #print(hash_list(to_hash))
        
        c = int(hash_list(to_hash),16)
        
        send_r = []
        send_d = []
        for i in range(K):
            if i_obu == i:
                d_copy = d_js.copy()
                d_copy.pop(i)
                
                d_new = (c - sum(d_copy))% ec.N
                send_d.append(d_new)
                send_r.append((w - y_obu*d_new)% ec.N)
            else:
                send_d.append(d_js[i])
                send_r.append(r_js[i])
        return MSG,c,idx,send_d, send_r
            
        
        
    except ValueError:
        print(f"Error in index searching")

def verify_inc_report(MSG, c, idx, d, r):
    W_list_verify = [
        ec.add(ec.multiply(aG, r[i]), ec.multiply(ayGs[idx[i]], d[i])) for i in range(K)
    ]

    to_hash2 = idx + W_list_verify
    to_hash2.append(MSG)

    c2 = int(hash_list(to_hash2),16)

    return c2 == c

G = ec.G
K = 10 

#to simulate obus key gen
# it is not sended, but required pub keys for protocol

y_secrect = []
y_pub_keys = []

for _ in range(100):
    y = random.randint(1, ec.P - 1)
    y_secrect.append(y)
    y_pub_keys.append(ec.multiply(G, y))

#######
# RSU calculates

ayGs = []
a = random.randint(1, ec.P - 1)
aG = ec.multiply(G, a)

for yG in y_pub_keys:
    ayGs.append(ec.multiply(yG, a))

random.shuffle(ayGs)

# random obu choosing and sending inc report

#OBU
i = random.randint(0, 99)
y_obu = y_secrect[i]
yG_obu = y_pub_keys[i]
ayG_obu = ec.multiply(aG, y_obu)

MSG, c, idx, d, r = gen_inc_report(y_obu, yG_obu, ayG_obu,aG, ayGs, "Incident happens")

print(verify_inc_report(MSG, c, idx, d, r))