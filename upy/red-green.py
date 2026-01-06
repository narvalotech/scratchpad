from machine import Pin
from neopixel import NeoPixel
import time
import ntptime, network

def led_write(leds, color):
    if color == "RED":
        leds[0] = (100, 0, 0)
    elif color == "GREEN":
        leds[0] = (0, 100, 0)
    elif color == "BLUE":
        leds[0] = (0, 0, 100)
    leds.write()
    leds.write()

def init_wifi():
    global inif
    wifi=network.WLAN(network.STA_IF)

    if not wifi.isconnected():
        print("Connecting to WiFi")
        wifi.active(True)
        wifi.connect("Non-Fi", "l33tp4ss")
        while not wifi.isconnected():
            pass
    print("Connected to WiFi")
    print('Config', wifi.ifconfig())

def sync_time():
    while True:
        try:
            print("Getting time")
            ntptime.host = '192.168.4.32'
            ntptime.settime()
            return
        except:
            print("Failed to get time")
            time.sleep(1)

def get_time():
    t = time.gmtime()
    hours, minutes, seconds = t[3], t[4], t[5]
    hours -= 7                  # calgary = UTC - 7
    if hours < 0:
        hours += 24
    return hours, minutes, seconds

def main():
    np = Pin(18, Pin.OUT)
    leds = NeoPixel(np, 1)
    led_write(leds, "BLUE")

    init_wifi()
    time.sleep(1)
    sync_time()

    while True:
        time.sleep(1)
        hours, minutes, seconds = get_time()
        nighttime = (hours >= 21) or (hours <= 7)
        n = "Nighttime" if nighttime else "Daytime"
        print(f"Hour: {hours}:{minutes}:{seconds} => {n}")

        w = "RED" if nighttime else "GREEN"
        led_write(leds, w)

        if minutes % 10 == 0:
            sync_time()

main()
