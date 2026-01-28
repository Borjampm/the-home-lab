# Smart Lights Network Discovery Report

**Date:** 2026-01-28

## Executive Summary

Two Xiaomi smart lights were discovered on the local WiFi network (192.168.68.0/24) using miio protocol discovery. Both devices respond on UDP port 54321 and can be controlled locally with the appropriate device tokens.

## Devices Found

| Device | IP Address | Device ID | MAC Prefix | Manufacturer |
|--------|------------|-----------|------------|--------------|
| Light 1 | 192.168.68.105 | 470877538 (0x1c110562) | ec:4d:3e | Beijing Xiaomi Mobile Software Co., Ltd |
| Light 2 | 192.168.68.110 | 470946739 (0x1c1213b3) | ec:4d:3e | Beijing Xiaomi Mobile Software Co., Ltd |

## Technical Details

### Protocol
- **Protocol:** miio (Xiaomi IoT protocol)
- **Port:** UDP 54321
- **Discovery method:** miio hello packet broadcast

### Discovery Process
1. Network scan identified active hosts on 192.168.68.0/24
2. MAC address lookup identified Xiaomi devices (ec:4d:3e prefix)
3. miio protocol hello packets confirmed smart light presence
4. Device IDs extracted from protocol responses

## Control Options

### 1. Mi Home App (Cloud-based)
- On/off control
- Brightness adjustment (0-100%)
- Color temperature adjustment
- RGB color control (if supported by model)
- Scheduling and timers
- Voice assistant integration (Google Home, Alexa)

### 2. Local Control (python-miio)
```bash
pip install python-miio
miiocli device --ip 192.168.68.105 --token <TOKEN> info
miiocli yeelight --ip 192.168.68.105 --token <TOKEN> on
miiocli yeelight --ip 192.168.68.105 --token <TOKEN> set_brightness 50
```

### 3. Home Automation Platforms
- **Home Assistant:** Native Xiaomi Miio integration
- **OpenHAB:** Xiaomi Mi Smart Home Binding
- **Node-RED:** node-red-contrib-miio

### 4. Python API Example
```python
from miio import Yeelight

light = Yeelight("192.168.68.105", "device_token_here")
light.on()                    # Turn on
light.off()                   # Turn off
light.set_brightness(80)      # Brightness 0-100%
light.set_color_temp(4000)    # Color temperature in Kelvin
light.set_rgb(0xFF0000)       # RGB color (if supported)
```

## Token Extraction

Device tokens are required for local control. Methods to obtain:

1. **Mi Home app database** (Android, requires root or older app version)
2. **Xiaomi Cloud Tokens Extractor** - [GitHub tool](https://github.com/PiotrMachworker/Xiaomi-cloud-tokens-extractor)
3. **Network sniffing during initial pairing** (factory reset required)

## Limitations & Uncertainties

- **Exact model unknown:** Without device tokens, cannot query specific model/capabilities
- **RGB support uncertain:** Some Xiaomi lights are white-only, others support full RGB
- **Token extraction required:** Local control requires obtaining device tokens from Mi Home app or cloud
- **Firmware version unknown:** Some older firmware may have different protocol behavior

## Network Context

Other devices identified on the network:
- Gateway/Router: 192.168.68.1 (TP-Link, d8:07:b6)
- Multiple TP-Link mesh nodes: 192.168.68.246-250
- Apple devices, Roku, Alarm.com security system

## Sources

- Network discovery via nmap and miio protocol
- MAC vendor lookup: [macvendors.com API](https://api.macvendors.com)
- Protocol reference: [python-miio documentation](https://python-miio.readthedocs.io/)
