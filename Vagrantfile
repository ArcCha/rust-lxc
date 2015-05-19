# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.define :lxc_dev do |lxc_dev|
    lxc_dev.vm.box = "ubuntu/trusty64"
    lxc_dev.vm.provision "shell", path: "provision.sh"
    lxc_dev.vm.provider :virtualbox do |vb|
      vb.name = "lxc_dev"
    end
  end
end
