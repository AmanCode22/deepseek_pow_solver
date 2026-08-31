# Deepseek's POW(Proof of Work) Challange solver
A reimplimentation of deepseek's web chat pow solver with the help of gemini in rust and wasm to get near native speed in python using wasmtime.

# Building Yourself
The .wasm file is uploaded arleady in releases but if you wanna build it yourself then 
- Clone the repo
- Install rust
- Add wasm32-unknown-unknown target( Run ``` rustup target add wasm32-unknown-unknown```)

Then run following command to build the wasm file
```bash
cargo build --target wasm32-unknown-unknown --release
```

Wasm file can be found in ```target/wasm32-unknown-unknown/release/deepseek_pow_solver.wasm```

I made this as to solve pow challange in python when i tested took 66.3s with pure python implimentation for same challange and 0.3s with this implimentation.
Deepseek's orignal wasm file was undocumented so that is why I reimplimented.

# Usage Example
Install python and wasmtime(using ```pip install wasmtime```)
Use the following script to run this and add binary path in wasm_path variable
```python
import wasmtime
wasm_path="ADD_YOUR_WASM_LIB_PATH"
engine = wasmtime.Engine()
with open(wasm_path, "rb") as f:
  wasm_bytes = f.read()
module = wasmtime.Module(engine, wasm_bytes)
store = wasmtime.Store(engine)
linker = wasmtime.Linker(engine)
instance = linker.instantiate(store, module)
memory = instance.exports(store)["memory"]
alloc_func = instance.exports(store)["alloc"]
solve_func = instance.exports(store)["solve_pow"]
def solve(challenge_hex, salt, expire_at, difficulty):
    def write_string(text):
        data = text.encode("utf-8")
        ptr = alloc_func(store, len(data))
        mem = memory.data_ptr(store)
        for i in range(len(data)):
            mem[ptr + i] = data[i]
        return ptr, len(data)

    ch_ptr, ch_len = write_string(challenge_hex)
    salt_ptr, salt_len = write_string(salt)

    result = solve_func(
        store, ch_ptr, ch_len, salt_ptr, salt_len, expire_at, difficulty
    )

    
    if result < 0:
        result = result + 0x10000000000000000

    return result if result != 0xFFFFFFFFFFFFFFFF else None

print("Answer:",solve("34d4c336676aa2e83c3308148e12ba8bc5e77ccb2fc12eeea1046e20c4c64eec","6c3a962c828dd81d7d69",1780227446451,144000))
```
Deepseek api returns json like below
```json
{
    "code": 0,
    "msg": "",
    "data": {
        "biz_code": 0,
        "biz_msg": "",
        "biz_data": {
            "challenge": {
                "algorithm": "DeepSeekHashV1",
                "challenge": "34d4c336676aa2e83c3308148e12ba8bc5e77ccb2fc12eeea1046e20c4c64eec",
                "salt": "6c3a962c828dd81d7d69",
                "signature": "f68a3f7b13fde1a3c5f484869bed76d82160e39b401172865e31485219682b74",
                "difficulty": 144000,
                "expire_at": 1780227446451,
                "expire_after": 300000,
                "target_path": "/api/v0/chat/completion"
            }
        }
    }
}
```
And it expects a base64 string like 
```base64
eyJhbGdvcml0aG0iOiJEZWVwU2Vla0hhc2hWMSIsImNoYWxsZW5nZSI6IjM0ZDRjMzM2Njc2YWEyZTgzYzMzMDgxNDhlMTJiYThiYzVlNzdjY2IyZmMxMmVlZWExMDQ2ZTIwYzRjNjRlZWMiLCJzYWx0IjoiNmMzYTk2MmM4MjhkZDgxZDdkNjkiLCJhbnN3ZXIiOjg2MDIyLCJzaWduYXR1cmUiOiJmNjhhM2Y3YjEzZmRlMWEzYzVmNDg0ODY5YmVkNzZkODIxNjBlMzliNDAxMTcyODY1ZTMxNDg1MjE5NjgyYjc0IiwidGFyZ2V0X3BhdGgiOiIvYXBpL3YwL2NoYXQvY29tcGxldGlvbiJ9
```
This base64 string decodes into a json
```json
{
   "algorithm":"DeepSeekHashV1",
   "challenge":"34d4c336676aa2e83c3308148e12ba8bc5e77ccb2fc12eeea1046e20c4c64eec",
   "salt":"6c3a962c828dd81d7d69",
   "answer":86022,
   "signature":"f68a3f7b13fde1a3c5f484869bed76d82160e39b401172865e31485219682b74",
   "target_path":"/api/v0/chat/completion"
}
```
So, here we have to put the answer that we get from the script and other values that we got from sever itself.
Then base64 encode it like shown above and can be used as ```x-ds-pow-response``` header.

# Provenance & Legal Statement
This is an independent, behavioral reimplementation of the DeepSeekHashV1 proof-of-work algorithm, created to enable interoperability with the author's own DeepSeek account. No source code, binaries, or assets from DeepSeek's web application were copied, decompiled, or redistributed. The algorithm was reconstructed from observed input/output behavior only. All code in this repository is original, with the some help of AI to observering better. "DeepSeek" is used solely to describe interoperability; this project is not affiliated with, endorsed by, or sponsored by DeepSeek.
