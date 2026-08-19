#!/bin/bash
# Generate rust definitions from C headers

CCFLAGS=" -g \
    -D __riscv_xlen=32 \
    -DconfigUSE_TICKLESS_IDLE=0 \
    -DFEATURE_WIFI_DISABLE=1 \
    -D BL_SDK_VER=\"0\" \
    -D BL_SDK_PHY_VER=\"0\" \
    -D BL_SDK_RF_VER=\"0\" \
    -D BL_CHIP_NAME=\"BL602\" \
    -MMD \
    -MP \
    -D BL_SDK_VER=\"0\" \
    -D BL_SDK_PHY_VER=\"0\" \
    -D BL_SDK_RF_VER=\"0\"  \
    -DARCH_RISCV \
    -DCONFIG_PSM_EASYFLASH_SIZE=16384 \
    -DconfigUSE_TICKLESS_IDLE=0 \
    -DFEATURE_WIFI_DISABLE=1 \
    -DARCH_RISCV \
    -DBFLB_CRYPT_HARDWARE \
    -DBFLB_PKA_HARDWARE \
    -DSTDDRV_VERSION=096d971a96c12b5857abc7606bfd5ac1bf371a41 \
    -DBL602_USE_HAL_DRIVER \
    -DCFG_COMPONENT_BLOG_ENABLE=0 \
    -D __FILENAME__=\"bl602_dma.c\" \
    -D __FILENAME_WO_SUFFIX__=\"bl602_dma\" \
    -D __FILENAME_WO_SUFFIX_DEQUOTED__=bl602_dma \
    -D __COMPONENT_NAME__=\"bl602_std\" \
    -D __COMPONENT_NAME_DEQUOTED__=bl602_std \
    -D __COMPONENT_FILE_NAME__=\"bl602_stdbl602_dma\" \
    -D__COMPONENT_FILE_NAMED__=bl602_std.bl602_dma \
    -D__COMPONENT_FILE_NAME_DEQUOTED__=bl602_stdbl602_dma \
    -I ../../bl602/bl602_std \
    -I ../../bl602/bl602_std/include \
    -I ../../bl602/bl602_std/bl602_std/StdDriver/Inc \
    -I ../../bl602/bl602_std/bl602_std/Device/Bouffalo/BL602/Peripherals \
    -I ../../bl602/bl602_std/bl602_std/RISCV/Device/Bouffalo/BL602/Startup \
    -I ../../bl602/bl602_std/bl602_std/RISCV/Core/Include \
    -I ../../bl602/bl602_std/bl602_std/Include \
    -I ../../bl602/bl602_std/bl602_std/Common/platform_print \
    -I ../../bl602/bl602_std/bl602_std/Common/soft_crc \
    -I ../../bl602/bl602_std/bl602_std/Common/partition \
    -I ../../bl602/bl602_std/bl602_std/Common/xz \
    -I ../../bl602/bl602_std/bl602_std/Common/cipher_suite/inc \
    -I ../../bl602/bl602_std/bl602_std/Common/ring_buffer \
    -I ../../bl602/bl602_wifidrv/bl60x_wifi_driver \
    -I ../../bl602/bl602 \
    -I ../../bl602/bl602/include \
    -I ../../stage/blfdt \
    -I ../../stage/blfdt/include \
    -I ../../stage/blfdt/inc \
    -I ../../sys/blmtd \
    -I ../../sys/blmtd/include \
    -I ../../sys/blmtd/include \
    -I ../../stage/blog \
    -I ../../stage/blog/include \
    -I ../../stage/blog \
    -I ../../stage/blog_testc \
    -I ../../stage/blog_testc/include \
    -I ../../stage/blog_testc \
    -I ../../sys/bloop/bloop \
    -I ../../sys/bloop/bloop/include \
    -I ../../sys/bloop/bloop/include \
    -I ../../sys/bltime \
    -I ../../sys/bltime/include \
    -I ../../sys/bltime/include \
    -I ../../stage/cli \
    -I ../../stage/cli/include \
    -I ../../stage/cli/cli/include \
    -I ../../stage/easyflash4 \
    -I ../../stage/easyflash4/include \
    -I ../../stage/easyflash4/inc \
    -I ../../freertos \
    -I ../../freertos/include \
    -I ../../freertos/config \
    -I ../../freertos/portable/GCC/RISC-V \
    -I ../../freertos/portable/GCC/RISC-V/chip_specific_extensions/RV32F_float_abi_single \
    -I ../../freertos/panic \
    -I ../../hal_drv \
    -I ../../hal_drv/include \
    -I ../../hal_drv/bl602_hal \
    -I ../../sys/bloop/looprt \
    -I ../../sys/bloop/looprt/include \
    -I ../../sys/bloop/loopset \
    -I ../../sys/bloop/loopset/include \
    -I ../../3rdparty/lora-sx1276/include \
    -I ../../3rdparty/lorawan/include \
    -I ../../network/lwip \
    -I ../../network/lwip/include \
    -I ../../network/lwip/src/include \
    -I ../../network/lwip/lwip-port \
    -I ../../network/lwip/lwip-port/config \
    -I ../../network/lwip/lwip-port/FreeRTOS \
    -I ../../network/lwip/lwip-port/arch \
    -I ../../3rdparty/nimble-porting-layer/include \
    -I ../../fs/romfs \
    -I ../../fs/romfs/include \
    -I ../../utils \
    -I ../../utils/include \
    -I ../../fs/vfs \
    -I ../../fs/vfs/include \
    -I ../../fs/vfs/posix/include \
    -I ../../stage/yloop \
    -I ../../stage/yloop/include \
    -I ../../stage/yloop/include \
    -I ../../security/mbedtls-bl602/include \
    -I ../../components/3rdparty/libcoap/include/coap3 \
    "

CCFLAGS_BINDGEN=" -D static= -D inline= "


bindgen \
    --verbose \
    --use-core \
    $includelist \
    --no-layout-tests \
    header.h \
    -- \
    $CCFLAGS \
    $CCFLAGS_BINDGEN
