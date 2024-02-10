# !/bin/bash

# reference: https://techblog.ap-com.co.jp/entry/2019/06/28/100439

set -eux

# 新しいネットワーク名前空間を作成
sudo ip netns add host1
sudo ip netns add router
sudo ip netns add host2

#　link add name 指定された名前のネットワークインターフェースを作成
# name xxx 作成する仮想イーサネットデバイスの名前
# 仮想イーサネットペアと呼ばれる2つの仮想インターフェースを作成 相互に通信可能
# peer name 作成する仮想イーサネットペアのもう一方のインターフェースの名前
sudo ip link add name host1-veth1 type veth peer name router-veth1
sudo ip link add name router-veth2 type veth peer name host2-veth1

sudo ip link set host1-veth1 netns host1
sudo ip link set router-veth1 netns router
sudo ip link set router-veth2 netns router
sudo ip link set host2-veth1 netns host2

sudo ip netns exec host1 ip addr add 10.0.0.1/24 dev host1-veth1
sudo ip netns exec router ip addr add 10.0.0.254/24 dev router-veth1
sudo ip netns exec router ip addr add 10.0.1.254/24 dev router-veth2
sudo ip netns exec host2 ip addr add 10.0.1.1/24 dev host2-veth1

sudo ip netns exec host1 ip link set host1-veth1 up
sudo ip netns exec router ip link set router-veth1 up
sudo ip netns exec router ip link set router-veth2 up
sudo ip netns exec host2 ip link set host2-veth1 up
sudo ip netns exec host1 ip link set lo up
sudo ip netns exec router ip link set lo up
sudo ip netns exec host2 ip link set lo up

sudo ip netns exec host1 ip route add 0.0.0.0/0 via 10.0.0.254
sudo ip netns exec host2 ip route add 0.0.0.0/0 via 10.0.1.254
sudo ip netns exec router sysctl -w net.ipv4.ip_forward=1

# drop RST
sudo ip netns exec host1 sudo iptables -A OUTPUT -p tcp --tcp-flags RST RST -j DROP
sudo ip netns exec host2 sudo iptables -A OUTPUT -p tcp --tcp-flags RST RST -j DROP

# turn off checksum offloading
sudo ip netns exec host2 sudo ethtool -K host2-veth1 tx off
sudo ip netns exec host1 sudo ethtool -K host1-veth1 tx off

# nc TCP/IPネットワーク上の接続を確立したり、データを送受信したりするためのツール 接続するサーバの IP アドレスを指定 接続するサーバの PORTを指定
# sudo ip netns exec host2 nc 10.0.0.1 40000

# tcコマンド　　トラフィックキュー　帯域制限、キューイング、パケット損失などの設定
# qdisc ネットワークデバイスに送受信されるパケットをどのように処理するかを決定します。
# add キューイングディシプリンの種類、設定などを同時に指定
# dev host1-veth1 ネットワークデバイスの名前
# root デバイスのすべてのインターフェースにキューイング ディシプリンが適用
# netem ネットワークの遅延、パケット損失、帯域制限などをエミュレートするための機能 ネットワークのトラブルシューティングやパフォーマンス検証などに使用
# loss 50% パケット損失率を指定
# sudo ip netns exec host1 tc qdisc add dev host1-veth1 root netem loss 50%

# 上記のコマンドで設定した内容を元に戻す
# sudo ip netns exec host1 tc qdisc del dev host1-veth1 root

# md5sumコマンド 
# ファイルのMD5ハッシュ値を計算
md5sum ファイル名

# チェックサムファイルの内容に基づいて、ファイルの整合性を検証
md5sum -c チェックサムファイル

# バイナリモードで実行
md5sum -b ファイル名

# テキストモードで実行
md5sum -t ファイル名

# 詳細な出力
md5sum -v ファイル名