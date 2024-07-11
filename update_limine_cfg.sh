losetup /dev/loop0 disk.img
echo "Loopback created"
mkdir ./tmp_mnt
echo "Made mountpoint"
mount /dev/loop0 ./tmp_mnt
echo "Mounted"
cp limine.cfg ./tmp_mnt/limine.cfg
echo "Copied over"
umount ./tmp_mnt
echo "Unmounted"
rmdir ./tmp_mnt
echo "Deleted mountpoint"
losetup -d /dev/loop0
echo "Deleted loopback"
