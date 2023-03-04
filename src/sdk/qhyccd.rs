#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
pub struct __BindgenUnionField<T>(std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
pub const QHYCCD_READ_DIRECTLY: u32 = 8193;
pub const QHYCCD_DELAY_200MS: u32 = 8192;
pub const QHYCCD_PCIE: u32 = 9;
pub const QHYCCD_WINPCAP: u32 = 8;
pub const QHYCCD_QGIGAE: u32 = 7;
pub const QHYCCD_USBSYNC: u32 = 6;
pub const QHYCCD_USBASYNC: u32 = 5;
pub const QHYCCD_COLOR: u32 = 4;
pub const QHYCCD_MONO: u32 = 3;
pub const QHYCCD_COOL: u32 = 2;
pub const QHYCCD_NOTCOOL: u32 = 1;
pub const QHYCCD_SUCCESS: u32 = 0;
pub const QHYCCD_ERROR: u32 = 4294967295;
pub const DEVICETYPE_IMG0S: u32 = 1000;
pub const DEVICETYPE_IMG0H: u32 = 1001;
pub const DEVICETYPE_IMG0L: u32 = 1002;
pub const DEVICETYPE_IMG0X: u32 = 1003;
pub const DEVICETYPE_IMG1S: u32 = 1004;
pub const DEVICETYPE_IMG2S: u32 = 1005;
pub const DEVICETYPE_IMG1E: u32 = 1006;
pub const DEVICETYPE_QHY5: u32 = 2001;
pub const DEVICETYPE_QHY5II: u32 = 3001;
pub const DEVICETYPE_QHY5LII_M: u32 = 3002;
pub const DEVICETYPE_QHY5LII_C: u32 = 3003;
pub const DEVICETYPE_QHY5TII: u32 = 3004;
pub const DEVICETYPE_QHY5RII: u32 = 3005;
pub const DEVICETYPE_QHY5PII: u32 = 3006;
pub const DEVICETYPE_QHY5VII: u32 = 3007;
pub const DEVICETYPE_QHY5HII: u32 = 3008;
pub const DEVICETYPE_MINICAM5S_M: u32 = 3009;
pub const DEVICETYPE_MINICAM5S_C: u32 = 3010;
pub const DEVICETYPE_QHY5PII_C: u32 = 3011;
pub const DEVICETYPE_QHY5RII_C: u32 = 3012;
pub const DEVICETYPE_MINICAM5F_M: u32 = 3013;
pub const DEVICETYPE_QHY5PII_M: u32 = 3014;
pub const DEVICETYPE_QHY5TII_C: u32 = 3015;
pub const DEVICETYPE_POLEMASTER: u32 = 3016;
pub const DEVICETYPE_QHY5IIEND: u32 = 3999;
pub const DEVICETYPE_QHY5III174: u32 = 4000;
pub const DEVICETYPE_QHY5III174M: u32 = 4001;
pub const DEVICETYPE_QHY5III174C: u32 = 4002;
pub const DEVICETYPE_QHY174: u32 = 4003;
pub const DEVICETYPE_QHY174M: u32 = 4004;
pub const DEVICETYPE_QHY174C: u32 = 4005;
pub const DEVICETYPE_QHY5III178: u32 = 4006;
pub const DEVICETYPE_QHY5III178C: u32 = 4007;
pub const DEVICETYPE_QHY5III178M: u32 = 4008;
pub const DEVICETYPE_QHY178: u32 = 4009;
pub const DEVICETYPE_QHY178M: u32 = 4010;
pub const DEVICETYPE_QHY178C: u32 = 4011;
pub const DEVICETYPE_QHY5III185: u32 = 4012;
pub const DEVICETYPE_QHY5III185C: u32 = 4013;
pub const DEVICETYPE_QHY5III185M: u32 = 4014;
pub const DEVICETYPE_QHY185: u32 = 4015;
pub const DEVICETYPE_QHY185M: u32 = 4016;
pub const DEVICETYPE_QHY185C: u32 = 4017;
pub const DEVICETYPE_QHY5III224: u32 = 4018;
pub const DEVICETYPE_QHY5III224C: u32 = 4019;
pub const DEVICETYPE_QHY5III224M: u32 = 4020;
pub const DEVICETYPE_QHY224: u32 = 4021;
pub const DEVICETYPE_QHY224M: u32 = 4022;
pub const DEVICETYPE_QHY224C: u32 = 4023;
pub const DEVICETYPE_QHY5III290: u32 = 4024;
pub const DEVICETYPE_QHY5III290C: u32 = 4025;
pub const DEVICETYPE_QHY5III290M: u32 = 4026;
pub const DEVICETYPE_QHY290: u32 = 4027;
pub const DEVICETYPE_QHY290M: u32 = 4028;
pub const DEVICETYPE_QHY290C: u32 = 4029;
pub const DEVICETYPE_QHY5III236: u32 = 4030;
pub const DEVICETYPE_QHY5III236C: u32 = 4031;
pub const DEVICETYPE_QHY5III236M: u32 = 4032;
pub const DEVICETYPE_QHY236: u32 = 4033;
pub const DEVICETYPE_QHY236M: u32 = 4034;
pub const DEVICETYPE_QHY236C: u32 = 4035;
pub const DEVICETYPE_QHY5IIIG400M: u32 = 4036;
pub const DEVICETYPE_QHY163: u32 = 4037;
pub const DEVICETYPE_QHY163M: u32 = 4038;
pub const DEVICETYPE_QHY163C: u32 = 4039;
pub const DEVICETYPE_QHY165: u32 = 4040;
pub const DEVICETYPE_QHY165C: u32 = 4041;
pub const DEVICETYPE_QHY367: u32 = 4042;
pub const DEVICETYPE_QHY367C: u32 = 4043;
pub const DEVICETYPE_QHY183: u32 = 4044;
pub const DEVICETYPE_QHY183C: u32 = 4045;
pub const DEVICETYPE_QHY5IIICOMMON: u32 = 4046;
pub const DEVICETYPE_QHY247: u32 = 4047;
pub const DEVICETYPE_QHY247C: u32 = 4048;
pub const DEVICETYPE_MINICAM6F: u32 = 4049;
pub const DEVICETYPE_QHY168: u32 = 4050;
pub const DEVICETYPE_QHY168C: u32 = 4051;
pub const DEVICETYPE_QHY128: u32 = 4052;
pub const DEVICETYPE_QHY128C: u32 = 4053;
pub const DEVICETYPE_QHY294: u32 = 4054;
pub const DEVICETYPE_QHY2020: u32 = 4055;
pub const DEVICETYPE_QHY4040: u32 = 4056;
pub const DEVICETYPE_QHY550: u32 = 4057;
pub const DEVICETYPE_QHY42PRO: u32 = 4058;
pub const DEVICETYPE_QHY6060: u32 = 4059;
pub const DEVICETYPE_QHY411: u32 = 4060;
pub const DEVICETYPE_QHY600: u32 = 4061;
pub const DEVICETYPE_QHY600C: u32 = 4062;
pub const DEVICETYPE_QHY600M: u32 = 4063;
pub const DEVICETYPE_QHY0204: u32 = 4064;
pub const DEVICETYPE_QHY411ERIS: u32 = 4065;
pub const DEVICETYPE_QHY411MERIS: u32 = 4066;
pub const DEVICETYPE_QHY411CERIS: u32 = 4067;
pub const DEVICETYPE_QHY367PROC: u32 = 4068;
pub const DEVICETYPE_QHY268C: u32 = 4069;
pub const DEVICETYPE_QHY410C: u32 = 4070;
pub const DEVICETYPE_QHY432: u32 = 4071;
pub const DEVICETYPE_QHY342: u32 = 4072;
pub const DEVICETYPE_QHY4040PRO: u32 = 4073;
pub const DEVICETYPE_QHY128PROC: u32 = 4074;
pub const DEVICETYPE_QHY5III462: u32 = 4075;
pub const DEVICETYPE_QHY5III462C: u32 = 4076;
pub const DEVICETYPE_QHY5III462M: u32 = 4077;
pub const DEVICETYPE_QHY533C: u32 = 4078;
pub const DEVICETYPE_QHY492M: u32 = 4079;
pub const DEVICETYPE_QHY461: u32 = 4080;
pub const DEVICETYPE_QHY461M: u32 = 4081;
pub const DEVICETYPE_QHY461C: u32 = 4082;
pub const DEVICETYPE_QHY492MT: u32 = 4083;
pub const DEVICETYPE_QHY5III485: u32 = 4084;
pub const DEVICETYPE_QHY294PRO: u32 = 4085;
pub const DEVICETYPE_QHY294MPRO: u32 = 4086;
pub const DEVICETYPE_QHY294CPRO: u32 = 4087;
pub const DEVICETYPE_QHY4040PRO_F: u32 = 4088;
pub const DEVICETYPE_QHY4040PRO_B: u32 = 4089;
pub const DEVICETYPE_QHY268M: u32 = 4090;
pub const DEVICETYPE_QHY4040PRO_FN: u32 = 4091;
pub const DEVICETYPE_QHY4040PRO_BN: u32 = 4092;
pub const DEVICETYPE_QHY990: u32 = 4093;
pub const DEVICETYPE_QHY5III178C_Celestron: u32 = 4094;
pub const DEVICETYPE_QHY5LII_C_OrionAllInOne: u32 = 4095;
pub const DEVICETYPE_QHY5LII_M_Orion_StarShoot: u32 = 4096;
pub const DEVICETYPE_QHY550_PM: u32 = 4097;
pub const DEVICETYPE_QHY550_PC: u32 = 4098;
pub const DEVICETYPE_QHY550_M: u32 = 4099;
pub const DEVICETYPE_QHY550_C: u32 = 4100;
pub const DEVICETYPE_QHY5III482C: u32 = 4101;
pub const DEVICETYPE_QHY5III464: u32 = 4102;
pub const DEVICETYPE_QHY183A: u32 = 4105;
pub const DEVICETYPE_QHY183A_M: u32 = 4106;
pub const DEVICETYPE_QHY183A_C: u32 = 4107;
pub const DEVICETYPE_QHY5III334: u32 = 4108;
pub const DEVICETYPE_QHY5III334M: u32 = 4109;
pub const DEVICETYPE_QHY5III334C: u32 = 4110;
pub const DEVICETYPE_QHY991: u32 = 4111;
pub const DEVICETYPE_QHY1253: u32 = 4112;
pub const DEVICETYPE_QHY5III415: u32 = 4113;
pub const DEVICETYPE_QHY5IIIEND: u32 = 4999;
pub const DEVICETYPE_QHY16: u32 = 16;
pub const DEVICETYPE_QHY6: u32 = 60;
pub const DEVICETYPE_QHY7: u32 = 70;
pub const DEVICETYPE_QHY2PRO: u32 = 221;
pub const DEVICETYPE_IMG2P: u32 = 220;
pub const DEVICETYPE_QHY8: u32 = 400;
pub const DEVICETYPE_QHY8PRO: u32 = 453;
pub const DEVICETYPE_QHY16000: u32 = 361;
pub const DEVICETYPE_QHY12: u32 = 613;
pub const DEVICETYPE_IC8300: u32 = 890;
pub const DEVICETYPE_QHY9S: u32 = 892;
pub const DEVICETYPE_QHY10: u32 = 893;
pub const DEVICETYPE_QHY8L: u32 = 891;
pub const DEVICETYPE_QHY11: u32 = 894;
pub const DEVICETYPE_QHY21: u32 = 895;
pub const DEVICETYPE_QHY22: u32 = 896;
pub const DEVICETYPE_QHY23: u32 = 897;
pub const DEVICETYPE_QHY15: u32 = 898;
pub const DEVICETYPE_QHY27: u32 = 899;
pub const DEVICETYPE_QHY28: u32 = 902;
pub const DEVICETYPE_QHY9T: u32 = 905;
pub const DEVICETYPE_QHY29: u32 = 907;
pub const DEVICETYPE_SOLAR1600: u32 = 908;
pub const DEVICETYPE_90A: u32 = 900;
pub const DEVICETYPE_16200A: u32 = 901;
pub const DEVICETYPE_814A: u32 = 903;
pub const DEVICETYPE_16803: u32 = 906;
pub const DEVICETYPE_09000: u32 = 930;
pub const DEVICETYPE_695A: u32 = 916;
pub const DEVICETYPE_QHY15G: u32 = 9000;
pub const DEVICETYPE_SOLAR800G: u32 = 9001;
pub const DEVICETYPE_A0340G: u32 = 9003;
pub const DEVICETYPE_QHY08050G: u32 = 9004;
pub const DEVICETYPE_QHY694G: u32 = 9005;
pub const DEVICETYPE_QHY27G: u32 = 9006;
pub const DEVICETYPE_QHY23G: u32 = 9007;
pub const DEVICETYPE_QHY16000G: u32 = 9008;
pub const DEVICETYPE_QHY160002AD: u32 = 9009;
pub const DEVICETYPE_QHY814G: u32 = 9010;
pub const DEVICETYPE_QHY45GX: u32 = 9011;
pub const DEVICETYPE_QHY10_FOCUS: u32 = 9012;
pub const DEVICETYPE_QHY50GX: u32 = 9013;
pub const DEVICETYPE_QHYPCIEBEGIN: u32 = 1712848896;
pub const DEVICETYPE_QHY4040PROPCIE: u32 = 1712865347;
pub const DEVICETYPE_QHY411ERISPCIE: u32 = 1712899092;
pub const DEVICETYPE_QHY600PCIE: u32 = 1712899587;
pub const DEVICETYPE_QHY268PCIE: u32 = 1712898665;
pub const DEVICETYPE_QHY461PCIE: u32 = 1712899170;
pub const DEVICETYPE_QHYPCIEEND: u32 = 1712914431;
pub const IMG132E_MAX_WIDTH: u32 = 1280;
pub const IMG132E_MAX_HEIGHT: u32 = 1024;
pub const IMG0H_MAX_WIDTH: u32 = 640;
pub const IMG0H_MAX_HEIGHT: u32 = 480;
pub const IMG0L_MAX_WIDTH: i32 = -1;
pub const IMG0L_MAX_HEIGHT: i32 = -1;
pub const IMG0X_MAX_WIDTH: i32 = -1;
pub const IMG0X_MAX_HEIGHT: i32 = -1;
pub const IMG1S_MAX_WIDTH: i32 = -1;
pub const IMG1S_MAX_HEIGHT: i32 = -1;
pub const IMG2S_MAX_WIDTH: i32 = -1;
pub const IMG2S_MAX_HEIGHT: i32 = -1;
pub const IMG1E_MAX_WIDTH: i32 = -1;
pub const IMG1E_MAX_HEIGHT: i32 = -1;
pub const QHY5_MAX_WIDTH: u32 = 1280;
pub const QHY5_MAX_HEIGHT: u32 = 1024;
pub const QHY5II_MAX_WIDTH: u32 = 1280;
pub const QHY5II_MAX_HEIGHT: u32 = 1024;
pub const QHY5LII_M_MAX_WIDTH: u32 = 1280;
pub const QHY5LII_M_MAX_HEIGHT: u32 = 960;
pub const QHY5LII_C_MAX_WIDTH: u32 = 1280;
pub const QHY5LII_C_MAX_HEIGHT: u32 = 960;
pub const QHY5TII_MAX_WIDTH: i32 = -1;
pub const QHY5TII_MAX_HEIGHT: i32 = -1;
pub const QHY5RII_MAX_WIDTH: i32 = -1;
pub const QHY5RII_MAX_HEIGHT: i32 = -1;
pub const QHY5PII_MAX_WIDTH: i32 = -1;
pub const QHY5PII_MAX_HEIGHT: i32 = -1;
pub const QHY5VII_MAX_WIDTH: i32 = -1;
pub const QHY5VII_MAX_HEIGHT: i32 = -1;
pub const QHY5HII_MAX_WIDTH: u32 = 1280;
pub const QHY5HII_MAX_HEIGHT: u32 = 960;
pub const MINICAM5S_M_MAX_WIDTH: u32 = 1280;
pub const MINICAM5S_M_MAX_HEIGHT: u32 = 960;
pub const MINICAM5S_C_MAX_WIDTH: u32 = 1280;
pub const MINICAM5S_C_MAX_HEIGHT: u32 = 960;
pub const QHY5PII_C_MAX_WIDTH: u32 = 2592;
pub const QHY5PII_C_MAX_HEIGHT: u32 = 1944;
pub const QHY5RII_C_MAX_WIDTH: u32 = 728;
pub const QHY5RII_C_MAX_HEIGHT: u32 = 512;
pub const MINICAM5F_M_MAX_WIDTH: u32 = 1280;
pub const MINICAM5F_M_MAX_HEIGHT: u32 = 960;
pub const QHY5PII_M_MAX_WIDTH: u32 = 2592;
pub const QHY5PII_M_MAX_HEIGHT: u32 = 1944;
pub const QHY5TII_C_MAX_WIDTH: u32 = 2048;
pub const QHY5TII_C_MAX_HEIGHT: u32 = 1536;
pub const POLEMASTER_MAX_WIDTH: u32 = 1280;
pub const POLEMASTER_MAX_HEIGHT: u32 = 960;
pub const QHY5III174_MAX_WIDTH: u32 = 1920;
pub const QHY5III174_MAX_HEIGHT: u32 = 1200;
pub const QHY5III174M_MAX_WIDTH: u32 = 1920;
pub const QHY5III174M_MAX_HEIGHT: u32 = 1200;
pub const QHY5III174C_MAX_WIDTH: u32 = 1920;
pub const QHY5III174C_MAX_HEIGHT: u32 = 1200;
pub const QHY174_MAX_WIDTH: u32 = 1920;
pub const QHY174_MAX_HEIGHT: u32 = 1200;
pub const QHY174M_MAX_WIDTH: u32 = 1920;
pub const QHY174M_MAX_HEIGHT: u32 = 1200;
pub const QHY174C_MAX_WIDTH: u32 = 1920;
pub const QHY174C_MAX_HEIGHT: u32 = 1200;
pub const QHY5III174BASE_MAX_WIDTH: u32 = 1920;
pub const QHY5III174BASE_MAX_HEIGHT: u32 = 1200;
pub const QHY5III178_MAX_WIDTH: u32 = 3056;
pub const QHY5III178_MAX_HEIGHT: u32 = 2048;
pub const QHY5III178C_MAX_WIDTH: u32 = 3056;
pub const QHY5III178C_MAX_HEIGHT: u32 = 2048;
pub const QHY5III178M_MAX_WIDTH: u32 = 3056;
pub const QHY5III178M_MAX_HEIGHT: u32 = 2048;
pub const QHY5III178BASE_MAX_WIDTH: u32 = 3056;
pub const QHY5III178BASE_MAX_HEIGHT: u32 = 2048;
pub const QHY178_MAX_WIDTH: u32 = 3056;
pub const QHY178_MAX_HEIGHT: u32 = 2048;
pub const QHY178M_MAX_WIDTH: u32 = 3056;
pub const QHY178M_MAX_HEIGHT: u32 = 2048;
pub const QHY178C_MAX_WIDTH: u32 = 3056;
pub const QHY178C_MAX_HEIGHT: u32 = 2048;
pub const QHY5III178COOLBASE_MAX_WIDTH: u32 = 3056;
pub const QHY5III178COOLBASE_MAX_HEIGHT: u32 = 2048;
pub const QHY5III185_MAX_WIDTH: u32 = 1920;
pub const QHY5III185_MAX_HEIGHT: u32 = 1200;
pub const QHY5III185C_MAX_WIDTH: u32 = 1920;
pub const QHY5III185C_MAX_HEIGHT: u32 = 1200;
pub const QHY5III185M_MAX_WIDTH: u32 = 1920;
pub const QHY5III185M_MAX_HEIGHT: u32 = 1200;
pub const QHY5III185BASE_MAX_WIDTH: u32 = 1920;
pub const QHY5III185BASE_MAX_HEIGHT: u32 = 1200;
pub const QHY185_MAX_WIDTH: i32 = -1;
pub const QHY185_MAX_HEIGHT: i32 = -1;
pub const QHY185M_MAX_WIDTH: i32 = -1;
pub const QHY185M_MAX_HEIGHT: i32 = -1;
pub const QHY185C_MAX_WIDTH: i32 = -1;
pub const QHY185C_MAX_HEIGHT: i32 = -1;
pub const QHY5III224_MAX_WIDTH: u32 = 1280;
pub const QHY5III224_MAX_HEIGHT: u32 = 960;
pub const QHY5III224C_MAX_WIDTH: u32 = 1280;
pub const QHY5III224C_MAX_HEIGHT: u32 = 960;
pub const QHY5III224M_MAX_WIDTH: u32 = 1280;
pub const QHY5III224M_MAX_HEIGHT: u32 = 960;
pub const QHY5III224BASE_MAX_WIDTH: u32 = 1280;
pub const QHY5III224BASE_MAX_HEIGHT: u32 = 960;
pub const QHY224_MAX_WIDTH: u32 = 1280;
pub const QHY224_MAX_HEIGHT: u32 = 960;
pub const QHY224M_MAX_WIDTH: u32 = 1280;
pub const QHY224M_MAX_HEIGHT: u32 = 960;
pub const QHY224C_MAX_WIDTH: u32 = 1280;
pub const QHY224C_MAX_HEIGHT: u32 = 960;
pub const QHY5III224COOLBASE_MAX_WIDTH: u32 = 1280;
pub const QHY5III224COOLBASE_MAX_HEIGHT: u32 = 960;
pub const QHY5III290_MAX_WIDTH: u32 = 1920;
pub const QHY5III290_MAX_HEIGHT: u32 = 1080;
pub const QHY5III290C_MAX_WIDTH: u32 = 1920;
pub const QHY5III290C_MAX_HEIGHT: u32 = 1080;
pub const QHY5III290M_MAX_WIDTH: u32 = 1920;
pub const QHY5III290M_MAX_HEIGHT: u32 = 1080;
pub const QHY5III290BASE_MAX_WIDTH: u32 = 1920;
pub const QHY5III290BASE_MAX_HEIGHT: u32 = 1080;
pub const QHY290_MAX_WIDTH: u32 = 1920;
pub const QHY290_MAX_HEIGHT: u32 = 1080;
pub const QHY290M_MAX_WIDTH: u32 = 1920;
pub const QHY290M_MAX_HEIGHT: u32 = 1080;
pub const QHY290C_MAX_WIDTH: u32 = 1920;
pub const QHY290C_MAX_HEIGHT: u32 = 1080;
pub const QHY5III290COOLBASE_MAX_WIDTH: u32 = 1920;
pub const QHY5III290COOLBASE_MAX_HEIGHT: u32 = 1080;
pub const QHY5III236_MAX_WIDTH: u32 = 1952;
pub const QHY5III236_MAX_HEIGHT: u32 = 1237;
pub const QHY5III236C_MAX_WIDTH: u32 = 1952;
pub const QHY5III236C_MAX_HEIGHT: u32 = 1237;
pub const QHY5III236M_MAX_WIDTH: u32 = 1952;
pub const QHY5III236M_MAX_HEIGHT: u32 = 1237;
pub const QHY236_MAX_WIDTH: i32 = -1;
pub const QHY236_MAX_HEIGHT: i32 = -1;
pub const QHY236M_MAX_WIDTH: i32 = -1;
pub const QHY236M_MAX_HEIGHT: i32 = -1;
pub const QHY236C_MAX_WIDTH: i32 = -1;
pub const QHY236C_MAX_HEIGHT: i32 = -1;
pub const QHY5IIIG400M_MAX_WIDTH: u32 = 2048;
pub const QHY5IIIG400M_MAX_HEIGHT: u32 = 2048;
pub const QHY163_MAX_WIDTH: u32 = 4656;
pub const QHY163_MAX_HEIGHT: u32 = 3522;
pub const QHY163M_MAX_WIDTH: u32 = 4656;
pub const QHY163M_MAX_HEIGHT: u32 = 3522;
pub const QHY163C_MAX_WIDTH: u32 = 4656;
pub const QHY163C_MAX_HEIGHT: u32 = 3522;
pub const QHY165_MAX_WIDTH: u32 = 4936;
pub const QHY165_MAX_HEIGHT: u32 = 3286;
pub const QHY165C_MAX_WIDTH: u32 = 4936;
pub const QHY165C_MAX_HEIGHT: u32 = 3286;
pub const QHY367_MAX_WIDTH: u32 = 7380;
pub const QHY367_MAX_HEIGHT: u32 = 4908;
pub const QHY367C_MAX_WIDTH: u32 = 7380;
pub const QHY367C_MAX_HEIGHT: u32 = 4908;
pub const QHY183_MAX_WIDTH: u32 = 5544;
pub const QHY183_MAX_HEIGHT: u32 = 3684;
pub const QHY183C_MAX_WIDTH: u32 = 5544;
pub const QHY183C_MAX_HEIGHT: u32 = 3684;
pub const QHY5IIICOMMON_MAX_WIDTH: u32 = 4144;
pub const QHY5IIICOMMON_MAX_HEIGHT: u32 = 3064;
pub const QHY247_MAX_WIDTH: u32 = 6088;
pub const QHY247_MAX_HEIGHT: u32 = 4052;
pub const QHY247C_MAX_WIDTH: u32 = 6088;
pub const QHY247C_MAX_HEIGHT: u32 = 4052;
pub const QHY5III247BASE_MAX_WIDTH: u32 = 6088;
pub const QHY5III247BASE_MAX_HEIGHT: u32 = 4052;
pub const MINICAM6F_MAX_WIDTH: u32 = 3056;
pub const MINICAM6F_MAX_HEIGHT: u32 = 2048;
pub const QHY168_MAX_WIDTH: u32 = 5056;
pub const QHY168_MAX_HEIGHT: u32 = 3358;
pub const QHY168C_MAX_WIDTH: u32 = 5056;
pub const QHY168C_MAX_HEIGHT: u32 = 3358;
pub const QHY5III168BASE_MAX_WIDTH: u32 = 5056;
pub const QHY5III168BASE_MAX_HEIGHT: u32 = 3358;
pub const QHY128_MAX_WIDTH: u32 = 6036;
pub const QHY128_MAX_HEIGHT: u32 = 4036;
pub const QHY128C_MAX_WIDTH: u32 = 6036;
pub const QHY128C_MAX_HEIGHT: u32 = 4036;
pub const QHY294_MAX_WIDTH: u32 = 4212;
pub const QHY294_MAX_HEIGHT: u32 = 2850;
pub const QHY294_M_C_PRO_MAX_WIDTH_Mod_14_11M: u32 = 4212;
pub const QHY294_M_C_PRO_MAX_HEIGHT_Mod_14_11M: u32 = 2850;
pub const QHY294_M_C_PRO_MAX_WIDTH_Mod_12_47M: u32 = 8432;
pub const QHY294_M_C_PRO_MAX_HEIGHT_Mod_12_47M: u32 = 5648;
pub const QHY492_MAX_WIDTH: u32 = 4212;
pub const QHY492_MAX_HEIGHT: u32 = 2850;
pub const QHY2020_MAX_WIDTH: u32 = 4096;
pub const QHY2020_MAX_HEIGHT: u32 = 2048;
pub const QHY4040_MAX_WIDTH: u32 = 4096;
pub const QHY4040_MAX_HEIGHT: u32 = 4118;
pub const QHY4040PRO_MAX_WIDTH: u32 = 4096;
pub const QHY4040PRO_MAX_HEIGHT: u32 = 4118;
pub const QHY550_MAX_WIDTH: u32 = 2496;
pub const QHY550_MAX_HEIGHT: u32 = 2080;
pub const QHY990_MAX_WIDTH: u32 = 1408;
pub const QHY990_MAX_HEIGHT: u32 = 1052;
pub const QHY991_MAX_WIDTH: u32 = 768;
pub const QHY991_MAX_HEIGHT: u32 = 540;
pub const QHY42PRO_MAX_WIDTH: u32 = 4096;
pub const QHY42PRO_MAX_HEIGHT: u32 = 2048;
pub const QHY6060_MAX_WIDTH: u32 = 7936;
pub const QHY6060_MAX_HEIGHT: u32 = 6134;
pub const QHY411_MAX_WIDTH: u32 = 14304;
pub const QHY411_MAX_HEIGHT: u32 = 10748;
pub const QHY411ERIS_MAX_WIDTH: u32 = 14304;
pub const QHY411ERIS_MAX_HEIGHT: u32 = 10748;
pub const QHY600_MAX_WIDTH: u32 = 9600;
pub const QHY600_MAX_HEIGHT: u32 = 6422;
pub const QHY461_MAX_WIDTH: u32 = 11760;
pub const QHY461_MAX_HEIGHT: u32 = 8842;
pub const QHY268C_MAX_WIDTH: u32 = 6280;
pub const QHY268C_MAX_HEIGHT: u32 = 4210;
pub const QHY410C_MAX_WIDTH: u32 = 6112;
pub const QHY410C_MAX_HEIGHT: u32 = 4040;
pub const QHY432_MAX_WIDTH: u32 = 1624;
pub const QHY432_MAX_HEIGHT: u32 = 1136;
pub const QHY342_MAX_WIDTH: u32 = 6512;
pub const QHY342_MAX_HEIGHT: u32 = 4870;
pub const QHY16_MAX_WIDTH: u32 = 4144;
pub const QHY16_MAX_HEIGHT: u32 = 4128;
pub const QHY6_MAX_WIDTH: u32 = 800;
pub const QHY6_MAX_HEIGHT: u32 = 596;
pub const QHY7_MAX_WIDTH: u32 = 2112;
pub const QHY7_MAX_HEIGHT: u32 = 2072;
pub const QHY2PRO_MAX_WIDTH: u32 = 1440;
pub const QHY2PRO_MAX_HEIGHT: u32 = 1050;
pub const IMG2P_MAX_WIDTH: u32 = 1436;
pub const IMG2P_MAX_HEIGHT: u32 = 1050;
pub const QHY8_MAX_WIDTH: u32 = 3328;
pub const QHY8_MAX_HEIGHT: u32 = 2030;
pub const QHY8PRO_MAX_WIDTH: u32 = 3328;
pub const QHY8PRO_MAX_HEIGHT: u32 = 2030;
pub const QHY16000_MAX_WIDTH: u32 = 4960;
pub const QHY16000_MAX_HEIGHT: u32 = 3328;
pub const QHY12_MAX_WIDTH: u32 = 3328;
pub const QHY12_MAX_HEIGHT: u32 = 4640;
pub const IC8300_MAX_WIDTH: u32 = 3584;
pub const IC8300_MAX_HEIGHT: u32 = 2576;
pub const QHY9S_MAX_WIDTH: u32 = 3584;
pub const QHY9S_MAX_HEIGHT: u32 = 2574;
pub const QHY10_MAX_WIDTH: u32 = 2816;
pub const QHY10_MAX_HEIGHT: u32 = 3940;
pub const QHY8L_MAX_WIDTH: u32 = 3328;
pub const QHY8L_MAX_HEIGHT: u32 = 2030;
pub const QHY11_MAX_WIDTH: u32 = 4096;
pub const QHY11_MAX_HEIGHT: u32 = 2720;
pub const QHY21_MAX_WIDTH: u32 = 2048;
pub const QHY21_MAX_HEIGHT: u32 = 1500;
pub const QHY23_MAX_WIDTH: u32 = 3584;
pub const QHY23_MAX_HEIGHT: u32 = 2728;
pub const QHY15_MAX_WIDTH: u32 = 3108;
pub const QHY15_MAX_HEIGHT: u32 = 3086;
pub const QHY27_MAX_WIDTH: u32 = 5120;
pub const QHY27_MAX_HEIGHT: u32 = 3332;
pub const QHY28_MAX_WIDTH: u32 = 5120;
pub const QHY28_MAX_HEIGHT: u32 = 3332;
pub const QHY9T_MAX_WIDTH: u32 = 3584;
pub const QHY9T_MAX_HEIGHT: u32 = 2574;
pub const QHY29_MAX_WIDTH: u32 = 6656;
pub const QHY29_MAX_HEIGHT: u32 = 4452;
pub const SOLAR1600_MAX_WIDTH: u32 = 4928;
pub const SOLAR1600_MAX_HEIGHT: u32 = 3264;
pub const QHY90A_MAX_WIDTH: u32 = 3584;
pub const QHY90A_MAX_HEIGHT: u32 = 2576;
pub const QHY16200A_MAX_WIDTH: u32 = 5120;
pub const QHY16200A_MAX_HEIGHT: u32 = 3696;
pub const QHY814A_MAX_WIDTH: u32 = 3584;
pub const QHY814A_MAX_HEIGHT: u32 = 2720;
pub const IC16803_MAX_WIDTH: u32 = 4192;
pub const IC16803_MAX_HEIGHT: u32 = 4128;
pub const QHY09000_MAX_WIDTH: u32 = 3140;
pub const QHY09000_MAX_HEIGHT: u32 = 3096;
pub const QHY695A_MAX_WIDTH: u32 = 3072;
pub const QHY695A_MAX_HEIGHT: u32 = 2240;
pub const QHY15G_MAX_WIDTH: i32 = -1;
pub const QHY15G_MAX_HEIGHT: i32 = -1;
pub const SOLAR800G_MAX_WIDTH: u32 = 3584;
pub const SOLAR800G_MAX_HEIGHT: u32 = 2574;
pub const A0340G_MAX_WIDTH: i32 = -1;
pub const A0340G_MAX_HEIGHT: i32 = -1;
pub const QHY08050G_MAX_WIDTH: u32 = 3378;
pub const QHY08050G_MAX_HEIGHT: u32 = 2560;
pub const QHY694G_MAX_WIDTH: i32 = -1;
pub const QHY694G_MAX_HEIGHT: i32 = -1;
pub const QHY27G_MAX_WIDTH: i32 = -1;
pub const QHY27G_MAX_HEIGHT: i32 = -1;
pub const QHY23G_MAX_WIDTH: i32 = -1;
pub const QHY23G_MAX_HEIGHT: i32 = -1;
pub const QHY16000G_MAX_WIDTH: u32 = 4960;
pub const QHY16000G_MAX_HEIGHT: u32 = 3328;
pub const QHY160002AD_MAX_WIDTH: u32 = 5120;
pub const QHY160002AD_MAX_HEIGHT: u32 = 3328;
pub const QHY814G_MAX_WIDTH: u32 = 3584;
pub const QHY814G_MAX_HEIGHT: u32 = 2728;
pub const QHY45GX_MAX_WIDTH: u32 = 1280;
pub const QHY45GX_MAX_HEIGHT: u32 = 1024;
pub const QHY50GX_MAX_WIDTH: u32 = 8400;
pub const QHY50GX_MAX_HEIGHT: u32 = 6220;
pub const DEVICETYPE_UNKNOW: i32 = -1;
pub const MAX_EXPOSURE_TIMES: u32 = 3600;
pub const REMAINING_MIN_EXPOSURETIMES: u32 = 3000;
pub const MAX_CAMERA_NUMBER: u32 = 200;
pub const CAMERA_ID_LENGTH: u32 = 64;
pub const MAXDEVICES: u32 = 10;
pub const MAX_READMODE_NAME: u32 = 256;
pub const MAX_READMODE_CAM_NUMBER: u32 = 6;
pub const SEND_CAMERA_NO_MESSAGE: u32 = 0;
pub const SEND_CAMERA_ID: u32 = 9501;
pub const SEND_CAMERA_STATUS: u32 = 9502;
pub const SEND_CAMERA_LIVE: u32 = 9503;
pub const SEND_CAMERA_SINGLE: u32 = 9504;
pub const SEND_CAMERA_CAPABLE: u32 = 9505;
pub const SEND_CAMERA_MAXMINSTEP: u32 = 9506;
pub const SEND_CAMERA_EXPOSING: u32 = 9507;
pub const CAMERA_HANDLE_INVALID: u32 = 9508;
pub const HANDLE_IS_NULL: u32 = 9509;
pub const USB_PORT_ERROR: u32 = 9510;
pub const CAMERA_LOSE: u32 = 10001;
pub const CAMERA_INSERT: u32 = 10002;
pub const CAMERA_LIVE_ERROR: u32 = 10003;
pub const CAMERA_LIVE_DATA_AVAILABLE: u32 = 10004;
pub const CAMERA_LIVE_STOP: u32 = 10005;
pub const CAMERA_LIVE_TIMEOUT: u32 = 10006;
pub const CAMERA_SINGLE_ERROR: u32 = 10011;
pub const CAMERA_SINGLE_DATA_AVAILABLE: u32 = 10012;
pub const CAMERA_SINGLE_STOP: u32 = 10013;
pub const CAMERA_SINGLE_TIMEOUT: u32 = 10014;
pub const SEND_MSG_NONE: u32 = 0;
pub const SEND_MSG_WHOLE: u32 = 1;
pub const SEND_MSG_PART: u32 = 2;
pub const SINGLE_MODE: u32 = 0;
pub const LIVE_MODE: u32 = 1;
pub const FLASH_CONFIG_PAGE_INDEX: u32 = 1;
pub const FPGA_MODE_DEFAULT: u32 = 99;
pub const FPGA_MODE_12_47M: u32 = 12;
pub const FPGA_MODE_14_11M: u32 = 14;
pub const GET_IMAGE_TIMEOUT: u32 = 60000;
pub const IS_OPERATOR_NONE: u32 = 0;
pub const IS_CAMARA_INIT: u32 = 1;
pub const IS_CAMARA_OPEN: u32 = 2;
pub const IS_CAMARA_CLOSE: u32 = 3;
pub const IS_CAMARA_STOP_LIVE: u32 = 4;
pub const IS_CAMARA_CAN_EXP: u32 = 5;
pub const IS_CAMARA_ERROR: u32 = 6;
pub const IS_GET_SINGLEPICTURE: u32 = 7;
pub const IS_GET_LIVEPICTURE: u32 = 8;
pub const COMMAND_QUEUE_PUSH: u32 = 1;
pub const COMMAND_QUEUE_POP: u32 = 2;
pub const DEMO_MAXDEVICES: u32 = 6;
pub const MESSAGE_SIZS: u32 = 1000;
pub const CFWSLOTS_NUM9: u32 = 9;
pub const CFWSLOTS_NUM8: u32 = 8;
pub const CFWSLOTS_NUM7: u32 = 7;
pub const CFWSLOTS_NUM6: u32 = 6;
pub const CFWSLOTS_DELAY: u32 = 4000;
pub const RESET_USB_PIPE: u32 = 1;
pub const ABORT_USB_PIPE: u32 = 2;
pub const MAX_PCIE_CHANNEL: u32 = 4;
pub const FPGA_CAMARA_LENGTH: u32 = 256;
pub const CAM_SCAN_INTERVAL: u32 = 1000;
pub const CRC32_POLY: u32 = 79764919;
pub const PCIE_CARD_HEAD: u32 = 1441704652;
pub const PCIE_CARD_READ_C: u32 = 21845;
pub const PCIE_CARD_WRITE_C: u32 = 13107;
pub const PCIE_COMMAND_HEAD: u32 = 2867737378;
pub const PCIE_FPGA_WRITE_C: u32 = 34945;
pub const PCIE_NIOS_READ_C: u32 = 30576;
pub const PCIE_NIOS_WRITE_C: u32 = 30577;
pub const PCIE_CAPTURE_WRITE_C: u32 = 1717986917;
pub const PCIE_DWORD: u32 = 4;
pub const FRAME_BUFF_NUM: u32 = 3;
pub const IS_WRITE: u32 = 1;
pub const IS_READ: u32 = 2;
pub const CAM_CONN_STATUS_TO_NOTHING: u32 = 1;
pub const CAM_CONN_STATUS_TO_PC: u32 = 2;
pub const CAM_CONN_STATUS_TO_SDK: u32 = 3;
pub const CALLBACK_MODE_SUPPORT: u32 = 1;
pub const IMAGEQUEUE_ORIG_MODE: u32 = 1;
pub const version_year: u32 = 21;
pub const version_month: u32 = 9;
pub const version_day: u32 = 10;
pub const version_subday: u32 = 19;
pub const SDK_SVN_REVISION: u32 = 11368;
pub const WINDOWS_PTHREAD_SUPPORT: u32 = 0;
pub const WINPCAP_MODE_SUPPORT: u32 = 0;
pub const PCIE_MODE_SUPPORT: u32 = 1;
pub const CYUSB_MODE_SUPPORT: u32 = 0;
pub const WINUSB_MODE_SUPPORT: u32 = 0;
pub const LIBUSB_MODE_SUPPORT: u32 = 1;
pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _ISOC95_SOURCE: u32 = 1;
pub const _ISOC99_SOURCE: u32 = 1;
pub const _ISOC11_SOURCE: u32 = 1;
pub const _ISOC2X_SOURCE: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const _XOPEN_SOURCE: u32 = 700;
pub const _XOPEN_SOURCE_EXTENDED: u32 = 1;
pub const _LARGEFILE64_SOURCE: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_ISOCXX11: u32 = 1;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const __USE_XOPEN: u32 = 1;
pub const __USE_XOPEN_EXTENDED: u32 = 1;
pub const __USE_UNIX98: u32 = 1;
pub const _LARGEFILE_SOURCE: u32 = 1;
pub const __USE_XOPEN2K8XSI: u32 = 1;
pub const __USE_XOPEN2KXSI: u32 = 1;
pub const __USE_LARGEFILE: u32 = 1;
pub const __USE_LARGEFILE64: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_GNU: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 31;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __LONG_DOUBLE_USES_FLOAT128: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 0;
pub const __GLIBC_USE_LIB_EXT2: u32 = 1;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 1;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 1;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 1;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 1;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 1;
pub const _BITS_TYPES_H: u32 = 1;
pub const __TIMESIZE: u32 = 64;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const INT8_WIDTH: u32 = 8;
pub const UINT8_WIDTH: u32 = 8;
pub const INT16_WIDTH: u32 = 16;
pub const UINT16_WIDTH: u32 = 16;
pub const INT32_WIDTH: u32 = 32;
pub const UINT32_WIDTH: u32 = 32;
pub const INT64_WIDTH: u32 = 64;
pub const UINT64_WIDTH: u32 = 64;
pub const INT_LEAST8_WIDTH: u32 = 8;
pub const UINT_LEAST8_WIDTH: u32 = 8;
pub const INT_LEAST16_WIDTH: u32 = 16;
pub const UINT_LEAST16_WIDTH: u32 = 16;
pub const INT_LEAST32_WIDTH: u32 = 32;
pub const UINT_LEAST32_WIDTH: u32 = 32;
pub const INT_LEAST64_WIDTH: u32 = 64;
pub const UINT_LEAST64_WIDTH: u32 = 64;
pub const INT_FAST8_WIDTH: u32 = 8;
pub const UINT_FAST8_WIDTH: u32 = 8;
pub const INT_FAST16_WIDTH: u32 = 64;
pub const UINT_FAST16_WIDTH: u32 = 64;
pub const INT_FAST32_WIDTH: u32 = 64;
pub const UINT_FAST32_WIDTH: u32 = 64;
pub const INT_FAST64_WIDTH: u32 = 64;
pub const UINT_FAST64_WIDTH: u32 = 64;
pub const INTPTR_WIDTH: u32 = 64;
pub const UINTPTR_WIDTH: u32 = 64;
pub const INTMAX_WIDTH: u32 = 64;
pub const UINTMAX_WIDTH: u32 = 64;
pub const PTRDIFF_WIDTH: u32 = 64;
pub const SIG_ATOMIC_WIDTH: u32 = 32;
pub const SIZE_WIDTH: u32 = 64;
pub const WCHAR_WIDTH: u32 = 32;
pub const WINT_WIDTH: u32 = 32;
pub const QHYCCD_REQUEST_READ: u32 = 192;
pub const QHYCCD_REQUEST_WRITE: u32 = 64;
pub const MACHANICALSHUTTER_OPEN: u32 = 0;
pub const MACHANICALSHUTTER_CLOSE: u32 = 1;
pub const MACHANICALSHUTTER_FREE: u32 = 2;
pub const MAX_READMODE_NUMBER: u32 = 8;
pub const MAX_READMODE_CAMARA_NUMBER: u32 = 8;
pub const _GLIBCXX_FUNCTIONAL: u32 = 1;
pub const _GLIBCXX_CXX_CONFIG_H: u32 = 1;
pub const _GLIBCXX_RELEASE: u32 = 9;
pub const __GLIBCXX__: u32 = 20200808;
pub const _GLIBCXX_HAVE_ATTRIBUTE_VISIBILITY: u32 = 1;
pub const _GLIBCXX_USE_DEPRECATED: u32 = 1;
pub const _GLIBCXX_EXTERN_TEMPLATE: u32 = 1;
pub const _GLIBCXX_USE_DUAL_ABI: u32 = 1;
pub const _GLIBCXX_USE_CXX11_ABI: u32 = 1;
pub const _GLIBCXX_INLINE_VERSION: u32 = 0;
pub const _GLIBCXX_USE_ALLOCATOR_NEW: u32 = 1;
pub const _GLIBCXX_OS_DEFINES: u32 = 1;
pub const __NO_CTYPE: u32 = 1;
pub const _GLIBCXX_CPU_DEFINES: u32 = 1;
pub const _GLIBCXX_FAST_MATH: u32 = 0;
pub const _GLIBCXX_USE_FLOAT128: u32 = 1;
pub const _GLIBCXX_HAVE_BUILTIN_HAS_UNIQ_OBJ_REP: u32 = 1;
pub const _GLIBCXX_HAVE_BUILTIN_IS_AGGREGATE: u32 = 1;
pub const _GLIBCXX_HAVE_BUILTIN_LAUNDER: u32 = 1;
pub const _GLIBCXX_HAVE_BUILTIN_IS_CONSTANT_EVALUATED: u32 = 1;
pub const _GLIBCXX_HAVE_ACOSF: u32 = 1;
pub const _GLIBCXX_HAVE_ACOSL: u32 = 1;
pub const _GLIBCXX_HAVE_ALIGNED_ALLOC: u32 = 1;
pub const _GLIBCXX_HAVE_ARPA_INET_H: u32 = 1;
pub const _GLIBCXX_HAVE_ASINF: u32 = 1;
pub const _GLIBCXX_HAVE_ASINL: u32 = 1;
pub const _GLIBCXX_HAVE_AS_SYMVER_DIRECTIVE: u32 = 1;
pub const _GLIBCXX_HAVE_ATAN2F: u32 = 1;
pub const _GLIBCXX_HAVE_ATAN2L: u32 = 1;
pub const _GLIBCXX_HAVE_ATANF: u32 = 1;
pub const _GLIBCXX_HAVE_ATANL: u32 = 1;
pub const _GLIBCXX_HAVE_ATOMIC_LOCK_POLICY: u32 = 1;
pub const _GLIBCXX_HAVE_AT_QUICK_EXIT: u32 = 1;
pub const _GLIBCXX_HAVE_CEILF: u32 = 1;
pub const _GLIBCXX_HAVE_CEILL: u32 = 1;
pub const _GLIBCXX_HAVE_COMPLEX_H: u32 = 1;
pub const _GLIBCXX_HAVE_COSF: u32 = 1;
pub const _GLIBCXX_HAVE_COSHF: u32 = 1;
pub const _GLIBCXX_HAVE_COSHL: u32 = 1;
pub const _GLIBCXX_HAVE_COSL: u32 = 1;
pub const _GLIBCXX_HAVE_DIRENT_H: u32 = 1;
pub const _GLIBCXX_HAVE_DLFCN_H: u32 = 1;
pub const _GLIBCXX_HAVE_EBADMSG: u32 = 1;
pub const _GLIBCXX_HAVE_ECANCELED: u32 = 1;
pub const _GLIBCXX_HAVE_ECHILD: u32 = 1;
pub const _GLIBCXX_HAVE_EIDRM: u32 = 1;
pub const _GLIBCXX_HAVE_ENDIAN_H: u32 = 1;
pub const _GLIBCXX_HAVE_ENODATA: u32 = 1;
pub const _GLIBCXX_HAVE_ENOLINK: u32 = 1;
pub const _GLIBCXX_HAVE_ENOSPC: u32 = 1;
pub const _GLIBCXX_HAVE_ENOSR: u32 = 1;
pub const _GLIBCXX_HAVE_ENOSTR: u32 = 1;
pub const _GLIBCXX_HAVE_ENOTRECOVERABLE: u32 = 1;
pub const _GLIBCXX_HAVE_ENOTSUP: u32 = 1;
pub const _GLIBCXX_HAVE_EOVERFLOW: u32 = 1;
pub const _GLIBCXX_HAVE_EOWNERDEAD: u32 = 1;
pub const _GLIBCXX_HAVE_EPERM: u32 = 1;
pub const _GLIBCXX_HAVE_EPROTO: u32 = 1;
pub const _GLIBCXX_HAVE_ETIME: u32 = 1;
pub const _GLIBCXX_HAVE_ETIMEDOUT: u32 = 1;
pub const _GLIBCXX_HAVE_ETXTBSY: u32 = 1;
pub const _GLIBCXX_HAVE_EWOULDBLOCK: u32 = 1;
pub const _GLIBCXX_HAVE_EXCEPTION_PTR_SINCE_GCC46: u32 = 1;
pub const _GLIBCXX_HAVE_EXECINFO_H: u32 = 1;
pub const _GLIBCXX_HAVE_EXPF: u32 = 1;
pub const _GLIBCXX_HAVE_EXPL: u32 = 1;
pub const _GLIBCXX_HAVE_FABSF: u32 = 1;
pub const _GLIBCXX_HAVE_FABSL: u32 = 1;
pub const _GLIBCXX_HAVE_FCNTL_H: u32 = 1;
pub const _GLIBCXX_HAVE_FENV_H: u32 = 1;
pub const _GLIBCXX_HAVE_FINITE: u32 = 1;
pub const _GLIBCXX_HAVE_FINITEF: u32 = 1;
pub const _GLIBCXX_HAVE_FINITEL: u32 = 1;
pub const _GLIBCXX_HAVE_FLOAT_H: u32 = 1;
pub const _GLIBCXX_HAVE_FLOORF: u32 = 1;
pub const _GLIBCXX_HAVE_FLOORL: u32 = 1;
pub const _GLIBCXX_HAVE_FMODF: u32 = 1;
pub const _GLIBCXX_HAVE_FMODL: u32 = 1;
pub const _GLIBCXX_HAVE_FREXPF: u32 = 1;
pub const _GLIBCXX_HAVE_FREXPL: u32 = 1;
pub const _GLIBCXX_HAVE_GETIPINFO: u32 = 1;
pub const _GLIBCXX_HAVE_GETS: u32 = 1;
pub const _GLIBCXX_HAVE_HYPOT: u32 = 1;
pub const _GLIBCXX_HAVE_HYPOTF: u32 = 1;
pub const _GLIBCXX_HAVE_HYPOTL: u32 = 1;
pub const _GLIBCXX_HAVE_ICONV: u32 = 1;
pub const _GLIBCXX_HAVE_INT64_T: u32 = 1;
pub const _GLIBCXX_HAVE_INT64_T_LONG: u32 = 1;
pub const _GLIBCXX_HAVE_INTTYPES_H: u32 = 1;
pub const _GLIBCXX_HAVE_ISINFF: u32 = 1;
pub const _GLIBCXX_HAVE_ISINFL: u32 = 1;
pub const _GLIBCXX_HAVE_ISNANF: u32 = 1;
pub const _GLIBCXX_HAVE_ISNANL: u32 = 1;
pub const _GLIBCXX_HAVE_ISWBLANK: u32 = 1;
pub const _GLIBCXX_HAVE_LC_MESSAGES: u32 = 1;
pub const _GLIBCXX_HAVE_LDEXPF: u32 = 1;
pub const _GLIBCXX_HAVE_LDEXPL: u32 = 1;
pub const _GLIBCXX_HAVE_LIBINTL_H: u32 = 1;
pub const _GLIBCXX_HAVE_LIMIT_AS: u32 = 1;
pub const _GLIBCXX_HAVE_LIMIT_DATA: u32 = 1;
pub const _GLIBCXX_HAVE_LIMIT_FSIZE: u32 = 1;
pub const _GLIBCXX_HAVE_LIMIT_RSS: u32 = 1;
pub const _GLIBCXX_HAVE_LIMIT_VMEM: u32 = 0;
pub const _GLIBCXX_HAVE_LINK: u32 = 1;
pub const _GLIBCXX_HAVE_LINUX_FUTEX: u32 = 1;
pub const _GLIBCXX_HAVE_LINUX_RANDOM_H: u32 = 1;
pub const _GLIBCXX_HAVE_LINUX_TYPES_H: u32 = 1;
pub const _GLIBCXX_HAVE_LOCALE_H: u32 = 1;
pub const _GLIBCXX_HAVE_LOG10F: u32 = 1;
pub const _GLIBCXX_HAVE_LOG10L: u32 = 1;
pub const _GLIBCXX_HAVE_LOGF: u32 = 1;
pub const _GLIBCXX_HAVE_LOGL: u32 = 1;
pub const _GLIBCXX_HAVE_MBSTATE_T: u32 = 1;
pub const _GLIBCXX_HAVE_MEMALIGN: u32 = 1;
pub const _GLIBCXX_HAVE_MEMORY_H: u32 = 1;
pub const _GLIBCXX_HAVE_MODF: u32 = 1;
pub const _GLIBCXX_HAVE_MODFF: u32 = 1;
pub const _GLIBCXX_HAVE_MODFL: u32 = 1;
pub const _GLIBCXX_HAVE_NETDB_H: u32 = 1;
pub const _GLIBCXX_HAVE_NETINET_IN_H: u32 = 1;
pub const _GLIBCXX_HAVE_NETINET_TCP_H: u32 = 1;
pub const _GLIBCXX_HAVE_POLL: u32 = 1;
pub const _GLIBCXX_HAVE_POLL_H: u32 = 1;
pub const _GLIBCXX_HAVE_POSIX_MEMALIGN: u32 = 1;
pub const _GLIBCXX_HAVE_POWF: u32 = 1;
pub const _GLIBCXX_HAVE_POWL: u32 = 1;
pub const _GLIBCXX_HAVE_QUICK_EXIT: u32 = 1;
pub const _GLIBCXX_HAVE_READLINK: u32 = 1;
pub const _GLIBCXX_HAVE_SETENV: u32 = 1;
pub const _GLIBCXX_HAVE_SINCOS: u32 = 1;
pub const _GLIBCXX_HAVE_SINCOSF: u32 = 1;
pub const _GLIBCXX_HAVE_SINCOSL: u32 = 1;
pub const _GLIBCXX_HAVE_SINF: u32 = 1;
pub const _GLIBCXX_HAVE_SINHF: u32 = 1;
pub const _GLIBCXX_HAVE_SINHL: u32 = 1;
pub const _GLIBCXX_HAVE_SINL: u32 = 1;
pub const _GLIBCXX_HAVE_SOCKATMARK: u32 = 1;
pub const _GLIBCXX_HAVE_SQRTF: u32 = 1;
pub const _GLIBCXX_HAVE_SQRTL: u32 = 1;
pub const _GLIBCXX_HAVE_STDALIGN_H: u32 = 1;
pub const _GLIBCXX_HAVE_STDBOOL_H: u32 = 1;
pub const _GLIBCXX_HAVE_STDINT_H: u32 = 1;
pub const _GLIBCXX_HAVE_STDLIB_H: u32 = 1;
pub const _GLIBCXX_HAVE_STRERROR_L: u32 = 1;
pub const _GLIBCXX_HAVE_STRERROR_R: u32 = 1;
pub const _GLIBCXX_HAVE_STRINGS_H: u32 = 1;
pub const _GLIBCXX_HAVE_STRING_H: u32 = 1;
pub const _GLIBCXX_HAVE_STRTOF: u32 = 1;
pub const _GLIBCXX_HAVE_STRTOLD: u32 = 1;
pub const _GLIBCXX_HAVE_STRUCT_DIRENT_D_TYPE: u32 = 1;
pub const _GLIBCXX_HAVE_STRXFRM_L: u32 = 1;
pub const _GLIBCXX_HAVE_SYMLINK: u32 = 1;
pub const _GLIBCXX_HAVE_SYMVER_SYMBOL_RENAMING_RUNTIME_SUPPORT: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_IOCTL_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_IPC_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_PARAM_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_RESOURCE_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_SDT_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_SEM_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_SOCKET_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_STATVFS_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_STAT_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_SYSINFO_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_TIME_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_TYPES_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_UIO_H: u32 = 1;
pub const _GLIBCXX_HAVE_S_ISREG: u32 = 1;
pub const _GLIBCXX_HAVE_TANF: u32 = 1;
pub const _GLIBCXX_HAVE_TANHF: u32 = 1;
pub const _GLIBCXX_HAVE_TANHL: u32 = 1;
pub const _GLIBCXX_HAVE_TANL: u32 = 1;
pub const _GLIBCXX_HAVE_TGMATH_H: u32 = 1;
pub const _GLIBCXX_HAVE_TIMESPEC_GET: u32 = 1;
pub const _GLIBCXX_HAVE_TLS: u32 = 1;
pub const _GLIBCXX_HAVE_TRUNCATE: u32 = 1;
pub const _GLIBCXX_HAVE_UCHAR_H: u32 = 1;
pub const _GLIBCXX_HAVE_UNISTD_H: u32 = 1;
pub const _GLIBCXX_HAVE_UTIME_H: u32 = 1;
pub const _GLIBCXX_HAVE_VFWSCANF: u32 = 1;
pub const _GLIBCXX_HAVE_VSWSCANF: u32 = 1;
pub const _GLIBCXX_HAVE_VWSCANF: u32 = 1;
pub const _GLIBCXX_HAVE_WCHAR_H: u32 = 1;
pub const _GLIBCXX_HAVE_WCSTOF: u32 = 1;
pub const _GLIBCXX_HAVE_WCTYPE_H: u32 = 1;
pub const _GLIBCXX_HAVE_WRITEV: u32 = 1;
pub const _GLIBCXX_HAVE___CXA_THREAD_ATEXIT_IMPL: u32 = 1;
pub const LT_OBJDIR: &'static [u8; 7usize] = b".libs/\0";
pub const _GLIBCXX_PACKAGE_BUGREPORT: &'static [u8; 1usize] = b"\0";
pub const _GLIBCXX_PACKAGE_NAME: &'static [u8; 15usize] = b"package-unused\0";
pub const _GLIBCXX_PACKAGE_STRING: &'static [u8; 30usize] = b"package-unused version-unused\0";
pub const _GLIBCXX_PACKAGE_TARNAME: &'static [u8; 10usize] = b"libstdc++\0";
pub const _GLIBCXX_PACKAGE_URL: &'static [u8; 1usize] = b"\0";
pub const _GLIBCXX_PACKAGE__GLIBCXX_VERSION: &'static [u8; 15usize] = b"version-unused\0";
pub const STDC_HEADERS: u32 = 1;
pub const _GLIBCXX_DARWIN_USE_64_BIT_INODE: u32 = 1;
pub const _GLIBCXX11_USE_C99_COMPLEX: u32 = 1;
pub const _GLIBCXX11_USE_C99_MATH: u32 = 1;
pub const _GLIBCXX11_USE_C99_STDIO: u32 = 1;
pub const _GLIBCXX11_USE_C99_STDLIB: u32 = 1;
pub const _GLIBCXX11_USE_C99_WCHAR: u32 = 1;
pub const _GLIBCXX98_USE_C99_COMPLEX: u32 = 1;
pub const _GLIBCXX98_USE_C99_MATH: u32 = 1;
pub const _GLIBCXX98_USE_C99_STDIO: u32 = 1;
pub const _GLIBCXX98_USE_C99_STDLIB: u32 = 1;
pub const _GLIBCXX98_USE_C99_WCHAR: u32 = 1;
pub const _GLIBCXX_ATOMIC_BUILTINS: u32 = 1;
pub const _GLIBCXX_FULLY_DYNAMIC_STRING: u32 = 0;
pub const _GLIBCXX_HAS_GTHREADS: u32 = 1;
pub const _GLIBCXX_HOSTED: u32 = 1;
pub const _GLIBCXX_RES_LIMITS: u32 = 1;
pub const _GLIBCXX_STDIO_EOF: i32 = -1;
pub const _GLIBCXX_STDIO_SEEK_CUR: u32 = 1;
pub const _GLIBCXX_STDIO_SEEK_END: u32 = 2;
pub const _GLIBCXX_SYMVER: u32 = 1;
pub const _GLIBCXX_SYMVER_GNU: u32 = 1;
pub const _GLIBCXX_USE_C11_UCHAR_CXX11: u32 = 1;
pub const _GLIBCXX_USE_C99: u32 = 1;
pub const _GLIBCXX_USE_C99_COMPLEX_TR1: u32 = 1;
pub const _GLIBCXX_USE_C99_CTYPE_TR1: u32 = 1;
pub const _GLIBCXX_USE_C99_FENV_TR1: u32 = 1;
pub const _GLIBCXX_USE_C99_INTTYPES_TR1: u32 = 1;
pub const _GLIBCXX_USE_C99_INTTYPES_WCHAR_T_TR1: u32 = 1;
pub const _GLIBCXX_USE_C99_MATH_TR1: u32 = 1;
pub const _GLIBCXX_USE_C99_STDINT_TR1: u32 = 1;
pub const _GLIBCXX_USE_CLOCK_MONOTONIC: u32 = 1;
pub const _GLIBCXX_USE_CLOCK_REALTIME: u32 = 1;
pub const _GLIBCXX_USE_DECIMAL_FLOAT: u32 = 1;
pub const _GLIBCXX_USE_DEV_RANDOM: u32 = 1;
pub const _GLIBCXX_USE_FCHMOD: u32 = 1;
pub const _GLIBCXX_USE_FCHMODAT: u32 = 1;
pub const _GLIBCXX_USE_GETTIMEOFDAY: u32 = 1;
pub const _GLIBCXX_USE_GET_NPROCS: u32 = 1;
pub const _GLIBCXX_USE_INT128: u32 = 1;
pub const _GLIBCXX_USE_LFS: u32 = 1;
pub const _GLIBCXX_USE_LONG_LONG: u32 = 1;
pub const _GLIBCXX_USE_LSTAT: u32 = 1;
pub const _GLIBCXX_USE_NANOSLEEP: u32 = 1;
pub const _GLIBCXX_USE_NLS: u32 = 1;
pub const _GLIBCXX_USE_PTHREAD_RWLOCK_T: u32 = 1;
pub const _GLIBCXX_USE_RANDOM_TR1: u32 = 1;
pub const _GLIBCXX_USE_REALPATH: u32 = 1;
pub const _GLIBCXX_USE_SCHED_YIELD: u32 = 1;
pub const _GLIBCXX_USE_SC_NPROCESSORS_ONLN: u32 = 1;
pub const _GLIBCXX_USE_SENDFILE: u32 = 1;
pub const _GLIBCXX_USE_ST_MTIM: u32 = 1;
pub const _GLIBCXX_USE_TMPNAM: u32 = 1;
pub const _GLIBCXX_USE_UTIME: u32 = 1;
pub const _GLIBCXX_USE_UTIMENSAT: u32 = 1;
pub const _GLIBCXX_USE_WCHAR_T: u32 = 1;
pub const _GLIBCXX_VERBOSE: u32 = 1;
pub const _GLIBCXX_X86_RDRAND: u32 = 1;
pub const _GTHREAD_USE_MUTEX_TIMEDLOCK: u32 = 1;
pub const _STL_FUNCTION_H: u32 = 1;
pub const _MOVE_H: u32 = 1;
pub const _CONCEPT_CHECK_H: u32 = 1;
pub const _GLIBCXX_TYPE_TRAITS: u32 = 1;
pub const __cpp_lib_integral_constant_callable: u32 = 201304;
pub const __cpp_lib_is_null_pointer: u32 = 201309;
pub const __cpp_lib_is_final: u32 = 201402;
pub const __cpp_lib_transformation_trait_aliases: u32 = 201304;
pub const __cpp_lib_result_of_sfinae: u32 = 201210;
pub const __cpp_lib_void_t: u32 = 201411;
pub const __cpp_lib_is_swappable: u32 = 201603;
pub const __cpp_lib_transparent_operators: u32 = 201510;
pub const _BACKWARD_BINDERS_H: u32 = 1;
pub const __EXCEPTION_H: u32 = 1;
pub const __cpp_lib_uncaught_exceptions: u32 = 201411;
pub const _EXCEPTION_DEFINES_H: u32 = 1;
pub const _CXXABI_INIT_EXCEPTION_H: u32 = 1;
pub const _GLIBCXX_HAVE_CDTOR_CALLABI: u32 = 0;
pub const _HASH_BYTES_H: u32 = 1;
pub const __GXX_MERGED_TYPEINFO_NAMES: u32 = 0;
pub const __GXX_TYPEINFO_EQUALITY_INLINE: u32 = 1;
pub const _GLIBCXX_NESTED_EXCEPTION_H: u32 = 1;
pub const _GLIBCXX_TUPLE: u32 = 1;
pub const _GLIBCXX_UTILITY: u32 = 1;
pub const _STL_RELOPS_H: u32 = 1;
pub const _STL_PAIR_H: u32 = 1;
pub const __cpp_lib_tuple_element_t: u32 = 201402;
pub const __cpp_lib_tuples_by_type: u32 = 201304;
pub const __cpp_lib_exchange_function: u32 = 201304;
pub const _GLIBCXX_USE_MAKE_INTEGER_SEQ: u32 = 1;
pub const __cpp_lib_integer_sequence: u32 = 201304;
pub const _GLIBCXX_ARRAY: u32 = 1;
pub const _GLIBCXX_STDEXCEPT: u32 = 1;
pub const _GLIBCXX_STRING: u32 = 1;
pub const _STRINGFWD_H: u32 = 1;
pub const _MEMORYFWD_H: u32 = 1;
pub const _CHAR_TRAITS_H: u32 = 1;
pub const _STL_ALGOBASE_H: u32 = 1;
pub const _FUNCTEXCEPT_H: u32 = 1;
pub const _CPP_TYPE_TRAITS_H: u32 = 1;
pub const _EXT_TYPE_TRAITS: u32 = 1;
pub const _EXT_NUMERIC_TRAITS: u32 = 1;
pub const _STL_ITERATOR_BASE_TYPES_H: u32 = 1;
pub const _STL_ITERATOR_BASE_FUNCS_H: u32 = 1;
pub const _GLIBCXX_DEBUG_ASSERTIONS_H: u32 = 1;
pub const _STL_ITERATOR_H: u32 = 1;
pub const _PTR_TRAITS_H: u32 = 1;
pub const __cpp_lib_make_reverse_iterator: u32 = 201402;
pub const _GLIBCXX_DEBUG_MACRO_SWITCH_H: u32 = 1;
pub const _GLIBCXX_PREDEFINED_OPS_H: u32 = 1;
pub const __cpp_lib_robust_nonmodifying_seq_ops: u32 = 201304;
pub const _GLIBCXX_POSTYPES_H: u32 = 1;
pub const _WCHAR_H: u32 = 1;
pub const __HAVE_FLOAT128: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
pub const __HAVE_FLOAT64X: u32 = 1;
pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
pub const __HAVE_FLOAT16: u32 = 0;
pub const __HAVE_FLOAT32: u32 = 1;
pub const __HAVE_FLOAT64: u32 = 1;
pub const __HAVE_FLOAT32X: u32 = 1;
pub const __HAVE_FLOAT128X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
pub const __GNUC_VA_LIST: u32 = 1;
pub const __wint_t_defined: u32 = 1;
pub const _WINT_T: u32 = 1;
pub const __mbstate_t_defined: u32 = 1;
pub const ____mbstate_t_defined: u32 = 1;
pub const ____FILE_defined: u32 = 1;
pub const __FILE_defined: u32 = 1;
pub const _BITS_TYPES_LOCALE_T_H: u32 = 1;
pub const _BITS_TYPES___LOCALE_T_H: u32 = 1;
pub const WEOF: u32 = 4294967295;
pub const _GLIBCXX_CWCHAR: u32 = 1;
pub const _GLIBCXX_CSTDINT: u32 = 1;
pub const _ALLOCATOR_H: u32 = 1;
pub const _GLIBCXX_CXX_ALLOCATOR_H: u32 = 1;
pub const _NEW_ALLOCATOR_H: u32 = 1;
pub const __cpp_lib_incomplete_container_elements: u32 = 201505;
pub const __cpp_lib_allocator_is_always_equal: u32 = 201411;
pub const _LOCALE_FWD_H: u32 = 1;
pub const _GLIBCXX_CXX_LOCALE_H: u32 = 1;
pub const _LOCALE_H: u32 = 1;
pub const _BITS_LOCALE_H: u32 = 1;
pub const __LC_CTYPE: u32 = 0;
pub const __LC_NUMERIC: u32 = 1;
pub const __LC_TIME: u32 = 2;
pub const __LC_COLLATE: u32 = 3;
pub const __LC_MONETARY: u32 = 4;
pub const __LC_MESSAGES: u32 = 5;
pub const __LC_ALL: u32 = 6;
pub const __LC_PAPER: u32 = 7;
pub const __LC_NAME: u32 = 8;
pub const __LC_ADDRESS: u32 = 9;
pub const __LC_TELEPHONE: u32 = 10;
pub const __LC_MEASUREMENT: u32 = 11;
pub const __LC_IDENTIFICATION: u32 = 12;
pub const LC_CTYPE: u32 = 0;
pub const LC_NUMERIC: u32 = 1;
pub const LC_TIME: u32 = 2;
pub const LC_COLLATE: u32 = 3;
pub const LC_MONETARY: u32 = 4;
pub const LC_MESSAGES: u32 = 5;
pub const LC_ALL: u32 = 6;
pub const LC_PAPER: u32 = 7;
pub const LC_NAME: u32 = 8;
pub const LC_ADDRESS: u32 = 9;
pub const LC_TELEPHONE: u32 = 10;
pub const LC_MEASUREMENT: u32 = 11;
pub const LC_IDENTIFICATION: u32 = 12;
pub const LC_CTYPE_MASK: u32 = 1;
pub const LC_NUMERIC_MASK: u32 = 2;
pub const LC_TIME_MASK: u32 = 4;
pub const LC_COLLATE_MASK: u32 = 8;
pub const LC_MONETARY_MASK: u32 = 16;
pub const LC_MESSAGES_MASK: u32 = 32;
pub const LC_PAPER_MASK: u32 = 128;
pub const LC_NAME_MASK: u32 = 256;
pub const LC_ADDRESS_MASK: u32 = 512;
pub const LC_TELEPHONE_MASK: u32 = 1024;
pub const LC_MEASUREMENT_MASK: u32 = 2048;
pub const LC_IDENTIFICATION_MASK: u32 = 4096;
pub const LC_ALL_MASK: u32 = 8127;
pub const _GLIBCXX_CLOCALE: u32 = 1;
pub const _GLIBCXX_C_LOCALE_GNU: u32 = 1;
pub const _GLIBCXX_NUM_CATEGORIES: u32 = 6;
pub const _GLIBCXX_IOSFWD: u32 = 1;
pub const _CTYPE_H: u32 = 1;
pub const _BITS_ENDIAN_H: u32 = 1;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __PDP_ENDIAN: u32 = 3412;
pub const _BITS_ENDIANNESS_H: u32 = 1;
pub const __BYTE_ORDER: u32 = 1234;
pub const __FLOAT_WORD_ORDER: u32 = 1234;
pub const _GLIBCXX_CCTYPE: u32 = 1;
pub const _OSTREAM_INSERT_H: u32 = 1;
pub const _CXXABI_FORCED_H: u32 = 1;
pub const _GLIBCXX_RANGE_ACCESS_H: u32 = 1;
pub const _BASIC_STRING_H: u32 = 1;
pub const _GLIBCXX_ATOMICITY_H: u32 = 1;
pub const _GLIBCXX_GTHREAD_USE_WEAK: u32 = 1;
pub const __GTHREADS: u32 = 1;
pub const __GTHREADS_CXX0X: u32 = 1;
pub const _PTHREAD_H: u32 = 1;
pub const _SCHED_H: u32 = 1;
pub const __time_t_defined: u32 = 1;
pub const _STRUCT_TIMESPEC: u32 = 1;
pub const _BITS_SCHED_H: u32 = 1;
pub const SCHED_OTHER: u32 = 0;
pub const SCHED_FIFO: u32 = 1;
pub const SCHED_RR: u32 = 2;
pub const SCHED_BATCH: u32 = 3;
pub const SCHED_ISO: u32 = 4;
pub const SCHED_IDLE: u32 = 5;
pub const SCHED_DEADLINE: u32 = 6;
pub const SCHED_RESET_ON_FORK: u32 = 1073741824;
pub const CSIGNAL: u32 = 255;
pub const CLONE_VM: u32 = 256;
pub const CLONE_FS: u32 = 512;
pub const CLONE_FILES: u32 = 1024;
pub const CLONE_SIGHAND: u32 = 2048;
pub const CLONE_PIDFD: u32 = 4096;
pub const CLONE_PTRACE: u32 = 8192;
pub const CLONE_VFORK: u32 = 16384;
pub const CLONE_PARENT: u32 = 32768;
pub const CLONE_THREAD: u32 = 65536;
pub const CLONE_NEWNS: u32 = 131072;
pub const CLONE_SYSVSEM: u32 = 262144;
pub const CLONE_SETTLS: u32 = 524288;
pub const CLONE_PARENT_SETTID: u32 = 1048576;
pub const CLONE_CHILD_CLEARTID: u32 = 2097152;
pub const CLONE_DETACHED: u32 = 4194304;
pub const CLONE_UNTRACED: u32 = 8388608;
pub const CLONE_CHILD_SETTID: u32 = 16777216;
pub const CLONE_NEWCGROUP: u32 = 33554432;
pub const CLONE_NEWUTS: u32 = 67108864;
pub const CLONE_NEWIPC: u32 = 134217728;
pub const CLONE_NEWUSER: u32 = 268435456;
pub const CLONE_NEWPID: u32 = 536870912;
pub const CLONE_NEWNET: u32 = 1073741824;
pub const CLONE_IO: u32 = 2147483648;
pub const _BITS_TYPES_STRUCT_SCHED_PARAM: u32 = 1;
pub const _BITS_CPU_SET_H: u32 = 1;
pub const __CPU_SETSIZE: u32 = 1024;
pub const CPU_SETSIZE: u32 = 1024;
pub const _TIME_H: u32 = 1;
pub const _BITS_TIME_H: u32 = 1;
pub const CLOCK_REALTIME: u32 = 0;
pub const CLOCK_MONOTONIC: u32 = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: u32 = 2;
pub const CLOCK_THREAD_CPUTIME_ID: u32 = 3;
pub const CLOCK_MONOTONIC_RAW: u32 = 4;
pub const CLOCK_REALTIME_COARSE: u32 = 5;
pub const CLOCK_MONOTONIC_COARSE: u32 = 6;
pub const CLOCK_BOOTTIME: u32 = 7;
pub const CLOCK_REALTIME_ALARM: u32 = 8;
pub const CLOCK_BOOTTIME_ALARM: u32 = 9;
pub const CLOCK_TAI: u32 = 11;
pub const TIMER_ABSTIME: u32 = 1;
pub const _BITS_TIMEX_H: u32 = 1;
pub const __timeval_defined: u32 = 1;
pub const ADJ_OFFSET: u32 = 1;
pub const ADJ_FREQUENCY: u32 = 2;
pub const ADJ_MAXERROR: u32 = 4;
pub const ADJ_ESTERROR: u32 = 8;
pub const ADJ_STATUS: u32 = 16;
pub const ADJ_TIMECONST: u32 = 32;
pub const ADJ_TAI: u32 = 128;
pub const ADJ_SETOFFSET: u32 = 256;
pub const ADJ_MICRO: u32 = 4096;
pub const ADJ_NANO: u32 = 8192;
pub const ADJ_TICK: u32 = 16384;
pub const ADJ_OFFSET_SINGLESHOT: u32 = 32769;
pub const ADJ_OFFSET_SS_READ: u32 = 40961;
pub const MOD_OFFSET: u32 = 1;
pub const MOD_FREQUENCY: u32 = 2;
pub const MOD_MAXERROR: u32 = 4;
pub const MOD_ESTERROR: u32 = 8;
pub const MOD_STATUS: u32 = 16;
pub const MOD_TIMECONST: u32 = 32;
pub const MOD_CLKB: u32 = 16384;
pub const MOD_CLKA: u32 = 32769;
pub const MOD_TAI: u32 = 128;
pub const MOD_MICRO: u32 = 4096;
pub const MOD_NANO: u32 = 8192;
pub const STA_PLL: u32 = 1;
pub const STA_PPSFREQ: u32 = 2;
pub const STA_PPSTIME: u32 = 4;
pub const STA_FLL: u32 = 8;
pub const STA_INS: u32 = 16;
pub const STA_DEL: u32 = 32;
pub const STA_UNSYNC: u32 = 64;
pub const STA_FREQHOLD: u32 = 128;
pub const STA_PPSSIGNAL: u32 = 256;
pub const STA_PPSJITTER: u32 = 512;
pub const STA_PPSWANDER: u32 = 1024;
pub const STA_PPSERROR: u32 = 2048;
pub const STA_CLOCKERR: u32 = 4096;
pub const STA_NANO: u32 = 8192;
pub const STA_MODE: u32 = 16384;
pub const STA_CLK: u32 = 32768;
pub const STA_RONLY: u32 = 65280;
pub const __clock_t_defined: u32 = 1;
pub const __struct_tm_defined: u32 = 1;
pub const __clockid_t_defined: u32 = 1;
pub const __timer_t_defined: u32 = 1;
pub const __itimerspec_defined: u32 = 1;
pub const TIME_UTC: u32 = 1;
pub const _BITS_PTHREADTYPES_COMMON_H: u32 = 1;
pub const _THREAD_SHARED_TYPES_H: u32 = 1;
pub const _BITS_PTHREADTYPES_ARCH_H: u32 = 1;
pub const __SIZEOF_PTHREAD_MUTEX_T: u32 = 40;
pub const __SIZEOF_PTHREAD_ATTR_T: u32 = 56;
pub const __SIZEOF_PTHREAD_RWLOCK_T: u32 = 56;
pub const __SIZEOF_PTHREAD_BARRIER_T: u32 = 32;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_COND_T: u32 = 48;
pub const __SIZEOF_PTHREAD_CONDATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: u32 = 8;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: u32 = 4;
pub const _THREAD_MUTEX_INTERNAL_H: u32 = 1;
pub const __PTHREAD_MUTEX_HAVE_PREV: u32 = 1;
pub const __have_pthread_attr_t: u32 = 1;
pub const _BITS_SETJMP_H: u32 = 1;
pub const PTHREAD_ONCE_INIT: u32 = 0;
pub const PTHREAD_BARRIER_SERIAL_THREAD: i32 = -1;
pub const __GTHREAD_HAS_COND: u32 = 1;
pub const __GTHREAD_ONCE_INIT: u32 = 0;
pub const _GLIBCXX_ATOMIC_WORD_H: u32 = 1;
pub const _EXT_ALLOC_TRAITS_H: u32 = 1;
pub const _ALLOC_TRAITS_H: u32 = 1;
pub const __cpp_lib_allocator_traits_is_always_equal: u32 = 201411;
pub const _STRING_CONVERSIONS_H: u32 = 1;
pub const _GLIBCXX_CSTDLIB: u32 = 1;
pub const _STDLIB_H: u32 = 1;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WSTOPPED: u32 = 2;
pub const WEXITED: u32 = 4;
pub const WCONTINUED: u32 = 8;
pub const WNOWAIT: u32 = 16777216;
pub const __WNOTHREAD: u32 = 536870912;
pub const __WALL: u32 = 1073741824;
pub const __WCLONE: u32 = 2147483648;
pub const __ENUM_IDTYPE_T: u32 = 1;
pub const __W_CONTINUED: u32 = 65535;
pub const __WCOREFLAG: u32 = 128;
pub const __ldiv_t_defined: u32 = 1;
pub const __lldiv_t_defined: u32 = 1;
pub const RAND_MAX: u32 = 2147483647;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const _SYS_TYPES_H: u32 = 1;
pub const __BIT_TYPES_DEFINED__: u32 = 1;
pub const _ENDIAN_H: u32 = 1;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const BYTE_ORDER: u32 = 1234;
pub const _BITS_BYTESWAP_H: u32 = 1;
pub const _BITS_UINTN_IDENTITY_H: u32 = 1;
pub const _SYS_SELECT_H: u32 = 1;
pub const __FD_ZERO_STOS: &'static [u8; 6usize] = b"stosq\0";
pub const __sigset_t_defined: u32 = 1;
pub const FD_SETSIZE: u32 = 1024;
pub const _ALLOCA_H: u32 = 1;
pub const _STDIO_H: u32 = 1;
pub const _____fpos_t_defined: u32 = 1;
pub const _____fpos64_t_defined: u32 = 1;
pub const __struct_FILE_defined: u32 = 1;
pub const _IO_EOF_SEEN: u32 = 16;
pub const _IO_ERR_SEEN: u32 = 32;
pub const _IO_USER_LOCK: u32 = 32768;
pub const __cookie_io_functions_t_defined: u32 = 1;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 1;
pub const _IONBF: u32 = 2;
pub const BUFSIZ: u32 = 8192;
pub const EOF: i32 = -1;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const SEEK_DATA: u32 = 3;
pub const SEEK_HOLE: u32 = 4;
pub const P_tmpdir: &'static [u8; 5usize] = b"/tmp\0";
pub const _BITS_STDIO_LIM_H: u32 = 1;
pub const L_tmpnam: u32 = 20;
pub const TMP_MAX: u32 = 238328;
pub const FILENAME_MAX: u32 = 4096;
pub const L_ctermid: u32 = 9;
pub const L_cuserid: u32 = 9;
pub const FOPEN_MAX: u32 = 16;
pub const RENAME_NOREPLACE: u32 = 1;
pub const RENAME_EXCHANGE: u32 = 2;
pub const RENAME_WHITEOUT: u32 = 4;
pub const _GLIBCXX_CSTDIO: u32 = 1;
pub const _ERRNO_H: u32 = 1;
pub const _BITS_ERRNO_H: u32 = 1;
pub const EPERM: u32 = 1;
pub const ENOENT: u32 = 2;
pub const ESRCH: u32 = 3;
pub const EINTR: u32 = 4;
pub const EIO: u32 = 5;
pub const ENXIO: u32 = 6;
pub const E2BIG: u32 = 7;
pub const ENOEXEC: u32 = 8;
pub const EBADF: u32 = 9;
pub const ECHILD: u32 = 10;
pub const EAGAIN: u32 = 11;
pub const ENOMEM: u32 = 12;
pub const EACCES: u32 = 13;
pub const EFAULT: u32 = 14;
pub const ENOTBLK: u32 = 15;
pub const EBUSY: u32 = 16;
pub const EEXIST: u32 = 17;
pub const EXDEV: u32 = 18;
pub const ENODEV: u32 = 19;
pub const ENOTDIR: u32 = 20;
pub const EISDIR: u32 = 21;
pub const EINVAL: u32 = 22;
pub const ENFILE: u32 = 23;
pub const EMFILE: u32 = 24;
pub const ENOTTY: u32 = 25;
pub const ETXTBSY: u32 = 26;
pub const EFBIG: u32 = 27;
pub const ENOSPC: u32 = 28;
pub const ESPIPE: u32 = 29;
pub const EROFS: u32 = 30;
pub const EMLINK: u32 = 31;
pub const EPIPE: u32 = 32;
pub const EDOM: u32 = 33;
pub const ERANGE: u32 = 34;
pub const EDEADLK: u32 = 35;
pub const ENAMETOOLONG: u32 = 36;
pub const ENOLCK: u32 = 37;
pub const ENOSYS: u32 = 38;
pub const ENOTEMPTY: u32 = 39;
pub const ELOOP: u32 = 40;
pub const EWOULDBLOCK: u32 = 11;
pub const ENOMSG: u32 = 42;
pub const EIDRM: u32 = 43;
pub const ECHRNG: u32 = 44;
pub const EL2NSYNC: u32 = 45;
pub const EL3HLT: u32 = 46;
pub const EL3RST: u32 = 47;
pub const ELNRNG: u32 = 48;
pub const EUNATCH: u32 = 49;
pub const ENOCSI: u32 = 50;
pub const EL2HLT: u32 = 51;
pub const EBADE: u32 = 52;
pub const EBADR: u32 = 53;
pub const EXFULL: u32 = 54;
pub const ENOANO: u32 = 55;
pub const EBADRQC: u32 = 56;
pub const EBADSLT: u32 = 57;
pub const EDEADLOCK: u32 = 35;
pub const EBFONT: u32 = 59;
pub const ENOSTR: u32 = 60;
pub const ENODATA: u32 = 61;
pub const ETIME: u32 = 62;
pub const ENOSR: u32 = 63;
pub const ENONET: u32 = 64;
pub const ENOPKG: u32 = 65;
pub const EREMOTE: u32 = 66;
pub const ENOLINK: u32 = 67;
pub const EADV: u32 = 68;
pub const ESRMNT: u32 = 69;
pub const ECOMM: u32 = 70;
pub const EPROTO: u32 = 71;
pub const EMULTIHOP: u32 = 72;
pub const EDOTDOT: u32 = 73;
pub const EBADMSG: u32 = 74;
pub const EOVERFLOW: u32 = 75;
pub const ENOTUNIQ: u32 = 76;
pub const EBADFD: u32 = 77;
pub const EREMCHG: u32 = 78;
pub const ELIBACC: u32 = 79;
pub const ELIBBAD: u32 = 80;
pub const ELIBSCN: u32 = 81;
pub const ELIBMAX: u32 = 82;
pub const ELIBEXEC: u32 = 83;
pub const EILSEQ: u32 = 84;
pub const ERESTART: u32 = 85;
pub const ESTRPIPE: u32 = 86;
pub const EUSERS: u32 = 87;
pub const ENOTSOCK: u32 = 88;
pub const EDESTADDRREQ: u32 = 89;
pub const EMSGSIZE: u32 = 90;
pub const EPROTOTYPE: u32 = 91;
pub const ENOPROTOOPT: u32 = 92;
pub const EPROTONOSUPPORT: u32 = 93;
pub const ESOCKTNOSUPPORT: u32 = 94;
pub const EOPNOTSUPP: u32 = 95;
pub const EPFNOSUPPORT: u32 = 96;
pub const EAFNOSUPPORT: u32 = 97;
pub const EADDRINUSE: u32 = 98;
pub const EADDRNOTAVAIL: u32 = 99;
pub const ENETDOWN: u32 = 100;
pub const ENETUNREACH: u32 = 101;
pub const ENETRESET: u32 = 102;
pub const ECONNABORTED: u32 = 103;
pub const ECONNRESET: u32 = 104;
pub const ENOBUFS: u32 = 105;
pub const EISCONN: u32 = 106;
pub const ENOTCONN: u32 = 107;
pub const ESHUTDOWN: u32 = 108;
pub const ETOOMANYREFS: u32 = 109;
pub const ETIMEDOUT: u32 = 110;
pub const ECONNREFUSED: u32 = 111;
pub const EHOSTDOWN: u32 = 112;
pub const EHOSTUNREACH: u32 = 113;
pub const EALREADY: u32 = 114;
pub const EINPROGRESS: u32 = 115;
pub const ESTALE: u32 = 116;
pub const EUCLEAN: u32 = 117;
pub const ENOTNAM: u32 = 118;
pub const ENAVAIL: u32 = 119;
pub const EISNAM: u32 = 120;
pub const EREMOTEIO: u32 = 121;
pub const EDQUOT: u32 = 122;
pub const ENOMEDIUM: u32 = 123;
pub const EMEDIUMTYPE: u32 = 124;
pub const ECANCELED: u32 = 125;
pub const ENOKEY: u32 = 126;
pub const EKEYEXPIRED: u32 = 127;
pub const EKEYREVOKED: u32 = 128;
pub const EKEYREJECTED: u32 = 129;
pub const EOWNERDEAD: u32 = 130;
pub const ENOTRECOVERABLE: u32 = 131;
pub const ERFKILL: u32 = 132;
pub const EHWPOISON: u32 = 133;
pub const ENOTSUP: u32 = 95;
pub const __error_t_defined: u32 = 1;
pub const _GLIBCXX_CERRNO: u32 = 1;
pub const _FUNCTIONAL_HASH_H: u32 = 1;
pub const __cpp_lib_string_udls: u32 = 201304;
pub const _BASIC_STRING_TCC: u32 = 1;
pub const _USES_ALLOCATOR_H: u32 = 1;
pub const _GLIBCXX_INVOKE_H: u32 = 1;
pub const _GLIBCXX_REFWRAP_H: u32 = 1;
pub const _GLIBCXX_STD_FUNCTION_H: u32 = 1;
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___fsid_t() {
    assert_eq!(
        std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__fsid_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type QHYDWORD = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ccdreg {
    pub Gain: u8,
    pub Offset: u8,
    pub Exptime: u32,
    pub HBIN: u8,
    pub VBIN: u8,
    pub LineSize: u16,
    pub VerticalSize: u16,
    pub SKIP_TOP: u16,
    pub SKIP_BOTTOM: u16,
    pub LiveVideo_BeginLine: u16,
    pub AnitInterlace: u16,
    pub MultiFieldBIN: u8,
    pub AMPVOLTAGE: u8,
    pub DownloadSpeed: u8,
    pub TgateMode: u8,
    pub ShortExposure: u8,
    pub VSUB: u8,
    pub CLAMP: u8,
    pub TransferBIT: u8,
    pub TopSkipNull: u8,
    pub TopSkipPix: u16,
    pub MechanicalShutterMode: u8,
    pub DownloadCloseTEC: u8,
    pub SDRAM_MAXSIZE: u8,
    pub ClockADJ: u16,
    pub Trig: u8,
    pub MotorHeating: u8,
    pub WindowHeater: u8,
    pub ADCSEL: u8,
}
#[test]
fn bindgen_test_layout_ccdreg() {
    assert_eq!(
        std::mem::size_of::<ccdreg>(),
        44usize,
        concat!("Size of: ", stringify!(ccdreg))
    );
    assert_eq!(
        std::mem::align_of::<ccdreg>(),
        4usize,
        concat!("Alignment of ", stringify!(ccdreg))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).Gain as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(Gain)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).Offset as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(Offset)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).Exptime as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(Exptime)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).HBIN as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(HBIN)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).VBIN as *const _ as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(VBIN)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).LineSize as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(LineSize)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).VerticalSize as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(VerticalSize)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).SKIP_TOP as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(SKIP_TOP)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).SKIP_BOTTOM as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(SKIP_BOTTOM)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).LiveVideo_BeginLine as *const _ as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(LiveVideo_BeginLine)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).AnitInterlace as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(AnitInterlace)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).MultiFieldBIN as *const _ as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(MultiFieldBIN)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).AMPVOLTAGE as *const _ as usize },
        23usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(AMPVOLTAGE)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).DownloadSpeed as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(DownloadSpeed)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).TgateMode as *const _ as usize },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(TgateMode)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).ShortExposure as *const _ as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(ShortExposure)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).VSUB as *const _ as usize },
        27usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(VSUB)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).CLAMP as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(CLAMP)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).TransferBIT as *const _ as usize },
        29usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(TransferBIT)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).TopSkipNull as *const _ as usize },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(TopSkipNull)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).TopSkipPix as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(TopSkipPix)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).MechanicalShutterMode as *const _ as usize },
        34usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(MechanicalShutterMode)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).DownloadCloseTEC as *const _ as usize },
        35usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(DownloadCloseTEC)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).SDRAM_MAXSIZE as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(SDRAM_MAXSIZE)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).ClockADJ as *const _ as usize },
        38usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(ClockADJ)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).Trig as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(Trig)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).MotorHeating as *const _ as usize },
        41usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(MotorHeating)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).WindowHeater as *const _ as usize },
        42usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(WindowHeater)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ccdreg>())).ADCSEL as *const _ as usize },
        43usize,
        concat!(
            "Offset of field: ",
            stringify!(ccdreg),
            "::",
            stringify!(ADCSEL)
        )
    );
}
pub type CCDREG = ccdreg;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BIOREG {
    pub LineSize: u16,
    pub PatchNumber: u16,
    pub AMPVOLTAGE: u8,
    pub ShortExposure: u8,
    pub SDRAM_MAXSIZE: u8,
    pub DownloadSpeed: u8,
    pub TransferBIT: u8,
    pub BIOCCD_Mode: u8,
    pub BIOCCD_Video: u8,
    pub SDRAM_Bypass: u8,
}
#[test]
fn bindgen_test_layout_BIOREG() {
    assert_eq!(
        std::mem::size_of::<BIOREG>(),
        12usize,
        concat!("Size of: ", stringify!(BIOREG))
    );
    assert_eq!(
        std::mem::align_of::<BIOREG>(),
        2usize,
        concat!("Alignment of ", stringify!(BIOREG))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<BIOREG>())).LineSize as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(BIOREG),
            "::",
            stringify!(LineSize)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<BIOREG>())).PatchNumber as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(BIOREG),
            "::",
            stringify!(PatchNumber)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<BIOREG>())).AMPVOLTAGE as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(BIOREG),
            "::",
            stringify!(AMPVOLTAGE)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<BIOREG>())).ShortExposure as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(BIOREG),
            "::",
            stringify!(ShortExposure)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<BIOREG>())).SDRAM_MAXSIZE as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(BIOREG),
            "::",
            stringify!(SDRAM_MAXSIZE)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<BIOREG>())).DownloadSpeed as *const _ as usize },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(BIOREG),
            "::",
            stringify!(DownloadSpeed)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<BIOREG>())).TransferBIT as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(BIOREG),
            "::",
            stringify!(TransferBIT)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<BIOREG>())).BIOCCD_Mode as *const _ as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(BIOREG),
            "::",
            stringify!(BIOCCD_Mode)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<BIOREG>())).BIOCCD_Video as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(BIOREG),
            "::",
            stringify!(BIOCCD_Video)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<BIOREG>())).SDRAM_Bypass as *const _ as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(BIOREG),
            "::",
            stringify!(SDRAM_Bypass)
        )
    );
}
pub const CONTROL_ID_CONTROL_BRIGHTNESS: CONTROL_ID = 0;
pub const CONTROL_ID_CONTROL_CONTRAST: CONTROL_ID = 1;
pub const CONTROL_ID_CONTROL_WBR: CONTROL_ID = 2;
pub const CONTROL_ID_CONTROL_WBB: CONTROL_ID = 3;
pub const CONTROL_ID_CONTROL_WBG: CONTROL_ID = 4;
pub const CONTROL_ID_CONTROL_GAMMA: CONTROL_ID = 5;
pub const CONTROL_ID_CONTROL_GAIN: CONTROL_ID = 6;
pub const CONTROL_ID_CONTROL_OFFSET: CONTROL_ID = 7;
pub const CONTROL_ID_CONTROL_EXPOSURE: CONTROL_ID = 8;
pub const CONTROL_ID_CONTROL_SPEED: CONTROL_ID = 9;
pub const CONTROL_ID_CONTROL_TRANSFERBIT: CONTROL_ID = 10;
pub const CONTROL_ID_CONTROL_CHANNELS: CONTROL_ID = 11;
pub const CONTROL_ID_CONTROL_USBTRAFFIC: CONTROL_ID = 12;
pub const CONTROL_ID_CONTROL_ROWNOISERE: CONTROL_ID = 13;
pub const CONTROL_ID_CONTROL_CURTEMP: CONTROL_ID = 14;
pub const CONTROL_ID_CONTROL_CURPWM: CONTROL_ID = 15;
pub const CONTROL_ID_CONTROL_MANULPWM: CONTROL_ID = 16;
pub const CONTROL_ID_CONTROL_CFWPORT: CONTROL_ID = 17;
pub const CONTROL_ID_CONTROL_COOLER: CONTROL_ID = 18;
pub const CONTROL_ID_CONTROL_ST4PORT: CONTROL_ID = 19;
pub const CONTROL_ID_CAM_COLOR: CONTROL_ID = 20;
pub const CONTROL_ID_CAM_BIN1X1MODE: CONTROL_ID = 21;
pub const CONTROL_ID_CAM_BIN2X2MODE: CONTROL_ID = 22;
pub const CONTROL_ID_CAM_BIN3X3MODE: CONTROL_ID = 23;
pub const CONTROL_ID_CAM_BIN4X4MODE: CONTROL_ID = 24;
pub const CONTROL_ID_CAM_MECHANICALSHUTTER: CONTROL_ID = 25;
pub const CONTROL_ID_CAM_TRIGER_INTERFACE: CONTROL_ID = 26;
pub const CONTROL_ID_CAM_TECOVERPROTECT_INTERFACE: CONTROL_ID = 27;
pub const CONTROL_ID_CAM_SINGNALCLAMP_INTERFACE: CONTROL_ID = 28;
pub const CONTROL_ID_CAM_FINETONE_INTERFACE: CONTROL_ID = 29;
pub const CONTROL_ID_CAM_SHUTTERMOTORHEATING_INTERFACE: CONTROL_ID = 30;
pub const CONTROL_ID_CAM_CALIBRATEFPN_INTERFACE: CONTROL_ID = 31;
pub const CONTROL_ID_CAM_CHIPTEMPERATURESENSOR_INTERFACE: CONTROL_ID = 32;
pub const CONTROL_ID_CAM_USBREADOUTSLOWEST_INTERFACE: CONTROL_ID = 33;
pub const CONTROL_ID_CAM_8BITS: CONTROL_ID = 34;
pub const CONTROL_ID_CAM_16BITS: CONTROL_ID = 35;
pub const CONTROL_ID_CAM_GPS: CONTROL_ID = 36;
pub const CONTROL_ID_CAM_IGNOREOVERSCAN_INTERFACE: CONTROL_ID = 37;
pub const CONTROL_ID_QHYCCD_3A_AUTOBALANCE: CONTROL_ID = 38;
pub const CONTROL_ID_QHYCCD_3A_AUTOEXPOSURE: CONTROL_ID = 39;
pub const CONTROL_ID_QHYCCD_3A_AUTOFOCUS: CONTROL_ID = 40;
pub const CONTROL_ID_CONTROL_AMPV: CONTROL_ID = 41;
pub const CONTROL_ID_CONTROL_VCAM: CONTROL_ID = 42;
pub const CONTROL_ID_CAM_VIEW_MODE: CONTROL_ID = 43;
pub const CONTROL_ID_CONTROL_CFWSLOTSNUM: CONTROL_ID = 44;
pub const CONTROL_ID_IS_EXPOSING_DONE: CONTROL_ID = 45;
pub const CONTROL_ID_ScreenStretchB: CONTROL_ID = 46;
pub const CONTROL_ID_ScreenStretchW: CONTROL_ID = 47;
pub const CONTROL_ID_CONTROL_DDR: CONTROL_ID = 48;
pub const CONTROL_ID_CAM_LIGHT_PERFORMANCE_MODE: CONTROL_ID = 49;
pub const CONTROL_ID_CAM_QHY5II_GUIDE_MODE: CONTROL_ID = 50;
pub const CONTROL_ID_DDR_BUFFER_CAPACITY: CONTROL_ID = 51;
pub const CONTROL_ID_DDR_BUFFER_READ_THRESHOLD: CONTROL_ID = 52;
pub const CONTROL_ID_DefaultGain: CONTROL_ID = 53;
pub const CONTROL_ID_DefaultOffset: CONTROL_ID = 54;
pub const CONTROL_ID_OutputDataActualBits: CONTROL_ID = 55;
pub const CONTROL_ID_OutputDataAlignment: CONTROL_ID = 56;
pub const CONTROL_ID_CAM_SINGLEFRAMEMODE: CONTROL_ID = 57;
pub const CONTROL_ID_CAM_LIVEVIDEOMODE: CONTROL_ID = 58;
pub const CONTROL_ID_CAM_IS_COLOR: CONTROL_ID = 59;
pub const CONTROL_ID_hasHardwareFrameCounter: CONTROL_ID = 60;
pub const CONTROL_ID_CONTROL_MAX_ID_Error: CONTROL_ID = 61;
pub const CONTROL_ID_CAM_HUMIDITY: CONTROL_ID = 62;
pub const CONTROL_ID_CAM_PRESSURE: CONTROL_ID = 63;
pub const CONTROL_ID_CONTROL_VACUUM_PUMP: CONTROL_ID = 64;
pub const CONTROL_ID_CONTROL_SensorChamberCycle_PUMP: CONTROL_ID = 65;
pub const CONTROL_ID_CAM_32BITS: CONTROL_ID = 66;
pub const CONTROL_ID_CAM_Sensor_ULVO_Status: CONTROL_ID = 67;
pub const CONTROL_ID_CAM_SensorPhaseReTrain: CONTROL_ID = 68;
pub const CONTROL_ID_CAM_InitConfigFromFlash: CONTROL_ID = 69;
pub const CONTROL_ID_CAM_TRIGER_MODE: CONTROL_ID = 70;
pub const CONTROL_ID_CAM_TRIGER_OUT: CONTROL_ID = 71;
pub const CONTROL_ID_CAM_BURST_MODE: CONTROL_ID = 72;
pub const CONTROL_ID_CONTROL_MAX_ID: CONTROL_ID = 73;
pub type CONTROL_ID = ::std::os::raw::c_uint;
pub const BAYER_ID_BAYER_GB: BAYER_ID = 1;
pub const BAYER_ID_BAYER_GR: BAYER_ID = 2;
pub const BAYER_ID_BAYER_BG: BAYER_ID = 3;
pub const BAYER_ID_BAYER_RG: BAYER_ID = 4;
pub type BAYER_ID = ::std::os::raw::c_uint;
pub const CodecID_NONE_CODEC: CodecID = 0;
pub const CodecID_H261_CODEC: CodecID = 1;
pub type CodecID = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _QHYCamMinMaxStepValue {
    pub name: *const ::std::os::raw::c_char,
    pub min: f64,
    pub max: f64,
    pub step: f64,
}
#[test]
fn bindgen_test_layout__QHYCamMinMaxStepValue() {
    assert_eq!(
        std::mem::size_of::<_QHYCamMinMaxStepValue>(),
        32usize,
        concat!("Size of: ", stringify!(_QHYCamMinMaxStepValue))
    );
    assert_eq!(
        std::mem::align_of::<_QHYCamMinMaxStepValue>(),
        8usize,
        concat!("Alignment of ", stringify!(_QHYCamMinMaxStepValue))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_QHYCamMinMaxStepValue>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_QHYCamMinMaxStepValue),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_QHYCamMinMaxStepValue>())).min as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_QHYCamMinMaxStepValue),
            "::",
            stringify!(min)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_QHYCamMinMaxStepValue>())).max as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_QHYCamMinMaxStepValue),
            "::",
            stringify!(max)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_QHYCamMinMaxStepValue>())).step as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_QHYCamMinMaxStepValue),
            "::",
            stringify!(step)
        )
    );
}
pub type QHYCamMinMaxStepValue = _QHYCamMinMaxStepValue;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _QHYGetImageParam {
    pub handle: *mut ::std::os::raw::c_void,
    pub imgdata: *mut u8,
    pub w: u32,
    pub h: u32,
    pub bpp: u32,
    pub channels: u32,
    pub HaveImgData: bool,
}
#[test]
fn bindgen_test_layout__QHYGetImageParam() {
    assert_eq!(
        std::mem::size_of::<_QHYGetImageParam>(),
        40usize,
        concat!("Size of: ", stringify!(_QHYGetImageParam))
    );
    assert_eq!(
        std::mem::align_of::<_QHYGetImageParam>(),
        8usize,
        concat!("Alignment of ", stringify!(_QHYGetImageParam))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_QHYGetImageParam>())).handle as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_QHYGetImageParam),
            "::",
            stringify!(handle)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_QHYGetImageParam>())).imgdata as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_QHYGetImageParam),
            "::",
            stringify!(imgdata)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_QHYGetImageParam>())).w as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_QHYGetImageParam),
            "::",
            stringify!(w)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_QHYGetImageParam>())).h as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(_QHYGetImageParam),
            "::",
            stringify!(h)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_QHYGetImageParam>())).bpp as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_QHYGetImageParam),
            "::",
            stringify!(bpp)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_QHYGetImageParam>())).channels as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(_QHYGetImageParam),
            "::",
            stringify!(channels)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_QHYGetImageParam>())).HaveImgData as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_QHYGetImageParam),
            "::",
            stringify!(HaveImgData)
        )
    );
}
pub type QHYGetImageParam = _QHYGetImageParam;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HistCoordinates {
    pub x: [::std::os::raw::c_int; 3000usize],
    pub y: [::std::os::raw::c_int; 3000usize],
}
#[test]
fn bindgen_test_layout_HistCoordinates() {
    assert_eq!(
        std::mem::size_of::<HistCoordinates>(),
        24000usize,
        concat!("Size of: ", stringify!(HistCoordinates))
    );
    assert_eq!(
        std::mem::align_of::<HistCoordinates>(),
        4usize,
        concat!("Alignment of ", stringify!(HistCoordinates))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<HistCoordinates>())).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(HistCoordinates),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<HistCoordinates>())).y as *const _ as usize },
        12000usize,
        concat!(
            "Offset of field: ",
            stringify!(HistCoordinates),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StarRough {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub starLevel: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_StarRough() {
    assert_eq!(
        std::mem::size_of::<StarRough>(),
        12usize,
        concat!("Size of: ", stringify!(StarRough))
    );
    assert_eq!(
        std::mem::align_of::<StarRough>(),
        4usize,
        concat!("Alignment of ", stringify!(StarRough))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<StarRough>())).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(StarRough),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<StarRough>())).y as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(StarRough),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<StarRough>())).starLevel as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(StarRough),
            "::",
            stringify!(starLevel)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StarData {
    pub x: f64,
    pub y: f64,
    pub snr: f64,
    pub brightness: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_StarData() {
    assert_eq!(
        std::mem::size_of::<StarData>(),
        32usize,
        concat!("Size of: ", stringify!(StarData))
    );
    assert_eq!(
        std::mem::align_of::<StarData>(),
        8usize,
        concat!("Alignment of ", stringify!(StarData))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<StarData>())).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(StarData),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<StarData>())).y as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(StarData),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<StarData>())).snr as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(StarData),
            "::",
            stringify!(snr)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<StarData>())).brightness as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(StarData),
            "::",
            stringify!(brightness)
        )
    );
}
pub type QHYCCDProcCallBack = std::option::Option<
    unsafe extern "C" fn(
        handle: *mut ::std::os::raw::c_void,
        message: QHYDWORD,
        wParam: QHYDWORD,
        lParam: QHYDWORD,
    ) -> QHYDWORD,
>;
pub type std_size_t = ::std::os::raw::c_ulong;
pub type std_nullptr_t = *const ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_stringbuf {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_istringstream {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_ostringstream {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_stringstream {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_numpunct {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_numpunct_byname {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_collate {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_collate_byname {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_time_get {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_time_get_byname {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_money_get {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_money_put {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_messages {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_messages_byname {
    pub _address: u8,
}
#[repr(C)]
pub struct std_basic_string<_CharT> {
    pub _M_dataplus: std_basic_string__Alloc_hider,
    pub _M_string_length: std_basic_string_size_type,
    pub __bindgen_anon_1: std_basic_string__bindgen_ty_2<_CharT>,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_CharT>>,
}
pub type std_basic_string__Char_alloc_type = [u8; 0usize];
pub type std_basic_string__Alloc_traits = __gnu_cxx___alloc_traits;
pub type std_basic_string_traits_type<_Traits> = _Traits;
pub type std_basic_string_value_type = [u8; 0usize];
pub type std_basic_string_allocator_type = std_basic_string__Char_alloc_type;
pub type std_basic_string_size_type = [u8; 0usize];
pub type std_basic_string_difference_type = [u8; 0usize];
pub type std_basic_string_reference = [u8; 0usize];
pub type std_basic_string_const_reference = [u8; 0usize];
pub type std_basic_string_pointer = [u8; 0usize];
pub type std_basic_string_const_pointer = [u8; 0usize];
pub type std_basic_string_iterator = __gnu_cxx___normal_iterator<std_basic_string_pointer>;
pub type std_basic_string_const_iterator =
    __gnu_cxx___normal_iterator<std_basic_string_const_pointer>;
pub type std_basic_string_const_reverse_iterator =
    std_reverse_iterator<std_basic_string_const_iterator>;
pub type std_basic_string_reverse_iterator = std_reverse_iterator<std_basic_string_iterator>;
pub type std_basic_string___const_iterator = std_basic_string_const_iterator;
#[repr(C)]
pub struct std_basic_string__Alloc_hider {
    pub _M_p: std_basic_string_pointer,
}
pub const std_basic_string__S_local_capacity: i32 = 0;
pub type std_basic_string__bindgen_ty_1 = i32;
#[repr(C)]
pub struct std_basic_string__bindgen_ty_2<_CharT> {
    pub _M_local_buf: __BindgenUnionField<*mut _CharT>,
    pub _M_allocated_capacity: __BindgenUnionField<std_basic_string_size_type>,
    pub bindgen_union_field: u64,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_CharT>>,
}
pub type std_integral_constant_value_type<_Tp> = _Tp;
pub type std_integral_constant_type = u8;
// extern "C" {
//     #[link_name = "\u{1}value"]
//     pub static std_value: _Tp;
// }
pub type std_true_type = u8;
pub type std_false_type = u8;
pub type std___bool_constant = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___or_ {
    pub _address: u8,
}
#[test]
fn __bindgen_test_layout_std___or__open0_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___or_>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std___or_))
    );
    assert_eq!(
        std::mem::align_of::<std___or_>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___or_)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___and_ {
    pub _address: u8,
}
#[test]
fn __bindgen_test_layout_std___and__open0_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___and_>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std___and_))
    );
    assert_eq!(
        std::mem::align_of::<std___and_>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___and_)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___not_ {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___success_type {
    pub _address: u8,
}
pub type std___success_type_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___failure_type {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___failure_type() {
    assert_eq!(
        std::mem::size_of::<std___failure_type>(),
        1usize,
        concat!("Size of: ", stringify!(std___failure_type))
    );
    assert_eq!(
        std::mem::align_of::<std___failure_type>(),
        1usize,
        concat!("Alignment of ", stringify!(std___failure_type))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_void_helper {
    pub _base: std_false_type,
}
#[test]
fn __bindgen_test_layout_std___is_void_helper_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_void_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_void_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_void_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_void_helper)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_void {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_integral_helper {
    pub _base: std_false_type,
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0_bool__close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0_signed_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0_unsigned_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0_wchar_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0_char16_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0_char32_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0_short_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0_unsigned_short_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0_int_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0_unsigned_int_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0_unsigned_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0_long_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0_unsigned_long_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0___int128_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integral_helper_open0_unsigned___int128_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integral_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integral_helper)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_integral {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_floating_point_helper {
    pub _base: std_false_type,
}
#[test]
fn __bindgen_test_layout_std___is_floating_point_helper_open0_float_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_floating_point_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_floating_point_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_floating_point_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_floating_point_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_floating_point_helper_open0_double_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_floating_point_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_floating_point_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_floating_point_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_floating_point_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_floating_point_helper_open0_long_double_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_floating_point_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_floating_point_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_floating_point_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_floating_point_helper)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_floating_point_helper_open0___float128_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_floating_point_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_floating_point_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_floating_point_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_floating_point_helper)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_floating_point {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_array {
    pub _base: std_false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_pointer_helper {
    pub _base: std_false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_pointer {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_lvalue_reference {
    pub _base: std_false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_rvalue_reference {
    pub _base: std_false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_member_object_pointer_helper {
    pub _base: std_false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_member_object_pointer {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_member_function_pointer_helper {
    pub _base: std_false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_member_function_pointer {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_enum {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_union {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_class {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_function {
    pub _base: std_false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_null_pointer_helper {
    pub _base: std_false_type,
}
#[test]
fn __bindgen_test_layout_std___is_null_pointer_helper_open0_nullptr_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_null_pointer_helper>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_null_pointer_helper)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_null_pointer_helper>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_null_pointer_helper)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_null_pointer {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_nullptr_t {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_reference {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_arithmetic {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_fundamental {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_object {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_scalar {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_compound {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_member_pointer_helper {
    pub _base: std_false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_member_pointer {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_referenceable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_const {
    pub _base: std_false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_volatile {
    pub _base: std_false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_trivial {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_trivially_copyable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_standard_layout {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_pod {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_literal_type {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_empty {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_polymorphic {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_final {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_abstract {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_signed {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_unsigned {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_array_known_bounds {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_array_unknown_bounds {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___do_is_destructible_impl {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___do_is_destructible_impl() {
    assert_eq!(
        std::mem::size_of::<std___do_is_destructible_impl>(),
        1usize,
        concat!("Size of: ", stringify!(std___do_is_destructible_impl))
    );
    assert_eq!(
        std::mem::align_of::<std___do_is_destructible_impl>(),
        1usize,
        concat!("Alignment of ", stringify!(std___do_is_destructible_impl))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_destructible_impl {
    pub _address: u8,
}
pub type std___is_destructible_impl_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_destructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___do_is_nt_destructible_impl {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___do_is_nt_destructible_impl() {
    assert_eq!(
        std::mem::size_of::<std___do_is_nt_destructible_impl>(),
        1usize,
        concat!("Size of: ", stringify!(std___do_is_nt_destructible_impl))
    );
    assert_eq!(
        std::mem::align_of::<std___do_is_nt_destructible_impl>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(std___do_is_nt_destructible_impl)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_nt_destructible_impl {
    pub _address: u8,
}
pub type std___is_nt_destructible_impl_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_nothrow_destructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_default_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_copy_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_move_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_nt_default_constructible_atom {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_nothrow_default_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_nt_constructible_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_nothrow_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_nothrow_copy_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_nothrow_move_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_copy_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_move_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_nt_assignable_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_nothrow_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_nothrow_copy_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_nothrow_move_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_trivially_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_trivially_default_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___do_is_implicitly_default_constructible_impl {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___do_is_implicitly_default_constructible_impl() {
    assert_eq!(
        std::mem::size_of::<std___do_is_implicitly_default_constructible_impl>(),
        1usize,
        concat!(
            "Size of: ",
            stringify!(std___do_is_implicitly_default_constructible_impl)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___do_is_implicitly_default_constructible_impl>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(std___do_is_implicitly_default_constructible_impl)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt45__do_is_implicitly_default_constructible_impl6__testEz"]
    pub fn std___do_is_implicitly_default_constructible_impl___test() -> std_false_type;
}
impl std___do_is_implicitly_default_constructible_impl {
    #[inline]
    pub unsafe fn __test() -> std_false_type {
        std___do_is_implicitly_default_constructible_impl___test()
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_implicitly_default_constructible_impl {
    pub _address: u8,
}
pub type std___is_implicitly_default_constructible_impl_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_implicitly_default_constructible_safe {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_implicitly_default_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_trivially_copy_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_trivially_move_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_trivially_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_trivially_copy_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_trivially_move_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_trivially_destructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_has_virtual_destructor {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_alignment_of {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_rank {
    pub _base: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_same {
    pub _base: std_false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_base_of {
    pub _address: u8,
}
pub type std___is_convertible_helper_type = std_is_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_convertible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_remove_const {
    pub _address: u8,
}
pub type std_remove_const_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_remove_volatile {
    pub _address: u8,
}
pub type std_remove_volatile_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_remove_cv {
    pub _address: u8,
}
pub type std_remove_cv_type = std_remove_const;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_add_const {
    pub _address: u8,
}
pub type std_add_const_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_add_volatile {
    pub _address: u8,
}
pub type std_add_volatile_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_add_cv {
    pub _address: u8,
}
pub type std_add_cv_type = std_add_const;
pub type std_remove_const_t = std_remove_const;
pub type std_remove_volatile_t = std_remove_volatile;
pub type std_remove_cv_t = std_remove_cv;
pub type std_add_const_t = std_add_const;
pub type std_add_volatile_t = std_add_volatile;
pub type std_add_cv_t = std_add_cv;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_remove_reference {
    pub _address: u8,
}
pub type std_remove_reference_type<_Tp> = _Tp;
pub type std___add_lvalue_reference_helper_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_add_lvalue_reference {
    pub _address: u8,
}
pub type std___add_rvalue_reference_helper_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_add_rvalue_reference {
    pub _address: u8,
}
pub type std_remove_reference_t = std_remove_reference;
pub type std_add_lvalue_reference_t = std_add_lvalue_reference;
pub type std_add_rvalue_reference_t = std_add_rvalue_reference;
pub type std___match_cv_qualifiers___match = u8;
pub type std___match_cv_qualifiers___type = std___match_cv_qualifiers___match;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___make_unsigned {
    pub _address: u8,
}
pub type std___make_unsigned___type<_Tp> = _Tp;
#[test]
fn __bindgen_test_layout_std___make_unsigned_open0_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_unsigned_open0_signed_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_unsigned_open0_short_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_unsigned_open0_int_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_unsigned_open0_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_unsigned_open0_long_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_unsigned_open0___int128_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___make_unsigned_selector_base {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___make_unsigned_selector_base__List {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___make_unsigned_selector_base() {
    assert_eq!(
        std::mem::size_of::<std___make_unsigned_selector_base>(),
        1usize,
        concat!("Size of: ", stringify!(std___make_unsigned_selector_base))
    );
    assert_eq!(
        std::mem::align_of::<std___make_unsigned_selector_base>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(std___make_unsigned_selector_base)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_unsigned_open0_wchar_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_unsigned_open0_char16_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_unsigned_open0_char32_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_unsigned)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_make_unsigned {
    pub _address: u8,
}
pub type std_make_unsigned_type = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___make_signed {
    pub _address: u8,
}
pub type std___make_signed___type<_Tp> = _Tp;
#[test]
fn __bindgen_test_layout_std___make_signed_open0_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_signed>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_signed)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_signed>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_signed)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_signed_open0_unsigned_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_signed>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_signed)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_signed>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_signed)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_signed_open0_unsigned_short_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_signed>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_signed)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_signed>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_signed)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_signed_open0_unsigned_int_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_signed>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_signed)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_signed>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_signed)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_signed_open0_unsigned_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_signed>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_signed)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_signed>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_signed)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_signed_open0_unsigned_long_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_signed>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_signed)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_signed>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_signed)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_signed_open0_unsigned___int128_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_signed>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_signed)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_signed>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_signed)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_signed_open0_wchar_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_signed>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_signed)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_signed>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_signed)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_signed_open0_char16_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_signed>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_signed)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_signed>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_signed)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___make_signed_open0_char32_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_signed>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_signed)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_signed>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_signed)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_make_signed {
    pub _address: u8,
}
pub type std_make_signed_type = u8;
pub type std_make_signed_t = std_make_signed;
pub type std_make_unsigned_t = std_make_unsigned;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_remove_extent {
    pub _address: u8,
}
pub type std_remove_extent_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_remove_all_extents {
    pub _address: u8,
}
pub type std_remove_all_extents_type<_Tp> = _Tp;
pub type std_remove_extent_t = std_remove_extent;
pub type std_remove_all_extents_t = std_remove_all_extents;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___remove_pointer_helper {
    pub _address: u8,
}
pub type std___remove_pointer_helper_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_remove_pointer {
    pub _address: u8,
}
pub type std___add_pointer_helper_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_add_pointer {
    pub _address: u8,
}
pub type std_remove_pointer_t = std_remove_pointer;
pub type std_add_pointer_t = std_add_pointer;
#[repr(C)]
#[derive(Copy, Clone)]
pub union std___aligned_storage_msa___type {
    pub __data: *mut ::std::os::raw::c_uchar,
    pub __align: std___aligned_storage_msa___type__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___aligned_storage_msa___type__bindgen_ty_1 {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___aligned_storage_msa___type() {
    assert_eq!(
        std::mem::size_of::<std___aligned_storage_msa___type>(),
        8usize,
        concat!("Size of: ", stringify!(std___aligned_storage_msa___type))
    );
    assert_eq!(
        std::mem::align_of::<std___aligned_storage_msa___type>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(std___aligned_storage_msa___type)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union std_aligned_storage_type {
    pub __data: *mut ::std::os::raw::c_uchar,
    pub __align: std_aligned_storage_type__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_aligned_storage_type__bindgen_ty_1 {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std_aligned_storage_type() {
    assert_eq!(
        std::mem::size_of::<std_aligned_storage_type>(),
        8usize,
        concat!("Size of: ", stringify!(std_aligned_storage_type))
    );
    assert_eq!(
        std::mem::align_of::<std_aligned_storage_type>(),
        8usize,
        concat!("Alignment of ", stringify!(std_aligned_storage_type))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___strictest_alignment {
    pub _address: u8,
}
pub type std_aligned_union___strictest = std___strictest_alignment;
pub type std_aligned_union_type = u8;
extern "C" {
    #[link_name = "\u{1}alignment_value"]
    pub static std_alignment_value: std_size_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_decay {
    pub _address: u8,
}
pub type std_decay___remove_type = std_remove_reference;
pub type std_decay_type = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___strip_reference_wrapper {
    pub _address: u8,
}
pub type std___strip_reference_wrapper___type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___decay_and_strip {
    pub _address: u8,
}
pub type std___decay_and_strip___type = std___strip_reference_wrapper;
pub type std__Require = u8;
pub type std_conditional_type<_Iftrue> = _Iftrue;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_common_type {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___do_common_type_impl {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___do_common_type_impl() {
    assert_eq!(
        std::mem::size_of::<std___do_common_type_impl>(),
        1usize,
        concat!("Size of: ", stringify!(std___do_common_type_impl))
    );
    assert_eq!(
        std::mem::align_of::<std___do_common_type_impl>(),
        1usize,
        concat!("Alignment of ", stringify!(std___do_common_type_impl))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___common_type_impl {
    pub _address: u8,
}
pub type std___common_type_impl_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___do_member_type_wrapper {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___do_member_type_wrapper() {
    assert_eq!(
        std::mem::size_of::<std___do_member_type_wrapper>(),
        1usize,
        concat!("Size of: ", stringify!(std___do_member_type_wrapper))
    );
    assert_eq!(
        std::mem::align_of::<std___do_member_type_wrapper>(),
        1usize,
        concat!("Alignment of ", stringify!(std___do_member_type_wrapper))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___member_type_wrapper {
    pub _address: u8,
}
pub type std___member_type_wrapper_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___expanded_common_type_wrapper {
    pub _address: u8,
}
pub type std___expanded_common_type_wrapper_type = std_common_type;
#[test]
fn __bindgen_test_layout_std_common_type_open0_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_common_type>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_common_type)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_common_type>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_common_type)
        )
    );
}
pub type std___underlying_type_impl_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_underlying_type {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___declval_protector {
    pub _address: u8,
}
pub type std___remove_cvref_t = std_remove_cv;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_result_of {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___invoke_memfun_ref {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___invoke_memfun_ref() {
    assert_eq!(
        std::mem::size_of::<std___invoke_memfun_ref>(),
        1usize,
        concat!("Size of: ", stringify!(std___invoke_memfun_ref))
    );
    assert_eq!(
        std::mem::align_of::<std___invoke_memfun_ref>(),
        1usize,
        concat!("Alignment of ", stringify!(std___invoke_memfun_ref))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___invoke_memfun_deref {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___invoke_memfun_deref() {
    assert_eq!(
        std::mem::size_of::<std___invoke_memfun_deref>(),
        1usize,
        concat!("Size of: ", stringify!(std___invoke_memfun_deref))
    );
    assert_eq!(
        std::mem::align_of::<std___invoke_memfun_deref>(),
        1usize,
        concat!("Alignment of ", stringify!(std___invoke_memfun_deref))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___invoke_memobj_ref {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___invoke_memobj_ref() {
    assert_eq!(
        std::mem::size_of::<std___invoke_memobj_ref>(),
        1usize,
        concat!("Size of: ", stringify!(std___invoke_memobj_ref))
    );
    assert_eq!(
        std::mem::align_of::<std___invoke_memobj_ref>(),
        1usize,
        concat!("Alignment of ", stringify!(std___invoke_memobj_ref))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___invoke_memobj_deref {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___invoke_memobj_deref() {
    assert_eq!(
        std::mem::size_of::<std___invoke_memobj_deref>(),
        1usize,
        concat!("Size of: ", stringify!(std___invoke_memobj_deref))
    );
    assert_eq!(
        std::mem::align_of::<std___invoke_memobj_deref>(),
        1usize,
        concat!("Alignment of ", stringify!(std___invoke_memobj_deref))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___invoke_other {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___invoke_other() {
    assert_eq!(
        std::mem::size_of::<std___invoke_other>(),
        1usize,
        concat!("Size of: ", stringify!(std___invoke_other))
    );
    assert_eq!(
        std::mem::align_of::<std___invoke_other>(),
        1usize,
        concat!("Alignment of ", stringify!(std___invoke_other))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___result_of_success {
    pub _address: u8,
}
pub type std___result_of_success___invoke_type<_Tag> = _Tag;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___result_of_memfun_ref_impl {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___result_of_memfun_ref_impl() {
    assert_eq!(
        std::mem::size_of::<std___result_of_memfun_ref_impl>(),
        1usize,
        concat!("Size of: ", stringify!(std___result_of_memfun_ref_impl))
    );
    assert_eq!(
        std::mem::align_of::<std___result_of_memfun_ref_impl>(),
        1usize,
        concat!("Alignment of ", stringify!(std___result_of_memfun_ref_impl))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___result_of_memfun_ref {
    pub _address: u8,
}
pub type std___result_of_memfun_ref_type<_MemPtr> = _MemPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___result_of_memfun_deref_impl {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___result_of_memfun_deref_impl() {
    assert_eq!(
        std::mem::size_of::<std___result_of_memfun_deref_impl>(),
        1usize,
        concat!("Size of: ", stringify!(std___result_of_memfun_deref_impl))
    );
    assert_eq!(
        std::mem::align_of::<std___result_of_memfun_deref_impl>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(std___result_of_memfun_deref_impl)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___result_of_memfun_deref {
    pub _address: u8,
}
pub type std___result_of_memfun_deref_type<_MemPtr> = _MemPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___result_of_memobj_ref_impl {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___result_of_memobj_ref_impl() {
    assert_eq!(
        std::mem::size_of::<std___result_of_memobj_ref_impl>(),
        1usize,
        concat!("Size of: ", stringify!(std___result_of_memobj_ref_impl))
    );
    assert_eq!(
        std::mem::align_of::<std___result_of_memobj_ref_impl>(),
        1usize,
        concat!("Alignment of ", stringify!(std___result_of_memobj_ref_impl))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___result_of_memobj_ref {
    pub _address: u8,
}
pub type std___result_of_memobj_ref_type<_MemPtr> = _MemPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___result_of_memobj_deref_impl {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___result_of_memobj_deref_impl() {
    assert_eq!(
        std::mem::size_of::<std___result_of_memobj_deref_impl>(),
        1usize,
        concat!("Size of: ", stringify!(std___result_of_memobj_deref_impl))
    );
    assert_eq!(
        std::mem::align_of::<std___result_of_memobj_deref_impl>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(std___result_of_memobj_deref_impl)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___result_of_memobj_deref {
    pub _address: u8,
}
pub type std___result_of_memobj_deref_type<_MemPtr> = _MemPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___result_of_memobj {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___result_of_memfun {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___inv_unwrap {
    pub _address: u8,
}
pub type std___inv_unwrap_type<_Tp> = _Tp;
pub type std___result_of_impl_type = std___failure_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___result_of_other_impl {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___result_of_other_impl() {
    assert_eq!(
        std::mem::size_of::<std___result_of_other_impl>(),
        1usize,
        concat!("Size of: ", stringify!(std___result_of_other_impl))
    );
    assert_eq!(
        std::mem::align_of::<std___result_of_other_impl>(),
        1usize,
        concat!("Alignment of ", stringify!(std___result_of_other_impl))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___invoke_result {
    pub _address: u8,
}
pub type std_aligned_storage_t = u8;
pub type std_aligned_union_t = u8;
pub type std_decay_t = std_decay;
pub type std_enable_if_t = u8;
pub type std_conditional_t = u8;
pub type std_common_type_t = std_common_type;
pub type std_underlying_type_t = std_underlying_type;
pub type std_result_of_t = std_result_of;
pub type std___enable_if_t = u8;
pub type std___void_t = ::std::os::raw::c_void;
pub type std_void_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___detector {
    pub _address: u8,
}
pub type std___detector_value_t = std_false_type;
pub type std___detector_type<_Default> = _Default;
pub type std___detected_or = std___detector;
pub type std___detected_or_t = std___detected_or;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_tuple_like_impl {
    pub _base: std_false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_tuple_like {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___swappable_details___do_is_swappable_impl {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___swappable_details___do_is_swappable_impl() {
    assert_eq!(
        std::mem::size_of::<std___swappable_details___do_is_swappable_impl>(),
        1usize,
        concat!(
            "Size of: ",
            stringify!(std___swappable_details___do_is_swappable_impl)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___swappable_details___do_is_swappable_impl>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(std___swappable_details___do_is_swappable_impl)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___swappable_details___do_is_nothrow_swappable_impl {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___swappable_details___do_is_nothrow_swappable_impl() {
    assert_eq!(
        std::mem::size_of::<std___swappable_details___do_is_nothrow_swappable_impl>(),
        1usize,
        concat!(
            "Size of: ",
            stringify!(std___swappable_details___do_is_nothrow_swappable_impl)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___swappable_details___do_is_nothrow_swappable_impl>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(std___swappable_details___do_is_nothrow_swappable_impl)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_swappable_impl {
    pub _address: u8,
}
pub type std___is_swappable_impl_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_nothrow_swappable_impl {
    pub _address: u8,
}
pub type std___is_nothrow_swappable_impl_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_swappable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_nothrow_swappable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_swappable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_nothrow_swappable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___swappable_with_details___do_is_swappable_with_impl {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___swappable_with_details___do_is_swappable_with_impl() {
    assert_eq!(
        std::mem::size_of::<std___swappable_with_details___do_is_swappable_with_impl>(),
        1usize,
        concat!(
            "Size of: ",
            stringify!(std___swappable_with_details___do_is_swappable_with_impl)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___swappable_with_details___do_is_swappable_with_impl>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(std___swappable_with_details___do_is_swappable_with_impl)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___swappable_with_details___do_is_nothrow_swappable_with_impl {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___swappable_with_details___do_is_nothrow_swappable_with_impl() {
    assert_eq!(
        std::mem::size_of::<std___swappable_with_details___do_is_nothrow_swappable_with_impl>(),
        1usize,
        concat!(
            "Size of: ",
            stringify!(std___swappable_with_details___do_is_nothrow_swappable_with_impl)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___swappable_with_details___do_is_nothrow_swappable_with_impl>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(std___swappable_with_details___do_is_nothrow_swappable_with_impl)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_swappable_with_impl {
    pub _address: u8,
}
pub type std___is_swappable_with_impl_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_nothrow_swappable_with_impl {
    pub _address: u8,
}
pub type std___is_nothrow_swappable_with_impl_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_swappable_with {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_nothrow_swappable_with {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_invocable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___call_is_nothrow {
    pub _address: u8,
}
pub type std___call_is_nothrow_ = std___call_is_nothrow;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_nothrow_invocable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug)]
pub struct std___nonesuch {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___nonesuch() {
    assert_eq!(
        std::mem::size_of::<std___nonesuch>(),
        1usize,
        concat!("Size of: ", stringify!(std___nonesuch))
    );
    assert_eq!(
        std::mem::align_of::<std___nonesuch>(),
        1usize,
        concat!("Alignment of ", stringify!(std___nonesuch))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___move_if_noexcept_cond {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_unary_function {
    pub _address: u8,
}
pub type std_unary_function_argument_type<_Arg> = _Arg;
pub type std_unary_function_result_type<_Result> = _Result;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_binary_function {
    pub _address: u8,
}
pub type std_binary_function_first_argument_type<_Arg1> = _Arg1;
pub type std_binary_function_second_argument_type<_Arg2> = _Arg2;
pub type std_binary_function_result_type<_Result> = _Result;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_transparent {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_plus {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_minus {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_multiplies {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_divides {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_modulus {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_negate {
    pub _address: u8,
}
#[test]
fn __bindgen_test_layout_std_plus_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_plus>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_plus))
    );
    assert_eq!(
        std::mem::align_of::<std_plus>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_plus)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_minus_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_minus>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_minus))
    );
    assert_eq!(
        std::mem::align_of::<std_minus>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_minus)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_multiplies_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_multiplies>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_multiplies)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_multiplies>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_multiplies)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_divides_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_divides>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_divides))
    );
    assert_eq!(
        std::mem::align_of::<std_divides>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_divides)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_modulus_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_modulus>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_modulus))
    );
    assert_eq!(
        std::mem::align_of::<std_modulus>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_modulus)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_negate_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_negate>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_negate))
    );
    assert_eq!(
        std::mem::align_of::<std_negate>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_negate)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_equal_to {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_not_equal_to {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_greater {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_less {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_greater_equal {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_less_equal {
    pub _address: u8,
}
#[test]
fn __bindgen_test_layout_std_equal_to_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_equal_to>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_equal_to)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_equal_to>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_equal_to)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_not_equal_to_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_not_equal_to>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_not_equal_to)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_not_equal_to>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_not_equal_to)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_greater_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_greater>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_greater))
    );
    assert_eq!(
        std::mem::align_of::<std_greater>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_greater)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_less_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_less>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_less))
    );
    assert_eq!(
        std::mem::align_of::<std_less>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_less)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_greater_equal_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_greater_equal>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_greater_equal)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_greater_equal>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_greater_equal)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_less_equal_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_less_equal>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_less_equal)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_less_equal>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_less_equal)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_logical_and {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_logical_or {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_logical_not {
    pub _address: u8,
}
#[test]
fn __bindgen_test_layout_std_logical_and_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_logical_and>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_logical_and)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_logical_and>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_logical_and)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_logical_or_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_logical_or>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_logical_or)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_logical_or>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_logical_or)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_logical_not_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_logical_not>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_logical_not)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_logical_not>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_logical_not)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_bit_and {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_bit_or {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_bit_xor {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_bit_not {
    pub _address: u8,
}
#[test]
fn __bindgen_test_layout_std_bit_and_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_bit_and>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_bit_and))
    );
    assert_eq!(
        std::mem::align_of::<std_bit_and>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_bit_and)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_bit_or_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_bit_or>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_bit_or))
    );
    assert_eq!(
        std::mem::align_of::<std_bit_or>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_bit_or)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_bit_xor_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_bit_xor>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_bit_xor))
    );
    assert_eq!(
        std::mem::align_of::<std_bit_xor>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_bit_xor)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_bit_not_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_bit_not>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_bit_not))
    );
    assert_eq!(
        std::mem::align_of::<std_bit_not>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_bit_not)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_unary_negate<_Predicate> {
    pub _M_pred: _Predicate,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Predicate>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_binary_negate<_Predicate> {
    pub _M_pred: _Predicate,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Predicate>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_pointer_to_unary_function<_Arg, _Result> {
    pub _M_ptr: std::option::Option<unsafe extern "C" fn(arg1: _Arg) -> _Result>,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Arg>>,
    pub _phantom_1: std::marker::PhantomData<std::cell::UnsafeCell<_Result>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_pointer_to_binary_function<_Arg1, _Arg2, _Result> {
    pub _M_ptr: std::option::Option<unsafe extern "C" fn(arg1: _Arg1, arg2: _Arg2) -> _Result>,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Arg1>>,
    pub _phantom_1: std::marker::PhantomData<std::cell::UnsafeCell<_Arg2>>,
    pub _phantom_2: std::marker::PhantomData<std::cell::UnsafeCell<_Result>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Identity {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Select1st {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Select2nd {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_mem_fun_t<_Ret> {
    pub _M_f: std::option::Option<unsafe extern "C" fn() -> _Ret>,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Ret>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_const_mem_fun_t<_Ret> {
    pub _M_f: std::option::Option<unsafe extern "C" fn() -> _Ret>,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Ret>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_mem_fun_ref_t<_Ret> {
    pub _M_f: std::option::Option<unsafe extern "C" fn() -> _Ret>,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Ret>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_const_mem_fun_ref_t<_Ret> {
    pub _M_f: std::option::Option<unsafe extern "C" fn() -> _Ret>,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Ret>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_mem_fun1_t<_Ret, _Arg> {
    pub _M_f: std::option::Option<unsafe extern "C" fn(arg1: _Arg) -> _Ret>,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Ret>>,
    pub _phantom_1: std::marker::PhantomData<std::cell::UnsafeCell<_Arg>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_const_mem_fun1_t<_Ret, _Arg> {
    pub _M_f: std::option::Option<unsafe extern "C" fn(arg1: _Arg) -> _Ret>,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Ret>>,
    pub _phantom_1: std::marker::PhantomData<std::cell::UnsafeCell<_Arg>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_mem_fun1_ref_t<_Ret, _Arg> {
    pub _M_f: std::option::Option<unsafe extern "C" fn(arg1: _Arg) -> _Ret>,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Ret>>,
    pub _phantom_1: std::marker::PhantomData<std::cell::UnsafeCell<_Arg>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_const_mem_fun1_ref_t<_Ret, _Arg> {
    pub _M_f: std::option::Option<unsafe extern "C" fn(arg1: _Arg) -> _Ret>,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Ret>>,
    pub _phantom_1: std::marker::PhantomData<std::cell::UnsafeCell<_Arg>>,
}
#[repr(C)]
pub struct std_binder1st<_Operation> {
    pub op: _Operation,
    pub value: [u8; 0usize],
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Operation>>,
}
#[repr(C)]
pub struct std_binder2nd<_Operation> {
    pub op: _Operation,
    pub value: [u8; 0usize],
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Operation>>,
}
#[repr(C)]
pub struct std_exception__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct std_exception {
    pub vtable_: *const std_exception__bindgen_vtable,
}
#[test]
fn bindgen_test_layout_std_exception() {
    assert_eq!(
        std::mem::size_of::<std_exception>(),
        8usize,
        concat!("Size of: ", stringify!(std_exception))
    );
    assert_eq!(
        std::mem::align_of::<std_exception>(),
        8usize,
        concat!("Alignment of ", stringify!(std_exception))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt9exceptionD1Ev"]
    pub fn std_exception_exception_destructor(this: *mut std_exception);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt9exception4whatEv"]
    pub fn std_exception_what(this: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct std_bad_exception {
    pub _base: std_exception,
}
#[test]
fn bindgen_test_layout_std_bad_exception() {
    assert_eq!(
        std::mem::size_of::<std_bad_exception>(),
        8usize,
        concat!("Size of: ", stringify!(std_bad_exception))
    );
    assert_eq!(
        std::mem::align_of::<std_bad_exception>(),
        8usize,
        concat!("Alignment of ", stringify!(std_bad_exception))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt13bad_exceptionD1Ev"]
    pub fn std_bad_exception_bad_exception_destructor(this: *mut std_bad_exception);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt13bad_exception4whatEv"]
    pub fn std_bad_exception_what(
        this: *mut ::std::os::raw::c_void,
    ) -> *const ::std::os::raw::c_char;
}
pub type std_terminate_handler = std::option::Option<unsafe extern "C" fn()>;
pub type std_unexpected_handler = std::option::Option<unsafe extern "C" fn()>;
extern "C" {
    #[link_name = "\u{1}_ZSt13set_terminatePFvvE"]
    pub fn std_set_terminate(arg1: std_terminate_handler) -> std_terminate_handler;
}
extern "C" {
    #[link_name = "\u{1}_ZSt13get_terminatev"]
    pub fn std_get_terminate() -> std_terminate_handler;
}
extern "C" {
    #[link_name = "\u{1}_ZSt9terminatev"]
    pub fn std_terminate();
}
extern "C" {
    #[link_name = "\u{1}_ZSt14set_unexpectedPFvvE"]
    pub fn std_set_unexpected(arg1: std_unexpected_handler) -> std_unexpected_handler;
}
extern "C" {
    #[link_name = "\u{1}_ZSt14get_unexpectedv"]
    pub fn std_get_unexpected() -> std_unexpected_handler;
}
extern "C" {
    #[link_name = "\u{1}_ZSt10unexpectedv"]
    pub fn std_unexpected();
}
extern "C" {
    #[link_name = "\u{1}_ZSt18uncaught_exceptionv"]
    pub fn std_uncaught_exception() -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZSt19uncaught_exceptionsv"]
    pub fn std_uncaught_exceptions() -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_ZSt11_Hash_bytesPKvmm"]
    pub fn std__Hash_bytes(
        __ptr: *const ::std::os::raw::c_void,
        __len: std_size_t,
        __seed: std_size_t,
    ) -> std_size_t;
}
extern "C" {
    #[link_name = "\u{1}_ZSt15_Fnv_hash_bytesPKvmm"]
    pub fn std__Fnv_hash_bytes(
        __ptr: *const ::std::os::raw::c_void,
        __len: std_size_t,
        __seed: std_size_t,
    ) -> std_size_t;
}
#[repr(C)]
pub struct std_type_info__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct std_type_info {
    pub vtable_: *const std_type_info__bindgen_vtable,
    pub __name: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_std_type_info() {
    assert_eq!(
        std::mem::size_of::<std_type_info>(),
        16usize,
        concat!("Size of: ", stringify!(std_type_info))
    );
    assert_eq!(
        std::mem::align_of::<std_type_info>(),
        8usize,
        concat!("Alignment of ", stringify!(std_type_info))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<std_type_info>())).__name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(std_type_info),
            "::",
            stringify!(__name)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt9type_infoD1Ev"]
    pub fn std_type_info_type_info_destructor(this: *mut std_type_info);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt9type_info14__is_pointer_pEv"]
    pub fn std_type_info___is_pointer_p(this: *mut ::std::os::raw::c_void) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt9type_info15__is_function_pEv"]
    pub fn std_type_info___is_function_p(this: *mut ::std::os::raw::c_void) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt9type_info10__do_catchEPKS_PPvj"]
    pub fn std_type_info___do_catch(
        this: *mut ::std::os::raw::c_void,
        __thr_type: *const std_type_info,
        __thr_obj: *mut *mut ::std::os::raw::c_void,
        __outer: ::std::os::raw::c_uint,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt9type_info11__do_upcastEPKN10__cxxabiv117__class_type_infoEPPv"]
    pub fn std_type_info___do_upcast(
        this: *mut ::std::os::raw::c_void,
        __target: *const __cxxabiv1___class_type_info,
        __obj_ptr: *mut *mut ::std::os::raw::c_void,
    ) -> bool;
}
#[repr(C)]
#[derive(Debug)]
pub struct std_bad_cast {
    pub _base: std_exception,
}
#[test]
fn bindgen_test_layout_std_bad_cast() {
    assert_eq!(
        std::mem::size_of::<std_bad_cast>(),
        8usize,
        concat!("Size of: ", stringify!(std_bad_cast))
    );
    assert_eq!(
        std::mem::align_of::<std_bad_cast>(),
        8usize,
        concat!("Alignment of ", stringify!(std_bad_cast))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8bad_castD1Ev"]
    pub fn std_bad_cast_bad_cast_destructor(this: *mut std_bad_cast);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt8bad_cast4whatEv"]
    pub fn std_bad_cast_what(this: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct std_bad_typeid {
    pub _base: std_exception,
}
#[test]
fn bindgen_test_layout_std_bad_typeid() {
    assert_eq!(
        std::mem::size_of::<std_bad_typeid>(),
        8usize,
        concat!("Size of: ", stringify!(std_bad_typeid))
    );
    assert_eq!(
        std::mem::align_of::<std_bad_typeid>(),
        8usize,
        concat!("Alignment of ", stringify!(std_bad_typeid))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt10bad_typeidD1Ev"]
    pub fn std_bad_typeid_bad_typeid_destructor(this: *mut std_bad_typeid);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt10bad_typeid4whatEv"]
    pub fn std_bad_typeid_what(this: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct std___exception_ptr_exception_ptr {
    pub _M_exception_object: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_std___exception_ptr_exception_ptr() {
    assert_eq!(
        std::mem::size_of::<std___exception_ptr_exception_ptr>(),
        8usize,
        concat!("Size of: ", stringify!(std___exception_ptr_exception_ptr))
    );
    assert_eq!(
        std::mem::align_of::<std___exception_ptr_exception_ptr>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(std___exception_ptr_exception_ptr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<std___exception_ptr_exception_ptr>()))._M_exception_object
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(std___exception_ptr_exception_ptr),
            "::",
            stringify!(_M_exception_object)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt15__exception_ptr13exception_ptr4swapERS0_"]
    pub fn std___exception_ptr_exception_ptr_swap(
        this: *mut std___exception_ptr_exception_ptr,
        arg1: *mut std___exception_ptr_exception_ptr,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt15__exception_ptr13exception_ptr20__cxa_exception_typeEv"]
    pub fn std___exception_ptr_exception_ptr___cxa_exception_type(
        this: *const std___exception_ptr_exception_ptr,
    ) -> *const std_type_info;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt15__exception_ptr13exception_ptrC1Ev"]
    pub fn std___exception_ptr_exception_ptr_exception_ptr(
        this: *mut std___exception_ptr_exception_ptr,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt15__exception_ptr13exception_ptrC1ERKS0_"]
    pub fn std___exception_ptr_exception_ptr_exception_ptr1(
        this: *mut std___exception_ptr_exception_ptr,
        arg1: *const std___exception_ptr_exception_ptr,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt15__exception_ptr13exception_ptrD1Ev"]
    pub fn std___exception_ptr_exception_ptr_exception_ptr_destructor(
        this: *mut std___exception_ptr_exception_ptr,
    );
}
impl std___exception_ptr_exception_ptr {
    #[inline]
    pub unsafe fn swap(&mut self, arg1: *mut std___exception_ptr_exception_ptr) {
        std___exception_ptr_exception_ptr_swap(self, arg1)
    }
    #[inline]
    pub unsafe fn __cxa_exception_type(&self) -> *const std_type_info {
        std___exception_ptr_exception_ptr___cxa_exception_type(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std___exception_ptr_exception_ptr_exception_ptr(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(arg1: *const std___exception_ptr_exception_ptr) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std___exception_ptr_exception_ptr_exception_ptr1(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        std___exception_ptr_exception_ptr_exception_ptr_destructor(self)
    }
}
extern "C" {
    #[link_name = "\u{1}_ZSt17current_exceptionv"]
    pub fn std_current_exception() -> std___exception_ptr_exception_ptr;
}
extern "C" {
    #[link_name = "\u{1}_ZSt17rethrow_exceptionNSt15__exception_ptr13exception_ptrE"]
    pub fn std_rethrow_exception(arg1: std___exception_ptr_exception_ptr);
}
#[repr(C)]
pub struct std_nested_exception__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct std_nested_exception {
    pub vtable_: *const std_nested_exception__bindgen_vtable,
    pub _M_ptr: std___exception_ptr_exception_ptr,
}
#[test]
fn bindgen_test_layout_std_nested_exception() {
    assert_eq!(
        std::mem::size_of::<std_nested_exception>(),
        16usize,
        concat!("Size of: ", stringify!(std_nested_exception))
    );
    assert_eq!(
        std::mem::align_of::<std_nested_exception>(),
        8usize,
        concat!("Alignment of ", stringify!(std_nested_exception))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<std_nested_exception>()))._M_ptr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(std_nested_exception),
            "::",
            stringify!(_M_ptr)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt16nested_exceptionD1Ev"]
    pub fn std_nested_exception_nested_exception_destructor(this: *mut std_nested_exception);
}
#[repr(C)]
#[derive(Debug)]
pub struct std__Nested_exception<_Except> {
    pub _base: _Except,
    pub _base_1: std_nested_exception,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Except>>,
}
pub type std___rethrow_if_nested_cond = u8;
#[repr(C)]
#[derive(Debug)]
pub struct std_bad_alloc {
    pub _base: std_exception,
}
#[test]
fn bindgen_test_layout_std_bad_alloc() {
    assert_eq!(
        std::mem::size_of::<std_bad_alloc>(),
        8usize,
        concat!("Size of: ", stringify!(std_bad_alloc))
    );
    assert_eq!(
        std::mem::align_of::<std_bad_alloc>(),
        8usize,
        concat!("Alignment of ", stringify!(std_bad_alloc))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt9bad_allocD1Ev"]
    pub fn std_bad_alloc_bad_alloc_destructor(this: *mut std_bad_alloc);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt9bad_alloc4whatEv"]
    pub fn std_bad_alloc_what(this: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct std_bad_array_new_length {
    pub _base: std_bad_alloc,
}
#[test]
fn bindgen_test_layout_std_bad_array_new_length() {
    assert_eq!(
        std::mem::size_of::<std_bad_array_new_length>(),
        8usize,
        concat!("Size of: ", stringify!(std_bad_array_new_length))
    );
    assert_eq!(
        std::mem::align_of::<std_bad_array_new_length>(),
        8usize,
        concat!("Alignment of ", stringify!(std_bad_array_new_length))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt20bad_array_new_lengthD1Ev"]
    pub fn std_bad_array_new_length_bad_array_new_length_destructor(
        this: *mut std_bad_array_new_length,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt20bad_array_new_length4whatEv"]
    pub fn std_bad_array_new_length_what(
        this: *mut ::std::os::raw::c_void,
    ) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_nothrow_t {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std_nothrow_t() {
    assert_eq!(
        std::mem::size_of::<std_nothrow_t>(),
        1usize,
        concat!("Size of: ", stringify!(std_nothrow_t))
    );
    assert_eq!(
        std::mem::align_of::<std_nothrow_t>(),
        1usize,
        concat!("Alignment of ", stringify!(std_nothrow_t))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZSt7nothrow"]
    pub static std_nothrow: std_nothrow_t;
}
pub type std_new_handler = std::option::Option<unsafe extern "C" fn()>;
extern "C" {
    #[link_name = "\u{1}_ZSt15set_new_handlerPFvvE"]
    pub fn std_set_new_handler(arg1: std_new_handler) -> std_new_handler;
}
extern "C" {
    #[link_name = "\u{1}_ZSt15get_new_handlerv"]
    pub fn std_get_new_handler() -> std_new_handler;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_piecewise_construct_t {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std_piecewise_construct_t() {
    assert_eq!(
        std::mem::size_of::<std_piecewise_construct_t>(),
        1usize,
        concat!("Size of: ", stringify!(std_piecewise_construct_t))
    );
    assert_eq!(
        std::mem::align_of::<std_piecewise_construct_t>(),
        1usize,
        concat!("Alignment of ", stringify!(std_piecewise_construct_t))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZStL19piecewise_construct"]
    pub static std_piecewise_construct: std_piecewise_construct_t;
}
#[repr(C)]
#[derive(Debug)]
pub struct std___nonesuch_no_braces {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___nonesuch_no_braces() {
    assert_eq!(
        std::mem::size_of::<std___nonesuch_no_braces>(),
        1usize,
        concat!("Size of: ", stringify!(std___nonesuch_no_braces))
    );
    assert_eq!(
        std::mem::align_of::<std___nonesuch_no_braces>(),
        1usize,
        concat!("Alignment of ", stringify!(std___nonesuch_no_braces))
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct std___pair_base {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug)]
pub struct std_pair<_T1, _T2> {
    pub first: _T1,
    pub second: _T2,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_T1>>,
    pub _phantom_1: std::marker::PhantomData<std::cell::UnsafeCell<_T2>>,
}
pub type std_pair_first_type<_T1> = _T1;
pub type std_pair_second_type<_T2> = _T2;
pub type std_pair__PCCP = u8;
pub type std_pair__PCCFP = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_initializer_list<_E> {
    pub _M_array: std_initializer_list_iterator<_E>,
    pub _M_len: std_initializer_list_size_type,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_E>>,
}
pub type std_initializer_list_value_type<_E> = _E;
pub type std_initializer_list_reference<_E> = *const _E;
pub type std_initializer_list_const_reference<_E> = *const _E;
pub type std_initializer_list_size_type = std_size_t;
pub type std_initializer_list_iterator<_E> = *const _E;
pub type std_initializer_list_const_iterator<_E> = *const _E;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_tuple_size {
    pub _address: u8,
}
pub type std___enable_if_has_tuple_size<_Tp> = _Tp;
pub type std___tuple_element_t = u8;
pub type std_tuple_element_t = u8;
pub type std__Build_index_tuple__IdxTuple = u8;
pub type std__Build_index_tuple___type = u8;
pub type std_integer_sequence_value_type<_Tp> = _Tp;
pub type std_make_integer_sequence = u8;
pub type std_index_sequence = u8;
pub type std_make_index_sequence = std_make_integer_sequence;
pub type std_index_sequence_for = std_make_index_sequence;
pub type std_string = std_basic_string<::std::os::raw::c_char>;
pub type std_wstring = std_basic_string<u32>;
pub type std_u16string = std_basic_string<u16>;
pub type std_u32string = std_basic_string<u32>;
extern "C" {
    #[link_name = "\u{1}_ZSt21__throw_bad_exceptionv"]
    pub fn std___throw_bad_exception();
}
extern "C" {
    #[link_name = "\u{1}_ZSt17__throw_bad_allocv"]
    pub fn std___throw_bad_alloc();
}
extern "C" {
    #[link_name = "\u{1}_ZSt16__throw_bad_castv"]
    pub fn std___throw_bad_cast();
}
extern "C" {
    #[link_name = "\u{1}_ZSt18__throw_bad_typeidv"]
    pub fn std___throw_bad_typeid();
}
extern "C" {
    #[link_name = "\u{1}_ZSt19__throw_logic_errorPKc"]
    pub fn std___throw_logic_error(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt20__throw_domain_errorPKc"]
    pub fn std___throw_domain_error(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt24__throw_invalid_argumentPKc"]
    pub fn std___throw_invalid_argument(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt20__throw_length_errorPKc"]
    pub fn std___throw_length_error(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt20__throw_out_of_rangePKc"]
    pub fn std___throw_out_of_range(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt24__throw_out_of_range_fmtPKcz"]
    pub fn std___throw_out_of_range_fmt(arg1: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    #[link_name = "\u{1}_ZSt21__throw_runtime_errorPKc"]
    pub fn std___throw_runtime_error(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt19__throw_range_errorPKc"]
    pub fn std___throw_range_error(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt22__throw_overflow_errorPKc"]
    pub fn std___throw_overflow_error(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt23__throw_underflow_errorPKc"]
    pub fn std___throw_underflow_error(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt19__throw_ios_failurePKc"]
    pub fn std___throw_ios_failure(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt19__throw_ios_failurePKci"]
    pub fn std___throw_ios_failure1(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZSt20__throw_system_errori"]
    pub fn std___throw_system_error(arg1: ::std::os::raw::c_int);
}
extern "C" {
    #[link_name = "\u{1}_ZSt20__throw_future_errori"]
    pub fn std___throw_future_error(arg1: ::std::os::raw::c_int);
}
extern "C" {
    #[link_name = "\u{1}_ZSt25__throw_bad_function_callv"]
    pub fn std___throw_bad_function_call();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___true_type {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___true_type() {
    assert_eq!(
        std::mem::size_of::<std___true_type>(),
        1usize,
        concat!("Size of: ", stringify!(std___true_type))
    );
    assert_eq!(
        std::mem::align_of::<std___true_type>(),
        1usize,
        concat!("Alignment of ", stringify!(std___true_type))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___false_type {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___false_type() {
    assert_eq!(
        std::mem::size_of::<std___false_type>(),
        1usize,
        concat!("Size of: ", stringify!(std___false_type))
    );
    assert_eq!(
        std::mem::align_of::<std___false_type>(),
        1usize,
        concat!("Alignment of ", stringify!(std___false_type))
    );
}
pub type std___truth_type___type = std___false_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___traitor {
    pub _address: u8,
}
pub const std___traitor___value: i32 = 0;
pub type std___traitor__bindgen_ty_1 = i32;
pub type std___traitor___type = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___are_same {
    pub _address: u8,
}
pub const std___are_same___value: i32 = 0;
pub type std___are_same__bindgen_ty_1 = i32;
pub type std___are_same___type = std___false_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_void {
    pub _address: u8,
}
pub const std___is_void___value: i32 = 0;
pub type std___is_void__bindgen_ty_1 = i32;
pub type std___is_void___type = std___false_type;
#[test]
fn __bindgen_test_layout_std___is_void_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_void>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_void)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_void>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_void)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_integer {
    pub _address: u8,
}
pub const std___is_integer___value: i32 = 0;
pub type std___is_integer__bindgen_ty_1 = i32;
pub type std___is_integer___type = std___false_type;
#[test]
fn __bindgen_test_layout_std___is_integer_open0_bool__close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integer_open0_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integer_open0_signed_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integer_open0_unsigned_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integer_open0_wchar_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integer_open0_char16_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integer_open0_char32_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integer_open0_short_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integer_open0_unsigned_short_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integer_open0_int_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integer_open0_unsigned_int_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integer_open0_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integer_open0_unsigned_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integer_open0_long_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integer_open0_unsigned_long_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integer_open0___int128_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_integer_open0_unsigned___int128_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_integer>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_integer)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_integer>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_integer)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_floating {
    pub _address: u8,
}
pub const std___is_floating___value: i32 = 0;
pub type std___is_floating__bindgen_ty_1 = i32;
pub type std___is_floating___type = std___false_type;
#[test]
fn __bindgen_test_layout_std___is_floating_open0_float_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_floating>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_floating)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_floating>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_floating)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_floating_open0_double_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_floating>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_floating)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_floating>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_floating)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_floating_open0_long_double_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_floating>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_floating)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_floating>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_floating)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_pointer {
    pub _address: u8,
}
pub const std___is_pointer___value: i32 = 0;
pub type std___is_pointer__bindgen_ty_1 = i32;
pub type std___is_pointer___type = std___false_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_arithmetic {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_scalar {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_char {
    pub _address: u8,
}
pub const std___is_char___value: i32 = 0;
pub type std___is_char__bindgen_ty_1 = i32;
pub type std___is_char___type = std___false_type;
#[test]
fn __bindgen_test_layout_std___is_char_open0_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_char>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_char)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_char>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_char)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_char_open0_wchar_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_char>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_char)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_char>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_char)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_byte {
    pub _address: u8,
}
pub const std___is_byte___value: i32 = 0;
pub type std___is_byte__bindgen_ty_1 = i32;
pub type std___is_byte___type = std___false_type;
#[test]
fn __bindgen_test_layout_std___is_byte_open0_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_byte>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_byte)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_byte>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_byte)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_byte_open0_signed_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_byte>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_byte)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_byte>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_byte)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_byte_open0_unsigned_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___is_byte>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_byte)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_byte>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_byte)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_move_iterator {
    pub _address: u8,
}
pub const std___is_move_iterator___value: i32 = 0;
pub type std___is_move_iterator__bindgen_ty_1 = i32;
pub type std___is_move_iterator___type = std___false_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_input_iterator_tag {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std_input_iterator_tag() {
    assert_eq!(
        std::mem::size_of::<std_input_iterator_tag>(),
        1usize,
        concat!("Size of: ", stringify!(std_input_iterator_tag))
    );
    assert_eq!(
        std::mem::align_of::<std_input_iterator_tag>(),
        1usize,
        concat!("Alignment of ", stringify!(std_input_iterator_tag))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_output_iterator_tag {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std_output_iterator_tag() {
    assert_eq!(
        std::mem::size_of::<std_output_iterator_tag>(),
        1usize,
        concat!("Size of: ", stringify!(std_output_iterator_tag))
    );
    assert_eq!(
        std::mem::align_of::<std_output_iterator_tag>(),
        1usize,
        concat!("Alignment of ", stringify!(std_output_iterator_tag))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_forward_iterator_tag {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std_forward_iterator_tag() {
    assert_eq!(
        std::mem::size_of::<std_forward_iterator_tag>(),
        1usize,
        concat!("Size of: ", stringify!(std_forward_iterator_tag))
    );
    assert_eq!(
        std::mem::align_of::<std_forward_iterator_tag>(),
        1usize,
        concat!("Alignment of ", stringify!(std_forward_iterator_tag))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_bidirectional_iterator_tag {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std_bidirectional_iterator_tag() {
    assert_eq!(
        std::mem::size_of::<std_bidirectional_iterator_tag>(),
        1usize,
        concat!("Size of: ", stringify!(std_bidirectional_iterator_tag))
    );
    assert_eq!(
        std::mem::align_of::<std_bidirectional_iterator_tag>(),
        1usize,
        concat!("Alignment of ", stringify!(std_bidirectional_iterator_tag))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_random_access_iterator_tag {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std_random_access_iterator_tag() {
    assert_eq!(
        std::mem::size_of::<std_random_access_iterator_tag>(),
        1usize,
        concat!("Size of: ", stringify!(std_random_access_iterator_tag))
    );
    assert_eq!(
        std::mem::align_of::<std_random_access_iterator_tag>(),
        1usize,
        concat!("Alignment of ", stringify!(std_random_access_iterator_tag))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_iterator {
    pub _address: u8,
}
pub type std_iterator_iterator_category<_Category> = _Category;
pub type std_iterator_value_type<_Tp> = _Tp;
pub type std_iterator_difference_type<_Distance> = _Distance;
pub type std_iterator_pointer<_Pointer> = _Pointer;
pub type std_iterator_reference<_Reference> = _Reference;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___iterator_traits {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_iterator_traits {
    pub _address: u8,
}
pub type std__RequireInputIter = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__List_iterator {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__List_const_iterator {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___undefined {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___get_first_arg {
    pub _address: u8,
}
pub type std___get_first_arg_type = std___undefined;
pub type std___get_first_arg_t = std___get_first_arg;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___replace_first_arg {
    pub _address: u8,
}
pub type std___replace_first_arg_t = std___replace_first_arg;
pub type std___make_not_void = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_pointer_traits {
    pub _address: u8,
}
pub type std_pointer_traits___element_type = [u8; 0usize];
pub type std_pointer_traits___difference_type = [u8; 0usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_pointer_traits___rebind {
    pub _address: u8,
}
pub type std_pointer_traits_pointer<_Ptr> = _Ptr;
pub type std_pointer_traits_element_type = std___detected_or_t;
pub type std_pointer_traits_difference_type = std___detected_or_t;
pub type std_pointer_traits_rebind = std_pointer_traits___rebind;
pub type std___ptr_rebind = std_pointer_traits;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_reverse_iterator<_Iterator> {
    pub current: _Iterator,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Iterator>>,
}
pub type std_reverse_iterator___traits_type = std_iterator_traits;
pub type std_reverse_iterator_iterator_type<_Iterator> = _Iterator;
pub type std_reverse_iterator_difference_type = std_reverse_iterator___traits_type;
pub type std_reverse_iterator_pointer = std_reverse_iterator___traits_type;
pub type std_reverse_iterator_reference = std_reverse_iterator___traits_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_back_insert_iterator<_Container> {
    pub container: *mut _Container,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Container>>,
}
pub type std_back_insert_iterator_container_type<_Container> = _Container;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_front_insert_iterator<_Container> {
    pub container: *mut _Container,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Container>>,
}
pub type std_front_insert_iterator_container_type<_Container> = _Container;
#[repr(C)]
pub struct std_insert_iterator<_Container> {
    pub container: *mut _Container,
    pub iter: [u8; 0usize],
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Container>>,
}
pub type std_insert_iterator_container_type<_Container> = _Container;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_move_iterator<_Iterator> {
    pub _M_current: _Iterator,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Iterator>>,
}
pub type std_move_iterator___traits_type = std_iterator_traits;
pub type std_move_iterator___base_ref = std_move_iterator___traits_type;
pub type std_move_iterator_iterator_type<_Iterator> = _Iterator;
pub type std_move_iterator_iterator_category = std_move_iterator___traits_type;
pub type std_move_iterator_value_type = std_move_iterator___traits_type;
pub type std_move_iterator_difference_type = std_move_iterator___traits_type;
pub type std_move_iterator_pointer<_Iterator> = _Iterator;
pub type std_move_iterator_reference = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_istreambuf_iterator {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_ostreambuf_iterator {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___lc_rai {
    pub _address: u8,
}
#[test]
fn __bindgen_test_layout_std___lc_rai_open0_std_random_access_iterator_tag_std_random_access_iterator_tag_close0_instantiation(
) {
    assert_eq!(
        std::mem::size_of::<std___lc_rai>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___lc_rai)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___lc_rai>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___lc_rai)
        )
    );
}
pub type std_streamoff = ::std::os::raw::c_long;
pub type std_streamsize = isize;
#[repr(C)]
#[derive(Debug)]
pub struct std_fpos<_StateT> {
    pub _M_off: std_streamoff,
    pub _M_state: _StateT,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_StateT>>,
}
pub type std_streampos = std_fpos<mbstate_t>;
pub type std_wstreampos = std_fpos<mbstate_t>;
pub type std_u16streampos = std_fpos<mbstate_t>;
pub type std_u32streampos = std_fpos<mbstate_t>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_char_traits {
    pub _address: u8,
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char16_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char32_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
pub type std___allocator_base = __gnu_cxx_new_allocator;
#[test]
fn __bindgen_test_layout_std_allocator_open0_void_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct std_allocator {
    pub _address: u8,
}
pub type std_allocator_size_type = std_size_t;
pub type std_allocator_difference_type = isize;
pub type std_allocator_pointer<_Tp> = *mut _Tp;
pub type std_allocator_const_pointer<_Tp> = *const _Tp;
pub type std_allocator_reference<_Tp> = *mut _Tp;
pub type std_allocator_const_reference<_Tp> = *const _Tp;
pub type std_allocator_value_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_rebind {
    pub _address: u8,
}
pub type std_allocator_rebind_other = std_allocator;
pub type std_allocator_propagate_on_container_move_assignment = std_true_type;
pub type std_allocator_is_always_equal = std_true_type;
#[test]
fn __bindgen_test_layout_std_allocator_open0_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_wchar_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
pub type std___c_locale = __locale_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_ios_base {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_ios {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_streambuf {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_istream {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_ostream {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_iostream {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_filebuf {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_ifstream {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_ofstream {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_fstream {
    pub _address: u8,
}
pub type std_ios = std_basic_ios;
pub type std_streambuf = std_basic_streambuf;
pub type std_istream = std_basic_istream;
pub type std_ostream = std_basic_ostream;
pub type std_iostream = std_basic_iostream;
pub type std_stringbuf = std_basic_stringbuf;
pub type std_istringstream = std_basic_istringstream;
pub type std_ostringstream = std_basic_ostringstream;
pub type std_stringstream = std_basic_stringstream;
pub type std_filebuf = std_basic_filebuf;
pub type std_ifstream = std_basic_ifstream;
pub type std_ofstream = std_basic_ofstream;
pub type std_fstream = std_basic_fstream;
pub type std_wios = std_basic_ios;
pub type std_wstreambuf = std_basic_streambuf;
pub type std_wistream = std_basic_istream;
pub type std_wostream = std_basic_ostream;
pub type std_wiostream = std_basic_iostream;
pub type std_wstringbuf = std_basic_stringbuf;
pub type std_wistringstream = std_basic_istringstream;
pub type std_wostringstream = std_basic_ostringstream;
pub type std_wstringstream = std_basic_stringstream;
pub type std_wfilebuf = std_basic_filebuf;
pub type std_wifstream = std_basic_ifstream;
pub type std_wofstream = std_basic_ofstream;
pub type std_wfstream = std_basic_fstream;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_locale {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_ctype_base {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_ctype {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_ctype_byname {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_codecvt_base {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_codecvt {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_codecvt_byname {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_num_get {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_num_put {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_time_base {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_time_put {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_time_put_byname {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_money_base {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_messages_base {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_valarray {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___allocator_traits_base {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___allocator_traits_base___rebind {
    pub _address: u8,
}
pub type std___allocator_traits_base___pointer = [u8; 0usize];
pub type std___allocator_traits_base___c_pointer = [u8; 0usize];
pub type std___allocator_traits_base___v_pointer = [u8; 0usize];
pub type std___allocator_traits_base___cv_pointer = [u8; 0usize];
pub type std___allocator_traits_base___pocca = [u8; 0usize];
pub type std___allocator_traits_base___pocma = [u8; 0usize];
pub type std___allocator_traits_base___pocs = [u8; 0usize];
pub type std___allocator_traits_base___equal = [u8; 0usize];
#[test]
fn bindgen_test_layout_std___allocator_traits_base() {
    assert_eq!(
        std::mem::size_of::<std___allocator_traits_base>(),
        1usize,
        concat!("Size of: ", stringify!(std___allocator_traits_base))
    );
    assert_eq!(
        std::mem::align_of::<std___allocator_traits_base>(),
        1usize,
        concat!("Alignment of ", stringify!(std___allocator_traits_base))
    );
}
pub type std___alloc_rebind = std___allocator_traits_base;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_traits {
    pub _address: u8,
}
pub type std_allocator_traits_allocator_type<_Alloc> = _Alloc;
pub type std_allocator_traits_value_type = [u8; 0usize];
pub type std_allocator_traits_pointer = std___detected_or_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_traits__Ptr {
    pub _address: u8,
}
pub type std_allocator_traits__Ptr_type = [u8; 0usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_traits__Diff {
    pub _address: u8,
}
pub type std_allocator_traits__Diff_type = std_pointer_traits;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_traits__Size {
    pub _address: u8,
}
pub type std_allocator_traits_const_pointer = [u8; 0usize];
pub type std_allocator_traits_void_pointer = std_allocator_traits__Ptr;
pub type std_allocator_traits_const_void_pointer = std_allocator_traits__Ptr;
pub type std_allocator_traits_difference_type = [u8; 0usize];
pub type std_allocator_traits_size_type = [u8; 0usize];
pub type std_allocator_traits_propagate_on_container_copy_assignment = std___detected_or_t;
pub type std_allocator_traits_propagate_on_container_move_assignment = std___detected_or_t;
pub type std_allocator_traits_propagate_on_container_swap = std___detected_or_t;
pub type std_allocator_traits_is_always_equal = std___detected_or_t;
pub type std_allocator_traits_rebind_alloc = std___alloc_rebind;
pub type std_allocator_traits_rebind_traits = std_allocator_traits;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_traits___construct_helper {
    pub _address: u8,
}
pub type std_allocator_traits___construct_helper_type<_Alloc> = _Alloc;
pub type std_allocator_traits___has_construct = std_allocator_traits___construct_helper;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_alloc_insertable_impl {
    pub _base: std_false_type,
}
#[repr(C)]
pub struct std___is_copy_insertable {
    pub _address: u8,
}
#[repr(C)]
pub struct std___is_move_insertable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_allocator {
    pub _base: std_false_type,
}
pub type std__RequireAllocator = u8;
pub type std__RequireNotAllocator = u8;
extern "C" {
    #[link_name = "\u{1}_ZSt7getlineIcSt11char_traitsIcESaIcEERSt13basic_istreamIT_T0_ES7_RNSt7__cxx1112basic_stringIS4_S5_T1_EES4_"]
    pub fn std_getline(
        __in: *mut std_basic_istream,
        __str: *mut std_basic_string<::std::os::raw::c_char>,
        __delim: ::std::os::raw::c_char,
    ) -> *mut std_basic_istream;
}
extern "C" {
    #[link_name = "\u{1}_ZSt7getlineIwSt11char_traitsIwESaIwEERSt13basic_istreamIT_T0_ES7_RNSt7__cxx1112basic_stringIS4_S5_T1_EES4_"]
    pub fn std_getline1(
        __in: *mut std_basic_istream,
        __str: *mut std_basic_string<u32>,
        __delim: u32,
    ) -> *mut std_basic_istream;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___hash_base {
    pub _address: u8,
}
pub type std___hash_base_result_type<_Result> = _Result;
pub type std___hash_base_argument_type<_Arg> = _Arg;
#[repr(C)]
#[derive(Debug)]
pub struct std___poison_hash {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug)]
pub struct std_hash {
    pub _address: u8,
}
#[test]
fn __bindgen_test_layout_std_hash_open0_bool__close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_signed_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_unsigned_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_wchar_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_char16_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_char32_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_short_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_int_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_long_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_unsigned_short_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_unsigned_int_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_unsigned_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_unsigned_long_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0___int128_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_unsigned___int128_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Hash_impl {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std__Hash_impl() {
    assert_eq!(
        std::mem::size_of::<std__Hash_impl>(),
        1usize,
        concat!("Size of: ", stringify!(std__Hash_impl))
    );
    assert_eq!(
        std::mem::align_of::<std__Hash_impl>(),
        1usize,
        concat!("Alignment of ", stringify!(std__Hash_impl))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Fnv_hash_impl {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std__Fnv_hash_impl() {
    assert_eq!(
        std::mem::size_of::<std__Fnv_hash_impl>(),
        1usize,
        concat!("Size of: ", stringify!(std__Fnv_hash_impl))
    );
    assert_eq!(
        std::mem::align_of::<std__Fnv_hash_impl>(),
        1usize,
        concat!("Alignment of ", stringify!(std__Fnv_hash_impl))
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_float_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_double_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_long_double_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_fast_hash {
    pub _base: std_true_type,
}
#[test]
fn __bindgen_test_layout_std___is_fast_hash_open0_std_hash_open1_long_double_close1_close0_instantiation(
) {
    assert_eq!(
        std::mem::size_of::<std___is_fast_hash>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_fast_hash)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_fast_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_fast_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_std_basic_string_open1_char_std_char_traits_open2_char_close2_std_allocator_open2_char_close2_close1_close0_instantiation(
) {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_fast_hash_open0_std_hash_open1_std_basic_string_open2_char_std_char_traits_open3_char_close3_std_allocator_open3_char_close3_close2_close1_close0_instantiation(
) {
    assert_eq!(
        std::mem::size_of::<std___is_fast_hash>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_fast_hash)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_fast_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_fast_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_std_basic_string_open1_wchar_t_std_char_traits_open2_wchar_t_close2_std_allocator_open2_wchar_t_close2_close1_close0_instantiation(
) {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_fast_hash_open0_std_hash_open1_std_basic_string_open2_wchar_t_std_char_traits_open3_wchar_t_close3_std_allocator_open3_wchar_t_close3_close2_close1_close0_instantiation(
) {
    assert_eq!(
        std::mem::size_of::<std___is_fast_hash>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_fast_hash)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_fast_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_fast_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_std_basic_string_open1_char16_t_std_char_traits_open2_char16_t_close2_std_allocator_open2_char16_t_close2_close1_close0_instantiation(
) {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_fast_hash_open0_std_hash_open1_std_basic_string_open2_char16_t_std_char_traits_open3_char16_t_close3_std_allocator_open3_char16_t_close3_close2_close1_close0_instantiation(
) {
    assert_eq!(
        std::mem::size_of::<std___is_fast_hash>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_fast_hash)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_fast_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_fast_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_std_basic_string_open1_char32_t_std_char_traits_open2_char32_t_close2_std_allocator_open2_char32_t_close2_close1_close0_instantiation(
) {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std___is_fast_hash_open0_std_hash_open1_std_basic_string_open2_char32_t_std_char_traits_open3_char32_t_close3_std_allocator_open3_char32_t_close3_close2_close1_close0_instantiation(
) {
    assert_eq!(
        std::mem::size_of::<std___is_fast_hash>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___is_fast_hash)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___is_fast_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___is_fast_hash)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}npos"]
    pub static std_npos: std_basic_string_size_type;
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_char_std_char_traits_open1_char_close1_std_allocator_open1_char_close1_close0_instantiation(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<::std::os::raw::c_char>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<::std::os::raw::c_char>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<::std::os::raw::c_char>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<::std::os::raw::c_char>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_wchar_t_std_char_traits_open1_wchar_t_close1_std_allocator_open1_wchar_t_close1_close0_instantiation(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<u32>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<u32>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<u32>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<u32>)
        )
    );
}
#[repr(C)]
pub struct std___cow_string {
    pub __bindgen_anon_1: std___cow_string__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union std___cow_string__bindgen_ty_1 {
    pub _M_p: *const ::std::os::raw::c_char,
    pub _M_bytes: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout_std___cow_string__bindgen_ty_1() {
    assert_eq!(
        std::mem::size_of::<std___cow_string__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(std___cow_string__bindgen_ty_1))
    );
    assert_eq!(
        std::mem::align_of::<std___cow_string__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(std___cow_string__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<std___cow_string__bindgen_ty_1>()))._M_p as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(std___cow_string__bindgen_ty_1),
            "::",
            stringify!(_M_p)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<std___cow_string__bindgen_ty_1>()))._M_bytes as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(std___cow_string__bindgen_ty_1),
            "::",
            stringify!(_M_bytes)
        )
    );
}
#[test]
fn bindgen_test_layout_std___cow_string() {
    assert_eq!(
        std::mem::size_of::<std___cow_string>(),
        8usize,
        concat!("Size of: ", stringify!(std___cow_string))
    );
    assert_eq!(
        std::mem::align_of::<std___cow_string>(),
        8usize,
        concat!("Alignment of ", stringify!(std___cow_string))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12__cow_stringC1Ev"]
    pub fn std___cow_string___cow_string(this: *mut std___cow_string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12__cow_stringC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn std___cow_string___cow_string1(this: *mut std___cow_string, arg1: *const std_string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12__cow_stringC1EPKcm"]
    pub fn std___cow_string___cow_string2(
        this: *mut std___cow_string,
        arg1: *const ::std::os::raw::c_char,
        arg2: std_size_t,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12__cow_stringC1ERKS_"]
    pub fn std___cow_string___cow_string3(
        this: *mut std___cow_string,
        arg1: *const std___cow_string,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12__cow_stringC1EOS_"]
    pub fn std___cow_string___cow_string4(this: *mut std___cow_string, arg1: *mut std___cow_string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12__cow_stringD1Ev"]
    pub fn std___cow_string___cow_string_destructor(this: *mut std___cow_string);
}
impl std___cow_string {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std___cow_string___cow_string(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(arg1: *const std_string) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std___cow_string___cow_string1(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(arg1: *const ::std::os::raw::c_char, arg2: std_size_t) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std___cow_string___cow_string2(__bindgen_tmp.as_mut_ptr(), arg1, arg2);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new3(arg1: *const std___cow_string) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std___cow_string___cow_string3(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new4(arg1: *mut std___cow_string) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std___cow_string___cow_string4(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        std___cow_string___cow_string_destructor(self)
    }
}
pub type std___sso_string = std_basic_string<::std::os::raw::c_char>;
#[repr(C)]
pub struct std_logic_error {
    pub _base: std_exception,
    pub _M_msg: std___cow_string,
}
#[test]
fn bindgen_test_layout_std_logic_error() {
    assert_eq!(
        std::mem::size_of::<std_logic_error>(),
        16usize,
        concat!("Size of: ", stringify!(std_logic_error))
    );
    assert_eq!(
        std::mem::align_of::<std_logic_error>(),
        8usize,
        concat!("Alignment of ", stringify!(std_logic_error))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<std_logic_error>()))._M_msg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(std_logic_error),
            "::",
            stringify!(_M_msg)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt11logic_errorC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn std_logic_error_logic_error(this: *mut std_logic_error, __arg: *const std_string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt11logic_errorC1EPKc"]
    pub fn std_logic_error_logic_error1(
        this: *mut std_logic_error,
        arg1: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt11logic_errorC1EOS_"]
    pub fn std_logic_error_logic_error2(this: *mut std_logic_error, arg1: *mut std_logic_error);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt11logic_errorC1ERKS_"]
    pub fn std_logic_error_logic_error3(this: *mut std_logic_error, arg1: *const std_logic_error);
}
impl std_logic_error {
    #[inline]
    pub unsafe fn new(__arg: *const std_string) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_logic_error_logic_error(__bindgen_tmp.as_mut_ptr(), __arg);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_logic_error_logic_error1(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(arg1: *mut std_logic_error) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_logic_error_logic_error2(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new3(arg1: *const std_logic_error) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_logic_error_logic_error3(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt11logic_errorD1Ev"]
    pub fn std_logic_error_logic_error_destructor(this: *mut std_logic_error);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt11logic_error4whatEv"]
    pub fn std_logic_error_what(this: *mut ::std::os::raw::c_void)
        -> *const ::std::os::raw::c_char;
}
#[repr(C)]
pub struct std_domain_error {
    pub _base: std_logic_error,
}
#[test]
fn bindgen_test_layout_std_domain_error() {
    assert_eq!(
        std::mem::size_of::<std_domain_error>(),
        16usize,
        concat!("Size of: ", stringify!(std_domain_error))
    );
    assert_eq!(
        std::mem::align_of::<std_domain_error>(),
        8usize,
        concat!("Alignment of ", stringify!(std_domain_error))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12domain_errorC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn std_domain_error_domain_error(this: *mut std_domain_error, __arg: *const std_string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12domain_errorC1EPKc"]
    pub fn std_domain_error_domain_error1(
        this: *mut std_domain_error,
        arg1: *const ::std::os::raw::c_char,
    );
}
impl std_domain_error {
    #[inline]
    pub unsafe fn new(__arg: *const std_string) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_domain_error_domain_error(__bindgen_tmp.as_mut_ptr(), __arg);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_domain_error_domain_error1(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12domain_errorD1Ev"]
    pub fn std_domain_error_domain_error_destructor(this: *mut std_domain_error);
}
#[repr(C)]
pub struct std_invalid_argument {
    pub _base: std_logic_error,
}
#[test]
fn bindgen_test_layout_std_invalid_argument() {
    assert_eq!(
        std::mem::size_of::<std_invalid_argument>(),
        16usize,
        concat!("Size of: ", stringify!(std_invalid_argument))
    );
    assert_eq!(
        std::mem::align_of::<std_invalid_argument>(),
        8usize,
        concat!("Alignment of ", stringify!(std_invalid_argument))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt16invalid_argumentC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn std_invalid_argument_invalid_argument(
        this: *mut std_invalid_argument,
        __arg: *const std_string,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt16invalid_argumentC1EPKc"]
    pub fn std_invalid_argument_invalid_argument1(
        this: *mut std_invalid_argument,
        arg1: *const ::std::os::raw::c_char,
    );
}
impl std_invalid_argument {
    #[inline]
    pub unsafe fn new(__arg: *const std_string) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_invalid_argument_invalid_argument(__bindgen_tmp.as_mut_ptr(), __arg);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_invalid_argument_invalid_argument1(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt16invalid_argumentD1Ev"]
    pub fn std_invalid_argument_invalid_argument_destructor(this: *mut std_invalid_argument);
}
#[repr(C)]
pub struct std_length_error {
    pub _base: std_logic_error,
}
#[test]
fn bindgen_test_layout_std_length_error() {
    assert_eq!(
        std::mem::size_of::<std_length_error>(),
        16usize,
        concat!("Size of: ", stringify!(std_length_error))
    );
    assert_eq!(
        std::mem::align_of::<std_length_error>(),
        8usize,
        concat!("Alignment of ", stringify!(std_length_error))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12length_errorC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn std_length_error_length_error(this: *mut std_length_error, __arg: *const std_string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12length_errorC1EPKc"]
    pub fn std_length_error_length_error1(
        this: *mut std_length_error,
        arg1: *const ::std::os::raw::c_char,
    );
}
impl std_length_error {
    #[inline]
    pub unsafe fn new(__arg: *const std_string) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_length_error_length_error(__bindgen_tmp.as_mut_ptr(), __arg);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_length_error_length_error1(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12length_errorD1Ev"]
    pub fn std_length_error_length_error_destructor(this: *mut std_length_error);
}
#[repr(C)]
pub struct std_out_of_range {
    pub _base: std_logic_error,
}
#[test]
fn bindgen_test_layout_std_out_of_range() {
    assert_eq!(
        std::mem::size_of::<std_out_of_range>(),
        16usize,
        concat!("Size of: ", stringify!(std_out_of_range))
    );
    assert_eq!(
        std::mem::align_of::<std_out_of_range>(),
        8usize,
        concat!("Alignment of ", stringify!(std_out_of_range))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12out_of_rangeC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn std_out_of_range_out_of_range(this: *mut std_out_of_range, __arg: *const std_string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12out_of_rangeC1EPKc"]
    pub fn std_out_of_range_out_of_range1(
        this: *mut std_out_of_range,
        arg1: *const ::std::os::raw::c_char,
    );
}
impl std_out_of_range {
    #[inline]
    pub unsafe fn new(__arg: *const std_string) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_out_of_range_out_of_range(__bindgen_tmp.as_mut_ptr(), __arg);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_out_of_range_out_of_range1(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12out_of_rangeD1Ev"]
    pub fn std_out_of_range_out_of_range_destructor(this: *mut std_out_of_range);
}
#[repr(C)]
pub struct std_runtime_error {
    pub _base: std_exception,
    pub _M_msg: std___cow_string,
}
#[test]
fn bindgen_test_layout_std_runtime_error() {
    assert_eq!(
        std::mem::size_of::<std_runtime_error>(),
        16usize,
        concat!("Size of: ", stringify!(std_runtime_error))
    );
    assert_eq!(
        std::mem::align_of::<std_runtime_error>(),
        8usize,
        concat!("Alignment of ", stringify!(std_runtime_error))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<std_runtime_error>()))._M_msg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(std_runtime_error),
            "::",
            stringify!(_M_msg)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt13runtime_errorC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn std_runtime_error_runtime_error(this: *mut std_runtime_error, __arg: *const std_string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt13runtime_errorC1EPKc"]
    pub fn std_runtime_error_runtime_error1(
        this: *mut std_runtime_error,
        arg1: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt13runtime_errorC1EOS_"]
    pub fn std_runtime_error_runtime_error2(
        this: *mut std_runtime_error,
        arg1: *mut std_runtime_error,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt13runtime_errorC1ERKS_"]
    pub fn std_runtime_error_runtime_error3(
        this: *mut std_runtime_error,
        arg1: *const std_runtime_error,
    );
}
impl std_runtime_error {
    #[inline]
    pub unsafe fn new(__arg: *const std_string) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_runtime_error_runtime_error(__bindgen_tmp.as_mut_ptr(), __arg);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_runtime_error_runtime_error1(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(arg1: *mut std_runtime_error) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_runtime_error_runtime_error2(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new3(arg1: *const std_runtime_error) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_runtime_error_runtime_error3(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt13runtime_errorD1Ev"]
    pub fn std_runtime_error_runtime_error_destructor(this: *mut std_runtime_error);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt13runtime_error4whatEv"]
    pub fn std_runtime_error_what(
        this: *mut ::std::os::raw::c_void,
    ) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
pub struct std_range_error {
    pub _base: std_runtime_error,
}
#[test]
fn bindgen_test_layout_std_range_error() {
    assert_eq!(
        std::mem::size_of::<std_range_error>(),
        16usize,
        concat!("Size of: ", stringify!(std_range_error))
    );
    assert_eq!(
        std::mem::align_of::<std_range_error>(),
        8usize,
        concat!("Alignment of ", stringify!(std_range_error))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt11range_errorC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn std_range_error_range_error(this: *mut std_range_error, __arg: *const std_string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt11range_errorC1EPKc"]
    pub fn std_range_error_range_error1(
        this: *mut std_range_error,
        arg1: *const ::std::os::raw::c_char,
    );
}
impl std_range_error {
    #[inline]
    pub unsafe fn new(__arg: *const std_string) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_range_error_range_error(__bindgen_tmp.as_mut_ptr(), __arg);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_range_error_range_error1(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt11range_errorD1Ev"]
    pub fn std_range_error_range_error_destructor(this: *mut std_range_error);
}
#[repr(C)]
pub struct std_overflow_error {
    pub _base: std_runtime_error,
}
#[test]
fn bindgen_test_layout_std_overflow_error() {
    assert_eq!(
        std::mem::size_of::<std_overflow_error>(),
        16usize,
        concat!("Size of: ", stringify!(std_overflow_error))
    );
    assert_eq!(
        std::mem::align_of::<std_overflow_error>(),
        8usize,
        concat!("Alignment of ", stringify!(std_overflow_error))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt14overflow_errorC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn std_overflow_error_overflow_error(
        this: *mut std_overflow_error,
        __arg: *const std_string,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt14overflow_errorC1EPKc"]
    pub fn std_overflow_error_overflow_error1(
        this: *mut std_overflow_error,
        arg1: *const ::std::os::raw::c_char,
    );
}
impl std_overflow_error {
    #[inline]
    pub unsafe fn new(__arg: *const std_string) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_overflow_error_overflow_error(__bindgen_tmp.as_mut_ptr(), __arg);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_overflow_error_overflow_error1(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt14overflow_errorD1Ev"]
    pub fn std_overflow_error_overflow_error_destructor(this: *mut std_overflow_error);
}
#[repr(C)]
pub struct std_underflow_error {
    pub _base: std_runtime_error,
}
#[test]
fn bindgen_test_layout_std_underflow_error() {
    assert_eq!(
        std::mem::size_of::<std_underflow_error>(),
        16usize,
        concat!("Size of: ", stringify!(std_underflow_error))
    );
    assert_eq!(
        std::mem::align_of::<std_underflow_error>(),
        8usize,
        concat!("Alignment of ", stringify!(std_underflow_error))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt15underflow_errorC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn std_underflow_error_underflow_error(
        this: *mut std_underflow_error,
        __arg: *const std_string,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt15underflow_errorC1EPKc"]
    pub fn std_underflow_error_underflow_error1(
        this: *mut std_underflow_error,
        arg1: *const ::std::os::raw::c_char,
    );
}
impl std_underflow_error {
    #[inline]
    pub unsafe fn new(__arg: *const std_string) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_underflow_error_underflow_error(__bindgen_tmp.as_mut_ptr(), __arg);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std_underflow_error_underflow_error1(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt15underflow_errorD1Ev"]
    pub fn std_underflow_error_underflow_error_destructor(this: *mut std_underflow_error);
}
pub type std___array_traits__Type<_Tp> = *mut _Tp;
pub type std___array_traits__Is_swappable = std___is_swappable;
pub type std___array_traits__Is_nothrow_swappable = std___is_nothrow_swappable;
pub type std_array_value_type<_Tp> = _Tp;
pub type std_array_pointer<_Tp> = *mut std_array_value_type<_Tp>;
pub type std_array_const_pointer<_Tp> = *const std_array_value_type<_Tp>;
pub type std_array_reference<_Tp> = *mut std_array_value_type<_Tp>;
pub type std_array_const_reference<_Tp> = *const std_array_value_type<_Tp>;
pub type std_array_iterator<_Tp> = *mut std_array_value_type<_Tp>;
pub type std_array_const_iterator<_Tp> = *const std_array_value_type<_Tp>;
pub type std_array_size_type = std_size_t;
pub type std_array_difference_type = isize;
pub type std_array_reverse_iterator<_Tp> = std_reverse_iterator<std_array_iterator<_Tp>>;
pub type std_array_const_reverse_iterator<_Tp> =
    std_reverse_iterator<std_array_const_iterator<_Tp>>;
pub type std_array__AT_Type = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___erased_type {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___erased_type() {
    assert_eq!(
        std::mem::size_of::<std___erased_type>(),
        1usize,
        concat!("Size of: ", stringify!(std___erased_type))
    );
    assert_eq!(
        std::mem::align_of::<std___erased_type>(),
        1usize,
        concat!("Alignment of ", stringify!(std___erased_type))
    );
}
pub type std___is_erased_or_convertible = std___or_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_arg_t {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std_allocator_arg_t() {
    assert_eq!(
        std::mem::size_of::<std_allocator_arg_t>(),
        1usize,
        concat!("Size of: ", stringify!(std_allocator_arg_t))
    );
    assert_eq!(
        std::mem::align_of::<std_allocator_arg_t>(),
        1usize,
        concat!("Alignment of ", stringify!(std_allocator_arg_t))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZStL13allocator_arg"]
    pub static std_allocator_arg: std_allocator_arg_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___uses_allocator_helper {
    pub _base: std_false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_uses_allocator {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___uses_alloc_base {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___uses_alloc_base() {
    assert_eq!(
        std::mem::size_of::<std___uses_alloc_base>(),
        1usize,
        concat!("Size of: ", stringify!(std___uses_alloc_base))
    );
    assert_eq!(
        std::mem::align_of::<std___uses_alloc_base>(),
        1usize,
        concat!("Alignment of ", stringify!(std___uses_alloc_base))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___uses_alloc0 {
    pub _M_a: std___uses_alloc0__Sink,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___uses_alloc0__Sink {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std___uses_alloc0__Sink() {
    assert_eq!(
        std::mem::size_of::<std___uses_alloc0__Sink>(),
        1usize,
        concat!("Size of: ", stringify!(std___uses_alloc0__Sink))
    );
    assert_eq!(
        std::mem::align_of::<std___uses_alloc0__Sink>(),
        1usize,
        concat!("Alignment of ", stringify!(std___uses_alloc0__Sink))
    );
}
#[test]
fn bindgen_test_layout_std___uses_alloc0() {
    assert_eq!(
        std::mem::size_of::<std___uses_alloc0>(),
        1usize,
        concat!("Size of: ", stringify!(std___uses_alloc0))
    );
    assert_eq!(
        std::mem::align_of::<std___uses_alloc0>(),
        1usize,
        concat!("Alignment of ", stringify!(std___uses_alloc0))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<std___uses_alloc0>()))._M_a as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(std___uses_alloc0),
            "::",
            stringify!(_M_a)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___uses_alloc1<_Alloc> {
    pub _M_a: *const _Alloc,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Alloc>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___uses_alloc2<_Alloc> {
    pub _M_a: *const _Alloc,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Alloc>>,
}
pub type std___uses_alloc_t = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_uses_allocator_predicate {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_uses_allocator_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_nothrow_uses_allocator_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_empty_non_tuple {
    pub _address: u8,
}
pub type std___empty_not_final = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_tuple {
    pub _address: u8,
}
pub type std_tuple__Inherited = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_tuple__TC2 {
    pub _address: u8,
}
pub type std_tuple__TCC = u8;
pub type std_tuple__TMC = u8;
pub type std_tuple__TMCT = u8;
pub type std_tuple__TNTC = u8;
#[test]
fn __bindgen_test_layout_std_tuple_open0_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_tuple>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_tuple))
    );
    assert_eq!(
        std::mem::align_of::<std_tuple>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_tuple)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___do_make_tuple {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___make_tuple {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___combine_tuples {
    pub _address: u8,
}
#[test]
fn __bindgen_test_layout_std___combine_tuples_open0_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___combine_tuples>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___combine_tuples)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___combine_tuples>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___combine_tuples)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___tuple_cat_result {
    pub _address: u8,
}
pub type std___tuple_cat_result___type = std___combine_tuples;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___make_1st_indices {
    pub _address: u8,
}
#[test]
fn __bindgen_test_layout_std___make_1st_indices_open0_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std___make_1st_indices>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std___make_1st_indices)
        )
    );
    assert_eq!(
        std::mem::align_of::<std___make_1st_indices>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std___make_1st_indices)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___tuple_concater {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Swallow_assign {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std__Swallow_assign() {
    assert_eq!(
        std::mem::size_of::<std__Swallow_assign>(),
        1usize,
        concat!("Size of: ", stringify!(std__Swallow_assign))
    );
    assert_eq!(
        std::mem::align_of::<std__Swallow_assign>(),
        1usize,
        concat!("Alignment of ", stringify!(std__Swallow_assign))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZStL6ignore"]
    pub static std_ignore: std__Swallow_assign;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Maybe_unary_or_binary_function {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Mem_fn_traits {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Mem_fn_traits_base {
    pub _address: u8,
}
pub type std__Mem_fn_traits_base___result_type<_Res> = _Res;
pub type std__Mem_fn_traits_base___maybe_type = std__Maybe_unary_or_binary_function;
pub type std__Mem_fn_traits_base___arity = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Maybe_get_result_type {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Weak_result_type_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Weak_result_type {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Refwrap_base_arg1 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Refwrap_base_arg2 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Reference_wrapper_base {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_reference_wrapper<_Tp> {
    pub _M_data: *mut _Tp,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Tp>>,
}
pub type std_reference_wrapper___not_same = u8;
pub type std_reference_wrapper_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug)]
pub struct std_bad_function_call {
    pub _base: std_exception,
}
#[test]
fn bindgen_test_layout_std_bad_function_call() {
    assert_eq!(
        std::mem::size_of::<std_bad_function_call>(),
        8usize,
        concat!("Size of: ", stringify!(std_bad_function_call))
    );
    assert_eq!(
        std::mem::align_of::<std_bad_function_call>(),
        8usize,
        concat!("Alignment of ", stringify!(std_bad_function_call))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt17bad_function_callD1Ev"]
    pub fn std_bad_function_call_bad_function_call_destructor(this: *mut std_bad_function_call);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt17bad_function_call4whatEv"]
    pub fn std_bad_function_call_what(
        this: *mut ::std::os::raw::c_void,
    ) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_location_invariant {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Undefined_class {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union std__Nocopy_types {
    pub _M_object: *mut ::std::os::raw::c_void,
    pub _M_const_object: *const ::std::os::raw::c_void,
    pub _M_function_pointer: std::option::Option<unsafe extern "C" fn()>,
    pub _M_member_pointer: std::option::Option<unsafe extern "C" fn()>,
}
#[test]
fn bindgen_test_layout_std__Nocopy_types() {
    assert_eq!(
        std::mem::size_of::<std__Nocopy_types>(),
        16usize,
        concat!("Size of: ", stringify!(std__Nocopy_types))
    );
    assert_eq!(
        std::mem::align_of::<std__Nocopy_types>(),
        8usize,
        concat!("Alignment of ", stringify!(std__Nocopy_types))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<std__Nocopy_types>()))._M_object as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(std__Nocopy_types),
            "::",
            stringify!(_M_object)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<std__Nocopy_types>()))._M_const_object as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(std__Nocopy_types),
            "::",
            stringify!(_M_const_object)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<std__Nocopy_types>()))._M_function_pointer as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(std__Nocopy_types),
            "::",
            stringify!(_M_function_pointer)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<std__Nocopy_types>()))._M_member_pointer as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(std__Nocopy_types),
            "::",
            stringify!(_M_member_pointer)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union std__Any_data {
    pub _M_unused: std__Nocopy_types,
    pub _M_pod_data: [::std::os::raw::c_char; 16usize],
}
#[test]
fn bindgen_test_layout_std__Any_data() {
    assert_eq!(
        std::mem::size_of::<std__Any_data>(),
        16usize,
        concat!("Size of: ", stringify!(std__Any_data))
    );
    assert_eq!(
        std::mem::align_of::<std__Any_data>(),
        8usize,
        concat!("Alignment of ", stringify!(std__Any_data))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<std__Any_data>()))._M_unused as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(std__Any_data),
            "::",
            stringify!(_M_unused)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<std__Any_data>()))._M_pod_data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(std__Any_data),
            "::",
            stringify!(_M_pod_data)
        )
    );
}
pub const std__Manager_operation___get_type_info: std__Manager_operation = 0;
pub const std__Manager_operation___get_functor_ptr: std__Manager_operation = 1;
pub const std__Manager_operation___clone_functor: std__Manager_operation = 2;
pub const std__Manager_operation___destroy_functor: std__Manager_operation = 3;
pub type std__Manager_operation = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Simple_type_wrapper<_Tp> {
    pub __value: _Tp,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Tp>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_function {
    pub _address: u8,
}
#[repr(C)]
pub struct std__Function_base {
    pub _M_functor: std__Any_data,
    pub _M_manager: std__Function_base__Manager_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Function_base__Base_manager {
    pub _address: u8,
}
pub type std__Function_base__Base_manager__Local_storage = u8;
pub type std__Function_base__Manager_type = std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut std__Any_data,
        arg2: *const std__Any_data,
        arg3: std__Manager_operation,
    ) -> bool,
>;
pub const std__Function_base__M_max_size: std_size_t = 16;
pub const std__Function_base__M_max_align: std_size_t = 8;
#[test]
fn bindgen_test_layout_std__Function_base() {
    assert_eq!(
        std::mem::size_of::<std__Function_base>(),
        24usize,
        concat!("Size of: ", stringify!(std__Function_base))
    );
    assert_eq!(
        std::mem::align_of::<std__Function_base>(),
        8usize,
        concat!("Alignment of ", stringify!(std__Function_base))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<std__Function_base>()))._M_functor as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(std__Function_base),
            "::",
            stringify!(_M_functor)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<std__Function_base>()))._M_manager as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(std__Function_base),
            "::",
            stringify!(_M_manager)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Function_handler {
    pub _address: u8,
}
pub type std__Mem_fn_base__Traits = std__Mem_fn_traits;
pub type std__Mem_fn_base__Arity = std__Mem_fn_base__Traits;
pub type std__Mem_fn_base__Varargs = std__Mem_fn_base__Traits;
pub type std__Mem_fn_base_result_type = std__Mem_fn_base__Traits;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Mem_fn {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_bind_expression {
    pub _base: std_false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_placeholder {
    pub _base: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders2_1E"]
    pub static std_placeholders__1: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders2_2E"]
    pub static std_placeholders__2: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders2_3E"]
    pub static std_placeholders__3: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders2_4E"]
    pub static std_placeholders__4: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders2_5E"]
    pub static std_placeholders__5: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders2_6E"]
    pub static std_placeholders__6: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders2_7E"]
    pub static std_placeholders__7: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders2_8E"]
    pub static std_placeholders__8: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders2_9E"]
    pub static std_placeholders__9: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_10E"]
    pub static std_placeholders__10: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_11E"]
    pub static std_placeholders__11: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_12E"]
    pub static std_placeholders__12: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_13E"]
    pub static std_placeholders__13: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_14E"]
    pub static std_placeholders__14: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_15E"]
    pub static std_placeholders__15: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_16E"]
    pub static std_placeholders__16: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_17E"]
    pub static std_placeholders__17: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_18E"]
    pub static std_placeholders__18: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_19E"]
    pub static std_placeholders__19: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_20E"]
    pub static std_placeholders__20: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_21E"]
    pub static std_placeholders__21: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_22E"]
    pub static std_placeholders__22: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_23E"]
    pub static std_placeholders__23: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_24E"]
    pub static std_placeholders__24: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_25E"]
    pub static std_placeholders__25: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_26E"]
    pub static std_placeholders__26: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_27E"]
    pub static std_placeholders__27: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_28E"]
    pub static std_placeholders__28: u8;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12placeholders3_29E"]
    pub static std_placeholders__29: u8;
}
pub type std__Safe_tuple_element_t = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Bind {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Bind_result {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Bind_check_arity {
    pub _address: u8,
}
pub type std___is_socketlike = std___or_;
pub type std__Bind_helper___func_type = std_decay;
pub type std__Bind_helper_type = std__Bind;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Bindres_helper {
    pub _address: u8,
}
pub type std__Bindres_helper___functor_type = std_decay;
pub type std__Bindres_helper_type = std__Bind_result;
#[repr(C)]
#[derive(Debug)]
pub struct std__Not_fn<_Fn> {
    pub _M_fn: _Fn,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Fn>>,
}
pub type std__Not_fn___inv_res_t = std___invoke_result;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___is_byte_like {
    pub _base: std_false_type,
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_void_close0_instantiation_1() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_1() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_1() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char16_t_close0_instantiation_1() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char32_t_close0_instantiation_1() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN9__gnu_cxx27__verbose_terminate_handlerEv"]
    pub fn __gnu_cxx___verbose_terminate_handler();
}
pub type __gnu_cxx___conditional_type___type<_Iftrue> = _Iftrue;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___add_unsigned {
    pub _address: u8,
}
pub type __gnu_cxx___add_unsigned___if_type = u8;
pub type __gnu_cxx___add_unsigned___type = __gnu_cxx___add_unsigned___if_type;
#[test]
fn __bindgen_test_layout___gnu_cxx___add_unsigned_open0_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___add_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(__gnu_cxx___add_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___add_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(__gnu_cxx___add_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout___gnu_cxx___add_unsigned_open0_signed_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___add_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(__gnu_cxx___add_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___add_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(__gnu_cxx___add_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout___gnu_cxx___add_unsigned_open0_short_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___add_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(__gnu_cxx___add_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___add_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(__gnu_cxx___add_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout___gnu_cxx___add_unsigned_open0_int_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___add_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(__gnu_cxx___add_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___add_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(__gnu_cxx___add_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout___gnu_cxx___add_unsigned_open0_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___add_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(__gnu_cxx___add_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___add_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(__gnu_cxx___add_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout___gnu_cxx___add_unsigned_open0_long_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___add_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(__gnu_cxx___add_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___add_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(__gnu_cxx___add_unsigned)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___remove_unsigned {
    pub _address: u8,
}
pub type __gnu_cxx___remove_unsigned___if_type = u8;
pub type __gnu_cxx___remove_unsigned___type = __gnu_cxx___remove_unsigned___if_type;
#[test]
fn __bindgen_test_layout___gnu_cxx___remove_unsigned_open0_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___remove_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(__gnu_cxx___remove_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___remove_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(__gnu_cxx___remove_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout___gnu_cxx___remove_unsigned_open0_unsigned_char_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___remove_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(__gnu_cxx___remove_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___remove_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(__gnu_cxx___remove_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout___gnu_cxx___remove_unsigned_open0_unsigned_short_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___remove_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(__gnu_cxx___remove_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___remove_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(__gnu_cxx___remove_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout___gnu_cxx___remove_unsigned_open0_unsigned_int_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___remove_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(__gnu_cxx___remove_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___remove_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(__gnu_cxx___remove_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout___gnu_cxx___remove_unsigned_open0_unsigned_long_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___remove_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(__gnu_cxx___remove_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___remove_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(__gnu_cxx___remove_unsigned)
        )
    );
}
#[test]
fn __bindgen_test_layout___gnu_cxx___remove_unsigned_open0_unsigned_long_long_close0_instantiation()
{
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___remove_unsigned>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(__gnu_cxx___remove_unsigned)
        )
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___remove_unsigned>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(__gnu_cxx___remove_unsigned)
        )
    );
}
pub type __gnu_cxx___promote___type = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___promote_2 {
    pub _address: u8,
}
pub type __gnu_cxx___promote_2___type<_Tp2> = _Tp2;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___promote_3 {
    pub _address: u8,
}
pub type __gnu_cxx___promote_3___type<_Tp2> = _Tp2;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___promote_4 {
    pub _address: u8,
}
pub type __gnu_cxx___promote_4___type<_Tp2> = _Tp2;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___numeric_traits_integer {
    pub _address: u8,
}
// extern "C" {
//     #[link_name = "\u{1}__min"]
//     pub static __gnu_cxx___min: _Value;
// }
// extern "C" {
//     #[link_name = "\u{1}__max"]
//     pub static __gnu_cxx___max: _Value;
// }
extern "C" {
    #[link_name = "\u{1}__is_signed"]
    pub static __gnu_cxx___is_signed: bool;
}
extern "C" {
    #[link_name = "\u{1}__digits"]
    pub static __gnu_cxx___digits: ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___numeric_traits_floating {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}__max_digits10"]
    pub static __gnu_cxx___max_digits10: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__digits10"]
    pub static __gnu_cxx___digits10: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__max_exponent10"]
    pub static __gnu_cxx___max_exponent10: ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___numeric_traits {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___normal_iterator<_Iterator> {
    pub _M_current: _Iterator,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Iterator>>,
}
pub type __gnu_cxx___normal_iterator___traits_type = std_iterator_traits;
pub type __gnu_cxx___normal_iterator_iterator_type<_Iterator> = _Iterator;
pub type __gnu_cxx___normal_iterator_iterator_category = __gnu_cxx___normal_iterator___traits_type;
pub type __gnu_cxx___normal_iterator_value_type = __gnu_cxx___normal_iterator___traits_type;
pub type __gnu_cxx___normal_iterator_difference_type = __gnu_cxx___normal_iterator___traits_type;
pub type __gnu_cxx___normal_iterator_reference = __gnu_cxx___normal_iterator___traits_type;
pub type __gnu_cxx___normal_iterator_pointer = __gnu_cxx___normal_iterator___traits_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___ops__Iter_less_iter {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout___gnu_cxx___ops__Iter_less_iter() {
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___ops__Iter_less_iter>(),
        1usize,
        concat!("Size of: ", stringify!(__gnu_cxx___ops__Iter_less_iter))
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___ops__Iter_less_iter>(),
        1usize,
        concat!("Alignment of ", stringify!(__gnu_cxx___ops__Iter_less_iter))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___ops__Iter_less_val {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout___gnu_cxx___ops__Iter_less_val() {
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___ops__Iter_less_val>(),
        1usize,
        concat!("Size of: ", stringify!(__gnu_cxx___ops__Iter_less_val))
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___ops__Iter_less_val>(),
        1usize,
        concat!("Alignment of ", stringify!(__gnu_cxx___ops__Iter_less_val))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___ops__Val_less_iter {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout___gnu_cxx___ops__Val_less_iter() {
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___ops__Val_less_iter>(),
        1usize,
        concat!("Size of: ", stringify!(__gnu_cxx___ops__Val_less_iter))
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___ops__Val_less_iter>(),
        1usize,
        concat!("Alignment of ", stringify!(__gnu_cxx___ops__Val_less_iter))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___ops__Iter_equal_to_iter {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout___gnu_cxx___ops__Iter_equal_to_iter() {
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___ops__Iter_equal_to_iter>(),
        1usize,
        concat!("Size of: ", stringify!(__gnu_cxx___ops__Iter_equal_to_iter))
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___ops__Iter_equal_to_iter>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(__gnu_cxx___ops__Iter_equal_to_iter)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___ops__Iter_equal_to_val {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout___gnu_cxx___ops__Iter_equal_to_val() {
    assert_eq!(
        std::mem::size_of::<__gnu_cxx___ops__Iter_equal_to_val>(),
        1usize,
        concat!("Size of: ", stringify!(__gnu_cxx___ops__Iter_equal_to_val))
    );
    assert_eq!(
        std::mem::align_of::<__gnu_cxx___ops__Iter_equal_to_val>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(__gnu_cxx___ops__Iter_equal_to_val)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___ops__Iter_comp_iter<_Compare> {
    pub _M_comp: _Compare,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Compare>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___ops__Iter_comp_val<_Compare> {
    pub _M_comp: _Compare,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Compare>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___ops__Val_comp_iter<_Compare> {
    pub _M_comp: _Compare,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Compare>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___ops__Iter_equals_val<_Value> {
    pub _M_value: *mut _Value,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Value>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___ops__Iter_equals_iter<_Iterator1> {
    pub _M_it1: _Iterator1,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Iterator1>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___ops__Iter_pred<_Predicate> {
    pub _M_pred: _Predicate,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Predicate>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___ops__Iter_comp_to_val<_Compare, _Value> {
    pub _M_comp: _Compare,
    pub _M_value: *mut _Value,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Compare>>,
    pub _phantom_1: std::marker::PhantomData<std::cell::UnsafeCell<_Value>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___ops__Iter_comp_to_iter<_Compare, _Iterator1> {
    pub _M_comp: _Compare,
    pub _M_it1: _Iterator1,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Compare>>,
    pub _phantom_1: std::marker::PhantomData<std::cell::UnsafeCell<_Iterator1>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___ops__Iter_negate<_Predicate> {
    pub _M_pred: _Predicate,
    pub _phantom_0: std::marker::PhantomData<std::cell::UnsafeCell<_Predicate>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx__Char_types {
    pub _address: u8,
}
pub type __gnu_cxx__Char_types_int_type = ::std::os::raw::c_ulong;
pub type __gnu_cxx__Char_types_pos_type = std_streampos;
pub type __gnu_cxx__Char_types_off_type = std_streamoff;
pub type __gnu_cxx__Char_types_state_type = mbstate_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx_char_traits {
    pub _address: u8,
}
pub type __gnu_cxx_char_traits_char_type<_CharT> = _CharT;
pub type __gnu_cxx_char_traits_int_type = __gnu_cxx__Char_types;
pub type __gnu_cxx_char_traits_pos_type = __gnu_cxx__Char_types;
pub type __gnu_cxx_char_traits_off_type = __gnu_cxx__Char_types;
pub type __gnu_cxx_char_traits_state_type = __gnu_cxx__Char_types;
#[repr(C)]
#[derive(Debug)]
pub struct __gnu_cxx_new_allocator {
    pub _address: u8,
}
pub type __gnu_cxx_new_allocator_size_type = std_size_t;
pub type __gnu_cxx_new_allocator_difference_type = isize;
pub type __gnu_cxx_new_allocator_pointer<_Tp> = *mut _Tp;
pub type __gnu_cxx_new_allocator_const_pointer<_Tp> = *const _Tp;
pub type __gnu_cxx_new_allocator_reference<_Tp> = *mut _Tp;
pub type __gnu_cxx_new_allocator_const_reference<_Tp> = *const _Tp;
pub type __gnu_cxx_new_allocator_value_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx_new_allocator_rebind {
    pub _address: u8,
}
pub type __gnu_cxx_new_allocator_rebind_other = __gnu_cxx_new_allocator;
pub type __gnu_cxx_new_allocator_propagate_on_container_move_assignment = std_true_type;
extern "C" {
    #[link_name = "\u{1}__uselocale"]
    pub fn __gnu_cxx___uselocale(arg1: locale_t) -> locale_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___alloc_traits {
    pub _address: u8,
}
pub type __gnu_cxx___alloc_traits_allocator_type<_Alloc> = _Alloc;
pub type __gnu_cxx___alloc_traits__Base_type = std_allocator_traits;
pub type __gnu_cxx___alloc_traits_value_type = __gnu_cxx___alloc_traits__Base_type;
pub type __gnu_cxx___alloc_traits_pointer = __gnu_cxx___alloc_traits__Base_type;
pub type __gnu_cxx___alloc_traits_const_pointer = __gnu_cxx___alloc_traits__Base_type;
pub type __gnu_cxx___alloc_traits_size_type = __gnu_cxx___alloc_traits__Base_type;
pub type __gnu_cxx___alloc_traits_difference_type = __gnu_cxx___alloc_traits__Base_type;
pub type __gnu_cxx___alloc_traits_reference = *mut __gnu_cxx___alloc_traits_value_type;
pub type __gnu_cxx___alloc_traits_const_reference = *const __gnu_cxx___alloc_traits_value_type;
pub type __gnu_cxx___alloc_traits___is_custom_pointer = std___and_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___alloc_traits_rebind {
    pub _address: u8,
}
pub type __gnu_cxx___alloc_traits_rebind_other = __gnu_cxx___alloc_traits__Base_type;
pub type size_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(
        std::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        std::mem::align_of::<max_align_t>(),
        16usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<max_align_t>())).__clang_max_align_nonce1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<max_align_t>())).__clang_max_align_nonce2 as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __cxxabiv1___cxa_refcounted_exception {
    _unused: [u8; 0],
}
extern "C" {
    #[link_name = "\u{1}__cxa_allocate_exception"]
    pub fn __cxxabiv1___cxa_allocate_exception(arg1: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[link_name = "\u{1}__cxa_free_exception"]
    pub fn __cxxabiv1___cxa_free_exception(arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}__cxa_init_primary_exception"]
    pub fn __cxxabiv1___cxa_init_primary_exception(
        object: *mut ::std::os::raw::c_void,
        tinfo: *mut std_type_info,
        dest: std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    ) -> *mut __cxxabiv1___cxa_refcounted_exception;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __cxxabiv1___class_type_info {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct __cxxabiv1___forced_unwind__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct __cxxabiv1___forced_unwind {
    pub vtable_: *const __cxxabiv1___forced_unwind__bindgen_vtable,
}
#[test]
fn bindgen_test_layout___cxxabiv1___forced_unwind() {
    assert_eq!(
        std::mem::size_of::<__cxxabiv1___forced_unwind>(),
        8usize,
        concat!("Size of: ", stringify!(__cxxabiv1___forced_unwind))
    );
    assert_eq!(
        std::mem::align_of::<__cxxabiv1___forced_unwind>(),
        8usize,
        concat!("Alignment of ", stringify!(__cxxabiv1___forced_unwind))
    );
}
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = u128;
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
pub type wint_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __mbstate_t {
    pub __count: ::std::os::raw::c_int,
    pub __value: __mbstate_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t__bindgen_ty_1 {
    pub __wch: ::std::os::raw::c_uint,
    pub __wchb: [::std::os::raw::c_char; 4usize],
}
#[test]
fn bindgen_test_layout___mbstate_t__bindgen_ty_1() {
    assert_eq!(
        std::mem::size_of::<__mbstate_t__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(__mbstate_t__bindgen_ty_1))
    );
    assert_eq!(
        std::mem::align_of::<__mbstate_t__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(__mbstate_t__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__mbstate_t__bindgen_ty_1>())).__wch as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t__bindgen_ty_1),
            "::",
            stringify!(__wch)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__mbstate_t__bindgen_ty_1>())).__wchb as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t__bindgen_ty_1),
            "::",
            stringify!(__wchb)
        )
    );
}
#[test]
fn bindgen_test_layout___mbstate_t() {
    assert_eq!(
        std::mem::size_of::<__mbstate_t>(),
        8usize,
        concat!("Size of: ", stringify!(__mbstate_t))
    );
    assert_eq!(
        std::mem::align_of::<__mbstate_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__mbstate_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__mbstate_t>())).__count as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t),
            "::",
            stringify!(__count)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__mbstate_t>())).__value as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t),
            "::",
            stringify!(__value)
        )
    );
}
pub type mbstate_t = __mbstate_t;
pub type __FILE = _IO_FILE;
pub type FILE = _IO_FILE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13usize],
    pub __ctype_b: *const ::std::os::raw::c_ushort,
    pub __ctype_tolower: *const ::std::os::raw::c_int,
    pub __ctype_toupper: *const ::std::os::raw::c_int,
    pub __names: [*const ::std::os::raw::c_char; 13usize],
}
#[test]
fn bindgen_test_layout___locale_struct() {
    assert_eq!(
        std::mem::size_of::<__locale_struct>(),
        232usize,
        concat!("Size of: ", stringify!(__locale_struct))
    );
    assert_eq!(
        std::mem::align_of::<__locale_struct>(),
        8usize,
        concat!("Alignment of ", stringify!(__locale_struct))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__locale_struct>())).__locales as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__locales)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__locale_struct>())).__ctype_b as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__ctype_b)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__locale_struct>())).__ctype_tolower as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__ctype_tolower)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__locale_struct>())).__ctype_toupper as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__ctype_toupper)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__locale_struct>())).__names as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__names)
        )
    );
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
extern "C" {
    pub fn wcscpy(__dest: *mut u32, __src: *const u32) -> *mut u32;
}
extern "C" {
    pub fn wcsncpy(__dest: *mut u32, __src: *const u32, __n: size_t) -> *mut u32;
}
extern "C" {
    pub fn wcscat(__dest: *mut u32, __src: *const u32) -> *mut u32;
}
extern "C" {
    pub fn wcsncat(__dest: *mut u32, __src: *const u32, __n: size_t) -> *mut u32;
}
extern "C" {
    pub fn wcscmp(__s1: *const u32, __s2: *const u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsncmp(__s1: *const u32, __s2: *const u32, __n: size_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcscasecmp(__s1: *const u32, __s2: *const u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsncasecmp(__s1: *const u32, __s2: *const u32, __n: size_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcscasecmp_l(
        __s1: *const u32,
        __s2: *const u32,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsncasecmp_l(
        __s1: *const u32,
        __s2: *const u32,
        __n: size_t,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcscoll(__s1: *const u32, __s2: *const u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsxfrm(__s1: *mut u32, __s2: *const u32, __n: size_t) -> size_t;
}
extern "C" {
    pub fn wcscoll_l(__s1: *const u32, __s2: *const u32, __loc: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsxfrm_l(__s1: *mut u32, __s2: *const u32, __n: size_t, __loc: locale_t) -> size_t;
}
extern "C" {
    pub fn wcsdup(__s: *const u32) -> *mut u32;
}
extern "C" {
    pub fn wcschr(__wcs: *const u32, __wc: u32) -> *mut u32;
}
extern "C" {
    pub fn wcsrchr(__wcs: *const u32, __wc: u32) -> *mut u32;
}
extern "C" {
    pub fn wcschrnul(__s: *const u32, __wc: u32) -> *mut u32;
}
extern "C" {
    pub fn wcscspn(__wcs: *const u32, __reject: *const u32) -> size_t;
}
extern "C" {
    pub fn wcsspn(__wcs: *const u32, __accept: *const u32) -> size_t;
}
extern "C" {
    pub fn wcspbrk(__wcs: *const u32, __accept: *const u32) -> *mut u32;
}
extern "C" {
    pub fn wcsstr(__haystack: *const u32, __needle: *const u32) -> *mut u32;
}
extern "C" {
    pub fn wcstok(__s: *mut u32, __delim: *const u32, __ptr: *mut *mut u32) -> *mut u32;
}
extern "C" {
    pub fn wcslen(__s: *const u32) -> size_t;
}
extern "C" {
    pub fn wcswcs(__haystack: *const u32, __needle: *const u32) -> *mut u32;
}
extern "C" {
    pub fn wcsnlen(__s: *const u32, __maxlen: size_t) -> size_t;
}
extern "C" {
    pub fn wmemchr(__s: *const u32, __c: u32, __n: size_t) -> *mut u32;
}
extern "C" {
    pub fn wmemcmp(__s1: *const u32, __s2: *const u32, __n: size_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wmemcpy(__s1: *mut u32, __s2: *const u32, __n: size_t) -> *mut u32;
}
extern "C" {
    pub fn wmemmove(__s1: *mut u32, __s2: *const u32, __n: size_t) -> *mut u32;
}
extern "C" {
    pub fn wmemset(__s: *mut u32, __c: u32, __n: size_t) -> *mut u32;
}
extern "C" {
    pub fn wmempcpy(__s1: *mut u32, __s2: *const u32, __n: size_t) -> *mut u32;
}
extern "C" {
    pub fn btowc(__c: ::std::os::raw::c_int) -> wint_t;
}
extern "C" {
    pub fn wctob(__c: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbsinit(__ps: *const mbstate_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbrtowc(
        __pwc: *mut u32,
        __s: *const ::std::os::raw::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
}
extern "C" {
    pub fn wcrtomb(__s: *mut ::std::os::raw::c_char, __wc: u32, __ps: *mut mbstate_t) -> size_t;
}
extern "C" {
    pub fn __mbrlen(
        __s: *const ::std::os::raw::c_char,
        __n: size_t,
        __ps: *mut mbstate_t,
    ) -> size_t;
}
extern "C" {
    pub fn mbrlen(__s: *const ::std::os::raw::c_char, __n: size_t, __ps: *mut mbstate_t) -> size_t;
}
extern "C" {
    pub fn mbsrtowcs(
        __dst: *mut u32,
        __src: *mut *const ::std::os::raw::c_char,
        __len: size_t,
        __ps: *mut mbstate_t,
    ) -> size_t;
}
extern "C" {
    pub fn wcsrtombs(
        __dst: *mut ::std::os::raw::c_char,
        __src: *mut *const u32,
        __len: size_t,
        __ps: *mut mbstate_t,
    ) -> size_t;
}
extern "C" {
    pub fn mbsnrtowcs(
        __dst: *mut u32,
        __src: *mut *const ::std::os::raw::c_char,
        __nmc: size_t,
        __len: size_t,
        __ps: *mut mbstate_t,
    ) -> size_t;
}
extern "C" {
    pub fn wcsnrtombs(
        __dst: *mut ::std::os::raw::c_char,
        __src: *mut *const u32,
        __nwc: size_t,
        __len: size_t,
        __ps: *mut mbstate_t,
    ) -> size_t;
}
extern "C" {
    pub fn wcwidth(__c: u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcswidth(__s: *const u32, __n: size_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcstod(__nptr: *const u32, __endptr: *mut *mut u32) -> f64;
}
extern "C" {
    pub fn wcstof(__nptr: *const u32, __endptr: *mut *mut u32) -> f32;
}
extern "C" {
    pub fn wcstold(__nptr: *const u32, __endptr: *mut *mut u32) -> u128;
}
extern "C" {
    pub fn wcstof32(__nptr: *const u32, __endptr: *mut *mut u32) -> _Float32;
}
extern "C" {
    pub fn wcstof64(__nptr: *const u32, __endptr: *mut *mut u32) -> _Float64;
}
extern "C" {
    pub fn wcstof32x(__nptr: *const u32, __endptr: *mut *mut u32) -> _Float32x;
}
extern "C" {
    pub fn wcstof64x(__nptr: *const u32, __endptr: *mut *mut u32) -> _Float64x;
}
extern "C" {
    pub fn wcstol(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn wcstoul(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn wcstoll(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn wcstoull(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn wcstoq(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn wcstouq(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn wcstol_l(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn wcstoul_l(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn wcstoll_l(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn wcstoull_l(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn wcstod_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: locale_t) -> f64;
}
extern "C" {
    pub fn wcstof_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: locale_t) -> f32;
}
extern "C" {
    pub fn wcstold_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: locale_t) -> u128;
}
extern "C" {
    pub fn wcstof32_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: locale_t) -> _Float32;
}
extern "C" {
    pub fn wcstof64_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: locale_t) -> _Float64;
}
extern "C" {
    pub fn wcstof32x_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: locale_t) -> _Float32x;
}
extern "C" {
    pub fn wcstof64x_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: locale_t) -> _Float64x;
}
extern "C" {
    pub fn wcpcpy(__dest: *mut u32, __src: *const u32) -> *mut u32;
}
extern "C" {
    pub fn wcpncpy(__dest: *mut u32, __src: *const u32, __n: size_t) -> *mut u32;
}
extern "C" {
    pub fn open_wmemstream(__bufloc: *mut *mut u32, __sizeloc: *mut size_t) -> *mut __FILE;
}
extern "C" {
    pub fn fwide(__fp: *mut __FILE, __mode: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fwprintf(__stream: *mut __FILE, __format: *const u32, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wprintf(__format: *const u32, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swprintf(__s: *mut u32, __n: size_t, __format: *const u32, ...)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfwprintf(
        __s: *mut __FILE,
        __format: *const u32,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vwprintf(__format: *const u32, __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vswprintf(
        __s: *mut u32,
        __n: size_t,
        __format: *const u32,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fwscanf(__stream: *mut __FILE, __format: *const u32, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wscanf(__format: *const u32, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swscanf(__s: *const u32, __format: *const u32, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_fwscanf"]
    pub fn fwscanf1(__stream: *mut __FILE, __format: *const u32, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_wscanf"]
    pub fn wscanf1(__format: *const u32, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_swscanf"]
    pub fn swscanf1(__s: *const u32, __format: *const u32, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfwscanf(
        __s: *mut __FILE,
        __format: *const u32,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vwscanf(__format: *const u32, __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vswscanf(
        __s: *const u32,
        __format: *const u32,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vfwscanf"]
    pub fn vfwscanf1(
        __s: *mut __FILE,
        __format: *const u32,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vwscanf"]
    pub fn vwscanf1(__format: *const u32, __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vswscanf"]
    pub fn vswscanf1(
        __s: *const u32,
        __format: *const u32,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetwc(__stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn getwc(__stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn getwchar() -> wint_t;
}
extern "C" {
    pub fn fputwc(__wc: u32, __stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn putwc(__wc: u32, __stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn putwchar(__wc: u32) -> wint_t;
}
extern "C" {
    pub fn fgetws(__ws: *mut u32, __n: ::std::os::raw::c_int, __stream: *mut __FILE) -> *mut u32;
}
extern "C" {
    pub fn fputws(__ws: *const u32, __stream: *mut __FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ungetwc(__wc: wint_t, __stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn getwc_unlocked(__stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn getwchar_unlocked() -> wint_t;
}
extern "C" {
    pub fn fgetwc_unlocked(__stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn fputwc_unlocked(__wc: u32, __stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn putwc_unlocked(__wc: u32, __stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn putwchar_unlocked(__wc: u32) -> wint_t;
}
extern "C" {
    pub fn fgetws_unlocked(
        __ws: *mut u32,
        __n: ::std::os::raw::c_int,
        __stream: *mut __FILE,
    ) -> *mut u32;
}
extern "C" {
    pub fn fputws_unlocked(__ws: *const u32, __stream: *mut __FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsftime(
        __s: *mut u32,
        __maxsize: size_t,
        __format: *const u32,
        __tp: *const tm,
    ) -> size_t;
}
extern "C" {
    pub fn wcsftime_l(
        __s: *mut u32,
        __maxsize: size_t,
        __format: *const u32,
        __tp: *const tm,
        __loc: locale_t,
    ) -> size_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lconv {
    pub decimal_point: *mut ::std::os::raw::c_char,
    pub thousands_sep: *mut ::std::os::raw::c_char,
    pub grouping: *mut ::std::os::raw::c_char,
    pub int_curr_symbol: *mut ::std::os::raw::c_char,
    pub currency_symbol: *mut ::std::os::raw::c_char,
    pub mon_decimal_point: *mut ::std::os::raw::c_char,
    pub mon_thousands_sep: *mut ::std::os::raw::c_char,
    pub mon_grouping: *mut ::std::os::raw::c_char,
    pub positive_sign: *mut ::std::os::raw::c_char,
    pub negative_sign: *mut ::std::os::raw::c_char,
    pub int_frac_digits: ::std::os::raw::c_char,
    pub frac_digits: ::std::os::raw::c_char,
    pub p_cs_precedes: ::std::os::raw::c_char,
    pub p_sep_by_space: ::std::os::raw::c_char,
    pub n_cs_precedes: ::std::os::raw::c_char,
    pub n_sep_by_space: ::std::os::raw::c_char,
    pub p_sign_posn: ::std::os::raw::c_char,
    pub n_sign_posn: ::std::os::raw::c_char,
    pub int_p_cs_precedes: ::std::os::raw::c_char,
    pub int_p_sep_by_space: ::std::os::raw::c_char,
    pub int_n_cs_precedes: ::std::os::raw::c_char,
    pub int_n_sep_by_space: ::std::os::raw::c_char,
    pub int_p_sign_posn: ::std::os::raw::c_char,
    pub int_n_sign_posn: ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_lconv() {
    assert_eq!(
        std::mem::size_of::<lconv>(),
        96usize,
        concat!("Size of: ", stringify!(lconv))
    );
    assert_eq!(
        std::mem::align_of::<lconv>(),
        8usize,
        concat!("Alignment of ", stringify!(lconv))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).decimal_point as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(decimal_point)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).thousands_sep as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(thousands_sep)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).grouping as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(grouping)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).int_curr_symbol as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(int_curr_symbol)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).currency_symbol as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(currency_symbol)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).mon_decimal_point as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(mon_decimal_point)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).mon_thousands_sep as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(mon_thousands_sep)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).mon_grouping as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(mon_grouping)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).positive_sign as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(positive_sign)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).negative_sign as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(negative_sign)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).int_frac_digits as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(int_frac_digits)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).frac_digits as *const _ as usize },
        81usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(frac_digits)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).p_cs_precedes as *const _ as usize },
        82usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(p_cs_precedes)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).p_sep_by_space as *const _ as usize },
        83usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(p_sep_by_space)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).n_cs_precedes as *const _ as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(n_cs_precedes)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).n_sep_by_space as *const _ as usize },
        85usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(n_sep_by_space)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).p_sign_posn as *const _ as usize },
        86usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(p_sign_posn)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).n_sign_posn as *const _ as usize },
        87usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(n_sign_posn)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).int_p_cs_precedes as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(int_p_cs_precedes)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).int_p_sep_by_space as *const _ as usize },
        89usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(int_p_sep_by_space)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).int_n_cs_precedes as *const _ as usize },
        90usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(int_n_cs_precedes)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).int_n_sep_by_space as *const _ as usize },
        91usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(int_n_sep_by_space)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).int_p_sign_posn as *const _ as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(int_p_sign_posn)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lconv>())).int_n_sign_posn as *const _ as usize },
        93usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(int_n_sign_posn)
        )
    );
}
extern "C" {
    pub fn setlocale(
        __category: ::std::os::raw::c_int,
        __locale: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn localeconv() -> *mut lconv;
}
extern "C" {
    pub fn newlocale(
        __category_mask: ::std::os::raw::c_int,
        __locale: *const ::std::os::raw::c_char,
        __base: locale_t,
    ) -> locale_t;
}
extern "C" {
    pub fn duplocale(__dataset: locale_t) -> locale_t;
}
extern "C" {
    pub fn freelocale(__dataset: locale_t);
}
extern "C" {
    pub fn uselocale(__dataset: locale_t) -> locale_t;
}
pub const _ISupper: ::std::os::raw::c_uint = 256;
pub const _ISlower: ::std::os::raw::c_uint = 512;
pub const _ISalpha: ::std::os::raw::c_uint = 1024;
pub const _ISdigit: ::std::os::raw::c_uint = 2048;
pub const _ISxdigit: ::std::os::raw::c_uint = 4096;
pub const _ISspace: ::std::os::raw::c_uint = 8192;
pub const _ISprint: ::std::os::raw::c_uint = 16384;
pub const _ISgraph: ::std::os::raw::c_uint = 32768;
pub const _ISblank: ::std::os::raw::c_uint = 1;
pub const _IScntrl: ::std::os::raw::c_uint = 2;
pub const _ISpunct: ::std::os::raw::c_uint = 4;
pub const _ISalnum: ::std::os::raw::c_uint = 8;
pub type _bindgen_ty_35 = ::std::os::raw::c_uint;
extern "C" {
    pub fn __ctype_b_loc() -> *mut *const ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
extern "C" {
    pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
}
extern "C" {
    pub fn isalnum(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isalpha(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iscntrl(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isdigit(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn islower(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isgraph(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isprint(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ispunct(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isspace(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isupper(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isxdigit(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tolower(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn toupper(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isblank(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isctype(
        __c: ::std::os::raw::c_int,
        __mask: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isascii(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn toascii(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _toupper(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _tolower(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isalnum_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isalpha_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iscntrl_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isdigit_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn islower_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isgraph_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isprint_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ispunct_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isspace_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isupper_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isxdigit_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isblank_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __tolower_l(__c: ::std::os::raw::c_int, __l: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tolower_l(__c: ::std::os::raw::c_int, __l: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __toupper_l(__c: ::std::os::raw::c_int, __l: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn toupper_l(__c: ::std::os::raw::c_int, __l: locale_t) -> ::std::os::raw::c_int;
}
pub type time_t = __time_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[test]
fn bindgen_test_layout_timespec() {
    assert_eq!(
        std::mem::size_of::<timespec>(),
        16usize,
        concat!("Size of: ", stringify!(timespec))
    );
    assert_eq!(
        std::mem::align_of::<timespec>(),
        8usize,
        concat!("Alignment of ", stringify!(timespec))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timespec>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timespec>())).tv_nsec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_nsec)
        )
    );
}
pub type pid_t = __pid_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sched_param {
    pub sched_priority: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sched_param() {
    assert_eq!(
        std::mem::size_of::<sched_param>(),
        4usize,
        concat!("Size of: ", stringify!(sched_param))
    );
    assert_eq!(
        std::mem::align_of::<sched_param>(),
        4usize,
        concat!("Alignment of ", stringify!(sched_param))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<sched_param>())).sched_priority as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sched_param),
            "::",
            stringify!(sched_priority)
        )
    );
}
extern "C" {
    pub fn clone(
        __fn: std::option::Option<
            unsafe extern "C" fn(__arg: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
        >,
        __child_stack: *mut ::std::os::raw::c_void,
        __flags: ::std::os::raw::c_int,
        __arg: *mut ::std::os::raw::c_void,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unshare(__flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_getcpu() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getcpu(
        arg1: *mut ::std::os::raw::c_uint,
        arg2: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setns(
        __fd: ::std::os::raw::c_int,
        __nstype: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type __cpu_mask = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16usize],
}
#[test]
fn bindgen_test_layout_cpu_set_t() {
    assert_eq!(
        std::mem::size_of::<cpu_set_t>(),
        128usize,
        concat!("Size of: ", stringify!(cpu_set_t))
    );
    assert_eq!(
        std::mem::align_of::<cpu_set_t>(),
        8usize,
        concat!("Alignment of ", stringify!(cpu_set_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<cpu_set_t>())).__bits as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cpu_set_t),
            "::",
            stringify!(__bits)
        )
    );
}
extern "C" {
    pub fn __sched_cpucount(__setsize: size_t, __setp: *const cpu_set_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __sched_cpualloc(__count: size_t) -> *mut cpu_set_t;
}
extern "C" {
    pub fn __sched_cpufree(__set: *mut cpu_set_t);
}
extern "C" {
    pub fn sched_setparam(__pid: __pid_t, __param: *const sched_param) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_getparam(__pid: __pid_t, __param: *mut sched_param) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_setscheduler(
        __pid: __pid_t,
        __policy: ::std::os::raw::c_int,
        __param: *const sched_param,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_getscheduler(__pid: __pid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_yield() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_get_priority_max(__algorithm: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_get_priority_min(__algorithm: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_rr_get_interval(__pid: __pid_t, __t: *mut timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_setaffinity(
        __pid: __pid_t,
        __cpusetsize: size_t,
        __cpuset: *const cpu_set_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_getaffinity(
        __pid: __pid_t,
        __cpusetsize: size_t,
        __cpuset: *mut cpu_set_t,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[test]
fn bindgen_test_layout_timeval() {
    assert_eq!(
        std::mem::size_of::<timeval>(),
        16usize,
        concat!("Size of: ", stringify!(timeval))
    );
    assert_eq!(
        std::mem::align_of::<timeval>(),
        8usize,
        concat!("Alignment of ", stringify!(timeval))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timeval>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timeval>())).tv_usec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_usec)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timex {
    pub modes: ::std::os::raw::c_uint,
    pub offset: __syscall_slong_t,
    pub freq: __syscall_slong_t,
    pub maxerror: __syscall_slong_t,
    pub esterror: __syscall_slong_t,
    pub status: ::std::os::raw::c_int,
    pub constant: __syscall_slong_t,
    pub precision: __syscall_slong_t,
    pub tolerance: __syscall_slong_t,
    pub time: timeval,
    pub tick: __syscall_slong_t,
    pub ppsfreq: __syscall_slong_t,
    pub jitter: __syscall_slong_t,
    pub shift: ::std::os::raw::c_int,
    pub stabil: __syscall_slong_t,
    pub jitcnt: __syscall_slong_t,
    pub calcnt: __syscall_slong_t,
    pub errcnt: __syscall_slong_t,
    pub stbcnt: __syscall_slong_t,
    pub tai: ::std::os::raw::c_int,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 44usize]>,
}
#[test]
fn bindgen_test_layout_timex() {
    assert_eq!(
        std::mem::size_of::<timex>(),
        208usize,
        concat!("Size of: ", stringify!(timex))
    );
    assert_eq!(
        std::mem::align_of::<timex>(),
        8usize,
        concat!("Alignment of ", stringify!(timex))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).modes as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(modes)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).offset as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(offset)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).freq as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(freq)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).maxerror as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(maxerror)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).esterror as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(esterror)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).status as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).constant as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(constant)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).precision as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(precision)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).tolerance as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(tolerance)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).time as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).tick as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(tick)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).ppsfreq as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(ppsfreq)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).jitter as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(jitter)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).shift as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(shift)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).stabil as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(stabil)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).jitcnt as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(jitcnt)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).calcnt as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(calcnt)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).errcnt as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(errcnt)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).stbcnt as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(stbcnt)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<timex>())).tai as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(tai)
        )
    );
}
extern "C" {
    pub fn clock_adjtime(__clock_id: __clockid_t, __utx: *mut timex) -> ::std::os::raw::c_int;
}
pub type clock_t = __clock_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tm {
    pub tm_sec: ::std::os::raw::c_int,
    pub tm_min: ::std::os::raw::c_int,
    pub tm_hour: ::std::os::raw::c_int,
    pub tm_mday: ::std::os::raw::c_int,
    pub tm_mon: ::std::os::raw::c_int,
    pub tm_year: ::std::os::raw::c_int,
    pub tm_wday: ::std::os::raw::c_int,
    pub tm_yday: ::std::os::raw::c_int,
    pub tm_isdst: ::std::os::raw::c_int,
    pub tm_gmtoff: ::std::os::raw::c_long,
    pub tm_zone: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_tm() {
    assert_eq!(
        std::mem::size_of::<tm>(),
        56usize,
        concat!("Size of: ", stringify!(tm))
    );
    assert_eq!(
        std::mem::align_of::<tm>(),
        8usize,
        concat!("Alignment of ", stringify!(tm))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<tm>())).tm_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<tm>())).tm_min as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_min)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<tm>())).tm_hour as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_hour)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<tm>())).tm_mday as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_mday)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<tm>())).tm_mon as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_mon)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<tm>())).tm_year as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_year)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<tm>())).tm_wday as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_wday)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<tm>())).tm_yday as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_yday)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<tm>())).tm_isdst as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_isdst)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<tm>())).tm_gmtoff as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_gmtoff)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<tm>())).tm_zone as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_zone)
        )
    );
}
pub type clockid_t = __clockid_t;
pub type timer_t = __timer_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
#[test]
fn bindgen_test_layout_itimerspec() {
    assert_eq!(
        std::mem::size_of::<itimerspec>(),
        32usize,
        concat!("Size of: ", stringify!(itimerspec))
    );
    assert_eq!(
        std::mem::align_of::<itimerspec>(),
        8usize,
        concat!("Alignment of ", stringify!(itimerspec))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<itimerspec>())).it_interval as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(itimerspec),
            "::",
            stringify!(it_interval)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<itimerspec>())).it_value as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(itimerspec),
            "::",
            stringify!(it_value)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigevent {
    _unused: [u8; 0],
}
extern "C" {
    pub fn clock() -> clock_t;
}
extern "C" {
    pub fn time(__timer: *mut time_t) -> time_t;
}
extern "C" {
    pub fn difftime(__time1: time_t, __time0: time_t) -> f64;
}
extern "C" {
    pub fn mktime(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn strftime(
        __s: *mut ::std::os::raw::c_char,
        __maxsize: size_t,
        __format: *const ::std::os::raw::c_char,
        __tp: *const tm,
    ) -> size_t;
}
extern "C" {
    pub fn strptime(
        __s: *const ::std::os::raw::c_char,
        __fmt: *const ::std::os::raw::c_char,
        __tp: *mut tm,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strftime_l(
        __s: *mut ::std::os::raw::c_char,
        __maxsize: size_t,
        __format: *const ::std::os::raw::c_char,
        __tp: *const tm,
        __loc: locale_t,
    ) -> size_t;
}
extern "C" {
    pub fn strptime_l(
        __s: *const ::std::os::raw::c_char,
        __fmt: *const ::std::os::raw::c_char,
        __tp: *mut tm,
        __loc: locale_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gmtime(__timer: *const time_t) -> *mut tm;
}
extern "C" {
    pub fn localtime(__timer: *const time_t) -> *mut tm;
}
extern "C" {
    pub fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
extern "C" {
    pub fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
extern "C" {
    pub fn asctime(__tp: *const tm) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ctime(__timer: *const time_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn asctime_r(
        __tp: *const tm,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ctime_r(
        __timer: *const time_t,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub static mut __tzname: [*mut ::std::os::raw::c_char; 2usize];
}
extern "C" {
    pub static mut __daylight: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut __timezone: ::std::os::raw::c_long;
}
extern "C" {
    pub static mut tzname: [*mut ::std::os::raw::c_char; 2usize];
}
extern "C" {
    pub fn tzset();
}
extern "C" {
    pub static mut daylight: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut timezone: ::std::os::raw::c_long;
}
extern "C" {
    pub fn timegm(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn timelocal(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn dysize(__year: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_getres(__clock_id: clockid_t, __res: *mut timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_settime(__clock_id: clockid_t, __tp: *const timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_nanosleep(
        __clock_id: clockid_t,
        __flags: ::std::os::raw::c_int,
        __req: *const timespec,
        __rem: *mut timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_getcpuclockid(__pid: pid_t, __clock_id: *mut clockid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_create(
        __clock_id: clockid_t,
        __evp: *mut sigevent,
        __timerid: *mut timer_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_delete(__timerid: timer_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_settime(
        __timerid: timer_t,
        __flags: ::std::os::raw::c_int,
        __value: *const itimerspec,
        __ovalue: *mut itimerspec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_gettime(__timerid: timer_t, __value: *mut itimerspec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_getoverrun(__timerid: timer_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timespec_get(
        __ts: *mut timespec,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub static mut getdate_err: ::std::os::raw::c_int;
}
extern "C" {
    pub fn getdate(__string: *const ::std::os::raw::c_char) -> *mut tm;
}
extern "C" {
    pub fn getdate_r(
        __string: *const ::std::os::raw::c_char,
        __resbufp: *mut tm,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[test]
fn bindgen_test_layout___pthread_internal_list() {
    assert_eq!(
        std::mem::size_of::<__pthread_internal_list>(),
        16usize,
        concat!("Size of: ", stringify!(__pthread_internal_list))
    );
    assert_eq!(
        std::mem::align_of::<__pthread_internal_list>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_internal_list))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_internal_list>())).__prev as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_internal_list),
            "::",
            stringify!(__prev)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_internal_list>())).__next as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_internal_list),
            "::",
            stringify!(__next)
        )
    );
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_slist {
    pub __next: *mut __pthread_internal_slist,
}
#[test]
fn bindgen_test_layout___pthread_internal_slist() {
    assert_eq!(
        std::mem::size_of::<__pthread_internal_slist>(),
        8usize,
        concat!("Size of: ", stringify!(__pthread_internal_slist))
    );
    assert_eq!(
        std::mem::align_of::<__pthread_internal_slist>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_internal_slist))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_internal_slist>())).__next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_internal_slist),
            "::",
            stringify!(__next)
        )
    );
}
pub type __pthread_slist_t = __pthread_internal_slist;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: ::std::os::raw::c_int,
    pub __count: ::std::os::raw::c_uint,
    pub __owner: ::std::os::raw::c_int,
    pub __nusers: ::std::os::raw::c_uint,
    pub __kind: ::std::os::raw::c_int,
    pub __spins: ::std::os::raw::c_short,
    pub __elision: ::std::os::raw::c_short,
    pub __list: __pthread_list_t,
}
#[test]
fn bindgen_test_layout___pthread_mutex_s() {
    assert_eq!(
        std::mem::size_of::<__pthread_mutex_s>(),
        40usize,
        concat!("Size of: ", stringify!(__pthread_mutex_s))
    );
    assert_eq!(
        std::mem::align_of::<__pthread_mutex_s>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_mutex_s))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_mutex_s>())).__lock as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__lock)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_mutex_s>())).__count as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__count)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_mutex_s>())).__owner as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__owner)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_mutex_s>())).__nusers as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__nusers)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_mutex_s>())).__kind as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__kind)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_mutex_s>())).__spins as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__spins)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_mutex_s>())).__elision as *const _ as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__elision)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_mutex_s>())).__list as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__list)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::std::os::raw::c_uint,
    pub __writers: ::std::os::raw::c_uint,
    pub __wrphase_futex: ::std::os::raw::c_uint,
    pub __writers_futex: ::std::os::raw::c_uint,
    pub __pad3: ::std::os::raw::c_uint,
    pub __pad4: ::std::os::raw::c_uint,
    pub __cur_writer: ::std::os::raw::c_int,
    pub __shared: ::std::os::raw::c_int,
    pub __rwelision: ::std::os::raw::c_schar,
    pub __pad1: [::std::os::raw::c_uchar; 7usize],
    pub __pad2: ::std::os::raw::c_ulong,
    pub __flags: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_rwlock_arch_t() {
    assert_eq!(
        std::mem::size_of::<__pthread_rwlock_arch_t>(),
        56usize,
        concat!("Size of: ", stringify!(__pthread_rwlock_arch_t))
    );
    assert_eq!(
        std::mem::align_of::<__pthread_rwlock_arch_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_rwlock_arch_t))
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_rwlock_arch_t>())).__readers as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__readers)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_rwlock_arch_t>())).__writers as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__writers)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_rwlock_arch_t>())).__wrphase_futex as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__wrphase_futex)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_rwlock_arch_t>())).__writers_futex as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__writers_futex)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_rwlock_arch_t>())).__pad3 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad3)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_rwlock_arch_t>())).__pad4 as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_rwlock_arch_t>())).__cur_writer as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__cur_writer)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_rwlock_arch_t>())).__shared as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__shared)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_rwlock_arch_t>())).__rwelision as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__rwelision)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_rwlock_arch_t>())).__pad1 as *const _ as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad1)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_rwlock_arch_t>())).__pad2 as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad2)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_rwlock_arch_t>())).__flags as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__flags)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __bindgen_anon_1: __pthread_cond_s__bindgen_ty_1,
    pub __bindgen_anon_2: __pthread_cond_s__bindgen_ty_2,
    pub __g_refs: [::std::os::raw::c_uint; 2usize],
    pub __g_size: [::std::os::raw::c_uint; 2usize],
    pub __g1_orig_size: ::std::os::raw::c_uint,
    pub __wrefs: ::std::os::raw::c_uint,
    pub __g_signals: [::std::os::raw::c_uint; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_1 {
    pub __wseq: ::std::os::raw::c_ulonglong,
    pub __wseq32: __pthread_cond_s__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        std::mem::size_of::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        std::mem::align_of::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>())).__low
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(__low)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>())).__high
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(__high)
        )
    );
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_1() {
    assert_eq!(
        std::mem::size_of::<__pthread_cond_s__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(__pthread_cond_s__bindgen_ty_1))
    );
    assert_eq!(
        std::mem::align_of::<__pthread_cond_s__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_cond_s__bindgen_ty_1>())).__wseq as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1),
            "::",
            stringify!(__wseq)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_cond_s__bindgen_ty_1>())).__wseq32 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1),
            "::",
            stringify!(__wseq32)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_2 {
    pub __g1_start: ::std::os::raw::c_ulonglong,
    pub __g1_start32: __pthread_cond_s__bindgen_ty_2__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_2__bindgen_ty_1() {
    assert_eq!(
        std::mem::size_of::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        std::mem::align_of::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>())).__low
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(__low)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>())).__high
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(__high)
        )
    );
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_2() {
    assert_eq!(
        std::mem::size_of::<__pthread_cond_s__bindgen_ty_2>(),
        8usize,
        concat!("Size of: ", stringify!(__pthread_cond_s__bindgen_ty_2))
    );
    assert_eq!(
        std::mem::align_of::<__pthread_cond_s__bindgen_ty_2>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_cond_s__bindgen_ty_2>())).__g1_start as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2),
            "::",
            stringify!(__g1_start)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_cond_s__bindgen_ty_2>())).__g1_start32 as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2),
            "::",
            stringify!(__g1_start32)
        )
    );
}
#[test]
fn bindgen_test_layout___pthread_cond_s() {
    assert_eq!(
        std::mem::size_of::<__pthread_cond_s>(),
        48usize,
        concat!("Size of: ", stringify!(__pthread_cond_s))
    );
    assert_eq!(
        std::mem::align_of::<__pthread_cond_s>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_cond_s>())).__g_refs as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_refs)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_cond_s>())).__g_size as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_size)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_cond_s>())).__g1_orig_size as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g1_orig_size)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_cond_s>())).__wrefs as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__wrefs)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_cond_s>())).__g_signals as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_signals)
        )
    );
}
pub type pthread_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_pthread_mutexattr_t() {
    assert_eq!(
        std::mem::size_of::<pthread_mutexattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_mutexattr_t))
    );
    assert_eq!(
        std::mem::align_of::<pthread_mutexattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_mutexattr_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_mutexattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutexattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_mutexattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutexattr_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_pthread_condattr_t() {
    assert_eq!(
        std::mem::size_of::<pthread_condattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_condattr_t))
    );
    assert_eq!(
        std::mem::align_of::<pthread_condattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_condattr_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_condattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_condattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_condattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_condattr_t),
            "::",
            stringify!(__align)
        )
    );
}
pub type pthread_key_t = ::std::os::raw::c_uint;
pub type pthread_once_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_pthread_attr_t() {
    assert_eq!(
        std::mem::size_of::<pthread_attr_t>(),
        56usize,
        concat!("Size of: ", stringify!(pthread_attr_t))
    );
    assert_eq!(
        std::mem::align_of::<pthread_attr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_attr_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_attr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_attr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_attr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_attr_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::std::os::raw::c_char; 40usize],
    pub __align: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_pthread_mutex_t() {
    assert_eq!(
        std::mem::size_of::<pthread_mutex_t>(),
        40usize,
        concat!("Size of: ", stringify!(pthread_mutex_t))
    );
    assert_eq!(
        std::mem::align_of::<pthread_mutex_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_mutex_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_mutex_t>())).__data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_mutex_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_mutex_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::std::os::raw::c_char; 48usize],
    pub __align: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout_pthread_cond_t() {
    assert_eq!(
        std::mem::size_of::<pthread_cond_t>(),
        48usize,
        concat!("Size of: ", stringify!(pthread_cond_t))
    );
    assert_eq!(
        std::mem::align_of::<pthread_cond_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_cond_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_cond_t>())).__data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_cond_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_cond_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_pthread_rwlock_t() {
    assert_eq!(
        std::mem::size_of::<pthread_rwlock_t>(),
        56usize,
        concat!("Size of: ", stringify!(pthread_rwlock_t))
    );
    assert_eq!(
        std::mem::align_of::<pthread_rwlock_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_rwlock_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_rwlock_t>())).__data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_rwlock_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_rwlock_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [::std::os::raw::c_char; 8usize],
    pub __align: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_pthread_rwlockattr_t() {
    assert_eq!(
        std::mem::size_of::<pthread_rwlockattr_t>(),
        8usize,
        concat!("Size of: ", stringify!(pthread_rwlockattr_t))
    );
    assert_eq!(
        std::mem::align_of::<pthread_rwlockattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_rwlockattr_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_rwlockattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlockattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_rwlockattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlockattr_t),
            "::",
            stringify!(__align)
        )
    );
}
pub type pthread_spinlock_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [::std::os::raw::c_char; 32usize],
    pub __align: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_pthread_barrier_t() {
    assert_eq!(
        std::mem::size_of::<pthread_barrier_t>(),
        32usize,
        concat!("Size of: ", stringify!(pthread_barrier_t))
    );
    assert_eq!(
        std::mem::align_of::<pthread_barrier_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_barrier_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_barrier_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrier_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_barrier_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrier_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_pthread_barrierattr_t() {
    assert_eq!(
        std::mem::size_of::<pthread_barrierattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_barrierattr_t))
    );
    assert_eq!(
        std::mem::align_of::<pthread_barrierattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_barrierattr_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_barrierattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrierattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<pthread_barrierattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrierattr_t),
            "::",
            stringify!(__align)
        )
    );
}
pub type __jmp_buf = [::std::os::raw::c_long; 8usize];
pub const PTHREAD_CREATE_JOINABLE: ::std::os::raw::c_uint = 0;
pub const PTHREAD_CREATE_DETACHED: ::std::os::raw::c_uint = 1;
pub type _bindgen_ty_36 = ::std::os::raw::c_uint;
pub const PTHREAD_MUTEX_TIMED_NP: ::std::os::raw::c_uint = 0;
pub const PTHREAD_MUTEX_RECURSIVE_NP: ::std::os::raw::c_uint = 1;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: ::std::os::raw::c_uint = 2;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: ::std::os::raw::c_uint = 3;
pub const PTHREAD_MUTEX_NORMAL: ::std::os::raw::c_uint = 0;
pub const PTHREAD_MUTEX_RECURSIVE: ::std::os::raw::c_uint = 1;
pub const PTHREAD_MUTEX_ERRORCHECK: ::std::os::raw::c_uint = 2;
pub const PTHREAD_MUTEX_DEFAULT: ::std::os::raw::c_uint = 0;
pub const PTHREAD_MUTEX_FAST_NP: ::std::os::raw::c_uint = 0;
pub type _bindgen_ty_37 = ::std::os::raw::c_uint;
pub const PTHREAD_MUTEX_STALLED: ::std::os::raw::c_uint = 0;
pub const PTHREAD_MUTEX_STALLED_NP: ::std::os::raw::c_uint = 0;
pub const PTHREAD_MUTEX_ROBUST: ::std::os::raw::c_uint = 1;
pub const PTHREAD_MUTEX_ROBUST_NP: ::std::os::raw::c_uint = 1;
pub type _bindgen_ty_38 = ::std::os::raw::c_uint;
pub const PTHREAD_PRIO_NONE: ::std::os::raw::c_uint = 0;
pub const PTHREAD_PRIO_INHERIT: ::std::os::raw::c_uint = 1;
pub const PTHREAD_PRIO_PROTECT: ::std::os::raw::c_uint = 2;
pub type _bindgen_ty_39 = ::std::os::raw::c_uint;
pub const PTHREAD_RWLOCK_PREFER_READER_NP: ::std::os::raw::c_uint = 0;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NP: ::std::os::raw::c_uint = 1;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: ::std::os::raw::c_uint = 2;
pub const PTHREAD_RWLOCK_DEFAULT_NP: ::std::os::raw::c_uint = 0;
pub type _bindgen_ty_40 = ::std::os::raw::c_uint;
pub const PTHREAD_INHERIT_SCHED: ::std::os::raw::c_uint = 0;
pub const PTHREAD_EXPLICIT_SCHED: ::std::os::raw::c_uint = 1;
pub type _bindgen_ty_41 = ::std::os::raw::c_uint;
pub const PTHREAD_SCOPE_SYSTEM: ::std::os::raw::c_uint = 0;
pub const PTHREAD_SCOPE_PROCESS: ::std::os::raw::c_uint = 1;
pub type _bindgen_ty_42 = ::std::os::raw::c_uint;
pub const PTHREAD_PROCESS_PRIVATE: ::std::os::raw::c_uint = 0;
pub const PTHREAD_PROCESS_SHARED: ::std::os::raw::c_uint = 1;
pub type _bindgen_ty_43 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _pthread_cleanup_buffer {
    pub __routine: std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __arg: *mut ::std::os::raw::c_void,
    pub __canceltype: ::std::os::raw::c_int,
    pub __prev: *mut _pthread_cleanup_buffer,
}
#[test]
fn bindgen_test_layout__pthread_cleanup_buffer() {
    assert_eq!(
        std::mem::size_of::<_pthread_cleanup_buffer>(),
        32usize,
        concat!("Size of: ", stringify!(_pthread_cleanup_buffer))
    );
    assert_eq!(
        std::mem::align_of::<_pthread_cleanup_buffer>(),
        8usize,
        concat!("Alignment of ", stringify!(_pthread_cleanup_buffer))
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<_pthread_cleanup_buffer>())).__routine as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_pthread_cleanup_buffer),
            "::",
            stringify!(__routine)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_pthread_cleanup_buffer>())).__arg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_pthread_cleanup_buffer),
            "::",
            stringify!(__arg)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<_pthread_cleanup_buffer>())).__canceltype as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_pthread_cleanup_buffer),
            "::",
            stringify!(__canceltype)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_pthread_cleanup_buffer>())).__prev as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_pthread_cleanup_buffer),
            "::",
            stringify!(__prev)
        )
    );
}
pub const PTHREAD_CANCEL_ENABLE: ::std::os::raw::c_uint = 0;
pub const PTHREAD_CANCEL_DISABLE: ::std::os::raw::c_uint = 1;
pub type _bindgen_ty_44 = ::std::os::raw::c_uint;
pub const PTHREAD_CANCEL_DEFERRED: ::std::os::raw::c_uint = 0;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: ::std::os::raw::c_uint = 1;
pub type _bindgen_ty_45 = ::std::os::raw::c_uint;
extern "C" {
    pub fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
        >,
        __arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_exit(__retval: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_tryjoin_np(
        __th: pthread_t,
        __thread_return: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_timedjoin_np(
        __th: pthread_t,
        __thread_return: *mut *mut ::std::os::raw::c_void,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_clockjoin_np(
        __th: pthread_t,
        __thread_return: *mut *mut ::std::os::raw::c_void,
        __clockid: clockid_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_detach(__th: pthread_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_self() -> pthread_t;
}
extern "C" {
    pub fn pthread_equal(__thread1: pthread_t, __thread2: pthread_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getdetachstate(
        __attr: *const pthread_attr_t,
        __detachstate: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getguardsize(
        __attr: *const pthread_attr_t,
        __guardsize: *mut size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setguardsize(
        __attr: *mut pthread_attr_t,
        __guardsize: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getschedparam(
        __attr: *const pthread_attr_t,
        __param: *mut sched_param,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setschedparam(
        __attr: *mut pthread_attr_t,
        __param: *const sched_param,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getschedpolicy(
        __attr: *const pthread_attr_t,
        __policy: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setschedpolicy(
        __attr: *mut pthread_attr_t,
        __policy: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getinheritsched(
        __attr: *const pthread_attr_t,
        __inherit: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setinheritsched(
        __attr: *mut pthread_attr_t,
        __inherit: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getscope(
        __attr: *const pthread_attr_t,
        __scope: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setscope(
        __attr: *mut pthread_attr_t,
        __scope: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getstackaddr(
        __attr: *const pthread_attr_t,
        __stackaddr: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setstackaddr(
        __attr: *mut pthread_attr_t,
        __stackaddr: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getstacksize(
        __attr: *const pthread_attr_t,
        __stacksize: *mut size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getstack(
        __attr: *const pthread_attr_t,
        __stackaddr: *mut *mut ::std::os::raw::c_void,
        __stacksize: *mut size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setstack(
        __attr: *mut pthread_attr_t,
        __stackaddr: *mut ::std::os::raw::c_void,
        __stacksize: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setaffinity_np(
        __attr: *mut pthread_attr_t,
        __cpusetsize: size_t,
        __cpuset: *const cpu_set_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getaffinity_np(
        __attr: *const pthread_attr_t,
        __cpusetsize: size_t,
        __cpuset: *mut cpu_set_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_getattr_default_np(__attr: *mut pthread_attr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_setattr_default_np(__attr: *const pthread_attr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_getattr_np(
        __th: pthread_t,
        __attr: *mut pthread_attr_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_setschedparam(
        __target_thread: pthread_t,
        __policy: ::std::os::raw::c_int,
        __param: *const sched_param,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_getschedparam(
        __target_thread: pthread_t,
        __policy: *mut ::std::os::raw::c_int,
        __param: *mut sched_param,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_setschedprio(
        __target_thread: pthread_t,
        __prio: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_getname_np(
        __target_thread: pthread_t,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_setname_np(
        __target_thread: pthread_t,
        __name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_getconcurrency() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_setconcurrency(__level: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_yield() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_setaffinity_np(
        __th: pthread_t,
        __cpusetsize: size_t,
        __cpuset: *const cpu_set_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_getaffinity_np(
        __th: pthread_t,
        __cpusetsize: size_t,
        __cpuset: *mut cpu_set_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_setcancelstate(
        __state: ::std::os::raw::c_int,
        __oldstate: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_setcanceltype(
        __type: ::std::os::raw::c_int,
        __oldtype: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_cancel(__th: pthread_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_testcancel();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_unwind_buf_t {
    pub __cancel_jmp_buf: [__pthread_unwind_buf_t__bindgen_ty_1; 1usize],
    pub __pad: [*mut ::std::os::raw::c_void; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_unwind_buf_t__bindgen_ty_1 {
    pub __cancel_jmp_buf: __jmp_buf,
    pub __mask_was_saved: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___pthread_unwind_buf_t__bindgen_ty_1() {
    assert_eq!(
        std::mem::size_of::<__pthread_unwind_buf_t__bindgen_ty_1>(),
        72usize,
        concat!(
            "Size of: ",
            stringify!(__pthread_unwind_buf_t__bindgen_ty_1)
        )
    );
    assert_eq!(
        std::mem::align_of::<__pthread_unwind_buf_t__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(__pthread_unwind_buf_t__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_unwind_buf_t__bindgen_ty_1>())).__cancel_jmp_buf
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_unwind_buf_t__bindgen_ty_1),
            "::",
            stringify!(__cancel_jmp_buf)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_unwind_buf_t__bindgen_ty_1>())).__mask_was_saved
                as *const _ as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_unwind_buf_t__bindgen_ty_1),
            "::",
            stringify!(__mask_was_saved)
        )
    );
}
#[test]
fn bindgen_test_layout___pthread_unwind_buf_t() {
    assert_eq!(
        std::mem::size_of::<__pthread_unwind_buf_t>(),
        104usize,
        concat!("Size of: ", stringify!(__pthread_unwind_buf_t))
    );
    assert_eq!(
        std::mem::align_of::<__pthread_unwind_buf_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_unwind_buf_t))
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_unwind_buf_t>())).__cancel_jmp_buf as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_unwind_buf_t),
            "::",
            stringify!(__cancel_jmp_buf)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_unwind_buf_t>())).__pad as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_unwind_buf_t),
            "::",
            stringify!(__pad)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cleanup_frame {
    pub __cancel_routine:
        std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __cancel_arg: *mut ::std::os::raw::c_void,
    pub __do_it: ::std::os::raw::c_int,
    pub __cancel_type: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___pthread_cleanup_frame() {
    assert_eq!(
        std::mem::size_of::<__pthread_cleanup_frame>(),
        24usize,
        concat!("Size of: ", stringify!(__pthread_cleanup_frame))
    );
    assert_eq!(
        std::mem::align_of::<__pthread_cleanup_frame>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cleanup_frame))
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_cleanup_frame>())).__cancel_routine as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cleanup_frame),
            "::",
            stringify!(__cancel_routine)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_cleanup_frame>())).__cancel_arg as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cleanup_frame),
            "::",
            stringify!(__cancel_arg)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_cleanup_frame>())).__do_it as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cleanup_frame),
            "::",
            stringify!(__do_it)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_cleanup_frame>())).__cancel_type as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cleanup_frame),
            "::",
            stringify!(__cancel_type)
        )
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct __pthread_cleanup_class {
    pub __cancel_routine:
        std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __cancel_arg: *mut ::std::os::raw::c_void,
    pub __do_it: ::std::os::raw::c_int,
    pub __cancel_type: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___pthread_cleanup_class() {
    assert_eq!(
        std::mem::size_of::<__pthread_cleanup_class>(),
        24usize,
        concat!("Size of: ", stringify!(__pthread_cleanup_class))
    );
    assert_eq!(
        std::mem::align_of::<__pthread_cleanup_class>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cleanup_class))
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_cleanup_class>())).__cancel_routine as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cleanup_class),
            "::",
            stringify!(__cancel_routine)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_cleanup_class>())).__cancel_arg as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cleanup_class),
            "::",
            stringify!(__cancel_arg)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__pthread_cleanup_class>())).__do_it as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cleanup_class),
            "::",
            stringify!(__do_it)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<__pthread_cleanup_class>())).__cancel_type as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cleanup_class),
            "::",
            stringify!(__cancel_type)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __jmp_buf_tag {
    _unused: [u8; 0],
}
extern "C" {
    pub fn __sigsetjmp(
        __env: *mut __jmp_buf_tag,
        __savemask: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_timedlock(
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_clocklock(
        __mutex: *mut pthread_mutex_t,
        __clockid: clockid_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_getprioceiling(
        __mutex: *const pthread_mutex_t,
        __prioceiling: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_setprioceiling(
        __mutex: *mut pthread_mutex_t,
        __prioceiling: ::std::os::raw::c_int,
        __old_ceiling: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_consistent(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_consistent_np(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getpshared(
        __attr: *const pthread_mutexattr_t,
        __pshared: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setpshared(
        __attr: *mut pthread_mutexattr_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_gettype(
        __attr: *const pthread_mutexattr_t,
        __kind: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_settype(
        __attr: *mut pthread_mutexattr_t,
        __kind: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getprotocol(
        __attr: *const pthread_mutexattr_t,
        __protocol: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setprotocol(
        __attr: *mut pthread_mutexattr_t,
        __protocol: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getprioceiling(
        __attr: *const pthread_mutexattr_t,
        __prioceiling: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setprioceiling(
        __attr: *mut pthread_mutexattr_t,
        __prioceiling: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getrobust(
        __attr: *const pthread_mutexattr_t,
        __robustness: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getrobust_np(
        __attr: *const pthread_mutexattr_t,
        __robustness: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setrobust(
        __attr: *mut pthread_mutexattr_t,
        __robustness: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setrobust_np(
        __attr: *mut pthread_mutexattr_t,
        __robustness: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_destroy(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_tryrdlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_timedrdlock(
        __rwlock: *mut pthread_rwlock_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_clockrdlock(
        __rwlock: *mut pthread_rwlock_t,
        __clockid: clockid_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_trywrlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_timedwrlock(
        __rwlock: *mut pthread_rwlock_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_clockwrlock(
        __rwlock: *mut pthread_rwlock_t,
        __clockid: clockid_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_init(__attr: *mut pthread_rwlockattr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_destroy(__attr: *mut pthread_rwlockattr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_getpshared(
        __attr: *const pthread_rwlockattr_t,
        __pshared: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_setpshared(
        __attr: *mut pthread_rwlockattr_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_getkind_np(
        __attr: *const pthread_rwlockattr_t,
        __pref: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_setkind_np(
        __attr: *mut pthread_rwlockattr_t,
        __pref: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_cond_clockwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __clock_id: __clockid_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_condattr_init(__attr: *mut pthread_condattr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_condattr_destroy(__attr: *mut pthread_condattr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_condattr_getpshared(
        __attr: *const pthread_condattr_t,
        __pshared: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_condattr_setpshared(
        __attr: *mut pthread_condattr_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_condattr_getclock(
        __attr: *const pthread_condattr_t,
        __clock_id: *mut __clockid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_condattr_setclock(
        __attr: *mut pthread_condattr_t,
        __clock_id: __clockid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_spin_init(
        __lock: *mut pthread_spinlock_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_spin_destroy(__lock: *mut pthread_spinlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_spin_trylock(__lock: *mut pthread_spinlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_barrier_init(
        __barrier: *mut pthread_barrier_t,
        __attr: *const pthread_barrierattr_t,
        __count: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_barrier_destroy(__barrier: *mut pthread_barrier_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_barrier_wait(__barrier: *mut pthread_barrier_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_barrierattr_init(__attr: *mut pthread_barrierattr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_barrierattr_destroy(__attr: *mut pthread_barrierattr_t)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_barrierattr_getpshared(
        __attr: *const pthread_barrierattr_t,
        __pshared: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_barrierattr_setpshared(
        __attr: *mut pthread_barrierattr_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void),
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_key_delete(__key: pthread_key_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_getspecific(__key: pthread_key_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_getcpuclockid(
        __thread_id: pthread_t,
        __clock_id: *mut __clockid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_atfork(
        __prepare: std::option::Option<unsafe extern "C" fn()>,
        __parent: std::option::Option<unsafe extern "C" fn()>,
        __child: std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
pub type __gthread_t = pthread_t;
pub type __gthread_key_t = pthread_key_t;
pub type __gthread_once_t = pthread_once_t;
pub type __gthread_mutex_t = pthread_mutex_t;
pub type __gthread_recursive_mutex_t = pthread_mutex_t;
pub type __gthread_cond_t = pthread_cond_t;
pub type __gthread_time_t = timespec;
pub type _Atomic_word = ::std::os::raw::c_int;
pub const idtype_t_P_ALL: idtype_t = 0;
pub const idtype_t_P_PID: idtype_t = 1;
pub const idtype_t_P_PGID: idtype_t = 2;
pub type idtype_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct div_t {
    pub quot: ::std::os::raw::c_int,
    pub rem: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_div_t() {
    assert_eq!(
        std::mem::size_of::<div_t>(),
        8usize,
        concat!("Size of: ", stringify!(div_t))
    );
    assert_eq!(
        std::mem::align_of::<div_t>(),
        4usize,
        concat!("Alignment of ", stringify!(div_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<div_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(div_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<div_t>())).rem as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(div_t),
            "::",
            stringify!(rem)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_ldiv_t() {
    assert_eq!(
        std::mem::size_of::<ldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(ldiv_t))
    );
    assert_eq!(
        std::mem::align_of::<ldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ldiv_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ldiv_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ldiv_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<ldiv_t>())).rem as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ldiv_t),
            "::",
            stringify!(rem)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lldiv_t {
    pub quot: ::std::os::raw::c_longlong,
    pub rem: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout_lldiv_t() {
    assert_eq!(
        std::mem::size_of::<lldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(lldiv_t))
    );
    assert_eq!(
        std::mem::align_of::<lldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(lldiv_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lldiv_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(lldiv_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<lldiv_t>())).rem as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(lldiv_t),
            "::",
            stringify!(rem)
        )
    );
}
extern "C" {
    pub fn __ctype_get_mb_cur_max() -> size_t;
}
extern "C" {
    pub fn atof(__nptr: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn atoi(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atol(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn atoll(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtod(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtof(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f32;
}
extern "C" {
    pub fn strtold(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> u128;
}
extern "C" {
    pub fn strtof32(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> _Float32;
}
extern "C" {
    pub fn strtof64(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> _Float64;
}
extern "C" {
    pub fn strtof32x(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> _Float32x;
}
extern "C" {
    pub fn strtof64x(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> _Float64x;
}
extern "C" {
    pub fn strtol(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn strtoul(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtouq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn strtoll(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoull(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn strfromd(
        __dest: *mut ::std::os::raw::c_char,
        __size: size_t,
        __format: *const ::std::os::raw::c_char,
        __f: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfromf(
        __dest: *mut ::std::os::raw::c_char,
        __size: size_t,
        __format: *const ::std::os::raw::c_char,
        __f: f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfroml(
        __dest: *mut ::std::os::raw::c_char,
        __size: size_t,
        __format: *const ::std::os::raw::c_char,
        __f: u128,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfromf32(
        __dest: *mut ::std::os::raw::c_char,
        __size: size_t,
        __format: *const ::std::os::raw::c_char,
        __f: _Float32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfromf64(
        __dest: *mut ::std::os::raw::c_char,
        __size: size_t,
        __format: *const ::std::os::raw::c_char,
        __f: _Float64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfromf32x(
        __dest: *mut ::std::os::raw::c_char,
        __size: size_t,
        __format: *const ::std::os::raw::c_char,
        __f: _Float32x,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfromf64x(
        __dest: *mut ::std::os::raw::c_char,
        __size: size_t,
        __format: *const ::std::os::raw::c_char,
        __f: _Float64x,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strtol_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn strtoul_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoll_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoull_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn strtod_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> f64;
}
extern "C" {
    pub fn strtof_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> f32;
}
extern "C" {
    pub fn strtold_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> u128;
}
extern "C" {
    pub fn strtof32_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> _Float32;
}
extern "C" {
    pub fn strtof64_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> _Float64;
}
extern "C" {
    pub fn strtof32x_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> _Float32x;
}
extern "C" {
    pub fn strtof64x_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> _Float64x;
}
extern "C" {
    pub fn l64a(__n: ::std::os::raw::c_long) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn a64l(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type ino64_t = __ino64_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type off64_t = __off64_t;
pub type id_t = __id_t;
pub type ssize_t = __ssize_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type useconds_t = __useconds_t;
pub type suseconds_t = __suseconds_t;
pub type ulong = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint = ::std::os::raw::c_uint;
pub type u_int8_t = __uint8_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
pub type register_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___sigset_t() {
    assert_eq!(
        std::mem::size_of::<__sigset_t>(),
        128usize,
        concat!("Size of: ", stringify!(__sigset_t))
    );
    assert_eq!(
        std::mem::align_of::<__sigset_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__sigset_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__sigset_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigset_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type sigset_t = __sigset_t;
pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16usize],
}
#[test]
fn bindgen_test_layout_fd_set() {
    assert_eq!(
        std::mem::size_of::<fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(fd_set))
    );
    assert_eq!(
        std::mem::align_of::<fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(fd_set))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<fd_set>())).fds_bits as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fd_set),
            "::",
            stringify!(fds_bits)
        )
    );
}
pub type fd_mask = __fd_mask;
extern "C" {
    pub fn select(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pselect(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::std::os::raw::c_int;
}
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
pub type blkcnt64_t = __blkcnt64_t;
pub type fsblkcnt64_t = __fsblkcnt64_t;
pub type fsfilcnt64_t = __fsfilcnt64_t;
extern "C" {
    pub fn random() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srandom(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn initstate(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: size_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn setstate(__statebuf: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: ::std::os::raw::c_int,
    pub rand_deg: ::std::os::raw::c_int,
    pub rand_sep: ::std::os::raw::c_int,
    pub end_ptr: *mut i32,
}
#[test]
fn bindgen_test_layout_random_data() {
    assert_eq!(
        std::mem::size_of::<random_data>(),
        48usize,
        concat!("Size of: ", stringify!(random_data))
    );
    assert_eq!(
        std::mem::align_of::<random_data>(),
        8usize,
        concat!("Alignment of ", stringify!(random_data))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<random_data>())).fptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(fptr)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<random_data>())).rptr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rptr)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<random_data>())).state as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<random_data>())).rand_type as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rand_type)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<random_data>())).rand_deg as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rand_deg)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<random_data>())).rand_sep as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rand_sep)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<random_data>())).end_ptr as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(end_ptr)
        )
    );
}
extern "C" {
    pub fn random_r(__buf: *mut random_data, __result: *mut i32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srandom_r(
        __seed: ::std::os::raw::c_uint,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn initstate_r(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: size_t,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setstate_r(
        __statebuf: *mut ::std::os::raw::c_char,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rand_r(__seed: *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn drand48() -> f64;
}
extern "C" {
    pub fn erand48(__xsubi: *mut ::std::os::raw::c_ushort) -> f64;
}
extern "C" {
    pub fn lrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn nrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn mrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn jrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srand48(__seedval: ::std::os::raw::c_long);
}
extern "C" {
    pub fn seed48(__seed16v: *mut ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn lcong48(__param: *mut ::std::os::raw::c_ushort);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drand48_data {
    pub __x: [::std::os::raw::c_ushort; 3usize],
    pub __old_x: [::std::os::raw::c_ushort; 3usize],
    pub __c: ::std::os::raw::c_ushort,
    pub __init: ::std::os::raw::c_ushort,
    pub __a: ::std::os::raw::c_ulonglong,
}
#[test]
fn bindgen_test_layout_drand48_data() {
    assert_eq!(
        std::mem::size_of::<drand48_data>(),
        24usize,
        concat!("Size of: ", stringify!(drand48_data))
    );
    assert_eq!(
        std::mem::align_of::<drand48_data>(),
        8usize,
        concat!("Alignment of ", stringify!(drand48_data))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<drand48_data>())).__x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__x)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<drand48_data>())).__old_x as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__old_x)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<drand48_data>())).__c as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__c)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<drand48_data>())).__init as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__init)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<drand48_data>())).__a as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__a)
        )
    );
}
extern "C" {
    pub fn drand48_r(__buffer: *mut drand48_data, __result: *mut f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn erand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand48_r(
        __seedval: ::std::os::raw::c_long,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn seed48_r(
        __seed16v: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lcong48_r(
        __param: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn malloc(__size: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn realloc(
        __ptr: *mut ::std::os::raw::c_void,
        __size: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn reallocarray(
        __ptr: *mut ::std::os::raw::c_void,
        __nmemb: size_t,
        __size: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn free(__ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn alloca(__size: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn valloc(__size: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn posix_memalign(
        __memptr: *mut *mut ::std::os::raw::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aligned_alloc(__alignment: size_t, __size: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn atexit(__func: std::option::Option<unsafe extern "C" fn()>) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn at_quick_exit(
        __func: std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn on_exit(
        __func: std::option::Option<
            unsafe extern "C" fn(
                __status: ::std::os::raw::c_int,
                __arg: *mut ::std::os::raw::c_void,
            ),
        >,
        __arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn quick_exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _Exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn getenv(__name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn secure_getenv(__name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn putenv(__string: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setenv(
        __name: *const ::std::os::raw::c_char,
        __value: *const ::std::os::raw::c_char,
        __replace: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unsetenv(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearenv() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mktemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mkstemp(__template: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkstemp64(__template: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkstemps(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkstemps64(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkdtemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mkostemp(
        __template: *mut ::std::os::raw::c_char,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkostemp64(
        __template: *mut ::std::os::raw::c_char,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkostemps(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkostemps64(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn system(__command: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn canonicalize_file_name(
        __name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn realpath(
        __name: *const ::std::os::raw::c_char,
        __resolved: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
pub type __compar_fn_t = std::option::Option<
    unsafe extern "C" fn(
        arg1: *const ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type comparison_fn_t = __compar_fn_t;
pub type __compar_d_fn_t = std::option::Option<
    unsafe extern "C" fn(
        arg1: *const ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn bsearch(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn qsort(
        __base: *mut ::std::os::raw::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
extern "C" {
    pub fn qsort_r(
        __base: *mut ::std::os::raw::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_d_fn_t,
        __arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn abs(__x: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn labs(__x: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llabs(__x: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn div(__numer: ::std::os::raw::c_int, __denom: ::std::os::raw::c_int) -> div_t;
}
extern "C" {
    pub fn ldiv(__numer: ::std::os::raw::c_long, __denom: ::std::os::raw::c_long) -> ldiv_t;
}
extern "C" {
    pub fn lldiv(
        __numer: ::std::os::raw::c_longlong,
        __denom: ::std::os::raw::c_longlong,
    ) -> lldiv_t;
}
extern "C" {
    pub fn ecvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qecvt(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qfcvt(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qgcvt(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ecvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fcvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qecvt_r(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qfcvt_r(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mblen(__s: *const ::std::os::raw::c_char, __n: size_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbtowc(
        __pwc: *mut u32,
        __s: *const ::std::os::raw::c_char,
        __n: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wctomb(__s: *mut ::std::os::raw::c_char, __wchar: u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbstowcs(__pwcs: *mut u32, __s: *const ::std::os::raw::c_char, __n: size_t) -> size_t;
}
extern "C" {
    pub fn wcstombs(__s: *mut ::std::os::raw::c_char, __pwcs: *const u32, __n: size_t) -> size_t;
}
extern "C" {
    pub fn rpmatch(__response: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getsubopt(
        __optionp: *mut *mut ::std::os::raw::c_char,
        __tokens: *const *mut ::std::os::raw::c_char,
        __valuep: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn posix_openpt(__oflag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn grantpt(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unlockpt(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ptsname(__fd: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ptsname_r(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpt() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getloadavg(__loadavg: *mut f64, __nelem: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
#[test]
fn bindgen_test_layout__G_fpos_t() {
    assert_eq!(
        std::mem::size_of::<_G_fpos_t>(),
        16usize,
        concat!("Size of: ", stringify!(_G_fpos_t))
    );
    assert_eq!(
        std::mem::align_of::<_G_fpos_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_G_fpos_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_G_fpos_t>())).__pos as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_G_fpos_t),
            "::",
            stringify!(__pos)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_G_fpos_t>())).__state as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_G_fpos_t),
            "::",
            stringify!(__state)
        )
    );
}
pub type __fpos_t = _G_fpos_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos64_t {
    pub __pos: __off64_t,
    pub __state: __mbstate_t,
}
#[test]
fn bindgen_test_layout__G_fpos64_t() {
    assert_eq!(
        std::mem::size_of::<_G_fpos64_t>(),
        16usize,
        concat!("Size of: ", stringify!(_G_fpos64_t))
    );
    assert_eq!(
        std::mem::align_of::<_G_fpos64_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_G_fpos64_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_G_fpos64_t>())).__pos as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_G_fpos64_t),
            "::",
            stringify!(__pos)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_G_fpos64_t>())).__state as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_G_fpos64_t),
            "::",
            stringify!(__state)
        )
    );
}
pub type __fpos64_t = _G_fpos64_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: ::std::os::raw::c_int,
    pub _IO_read_ptr: *mut ::std::os::raw::c_char,
    pub _IO_read_end: *mut ::std::os::raw::c_char,
    pub _IO_read_base: *mut ::std::os::raw::c_char,
    pub _IO_write_base: *mut ::std::os::raw::c_char,
    pub _IO_write_ptr: *mut ::std::os::raw::c_char,
    pub _IO_write_end: *mut ::std::os::raw::c_char,
    pub _IO_buf_base: *mut ::std::os::raw::c_char,
    pub _IO_buf_end: *mut ::std::os::raw::c_char,
    pub _IO_save_base: *mut ::std::os::raw::c_char,
    pub _IO_backup_base: *mut ::std::os::raw::c_char,
    pub _IO_save_end: *mut ::std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::std::os::raw::c_int,
    pub _flags2: ::std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::std::os::raw::c_ushort,
    pub _vtable_offset: ::std::os::raw::c_schar,
    pub _shortbuf: [::std::os::raw::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::std::os::raw::c_void,
    pub __pad5: size_t,
    pub _mode: ::std::os::raw::c_int,
    pub _unused2: [::std::os::raw::c_char; 20usize],
}
#[test]
fn bindgen_test_layout__IO_FILE() {
    assert_eq!(
        std::mem::size_of::<_IO_FILE>(),
        216usize,
        concat!("Size of: ", stringify!(_IO_FILE))
    );
    assert_eq!(
        std::mem::align_of::<_IO_FILE>(),
        8usize,
        concat!("Alignment of ", stringify!(_IO_FILE))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._IO_read_ptr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._IO_read_end as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_end)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._IO_read_base as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_base)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._IO_write_base as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_base)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._IO_write_ptr as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._IO_write_end as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_end)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._IO_buf_base as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_buf_base)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._IO_buf_end as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_buf_end)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._IO_save_base as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_save_base)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._IO_backup_base as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_backup_base)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._IO_save_end as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_save_end)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._markers as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_markers)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._chain as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_chain)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._fileno as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_fileno)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._flags2 as *const _ as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_flags2)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._old_offset as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_old_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._cur_column as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_cur_column)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._vtable_offset as *const _ as usize },
        130usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_vtable_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._shortbuf as *const _ as usize },
        131usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_shortbuf)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._lock as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_lock)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._offset as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._codecvt as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_codecvt)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._wide_data as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_wide_data)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._freeres_list as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_freeres_list)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._freeres_buf as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_freeres_buf)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>())).__pad5 as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad5)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._mode as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_mode)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_FILE>()))._unused2 as *const _ as usize },
        196usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_unused2)
        )
    );
}
pub type cookie_read_function_t = std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __buf: *mut ::std::os::raw::c_char,
        __nbytes: size_t,
    ) -> __ssize_t,
>;
pub type cookie_write_function_t = std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __buf: *const ::std::os::raw::c_char,
        __nbytes: size_t,
    ) -> __ssize_t,
>;
pub type cookie_seek_function_t = std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __pos: *mut __off64_t,
        __w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type cookie_close_function_t = std::option::Option<
    unsafe extern "C" fn(__cookie: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_cookie_io_functions_t {
    pub read: cookie_read_function_t,
    pub write: cookie_write_function_t,
    pub seek: cookie_seek_function_t,
    pub close: cookie_close_function_t,
}
#[test]
fn bindgen_test_layout__IO_cookie_io_functions_t() {
    assert_eq!(
        std::mem::size_of::<_IO_cookie_io_functions_t>(),
        32usize,
        concat!("Size of: ", stringify!(_IO_cookie_io_functions_t))
    );
    assert_eq!(
        std::mem::align_of::<_IO_cookie_io_functions_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_IO_cookie_io_functions_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_cookie_io_functions_t>())).read as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_cookie_io_functions_t),
            "::",
            stringify!(read)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_cookie_io_functions_t>())).write as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_cookie_io_functions_t),
            "::",
            stringify!(write)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_cookie_io_functions_t>())).seek as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_cookie_io_functions_t),
            "::",
            stringify!(seek)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<_IO_cookie_io_functions_t>())).close as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_cookie_io_functions_t),
            "::",
            stringify!(close)
        )
    );
}
pub type cookie_io_functions_t = _IO_cookie_io_functions_t;
pub type fpos_t = __fpos_t;
pub type fpos64_t = __fpos64_t;
extern "C" {
    pub static mut stdin: *mut FILE;
}
extern "C" {
    pub static mut stdout: *mut FILE;
}
extern "C" {
    pub static mut stderr: *mut FILE;
}
extern "C" {
    pub fn remove(__filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rename(
        __old: *const ::std::os::raw::c_char,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn renameat(
        __oldfd: ::std::os::raw::c_int,
        __old: *const ::std::os::raw::c_char,
        __newfd: ::std::os::raw::c_int,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn renameat2(
        __oldfd: ::std::os::raw::c_int,
        __old: *const ::std::os::raw::c_char,
        __newfd: ::std::os::raw::c_int,
        __new: *const ::std::os::raw::c_char,
        __flags: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpfile64() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpnam_r(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tempnam(
        __dir: *const ::std::os::raw::c_char,
        __pfx: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fcloseall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn freopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fopen64(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn freopen64(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fdopen(__fd: ::std::os::raw::c_int, __modes: *const ::std::os::raw::c_char)
        -> *mut FILE;
}
extern "C" {
    pub fn fopencookie(
        __magic_cookie: *mut ::std::os::raw::c_void,
        __modes: *const ::std::os::raw::c_char,
        __io_funcs: cookie_io_functions_t,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fmemopen(
        __s: *mut ::std::os::raw::c_void,
        __len: size_t,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn open_memstream(
        __bufloc: *mut *mut ::std::os::raw::c_char,
        __sizeloc: *mut size_t,
    ) -> *mut FILE;
}
extern "C" {
    pub fn setbuf(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::std::os::raw::c_char,
        __modes: ::std::os::raw::c_int,
        __n: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuffer(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char, __size: size_t);
}
extern "C" {
    pub fn setlinebuf(__stream: *mut FILE);
}
extern "C" {
    pub fn fprintf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn printf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfprintf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vprintf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn snprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: size_t,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsnprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: size_t,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vasprintf(
        __ptr: *mut *mut ::std::os::raw::c_char,
        __f: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __asprintf(
        __ptr: *mut *mut ::std::os::raw::c_char,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn asprintf(
        __ptr: *mut *mut ::std::os::raw::c_char,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vdprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fscanf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scanf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_fscanf"]
    pub fn fscanf1(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_scanf"]
    pub fn scanf1(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_sscanf"]
    pub fn sscanf1(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfscanf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vscanf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vfscanf"]
    pub fn vfscanf1(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vscanf"]
    pub fn vscanf1(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vsscanf"]
    pub fn vsscanf1(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar_unlocked() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar_unlocked(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getw(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putw(__w: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgets(
        __s: *mut ::std::os::raw::c_char,
        __n: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fgets_unlocked(
        __s: *mut ::std::os::raw::c_char,
        __n: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut size_t,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut size_t,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getline(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn fputs(__s: *const ::std::os::raw::c_char, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn puts(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ungetc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread(
        __ptr: *mut ::std::os::raw::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
}
extern "C" {
    pub fn fwrite(
        __ptr: *const ::std::os::raw::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
}
extern "C" {
    pub fn fputs_unlocked(
        __s: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread_unlocked(
        __ptr: *mut ::std::os::raw::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
}
extern "C" {
    pub fn fwrite_unlocked(
        __ptr: *const ::std::os::raw::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
}
extern "C" {
    pub fn fseek(
        __stream: *mut FILE,
        __off: ::std::os::raw::c_long,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftell(__stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn rewind(__stream: *mut FILE);
}
extern "C" {
    pub fn fseeko(
        __stream: *mut FILE,
        __off: __off_t,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftello(__stream: *mut FILE) -> __off_t;
}
extern "C" {
    pub fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fseeko64(
        __stream: *mut FILE,
        __off: __off64_t,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftello64(__stream: *mut FILE) -> __off64_t;
}
extern "C" {
    pub fn fgetpos64(__stream: *mut FILE, __pos: *mut fpos64_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fsetpos64(__stream: *mut FILE, __pos: *const fpos64_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr(__stream: *mut FILE);
}
extern "C" {
    pub fn feof(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr_unlocked(__stream: *mut FILE);
}
extern "C" {
    pub fn feof_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn perror(__s: *const ::std::os::raw::c_char);
}
extern "C" {
    pub static mut sys_nerr: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut sys_errlist: [*const ::std::os::raw::c_char; 0usize];
}
extern "C" {
    pub static mut _sys_nerr: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut _sys_errlist: [*const ::std::os::raw::c_char; 0usize];
}
extern "C" {
    pub fn fileno(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fileno_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn popen(
        __command: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn pclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ctermid(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn cuserid(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct obstack {
    _unused: [u8; 0],
}
extern "C" {
    pub fn obstack_printf(
        __obstack: *mut obstack,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn obstack_vprintf(
        __obstack: *mut obstack,
        __format: *const ::std::os::raw::c_char,
        __args: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn flockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn ftrylockfile(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn funlockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn __uflow(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __overflow(arg1: *mut FILE, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __errno_location() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub static mut program_invocation_name: *mut ::std::os::raw::c_char;
}
extern "C" {
    pub static mut program_invocation_short_name: *mut ::std::os::raw::c_char;
}
pub type error_t = ::std::os::raw::c_int;
pub type qhyccd_handle = ::std::os::raw::c_void;
extern "C" {
    pub fn OutputQHYCCDDebug(strOutput: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn SetQHYCCDAutoDetectCamera(enable: bool);
}
extern "C" {
    pub fn SetQHYCCDLogLevel(logLevel: u8);
}
extern "C" {
    pub fn SetQHYCCDLogFunction(logFunction: std_function);
}
extern "C" {
    pub fn SetQHYCCDBufferNumber(BufNumber: u32);
}
extern "C" {
    pub fn EnableQHYCCDMessage(enable: bool);
}
extern "C" {
    pub fn EnableQHYCCDLogFile(enable: bool);
}
extern "C" {
    pub fn SetQHYCCDSingleFrameTimeOut(h: *mut qhyccd_handle, time: u32) -> u32;
}
extern "C" {
    pub fn GetTimeStamp() -> *const ::std::os::raw::c_char;
}
#[link(name = "qhyccd")]
#[link(name = "stdc++")]
#[link(name = "usb-1.0")]
extern "C" {
    #[doc = " \\fn uint32_t InitQHYCCDResource()"]
    #[doc = "\\brief initialize QHYCCD SDK resource"]
    #[doc = "\\return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "QHYCCD_ERROR_INITRESOURCE if the initialize failed \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn InitQHYCCDResource() -> u32;
}
extern "C" {
    #[doc = " \\fn uint32_t ReleaseQHYCCDResource()"]
    #[doc = "\\brief release QHYCCD SDK resource"]
    #[doc = "\\return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "QHYCCD_ERROR_RELEASERESOURCE if the release failed \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn ReleaseQHYCCDResource() -> u32;
}
extern "C" {
    #[doc = " \\fn uint32_t ScanQHYCCD()"]
    #[doc = "\\brief scan the connected cameras"]
    #[doc = "\\return"]
    #[doc = "on success,return the number of connected cameras \\n"]
    #[doc = "QHYCCD_ERROR_NO_DEVICE,if no camera connect to computer"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn ScanQHYCCD() -> u32;
}
extern "C" {
    #[doc = " \\fn uint32_t GetQHYCCDId(uint32_t index,char *id)"]
    #[doc = "\\brief get the id from camera"]
    #[doc = "\\param index sequence number of the connected cameras"]
    #[doc = "\\param id the id for camera,each camera has only id"]
    #[doc = "\\return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn GetQHYCCDId(index: u32, id: *mut ::std::os::raw::c_char) -> u32;
}
extern "C" {
    #[doc = " \\fn uint32_t GetQHYCCDModel(char *id, char *model)"]
    #[doc = "\\brief get camera model name by id"]
    #[doc = "\\param id the id of the camera"]
    #[doc = "\\param model the camera model"]
    #[doc = "\\return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code in failure"]
    pub fn GetQHYCCDModel(
        id: *mut ::std::os::raw::c_char,
        model: *mut ::std::os::raw::c_char,
    ) -> u32;
}
extern "C" {
    #[doc = " \\fn qhyccd_handle *OpenQHYCCD(char *id)"]
    #[doc = "\\brief open camera by camera id"]
    #[doc = "\\param id the id for camera,each camera has only id"]
    #[doc = "\\return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn OpenQHYCCD(id: *mut ::std::os::raw::c_char) -> *mut qhyccd_handle;
}
extern "C" {
    #[doc = " \\fn uint32_t CloseQHYCCD(qhyccd_handle *handle)"]
    #[doc = "\\brief close camera by handle"]
    #[doc = "\\param handle camera handle"]
    #[doc = "\\return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn CloseQHYCCD(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t SetQHYCCDStreamMode(qhyccd_handle *handle,uint8_t mode)"]
    #[doc = "@brief Set the camera's mode to chose the way reading data from camera"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param mode the stream mode \\n"]
    #[doc = "0x00:default mode,single frame mode \\n"]
    #[doc = "0x01:live mode \\n"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn SetQHYCCDStreamMode(handle: *mut qhyccd_handle, mode: u8) -> u32;
}
extern "C" {
    #[doc = " \\fn uint32_t InitQHYCCD(qhyccd_handle *handle)"]
    #[doc = "\\brief initialization specified camera by camera handle"]
    #[doc = "\\param handle camera control handle"]
    #[doc = "\\return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "on failed,return QHYCCD_ERROR_INITCAMERA \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn InitQHYCCD(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    #[doc = " @fn uint32_t IsQHYCCDControlAvailable(qhyccd_handle *handle,CONTROL_ID controlId)"]
    #[doc = "@brief check the camera has the queried function or not"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param controlId function type"]
    #[doc = "@return"]
    #[doc = "on have,return QHYCCD_SUCCESS \\n"]
    #[doc = "on do not have,return QHYCCD_ERROR_NOTSUPPORT \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn IsQHYCCDControlAvailable(handle: *mut qhyccd_handle, controlId: CONTROL_ID) -> u32;
}
extern "C" {
    #[doc = " \\fn uint32_t SetQHYCCDParam(qhyccd_handle *handle,CONTROL_ID controlId,double value)"]
    #[doc = "\\brief set params to camera"]
    #[doc = "\\param handle camera control handle"]
    #[doc = "\\param controlId function type"]
    #[doc = "\\param value value to camera"]
    #[doc = "\\return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "QHYCCD_ERROR_NOTSUPPORT,if the camera do not have the function \\n"]
    #[doc = "QHYCCD_ERROR_SETPARAMS,if set params to camera failed \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn SetQHYCCDParam(handle: *mut qhyccd_handle, controlId: CONTROL_ID, value: f64) -> u32;
}
extern "C" {
    #[doc = " \\fn double GetQHYCCDParam(qhyccd_handle *handle,CONTROL_ID controlId)"]
    #[doc = "\\brief get the params value from camera"]
    #[doc = "\\param handle camera control handle"]
    #[doc = "\\param controlId function type"]
    #[doc = "\\return"]
    #[doc = "on success,return the value\\n"]
    #[doc = "QHYCCD_ERROR_NOTSUPPORT,if the camera do not have the function \\n"]
    #[doc = "QHYCCD_ERROR_GETPARAMS,if get camera params'value failed \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn GetQHYCCDParam(handle: *mut qhyccd_handle, controlId: CONTROL_ID) -> f64;
}
extern "C" {
    #[doc = " \\fn uint32_t GetQHYCCDParamMinMaxStep(qhyccd_handle *handle,CONTROL_ID controlId,double *min,double *max,double *step)"]
    #[doc = "\\brief get the params value from camera"]
    #[doc = "\\param handle camera control handle"]
    #[doc = "\\param controlId function type"]
    #[doc = "\\param *min the pointer to the function's min value"]
    #[doc = "\\param *max the pointer to the function's max value"]
    #[doc = "\\param *step the pointer to the function's single step value"]
    #[doc = "\\return"]
    #[doc = "on success,return QHYCCD_SUCCESS\\n"]
    #[doc = "QHYCCD_ERROR_NOTSUPPORT,if the camera do not have the function \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn GetQHYCCDParamMinMaxStep(
        handle: *mut qhyccd_handle,
        controlId: CONTROL_ID,
        min: *mut f64,
        max: *mut f64,
        step: *mut f64,
    ) -> u32;
}
extern "C" {
    #[doc = " @fn uint32_t SetQHYCCDResolution(qhyccd_handle *handle,uint32_t x,uint32_t y,uint32_t xsize,uint32_t ysize)"]
    #[doc = "@brief set camera ouput resolution"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param x the top left position x"]
    #[doc = "@param y the top left position y"]
    #[doc = "@param xsize the image width"]
    #[doc = "@param ysize the image height"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS\\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn SetQHYCCDResolution(
        handle: *mut qhyccd_handle,
        x: u32,
        y: u32,
        xsize: u32,
        ysize: u32,
    ) -> u32;
}
extern "C" {
    #[doc = " \\fn uint32_t GetQHYCCDMemLength(qhyccd_handle *handle)"]
    #[doc = "\\brief get the minimum memory space for image data to save(byte)"]
    #[doc = "\\param handle camera control handle"]
    #[doc = "\\return"]
    #[doc = "on success,return the total memory space for image data(byte) \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn GetQHYCCDMemLength(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    #[doc = " \\fn uint32_t ExpQHYCCDSingleFrame(qhyccd_handle *handle)"]
    #[doc = "\\brief start to expose one frame"]
    #[doc = "\\param handle camera control handle"]
    #[doc = "\\return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "QHYCCD_ERROR_EXPOSING,if the camera is exposing \\n"]
    #[doc = "QHYCCD_ERROR_EXPFAILED,if start failed \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn ExpQHYCCDSingleFrame(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t GetQHYCCDSingleFrame(qhyccd_handle *handle,uint32_t *w,uint32_t *h,uint32_t *bpp,uint32_t *channels,uint8_t *imgdata)"]
    #[doc = "@brief get live frame data from camera"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param *w pointer to width of ouput image"]
    #[doc = "@param *h pointer to height of ouput image"]
    #[doc = "@param *bpp pointer to depth of ouput image"]
    #[doc = "@param *channels pointer to channels of ouput image"]
    #[doc = "@param *imgdata image data buffer"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "QHYCCD_ERROR_GETTINGFAILED,if get data failed \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn GetQHYCCDSingleFrame(
        handle: *mut qhyccd_handle,
        w: *mut u32,
        h: *mut u32,
        bpp: *mut u32,
        channels: *mut u32,
        imgdata: *mut u16,
    ) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t CancelQHYCCDExposing(qhyccd_handle *handle)"]
    #[doc = "@brief force stop the camera long exposure. But host software must readout the image data. Please note not all camera can use this method."]
    #[doc = "@param handle camera control handle"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn CancelQHYCCDExposing(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t CancelQHYCCDExposingAndReadout(qhyccd_handle *handle)"]
    #[doc = "@brief force stop the camera long exposure. And also camera does not send back the image data. Host software must not readout the data. All camera support this mode."]
    #[doc = "@param handle camera control handle"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn CancelQHYCCDExposingAndReadout(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t BeginQHYCCDLive(qhyccd_handle *handle)"]
    #[doc = "@brief in live video mode, start continue exposing. Only need to start once before StopQHYCCDLive."]
    #[doc = "@param handle camera control handle"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn BeginQHYCCDLive(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t GetQHYCCDLiveFrame(qhyccd_handle *handle,uint32_t *w,uint32_t *h,uint32_t *bpp,uint32_t *channels,uint8_t *imgdata)"]
    #[doc = "@brief get live frame data from camera"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param *w pointer to width of ouput image"]
    #[doc = "@param *h pointer to height of ouput image"]
    #[doc = "@param *bpp pointer to depth of ouput image"]
    #[doc = "@param *channels pointer to channels of ouput image"]
    #[doc = "@param *imgdata image data buffer"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "QHYCCD_ERROR_GETTINGFAILED,if get data failed \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn GetQHYCCDLiveFrame(
        handle: *mut qhyccd_handle,
        w: *mut u32,
        h: *mut u32,
        bpp: *mut u32,
        channels: *mut u32,
        imgdata: *mut u8,
    ) -> u32;
}
extern "C" {
    #[doc = " \\fn uint32_t StopQHYCCDLive(qhyccd_handle *handle)"]
    #[doc = "\\brief stop the camera continue exposing"]
    #[doc = "\\param handle camera control handle"]
    #[doc = "\\return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn StopQHYCCDLive(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    #[doc = " \\"]
    #[doc = "@fn uint32_t SetQHYCCDBinMode(qhyccd_handle *handle,uint32_t wbin,uint32_t hbin)"]
    #[doc = "@brief set camera's bin mode for ouput image data"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param wbin width bin mode"]
    #[doc = "@param hbin height bin mode"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn QHYCCDPcieRecv(
        handle: *mut qhyccd_handle,
        data: *mut ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        timeout: u64,
    ) -> u32;
}
extern "C" {
    pub fn GetQHYCCDPcieDDRNum(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    pub fn SetQHYCCDBinMode(handle: *mut qhyccd_handle, wbin: u32, hbin: u32) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t SetQHYCCDBitsMode(qhyccd_handle *handle,uint32_t bits)"]
    #[doc = "@brief set camera's depth bits for ouput image data"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param bits image depth"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn SetQHYCCDBitsMode(handle: *mut qhyccd_handle, bits: u32) -> u32;
}
extern "C" {
    #[doc = " \\fn uint32_t ControlQHYCCDTemp(qhyccd_handle *handle,double targettemp)"]
    #[doc = "\\brief This is a auto temprature control for QHYCCD cameras. \\n"]
    #[doc = "To control this,you need call this api every single second"]
    #[doc = "\\param handle camera control handle"]
    #[doc = "\\param targettemp the target control temprature"]
    #[doc = "\\return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn ControlQHYCCDTemp(handle: *mut qhyccd_handle, targettemp: f64) -> u32;
}
extern "C" {
    #[doc = " \\fn uint32_t ControlQHYCCDGuide(qhyccd_handle *handle,uint32_t direction,uint16_t duration)"]
    #[doc = "\\brief control the camera' guide port"]
    #[doc = "\\param handle camera control handle"]
    #[doc = "\\param direction direction \\n"]
    #[doc = "0: EAST RA+   \\n"]
    #[doc = "3: WEST RA-   \\n"]
    #[doc = "1: NORTH DEC+ \\n"]
    #[doc = "2: SOUTH DEC- \\n"]
    #[doc = "\\param duration duration of the direction"]
    #[doc = "\\return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn ControlQHYCCDGuide(handle: *mut qhyccd_handle, direction: u32, duration: u16) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t SendOrder2QHYCCDCFW(qhyccd_handle *handle,char *order,uint32_t length)"]
    #[doc = "@brief control color filter wheel port"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param order order send to color filter wheel"]
    #[doc = "@param length the order string length"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn SendOrder2QHYCCDCFW(
        handle: *mut qhyccd_handle,
        order: *mut ::std::os::raw::c_char,
        length: u32,
    ) -> u32;
}
extern "C" {
    #[doc = "@fn \tuint32_t GetQHYCCDCFWStatus(qhyccd_handle *handle,char *status)"]
    #[doc = "@brief control color filter wheel port"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param status the color filter wheel position status"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn GetQHYCCDCFWStatus(
        handle: *mut qhyccd_handle,
        status: *mut ::std::os::raw::c_char,
    ) -> u32;
}
extern "C" {
    #[doc = "@fn \tuint32_t IsQHYCCDCFWPlugged(qhyccd_handle *handle)"]
    #[doc = "@brief control color filter wheel port"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn IsQHYCCDCFWPlugged(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    pub fn GetQHYCCDTrigerModeNumber(handle: *mut qhyccd_handle, modeNumber: *mut u32) -> u32;
}
extern "C" {
    pub fn GetQHYCCDTrigerModeName(
        handle: *mut qhyccd_handle,
        modeNumber: u32,
        name: *mut ::std::os::raw::c_char,
    ) -> u32;
}
extern "C" {
    pub fn SetQHYCCDTrigerFunction(h: *mut qhyccd_handle, value: bool) -> u32;
}
extern "C" {
    #[doc = "\\fn   uint32_t SetQHYCCDTrigerMode(qhyccd_handle *handle,uint32_t trigerMode)"]
    #[doc = "\\brief set camera triger mode"]
    #[doc = "\\param handle camera control handle"]
    #[doc = "\\param trigerMode triger mode"]
    #[doc = "\\return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn SetQHYCCDTrigerMode(handle: *mut qhyccd_handle, trigerMode: u32) -> u32;
}
extern "C" {
    pub fn EnableQHYCCDTrigerOut(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    pub fn SendSoftTriger2QHYCCDCam(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    pub fn SetQHYCCDTrigerFilterOnOff(handle: *mut qhyccd_handle, onoff: bool) -> u32;
}
extern "C" {
    pub fn SetQHYCCDTrigerFilterTime(handle: *mut qhyccd_handle, time: u32) -> u32;
}
extern "C" {
    #[doc = " \\fn void Bits16ToBits8(qhyccd_handle *h,uint8_t *InputData16,uint8_t *OutputData8,uint32_t imageX,uint32_t imageY,uint16_t B,uint16_t W)"]
    #[doc = "\\brief turn 16bits data into 8bits"]
    #[doc = "\\param h camera control handle"]
    #[doc = "\\param InputData16 for 16bits data memory"]
    #[doc = "\\param OutputData8 for 8bits data memory"]
    #[doc = "\\param imageX image width"]
    #[doc = "\\param imageY image height"]
    #[doc = "\\param B for stretch balck"]
    #[doc = "\\param W for stretch white"]
    pub fn Bits16ToBits8(
        h: *mut qhyccd_handle,
        InputData16: *mut u8,
        OutputData8: *mut u8,
        imageX: u32,
        imageY: u32,
        B: u16,
        W: u16,
    );
}
extern "C" {
    #[doc = "@fn void HistInfo192x130(qhyccd_handle *h,uint32_t x,uint32_t y,uint8_t *InBuf,uint8_t *OutBuf)"]
    #[doc = "@brief make the hist info"]
    #[doc = "@param h camera control handle"]
    #[doc = "@param x image width"]
    #[doc = "@param y image height"]
    #[doc = "@param InBuf for the raw image data"]
    #[doc = "@param OutBuf for 192x130 8bits 3 channels image"]
    pub fn HistInfo192x130(h: *mut qhyccd_handle, x: u32, y: u32, InBuf: *mut u8, OutBuf: *mut u8);
}
extern "C" {
    #[doc = "@fn uint32_t OSXInitQHYCCDFirmware(char *path)"]
    #[doc = "@brief download the firmware to camera.(this api just need call in OSX system)"]
    #[doc = "@param path path to HEX file"]
    pub fn OSXInitQHYCCDFirmware(path: *mut ::std::os::raw::c_char) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t OSXInitQHYCCDFirmware(char *path)"]
    #[doc = "@brief download the firmware to camera.(this api just need call in OSX system)"]
    #[doc = "@param path path to HEX file"]
    pub fn OSXInitQHYCCDFirmwareArray() -> u32;
}
extern "C" {
    pub fn OSXInitQHYCCDAndroidFirmwareArray(
        idVendor: ::std::os::raw::c_int,
        idProduct: ::std::os::raw::c_int,
        handle: *mut qhyccd_handle,
    ) -> u32;
}
extern "C" {
    #[doc = " @fn uint32_t GetQHYCCDChipInfo(qhyccd_handle *h,double *chipw,double *chiph,uint32_t *imagew,uint32_t *imageh,double *pixelw,double *pixelh,uint32_t *bpp)"]
    #[doc = "@brief get the camera's ccd/cmos chip info"]
    #[doc = "@param h camera control handle"]
    #[doc = "@param chipw chip size width"]
    #[doc = "@param chiph chip size height"]
    #[doc = "@param imagew chip output image width"]
    #[doc = "@param imageh chip output image height"]
    #[doc = "@param pixelw chip pixel size width"]
    #[doc = "@param pixelh chip pixel size height"]
    #[doc = "@param bpp chip pixel depth"]
    pub fn GetQHYCCDChipInfo(
        h: *mut qhyccd_handle,
        chipw: *mut f64,
        chiph: *mut f64,
        imagew: *mut u32,
        imageh: *mut u32,
        pixelw: *mut f64,
        pixelh: *mut f64,
        bpp: *mut u32,
    ) -> u32;
}
extern "C" {
    #[doc = " @fn uint32_t GetQHYCCDEffectiveArea(qhyccd_handle *h,uint32_t *startX, uint32_t *startY, uint32_t *sizeX, uint32_t *sizeY)"]
    #[doc = "@brief get the camera's ccd/cmos chip info"]
    #[doc = "@param h camera control handle"]
    #[doc = "@param startX the Effective area x position"]
    #[doc = "@param startY the Effective area y position"]
    #[doc = "@param sizeX the Effective area x size"]
    #[doc = "@param sizeY the Effective area y size"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn GetQHYCCDEffectiveArea(
        h: *mut qhyccd_handle,
        startX: *mut u32,
        startY: *mut u32,
        sizeX: *mut u32,
        sizeY: *mut u32,
    ) -> u32;
}
extern "C" {
    #[doc = " @fn uint32_t GetQHYCCDOverScanArea(qhyccd_handle *h,uint32_t *startX, uint32_t *startY, uint32_t *sizeX, uint32_t *sizeY)"]
    #[doc = "@brief get the camera's ccd/cmos chip info"]
    #[doc = "@param h camera control handle"]
    #[doc = "@param startX the OverScan area x position"]
    #[doc = "@param startY the OverScan area y position"]
    #[doc = "@param sizeX the OverScan area x size"]
    #[doc = "@param sizeY the OverScan area y size"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn GetQHYCCDOverScanArea(
        h: *mut qhyccd_handle,
        startX: *mut u32,
        startY: *mut u32,
        sizeX: *mut u32,
        sizeY: *mut u32,
    ) -> u32;
}
extern "C" {
    pub fn GetQHYCCDCurrentROI(
        handle: *mut qhyccd_handle,
        startX: *mut u32,
        startY: *mut u32,
        sizeX: *mut u32,
        sizeY: *mut u32,
    ) -> u32;
}
extern "C" {
    #[doc = " @fn uint32_t SetQHYCCDFocusSetting(qhyccd_handle *h,uint32_t focusCenterX, uint32_t focusCenterY)"]
    #[doc = "@brief Set the camera on focus mode"]
    #[doc = "@param h camera control handle"]
    #[doc = "@param focusCenterX"]
    #[doc = "@param focusCenterY"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = ""]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn SetQHYCCDFocusSetting(
        h: *mut qhyccd_handle,
        focusCenterX: u32,
        focusCenterY: u32,
    ) -> u32;
}
extern "C" {
    #[doc = " @fn uint32_t GetQHYCCDExposureRemaining(qhyccd_handle *h)"]
    #[doc = "@brief Get remaining ccd/cmos expose time"]
    #[doc = "@param h camera control handle"]
    #[doc = "@return"]
    #[doc = "100 or less 100,it means exposoure is over \\n"]
    #[doc = "another is remaining time"]
    pub fn GetQHYCCDExposureRemaining(h: *mut qhyccd_handle) -> u32;
}
extern "C" {
    #[doc = " @fn uint32_t GetQHYCCDFWVersion(qhyccd_handle *h,uint8_t *buf)"]
    #[doc = "@brief Get the QHYCCD's firmware version"]
    #[doc = "@param h camera control handle"]
    #[doc = "@param buf buffer for version info"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = ""]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn GetQHYCCDFWVersion(h: *mut qhyccd_handle, buf: *mut u8) -> u32;
}
extern "C" {
    pub fn GetQHYCCDFPGAVersion(h: *mut qhyccd_handle, fpga_index: u8, buf: *mut u8) -> u32;
}
extern "C" {
    #[doc = " @fn uint32_t SetQHYCCDInterCamSerialParam(qhyccd_handle *h,uint32_t opt)"]
    #[doc = "@brief Set InterCam serial2 params"]
    #[doc = "@param h camera control handle"]
    #[doc = "@param opt the param \\n"]
    #[doc = "opt: \\n"]
    #[doc = "0x00 baud rate 9600bps  8N1 \\n"]
    #[doc = "0x01 baud rate 4800bps  8N1 \\n"]
    #[doc = "0x02 baud rate 19200bps 8N1 \\n"]
    #[doc = "0x03 baud rate 28800bps 8N1 \\n"]
    #[doc = "0x04 baud rate 57600bps 8N1"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = ""]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn SetQHYCCDInterCamSerialParam(h: *mut qhyccd_handle, opt: u32) -> u32;
}
extern "C" {
    #[doc = " @fn uint32_t QHYCCDInterCamSerialTX(qhyccd_handle *h,char *buf,uint32_t length)"]
    #[doc = "@brief Send data to InterCam serial2"]
    #[doc = "@param h camera control handle"]
    #[doc = "@param buf buffer for data"]
    #[doc = "@param length to send"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = ""]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn QHYCCDInterCamSerialTX(
        h: *mut qhyccd_handle,
        buf: *mut ::std::os::raw::c_char,
        length: u32,
    ) -> u32;
}
extern "C" {
    #[doc = " @fn uint32_t QHYCCDInterCamSerialRX(qhyccd_handle *h,char *buf)"]
    #[doc = "@brief Get data from InterCam serial2"]
    #[doc = "@param h camera control handle"]
    #[doc = "@param buf buffer for data"]
    #[doc = "@return"]
    #[doc = "on success,return the data number \\n"]
    #[doc = ""]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn QHYCCDInterCamSerialRX(h: *mut qhyccd_handle, buf: *mut ::std::os::raw::c_char) -> u32;
}
extern "C" {
    #[doc = " @fn uint32_t QHYCCDInterCamOledOnOff(qhyccd_handle *handle,uint8_t onoff)"]
    #[doc = "@brief turn off or turn on the InterCam's Oled"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param onoff on or off the oled \\n"]
    #[doc = "1:on \\n"]
    #[doc = "0:off \\n"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn QHYCCDInterCamOledOnOff(handle: *mut qhyccd_handle, onoff: u8) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t SetQHYCCDInterCamOledBrightness(qhyccd_handle *handle,uint8_t brightness)"]
    #[doc = "@brief send data to show on InterCam's OLED"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param brightness the oled's brightness"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn SetQHYCCDInterCamOledBrightness(handle: *mut qhyccd_handle, brightness: u8) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t SendFourLine2QHYCCDInterCamOled(qhyccd_handle *handle,char *messagetemp,char *messageinfo,char *messagetime,char *messagemode)"]
    #[doc = "@brief spilit the message to two line,send to camera"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param messagetemp message for the oled's 1st line"]
    #[doc = "@param messageinfo message for the oled's 2nd line"]
    #[doc = "@param messagetime message for the oled's 3rd line"]
    #[doc = "@param messagemode message for the oled's 4th line"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn SendFourLine2QHYCCDInterCamOled(
        handle: *mut qhyccd_handle,
        messagetemp: *mut ::std::os::raw::c_char,
        messageinfo: *mut ::std::os::raw::c_char,
        messagetime: *mut ::std::os::raw::c_char,
        messagemode: *mut ::std::os::raw::c_char,
    ) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t SendTwoLine2QHYCCDInterCamOled(qhyccd_handle *handle,char *messageTop,char *messageBottom)"]
    #[doc = "@brief spilit the message to two line,send to camera"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param messageTop message for the oled's 1st line"]
    #[doc = "@param messageBottom message for the oled's 2nd line"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn SendTwoLine2QHYCCDInterCamOled(
        handle: *mut qhyccd_handle,
        messageTop: *mut ::std::os::raw::c_char,
        messageBottom: *mut ::std::os::raw::c_char,
    ) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t SendOneLine2QHYCCDInterCamOled(qhyccd_handle *handle,char *messageTop)"]
    #[doc = "@brief spilit the message to two line,send to camera"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param messageTop message for all the oled"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn SendOneLine2QHYCCDInterCamOled(
        handle: *mut qhyccd_handle,
        messageTop: *mut ::std::os::raw::c_char,
    ) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t GetQHYCCDCameraStatus(qhyccd_handle *h,uint8_t *buf)"]
    #[doc = "@brief Get the camera statu"]
    #[doc = "@param h camera control handle"]
    #[doc = "@param buf camera's status save space"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn GetQHYCCDCameraStatus(h: *mut qhyccd_handle, buf: *mut u8) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t GetQHYCCDShutterStatus(qhyccd_handle *handle)"]
    #[doc = "@brief get the camera's shutter status"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@return"]
    #[doc = "on success,return status \\n"]
    #[doc = "0x00:shutter turn to right \\n"]
    #[doc = "0x01:shutter from right turn to middle \\n"]
    #[doc = "0x02:shutter from left turn to middle \\n"]
    #[doc = "0x03:shutter turn to left \\n"]
    #[doc = "0xff:IDLE \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn GetQHYCCDShutterStatus(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t ControlQHYCCDShutter(qhyccd_handle *handle,uint8_t status)"]
    #[doc = "@brief control camera's shutter"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param status the shutter status \\n"]
    #[doc = "0x00:shutter turn to right \\n"]
    #[doc = "0x01:shutter from right turn to middle \\n"]
    #[doc = "0x02:shutter from left turn to middle \\n"]
    #[doc = "0x03:shutter turn to left \\n"]
    #[doc = "0xff:IDLE"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn ControlQHYCCDShutter(handle: *mut qhyccd_handle, status: u8) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t GetQHYCCDHumidity(qhyccd_handle *handle,double *hd)"]
    #[doc = "@brief query cavity's humidity"]
    #[doc = "@param handle control handle"]
    #[doc = "@param hd the humidity value"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn GetQHYCCDPressure(handle: *mut qhyccd_handle, pressure: *mut f64) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t GetQHYCCDPressure(qhyccd_handle *handle,double *pressure)"]
    #[doc = "@get the pressure of sensor chamber"]
    #[doc = "@param handle control handle"]
    #[doc = "@param pressure : the sensor chamber pressure . unit is mbar  range 0.0-2000.0"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn GetQHYCCDHumidity(handle: *mut qhyccd_handle, hd: *mut f64) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t QHYCCDI2CTwoWrite(qhyccd_handle *handle,uint16_t addr,uint16_t value)"]
    #[doc = "@brief Set the value of the addr register in the camera."]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param addr the address of register"]
    #[doc = "@param value the value of the address"]
    #[doc = "@return"]
    #[doc = "on success,return QHYCCD_SUCCESS \\n"]
    #[doc = "another QHYCCD_ERROR code on other failures"]
    pub fn QHYCCDI2CTwoWrite(handle: *mut qhyccd_handle, addr: u16, value: u16) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t QHYCCDI2CTwoRead(qhyccd_handle *handle,uint16_t addr)"]
    #[doc = "@brief Get the value of the addr register in the camera."]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param addr the address of register"]
    #[doc = "@return value of the addr register"]
    pub fn QHYCCDI2CTwoRead(handle: *mut qhyccd_handle, addr: u16) -> u32;
}
extern "C" {
    #[doc = "@fn double GetQHYCCDReadingProgress(qhyccd_handle *handle)"]
    #[doc = "@brief get reading data from camera progress"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@return current progress"]
    pub fn GetQHYCCDReadingProgress(handle: *mut qhyccd_handle) -> f64;
}
extern "C" {
    #[doc = "test pid parameters"]
    pub fn TestQHYCCDPIDParas(h: *mut qhyccd_handle, p: f64, i: f64, d: f64) -> u32;
}
extern "C" {
    pub fn DownloadFX3FirmWare(vid: u16, pid: u16, imgpath: *mut ::std::os::raw::c_char) -> u32;
}
extern "C" {
    pub fn GetQHYCCDType(h: *mut qhyccd_handle) -> u32;
}
extern "C" {
    pub fn SetQHYCCDDebayerOnOff(h: *mut qhyccd_handle, onoff: bool) -> u32;
}
extern "C" {
    pub fn SetQHYCCDFineTone(
        h: *mut qhyccd_handle,
        setshporshd: u8,
        shdloc: u8,
        shploc: u8,
        shwidth: u8,
    ) -> u32;
}
extern "C" {
    pub fn SetQHYCCDGPSVCOXFreq(handle: *mut qhyccd_handle, i: u16) -> u32;
}
extern "C" {
    pub fn SetQHYCCDGPSLedCalMode(handle: *mut qhyccd_handle, i: u8) -> u32;
}
extern "C" {
    pub fn SetQHYCCDGPSLedCal(handle: *mut qhyccd_handle, pos: u32, width: u8);
}
extern "C" {
    pub fn SetQHYCCDGPSPOSA(handle: *mut qhyccd_handle, is_slave: u8, pos: u32, width: u8);
}
extern "C" {
    pub fn SetQHYCCDGPSPOSB(handle: *mut qhyccd_handle, is_slave: u8, pos: u32, width: u8);
}
extern "C" {
    pub fn SetQHYCCDGPSMasterSlave(handle: *mut qhyccd_handle, i: u8) -> u32;
}
extern "C" {
    pub fn SetQHYCCDGPSSlaveModeParameter(
        handle: *mut qhyccd_handle,
        target_sec: u32,
        target_us: u32,
        deltaT_sec: u32,
        deltaT_us: u32,
        expTime: u32,
    );
}
extern "C" {
    pub fn SetQHYCCDQuit();
}
extern "C" {
    pub fn QHYCCDVendRequestWrite(
        h: *mut qhyccd_handle,
        req: u8,
        value: u16,
        index1: u16,
        length: u32,
        data: *mut u8,
    ) -> u32;
}
extern "C" {
    pub fn QHYCCDReadUSB_SYNC(
        pDevHandle: *mut qhyccd_handle,
        endpoint: u8,
        length: u32,
        data: *mut u8,
        timeout: u32,
    ) -> u32;
}
extern "C" {
    pub fn QHYCCDLibusbBulkTransfer(
        pDevHandle: *mut qhyccd_handle,
        endpoint: u8,
        data: *mut u8,
        length: u32,
        transferred: *mut i32,
        timeout: u32,
    ) -> u32;
}
extern "C" {
    pub fn GetQHYCCDSDKVersion(
        year: *mut u32,
        month: *mut u32,
        day: *mut u32,
        subday: *mut u32,
    ) -> u32;
}
extern "C" {
    pub fn GetQHYCCDNumberOfReadModes(h: *mut qhyccd_handle, numModes: *mut u32) -> u32;
}
extern "C" {
    pub fn GetQHYCCDReadModeResolution(
        h: *mut qhyccd_handle,
        modeNumber: u32,
        width: *mut u32,
        height: *mut u32,
    ) -> u32;
}
extern "C" {
    pub fn GetQHYCCDReadModeName(
        h: *mut qhyccd_handle,
        modeNumber: u32,
        name: *mut ::std::os::raw::c_char,
    ) -> u32;
}
extern "C" {
    pub fn SetQHYCCDReadMode(h: *mut qhyccd_handle, modeNumber: u32) -> u32;
}
extern "C" {
    pub fn GetQHYCCDReadMode(h: *mut qhyccd_handle, modeNumber: *mut u32) -> u32;
}
extern "C" {
    pub fn GetQHYCCDBeforeOpenParam(p: *mut QHYCamMinMaxStepValue, controlId: CONTROL_ID) -> u32;
}
extern "C" {
    pub fn EnableQHYCCDBurstMode(h: *mut qhyccd_handle, i: bool) -> u32;
}
extern "C" {
    pub fn SetQHYCCDBurstModeStartEnd(
        h: *mut qhyccd_handle,
        start: ::std::os::raw::c_ushort,
        end: ::std::os::raw::c_ushort,
    ) -> u32;
}
extern "C" {
    pub fn EnableQHYCCDBurstCountFun(h: *mut qhyccd_handle, i: bool) -> u32;
}
extern "C" {
    pub fn ResetQHYCCDFrameCounter(h: *mut qhyccd_handle) -> u32;
}
extern "C" {
    pub fn SetQHYCCDBurstIDLE(h: *mut qhyccd_handle) -> u32;
}
extern "C" {
    pub fn ReleaseQHYCCDBurstIDLE(h: *mut qhyccd_handle) -> u32;
}
extern "C" {
    pub fn SetQHYCCDBurstModePatchNumber(h: *mut qhyccd_handle, value: u32) -> u32;
}
extern "C" {
    pub fn SetQHYCCDEnableLiveModeAntiRBI(h: *mut qhyccd_handle, value: u32) -> u32;
}
extern "C" {
    pub fn SetQHYCCDWriteFPGA(h: *mut qhyccd_handle, number: u8, regindex: u8, regvalue: u8)
        -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t SetQHYCCDWriteFPGA(qhyccd_handle *h,uint8_t number,uint8_t regindex,uint8_t regvalue);"]
    #[doc = "@brief Write FPGA register of the camera directly for advanced control"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param number:  if there is multiple FPGA, this is the sequence number . default is 0."]
    #[doc = "@param regindex:  register index. It is 8bit."]
    #[doc = "@param regindex:  register value. It is 8bit."]
    #[doc = "@return QHYCCD_SUCCESS or QHYCCD_ERROR. If it is QHYCCD_ERROR, it means (1) this model may have not support this function or (2) the API failur to run."]
    pub fn SetQHYCCDWriteCMOS(
        h: *mut qhyccd_handle,
        number: u8,
        regindex: u16,
        regvalue: u16,
    ) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t SetQHYCCDWriteCMOS(qhyccd_handle *h,uint8_t number,uint16_t regindex,uint16_t regvalue);"]
    #[doc = "@brief Write CMOS register of the camera directly for advanced control"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param number:  if there is multiple CMOS, this is the sequence number . default is 0."]
    #[doc = "@param regindex:  register index. It is 16bit."]
    #[doc = "@param regindex:  register value. It is 16bit."]
    #[doc = "@return QHYCCD_SUCCESS or QHYCCD_ERROR. If it is QHYCCD_ERROR, it means (1) this model may have not support this function or (2) the API failur to run."]
    pub fn SetQHYCCDTwoChannelCombineParameter(
        handle: *mut qhyccd_handle,
        x: f64,
        ah: f64,
        bh: f64,
        al: f64,
        bl: f64,
    ) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t SetQHYCCDTwoChannelCombineParameter(qhyccd_handle *handle, double x,double ah,double bh,double al,double bl);"]
    #[doc = "@brief For the camera with high gain low gain two channel combine to 16bit function, this API can set the combination parameters"]
    #[doc = "@param handle camera control handle"]
    #[doc = "@param x:  High gain low gain channel data switch point. (based on the high gain channel data)"]
    #[doc = "@param ah: High gain channel ratio   (y=ax+b)"]
    #[doc = "@param bh: High gain channel offset  (y=ax+b)"]
    #[doc = "@param al: Low gain channel ratio    (y=ax+b)"]
    #[doc = "@param bl: Low gain channel offset   (y=ax+b)"]
    #[doc = "@return QHYCCD_SUCCESS or QHYCCD_ERROR. If it is QHYCCD_ERROR, it means (1) this model may have not support this function or (2) the API failur to run."]
    pub fn EnableQHYCCDImageOSD(h: *mut qhyccd_handle, i: u32) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t GetQHYCCDPreciseExposureInfo(qhyccd_handle *h,"]
    #[doc = "uint32_t *PixelPeriod_ps,"]
    #[doc = "uint32_t *LinePeriod_ns,"]
    #[doc = "uint32_t *FramePeriod_us,"]
    #[doc = "uint32_t *ClocksPerLine,"]
    #[doc = "uint32_t *LinesPerFrame,"]
    #[doc = "uint32_t *ActualExposureTime,"]
    #[doc = "uint8_t  *isLongExposureMode);"]
    #[doc = "@brief get the sensor precise timing data from camera. These data can be used for high precise GPS time calculation"]
    #[doc = "@param h camera control handle"]
    #[doc = "@param PixelPeriod_ps return pixel period, unit is ps \\n"]
    #[doc = "@param LinePeriod_ns return row period, unit is ns \\n"]
    #[doc = "@param FramePeriod_us return frame period, unit is us \\n"]
    #[doc = "@param ClocksPerLine return how many clocks per line \\n"]
    #[doc = "@param LinesPerFrame return how many rows per frame. Please note this maybe not the picture y size. \\n"]
    #[doc = "@param ActualExposureTime return actual exposure time. most cmos exposure is row based. So the exposure time is n*row period. It maybe has a little difference with the set value. \\n"]
    #[doc = "@param isLongExposureMode return if camera works in long exposure mode. For cmos camera. When exposure time > frame period. It will add the verical blanking rows. in this case it is long exposure mode \\n"]
    #[doc = "@return QHYCCD_SUCCESS or QHYCCD_ERROR. If the camera does not support this function, it will return QHYCCD_ERROR \\n"]
    pub fn GetQHYCCDPreciseExposureInfo(
        h: *mut qhyccd_handle,
        PixelPeriod_ps: *mut u32,
        LinePeriod_ns: *mut u32,
        FramePeriod_us: *mut u32,
        ClocksPerLine: *mut u32,
        LinesPerFrame: *mut u32,
        ActualExposureTime: *mut u32,
        isLongExposureMode: *mut u8,
    ) -> u32;
}
extern "C" {
    #[doc = "@fn uint32_t GetQHYCCDRollingShutterEndOffset(qhyccd_handle *h,uint32_t row,uint32_t *offset);"]
    #[doc = "@brief for rolling shutter camera with GPS meassurement signal output or with GPSBOX connection. it will output the meassurement pulse. But the pulse is not at the exactly time of the \\n"]
    #[doc = "the exposure time. This api will return the calibrated data of the offset value from GPS meassurement pulse to the end of exposure time of certain row."]
    #[doc = "@param h camera control handle \\n"]
    #[doc = "@param row the shutter status \\n"]
    #[doc = "@param offset the shutter offset value from shutter meassure signal falling edge (gps messured end exposure) . unit us \\n"]
    #[doc = "@return QHYCCD_SUCCESS or QHYCCD_ERROR. If the camera does not support this function, it will return QHYCCD_ERROR \\n"]
    pub fn GetQHYCCDRollingShutterEndOffset(
        h: *mut qhyccd_handle,
        row: u32,
        offset: *mut f64,
    ) -> u32;
}
extern "C" {
    pub fn QHYCCDQuit();
}
extern "C" {
    pub fn SetQHYCCDCallBack(ProcCallBack: QHYCCDProcCallBack, Flag: i32) -> QHYDWORD;
}
extern "C" {
    pub fn RegisterPnpEventIn(
        in_pnp_event_in_func: std::option::Option<
            unsafe extern "C" fn(id: *mut ::std::os::raw::c_char),
        >,
    );
}
extern "C" {
    pub fn RegisterPnpEventOut(
        in_pnp_event_out_func: std::option::Option<
            unsafe extern "C" fn(id: *mut ::std::os::raw::c_char),
        >,
    );
}
extern "C" {
    pub fn resetDev(
        deviceID: *mut ::std::os::raw::c_char,
        readModeIndex: u32,
        streamMode: u8,
        devHandle: *mut qhyccd_handle,
        imageWidth: *mut u32,
        imageHigh: *mut u32,
        bitDepth: u32,
    ) -> u32;
}
extern "C" {
    pub fn RegisterDataEventSingle(
        in_data_event_single_func: std::option::Option<
            unsafe extern "C" fn(id: *mut ::std::os::raw::c_char, imgdata: *mut u8),
        >,
    );
}
extern "C" {
    pub fn RegisterDataEventLive(
        in_data_event_live_func: std::option::Option<
            unsafe extern "C" fn(id: *mut ::std::os::raw::c_char, imgdata: *mut u8),
        >,
    );
}
extern "C" {
    pub fn RegisterTransferEventError(
        transfer_event_error_func: std::option::Option<unsafe extern "C" fn()>,
    );
}
extern "C" {
    pub fn GetReadModesNumber(deviceID: *mut ::std::os::raw::c_char, numModes: *mut u32) -> u32;
}
extern "C" {
    pub fn GetReadModeName(
        deviceID: *mut ::std::os::raw::c_char,
        modeIndex: u32,
        modeName: *mut ::std::os::raw::c_char,
    ) -> u32;
}
extern "C" {
    pub fn QHYCCDSensorPhaseReTrain(handle: *mut qhyccd_handle);
}
extern "C" {
    pub fn QHYCCDReadInitConfigFlash(
        handle: *mut qhyccd_handle,
        configString_raw64: *mut ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn QHYCCDEraseInitConfigFlash(handle: *mut qhyccd_handle);
}
extern "C" {
    pub fn QHYCCDResetFlashULVOError(handle: *mut qhyccd_handle);
}
extern "C" {
    pub fn QHYCCDTestFlashULVOError(handle: *mut qhyccd_handle);
}
extern "C" {
    pub fn QHYCCDSetFlashInitPWM(handle: *mut qhyccd_handle, pwm: u8);
}
extern "C" {
    pub fn QHYCCDGetDebugDataD3(
        handle: *mut qhyccd_handle,
        debugData_raw64: *mut ::std::os::raw::c_char,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpga_info_list {
    _unused: [u8; 0],
}
extern "C" {
    pub fn QHYCCD_fpga_list(list: *mut fpga_info_list) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn QHYCCD_fpga_open(id: ::std::os::raw::c_int) -> u32;
}
extern "C" {
    pub fn QHYCCD_fpga_close();
}
extern "C" {
    pub fn QHYCCD_fpga_send(
        chnl: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        destoff: ::std::os::raw::c_int,
        last: ::std::os::raw::c_int,
        timeout: u64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn QHYCCD_fpga_recv(
        chnl: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        timeout: u64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn QHYCCD_fpga_reset();
}
extern "C" {
    #[doc = " ----------------------------------"]
    #[link_name = "\u{1}_Z14call_pnp_eventv"]
    pub fn call_pnp_event();
}
extern "C" {
    #[link_name = "\u{1}_Z20call_data_event_livePcPh"]
    pub fn call_data_event_live(id: *mut ::std::os::raw::c_char, imgdata: *mut u8);
}
extern "C" {
    #[link_name = "\u{1}_Z25call_transfer_event_errorv"]
    pub fn call_transfer_event_error();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_1 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_2 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_3 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_4 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_5 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_6 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_7 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_8 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_9 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_10 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_11 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_12 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_13 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_14 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_15 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_16 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_17 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_18 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_19 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_20 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_21 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_22 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_23 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_24 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_25 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_26 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_27 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_28 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_29 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_30 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_31 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_32 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_33 {
    pub _address: u8,
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_char_std_char_traits_open1_char_close1_std_allocator_open1_char_close1_close0_instantiation_1(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<::std::os::raw::c_char>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<::std::os::raw::c_char>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<::std::os::raw::c_char>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<::std::os::raw::c_char>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_2() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_char_close0_instantiation_1() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_wchar_t_std_char_traits_open1_wchar_t_close1_std_allocator_open1_wchar_t_close1_close0_instantiation_1(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<u32>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<u32>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<u32>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<u32>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_2() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_wchar_t_close0_instantiation_1() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_char16_t_std_char_traits_open1_char16_t_close1_std_allocator_open1_char16_t_close1_close0_instantiation(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<u16>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<u16>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<u16>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<u16>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char16_t_close0_instantiation_2() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_char16_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_char32_t_std_char_traits_open1_char32_t_close1_std_allocator_open1_char32_t_close1_close0_instantiation(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<u32>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<u32>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<u32>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<u32>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char32_t_close0_instantiation_2() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_char32_t_close0_instantiation() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_34 {
    pub _address: u8,
}
#[test]
fn __bindgen_test_layout_std_iterator_open0_std_output_iterator_tag_void_void_void_void_close0_instantiation(
) {
    assert_eq!(
        std::mem::size_of::<std_iterator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_iterator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_iterator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_iterator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_iterator_open0_std_output_iterator_tag_void_void_void_void_close0_instantiation_1(
) {
    assert_eq!(
        std::mem::size_of::<std_iterator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_iterator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_iterator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_iterator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_iterator_open0_std_output_iterator_tag_void_void_void_void_close0_instantiation_2(
) {
    assert_eq!(
        std::mem::size_of::<std_iterator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_iterator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_iterator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_iterator)
        )
    );
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    assert_eq!(
        std::mem::size_of::<__va_list_tag>(),
        24usize,
        concat!("Size of: ", stringify!(__va_list_tag))
    );
    assert_eq!(
        std::mem::align_of::<__va_list_tag>(),
        8usize,
        concat!("Alignment of ", stringify!(__va_list_tag))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__va_list_tag>())).gp_offset as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(gp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__va_list_tag>())).fp_offset as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(fp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__va_list_tag>())).overflow_arg_area as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(overflow_arg_area)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__va_list_tag>())).reg_save_area as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(reg_save_area)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_data {
    pub _address: u8,
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_3() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_4() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_5() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_6() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_7() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_8() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_char_close0_instantiation_2() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_9() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_char_close0_instantiation_3() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_10() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_char_close0_instantiation_4() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_11() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_char_close0_instantiation_5() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_12() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_13() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_14() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_15() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_3() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_4() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_5() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_6() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_7() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_8() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_wchar_t_close0_instantiation_2() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_9() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_wchar_t_close0_instantiation_3() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_10() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_wchar_t_close0_instantiation_4() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_11() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_wchar_t_close0_instantiation_5() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_12() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_13() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_14() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_15() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_16() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_char_std_char_traits_open1_char_close1_std_allocator_open1_char_close1_close0_instantiation_2(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<::std::os::raw::c_char>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<::std::os::raw::c_char>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<::std::os::raw::c_char>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<::std::os::raw::c_char>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_17() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_char_close0_instantiation_6() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_18() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_16() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_wchar_t_std_char_traits_open1_wchar_t_close1_std_allocator_open1_wchar_t_close1_close0_instantiation_2(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<u32>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<u32>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<u32>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<u32>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_17() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_wchar_t_close0_instantiation_6() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_18() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_long_double_close0_instantiation_1() {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_char_std_char_traits_open1_char_close1_std_allocator_open1_char_close1_close0_instantiation_3(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<::std::os::raw::c_char>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<::std::os::raw::c_char>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<::std::os::raw::c_char>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<::std::os::raw::c_char>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_19() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_char_close0_instantiation_7() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_std_basic_string_open1_char_std_char_traits_open2_char_close2_std_allocator_open2_char_close2_close1_close0_instantiation_1(
) {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_char_std_char_traits_open1_char_close1_std_allocator_open1_char_close1_close0_instantiation_4(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<::std::os::raw::c_char>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<::std::os::raw::c_char>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<::std::os::raw::c_char>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<::std::os::raw::c_char>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_20() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_char_close0_instantiation_8() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_wchar_t_std_char_traits_open1_wchar_t_close1_std_allocator_open1_wchar_t_close1_close0_instantiation_3(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<u32>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<u32>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<u32>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<u32>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_19() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_wchar_t_close0_instantiation_7() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_std_basic_string_open1_wchar_t_std_char_traits_open2_wchar_t_close2_std_allocator_open2_wchar_t_close2_close1_close0_instantiation_1(
) {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_wchar_t_std_char_traits_open1_wchar_t_close1_std_allocator_open1_wchar_t_close1_close0_instantiation_4(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<u32>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<u32>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<u32>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<u32>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_20() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_wchar_t_close0_instantiation_8() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_char16_t_std_char_traits_open1_char16_t_close1_std_allocator_open1_char16_t_close1_close0_instantiation_1(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<u16>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<u16>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<u16>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<u16>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char16_t_close0_instantiation_3() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_char16_t_close0_instantiation_1() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_std_basic_string_open1_char16_t_std_char_traits_open2_char16_t_close2_std_allocator_open2_char16_t_close2_close1_close0_instantiation_1(
) {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_char16_t_std_char_traits_open1_char16_t_close1_std_allocator_open1_char16_t_close1_close0_instantiation_2(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<u16>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<u16>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<u16>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<u16>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char16_t_close0_instantiation_4() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_char16_t_close0_instantiation_2() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_char32_t_std_char_traits_open1_char32_t_close1_std_allocator_open1_char32_t_close1_close0_instantiation_1(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<u32>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<u32>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<u32>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<u32>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char32_t_close0_instantiation_3() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_char32_t_close0_instantiation_1() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_hash_open0_std_basic_string_open1_char32_t_std_char_traits_open2_char32_t_close2_std_allocator_open2_char32_t_close2_close1_close0_instantiation_1(
) {
    assert_eq!(
        std::mem::size_of::<std_hash>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(std_hash))
    );
    assert_eq!(
        std::mem::align_of::<std_hash>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_hash)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_char32_t_std_char_traits_open1_char32_t_close1_std_allocator_open1_char32_t_close1_close0_instantiation_2(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<u32>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<u32>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<u32>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<u32>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char32_t_close0_instantiation_4() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_char32_t_close0_instantiation_2() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_21() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_char_close0_instantiation_9() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_wchar_t_close0_instantiation_21() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_wchar_t_close0_instantiation_9() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_basic_string_open0_char_std_char_traits_open1_char_close1_std_allocator_open1_char_close1_close0_instantiation_5(
) {
    assert_eq!(
        std::mem::size_of::<std_basic_string<::std::os::raw::c_char>>(),
        32usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_basic_string<::std::os::raw::c_char>)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_basic_string<::std::os::raw::c_char>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_basic_string<::std::os::raw::c_char>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_char_traits_open0_char_close0_instantiation_22() {
    assert_eq!(
        std::mem::size_of::<std_char_traits>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_char_traits)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_char_traits>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_char_traits)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_char_close0_instantiation_10() {
    assert_eq!(
        std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_46 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_47 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_48 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_49 {
    pub _address: u8,
}
