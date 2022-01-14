#!usr/bin/env python3.9
# Arp is used to translate IP addr to MAC addr on a local network aka physical connection id

from scapy.all import *
import os,sys

def main():
    os.system("echo 1 > /proc/sys/net/ipv4/ip_forward")

    interface = "eth0"
    target_ip = "172.18.0.5"
    gateway_ip = "172.18.0.14"
    packet_count = 1000

    conf.iface = interface
    conf.verb = 0

    gateway_mac = get_mac(gateway_ip)

    if gateway_mac is None:
        print("[X] Failed to get mac address")
        os.system("echo 0 > /proc/sys/net/ipv4/ip_forward")
        sys.exit(0)
    os.system("echo 0 > /proc/sys/net/ipv4/ip_forward")

    target_mac = get_mac(target_ip)

    if target_mac is None:
        print("[X] Failed to get mac address")
        os.system("echo 0 > /proc/sys/net/ipv4/ip_forward")
        sys.exit(0)
    os.system("echo 0 > /proc/sys/net/ipv4/ip_forward")

def get_mac(ip_address):
    responses, unanswerd = \
        srp(Ether(dst="ff:ff:ff:ff:ff:ff")/ARP(pdst=ip_address)),\
            timeout=2, retry=10)

    for s,r in responses:
        return r[Ether].src

    return None