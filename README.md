# eos-stream

Streams preview video feed from Canon EOS cameras using gPhoto2

```bash
$ eos-stream | ffmpeg -i - -vcodec rawvideo -pix_fmt yuv420p -threads 0 -f v4l2 /dev/video0
```

### Focus adjustment

Press the <kbd>+</kbd> / <kbd>-</kbd> keys in the terminal window to manually
drive focus around.

