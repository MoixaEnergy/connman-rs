#!/bin/bash

set -x

CONNMAN_SRC_PATH="./connman"

do_action_prep() {
    sudo apt install \
        linux-modules-extra-`uname -r` \
        dbus \
        libdbus-1-dev \
        gnutls-dev \
        gnutls-bin \
        xtables-addons-common \
        xtables-addons-source \
        rfkill

    # TODO: Install deps to be able to run stock `bootstrap-config`
    #sudo apt install openconnect
    #sudo apt install openvpn
    #sudo apt install nftables
    #sudo apt install libnftnl-dev
    #sudo apt install vpnc

    sudo modprobe mac80211_hwsim radios=2 fake_hw_scan=1

    sudo rfkill unblock all

    iw reg set US

    # STA
    sudo ifconfig wlan0 up 192.168.1.2

    # AP (used by hostapd)
    sudo ifconfig wlan1 up 192.168.1.1

    git clone --depth 1 -b 1.36 \
        https://git.kernel.org/pub/scm/network/connman/connman.git \
        ${CONNMAN_SRC_PATH}
}

do_action_build() {
    cd ${CONNMAN_SRC_PATH}
    ./bootstrap
    ./configure
    make
    sudo make install
    cd ..

    # This may be done automatically if using `bootstrap-config`
    sudo cp connman/src/connman-dbus.conf /etc/dbus-1/system.d/
    sudo systemctl reload dbus
}

do_action_run() {
    sudo connmand -d --nodnsproxy -i wlan0

    # Make sure wifi is enabled
    sudo connmanctl enable wifi

    sudo ifconfig wlan0 down
    sudo ifconfig wlan0 up
}

action="$1"
shift
args="$@"
case $action in
    prep)
        do_action_prep
        ;;
    build)
        do_action_build
        ;;
    run)
        do_action_run
        ;;
    *)
        >&2 echo Unknown action $action
        exit 1
        ;;
esac
