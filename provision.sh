# apt-get -y install software-properties-common
add-apt-repository ppa:ubuntu-lxc/lxc-stable
apt-get update
# apt-get -y install haveged
# update-rc.d haveged enable
# service haveged start
apt-get -y install lxc
touch /etc/lxc/lxc-usernet
echo 'vagrant veth lxcbr0 10' | tee --append /etc/lxc/lxc-usernet > /dev/null
mkdir -p /home/vagrant/.config/lxc
cp /etc/lxc/default.conf /home/vagrant/.config/lxc/default.conf
echo 'lxc.id_map = u 0 100000 65536' | tee --append /home/vagrant/.config/lxc/default.conf > /dev/null
echo 'lxc.id_map = g 0 100000 65536' | tee --append /home/vagrant/.config/lxc/default.conf > /dev/null
chown -R vagrant:vagrant /home/vagrant/.config/
mkdir -p "/home/vagrant/.local/share/lxc/"
chown -R vagrant:vagrant "/home/vagrant/.local/share/lxc/"

# Automatically go to /vagrant
echo "cd /vagrant" >> /home/vagrant/.bashrc
