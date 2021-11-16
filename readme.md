# Maion

## The tunnel

### Set up connection to tunnel

```bash
cloudflared access tcp --hostname staging.maion.cc --url 127.0.0.55:25565
```

### Set up the tunnel itself

```bash
cloudflared tunnel --hostname staging.maion.cc --url tcp://127.0.0.55:25565 --name maion-staging
```
