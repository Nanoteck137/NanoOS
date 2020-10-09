#!/bin/sh

NAME_OF_IMAGE=disk_image.img
SECTOR_SIZE=512
SIZE_IN_BYTES=67108864
SIZE_IN_SECTORS=$((SIZE_IN_BYTES/SECTOR_SIZE));

echo $SIZE_IN_BYTES
echo $SIZE_IN_SECTORS

dd if=/dev/zero of=$NAME_OF_IMAGE bs=$SECTOR_SIZE count=$SIZE_IN_SECTORS
sfdisk $NAME_OF_IMAGE < partition_layout.sfdisk

/usr/local/sbin/mkfs.fat --offset=2048 -F32 $NAME_OF_IMAGE

# TODO(patrik): Change the 1048576 (2048 * 512) to not be hardcoded
mcopy -i disk_image.img@@1048576 fs/* ::/
mdir -i disk_image.img@@1048576

