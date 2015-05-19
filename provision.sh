sudo apt-get -y install lxc
sudo touch /etc/lxc/lxc-usernet
echo 'vagrant veth lxcbr0 10' | sudo tee --append /etc/lxc/lxc-usernet > /dev/null
mkdir -p /home/vagrant/.config/lxc
cp /etc/lxc/default.conf /home/vagrant/.config/lxc/default.conf
echo 'lxc.id_map = u 0 100000 65536' | tee --append /home/vagrant/.config/lxc/default.conf > /dev/null
echo 'lxc.id_map = g 0 100000 65536' | tee --append /home/vagrant/.config/lxc/default.conf > /dev/null