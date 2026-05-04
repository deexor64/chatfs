# Installation

ChatFS is a small CLI tool you run locally.
Once it's running, it connects your workspace to the ChatFS server so your AI can interact with it.

---

## 1. Get the binary

Download the latest release for your platform from the GitHub releases page.

After downloading:

### Linux / macOS

```bash
chmod +x chatfs
```

(Optional) Move it somewhere in your PATH:

```bash
mv chatfs /usr/local/bin/
```

---

### Windows

Just use the `.exe` directly:

```
chatfs-windows-x86_64.exe
```

You can rename it to `chatfs.exe` if you want.

---

## 2. Set the gateway (required)

Before running anything, you need to tell ChatFS where your server is:

```bash
chatfs set-config gateway <URL>
```

Example:

```bash
chatfs set-config gateway "wss://your-server.com/client/"
```

---

## 3. Run ChatFS

By default, ChatFS shares the **current directory**:

```bash
chatfs run
```

---

### Run on a specific path

```bash
chatfs run <PATH>
```

Example:

```bash
chatfs run ./my-project
```

---

## 4. Use the session URL

Once running, ChatFS will give you a workspace URL.

Paste that into your AI chat (Grok, Qwen, etc.), and it can start interacting with your files.

---

## CLI help

To see all available commands:

```bash
chatfs help
```

---

## Notes

* The tool shares whatever directory you run it in (or specify)
* Make sure you trust the environment before running it
* For better privacy, consider self-hosting your [ChatFS - server](https://github.com/alien5516788/chatfs-server)
