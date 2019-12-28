import sys
import socket
import threading
import math
import time
from PIL import Image

v6 = "2001:67c:20a1:1111:2051:5dff:feda:2983"
v4 = "151.217.111.34"

TOTAL_WORKER = 32
OFFSET_X = 950
OFFSET_Y = 700

print("drawing to x: " + str(OFFSET_X) + " ; y: " + str(OFFSET_Y))

blocks = math.sqrt(TOTAL_WORKER)

# open image
im = Image.open(sys.argv[1]).convert('RGBA')
width, height = im.size


width_per_thread = int(math.ceil(width / float(blocks)))
height_per_thread = int(math.ceil(height / float(blocks)))

imageData = []


def createImageData():
    for x in range(0, width):
        imageData.append([])
        for y in range(0, height):
            r, g, b, a = im.getpixel((x, y))
            if a == 255:
                imageData.append(b'PX %d %d %02x%02x%02x\n' % (x, y, r, g, b))
            else:
                imageData.append(b'PX %d %d %02x%02x%02x%02x\n' %
                                 (x, y, r, g, b, a))


createImageData()


def serve(x1, x2, y1, y2):

    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    while True:
        try:
            print("connect")
            sock.connect((v4, 1234))
            print("connected")
            break
        except Exception as e:
            print(f"Exception while connecting: {e}")
            time.sleep(1)

    def pixel(x, y):
        sock.send(imageData[x*width+y])

    while True:
        for x in range(x1, min(width, x2)+1):
            for y in range(y1, min(height, y2) + 1):
                try:
                    pixel(OFFSET_X+x, OFFSET_Y+y)
                except Exception as e:
                    pass


for thread_id in range(0, TOTAL_WORKER):

    width_start = int(thread_id % blocks)
    height_start = int(thread_id / blocks)

    x1 = width_start*width_per_thread
    y1 = width_start*width_per_thread

    print(
        f"Thread {thread_id}: x: {x1} to {x1+width_per_thread} y: {y1} to {y1+height_per_thread}")

    threading.Thread(target=serve, args=(
        x1, x1+width_per_thread, y1, y1+height_per_thread, )).start()
